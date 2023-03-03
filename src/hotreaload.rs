use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
use std::fmt::{format, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::{Cursor, Seek};
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::{io, slice};

use crate::write::{make_density_function, math_into_glsl, GlslCompiler};
use graph::device::reflection::ReflectedLayout;
use graph::device::{self, read_spirv, DeviceCreateInfo, QueueFamilySelection};
use graph::graph::compile::GraphCompiler;
use graph::instance::{Instance, InstanceCreateInfo, OwnedInstance};
use graph::object::{self, ImageCreateInfo, PipelineStage, SwapchainCreateInfo};
use graph::passes::{self, ClearImage, SimpleShader};
use graph::smallvec::{smallvec, SmallVec};
use graph::tracing::tracing_subscriber::install_tracing_subscriber;
use graph::vma::{AllocationCreateFlags, AllocationCreateInfo};
use notify::event::{DataChange, ModifyKind};
use notify::Watcher;
use pumice::VulkanResult;
use pumice::{util::ApiLoadConfig, vk};
use rustyline::Editor;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use crate::write::make_glsl_math;

pub struct ModuleEntry {
    pub module: object::ShaderModule,
    pub spirv: Vec<u32>,
    pub generation: u32,
}

impl Hash for ModuleEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.generation.hash(state);
    }
}

struct MetaModuleEntry {
    needs_density_fn: bool,
    module: Rc<ModuleEntry>,
    dirty: bool,
}

#[derive(PartialEq, Eq)]
pub enum PollResult {
    Recreate,
    Skip,
    Ok,
    Exit,
}

struct StdinWatcherData {
    thread: JoinHandle<()>,
}

pub struct ShaderModules {
    density_function: Option<String>,
    modules: HashMap<PathBuf, MetaModuleEntry>,
    watcher: notify::RecommendedWatcher,
    receiver: Receiver<AsyncEvent>,
    stdin: Option<JoinHandle<()>>,
    compiler: GlslCompiler,
}

const DENSITY_FUNCTION_MAGIC: &str = "float density(vec4 d);";

pub enum ShaderModulesConfig<'a> {
    Static(&'a str),
    WatchStdin,
}

impl ShaderModules {
    pub fn new(config: ShaderModulesConfig) -> Self {
        let (sender, receiver) = mpsc::channel();

        let mut sender_copy = sender.clone();
        let watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(ok) => match ok.kind {
                    notify::EventKind::Modify(_) => {
                        sender_copy.send(AsyncEvent::FilesChanged(ok.paths));
                    }
                    _ => {}
                },
                Err(e) => panic!("{:?}", e),
            },
        )
        .unwrap();

        let stdin_watcher = move || {
            use std::io::Write;

            let path = PathBuf::from_str(&rust_target_dir())
                .unwrap()
                .join("history.txt");

            let mut rl = Editor::<()>::new().unwrap();
            rl.load_history(&path);

            if let Some(last) = rl.history().last() {
                sender.send(AsyncEvent::StdinLine(last.to_owned()));
            }

            let mut history_file = File::options()
                .create(true)
                .append(true)
                .open(&path)
                .unwrap();

            // we manually create the history file since we need to append an entry immediatelly after a line is read
            // since this thread otherwise blocks on stdin the rest of the time and the main thread ending will terminate it immediatelly
            let size = history_file.seek(io::SeekFrom::End(0)).unwrap();
            if size == 0 {
                // version header or something
                writeln!(history_file, "#V2").unwrap();
            }

            loop {
                let readline = rl.readline("❯ ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        // append and entry to the history
                        writeln!(history_file, "{line}").unwrap();

                        match sender.send(AsyncEvent::StdinLine(line)) {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    }
                    Err(_) => {
                        sender.send(AsyncEvent::Exit);
                        break;
                    }
                }
            }
        };

        let mut s = Self {
            density_function: None,
            modules: Default::default(),
            watcher,
            receiver,
            stdin: None,
            compiler: GlslCompiler::new(),
        };

        match config {
            ShaderModulesConfig::Static(str) => {
                s.density_fn_changed(math_into_glsl(str).unwrap());
            }
            ShaderModulesConfig::WatchStdin => {
                s.stdin = Some(
                    std::thread::Builder::new()
                        .name("stdin reader".to_owned())
                        .spawn(stdin_watcher)
                        .unwrap(),
                );
            }
        };

        s
    }
    fn density_fn_changed(&mut self, new: String) {
        self.density_function = Some(new);
        for m in self.modules.values_mut() {
            if m.needs_density_fn {
                m.dirty = true;
            }
        }
    }
    fn remove_dirty_files(&mut self, dirty: &[PathBuf]) {
        for path in dirty {
            if let Some(m) = self.modules.get_mut(path) {
                m.dirty = true;
            }
        }
    }
    pub fn poll(&mut self) -> PollResult {
        let mut new_density_function = String::new();
        let mut files = Vec::new();
        loop {
            match self.receiver.try_recv() {
                Ok(event) => match event {
                    AsyncEvent::FilesChanged(changed) => files.extend(changed),
                    AsyncEvent::StdinLine(line) => new_density_function = line,
                    AsyncEvent::Exit => return PollResult::Exit,
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Sender threads exited, this is an error")
                }
            }
        }

        if self.density_function.is_none() && new_density_function.is_empty() {
            return PollResult::Skip;
        }

        let mut status = PollResult::Ok;
        if !new_density_function.is_empty() {
            match math_into_glsl(&new_density_function) {
                Ok(glsl) => {
                    self.density_fn_changed(glsl);
                    status = PollResult::Recreate;
                }
                Err(e) => {
                    eprintln!("Error parsing expression {:?}", e)
                }
            }
        }

        if !files.is_empty() {
            status = PollResult::Recreate;
            files.sort();
            files.dedup();

            self.remove_dirty_files(&files);

            // eprintln!("Recompiling files: {:?}", files);
        }

        status
    }
    pub unsafe fn retrieve(
        &mut self,
        path: impl AsRef<Path>,
        device: &device::Device,
    ) -> Result<Rc<ModuleEntry>, Box<dyn Error>> {
        let path = path.as_ref().canonicalize().unwrap();

        let make_new = |generation: u32| -> Result<(Rc<ModuleEntry>, bool), Box<dyn Error>> {
            let source = std::fs::read(&path)?;
            let mut source = String::from_utf8(source)?;

            // I would've used the preprocessor to insert the function but then the function isn't
            // available in the sourcecode which makes the vscode plugin unhappy and I can't make
            // the linter use some dummy definition for the macro because its arguments are given
            // just as a string so nothing with whitespace can be defined }and quotes don't work for
            // some reason
            let needs_density_fn = source.contains(DENSITY_FUNCTION_MAGIC);

            if needs_density_fn {
                assert!(self.density_function.is_some());
                source = source.replace(
                    DENSITY_FUNCTION_MAGIC,
                    self.density_function.as_ref().unwrap(),
                );
            }

            let spirv = self
                .compiler
                .compile(&source, path.file_name().unwrap().to_str().unwrap())
                .map_err(|e| format!("Error compiling shader '{path:?}':\n{e}"))?;

            let module = device.create_shader_module_spirv(&spirv)?;

            Ok((
                Rc::new(ModuleEntry {
                    module,
                    spirv,
                    generation,
                }),
                needs_density_fn,
            ))
        };

        match self.modules.entry(path.clone()) {
            Entry::Occupied(mut exists) => {
                let meta = exists.get_mut();
                if meta.dirty {
                    let next_generation = meta.module.generation + 1;
                    let (new, needs_density_fn) = make_new(next_generation)?;
                    meta.module = new;
                    meta.dirty = false;
                    meta.needs_density_fn = needs_density_fn;
                }

                Ok(meta.module.clone())
            }
            Entry::Vacant(no) => {
                let (new, needs_density_fn) = make_new(0)?;

                no.insert(MetaModuleEntry {
                    needs_density_fn,
                    module: new.clone(),
                    dirty: false,
                });

                assert!(&path.exists());
                self.watcher
                    .watch(&path, notify::RecursiveMode::NonRecursive);

                Ok(new)
            }
        }
    }
}

enum AsyncEvent {
    FilesChanged(Vec<PathBuf>),
    StdinLine(String),
    Exit,
}

pub fn rust_target_dir() -> Cow<'static, str> {
    std::env::var("CARGO_TARGET_DIR")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("target"))
}
