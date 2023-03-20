use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
use std::fmt::{format, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::{Cursor, Seek, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::{io, slice};

use crate::parse::{math_into_glsl, GlslCompiler};
use graph::device::debug::LazyDisplay;
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

struct SimpleFileEntry {
    generation: u32,
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
    density_function: Option<(String, String)>,
    modules: HashMap<PathBuf, MetaModuleEntry>,
    simple_files: HashMap<PathBuf, SimpleFileEntry>,
    watcher: notify::RecommendedWatcher,
    sender: Sender<AsyncEvent>,
    receiver: Receiver<AsyncEvent>,
    stdin: Option<JoinHandle<()>>,
    compiler: GlslCompiler,
}

const DENSITY_FUNCTION_MAGIC: &str = "float density(vec4 d);";
const GRADIENT_FUNCTION_MAGIC: &str = "vec4 gradient_density(vec4 d);";

pub enum ShaderModulesConfig<'a> {
    Static(&'a str),
    WatchStdin,
    None,
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

        let mut sender_copy = sender.clone();
        let stdin_watcher = move || {
            use std::io::Write;

            let path = PathBuf::from_str(&rust_target_dir())
                .unwrap()
                .join("history.txt");

            let mut rl = Editor::<()>::new().unwrap();
            rl.load_history(&path);

            if let Some(line) = rl.history().last() {
                println!("Displaying last function\n  {line}");
                if send_fun(line, &sender_copy) {
                    return;
                }
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
                writeln!(history_file, "#V2").unwrap();
            }

            loop {
                let readline = rl.readline("❯ ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        // append and entry to the history
                        writeln!(history_file, "{line}").unwrap();

                        if send_fun(&line, &sender_copy) {
                            break;
                        }
                    }
                    Err(_) => {
                        sender_copy.send(AsyncEvent::Exit);
                        break;
                    }
                }
            }
        };

        let mut s = Self {
            density_function: None,
            modules: Default::default(),
            simple_files: Default::default(),
            watcher,
            receiver,
            sender,
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
            ShaderModulesConfig::None => {}
        };

        s
    }
    pub fn event_sender(&self) -> Sender<AsyncEvent> {
        self.sender.clone()
    }
    fn density_fn_changed(&mut self, new: (String, String)) {
        self.density_function = Some(new);
        for m in self.modules.values_mut() {
            if m.needs_density_fn {
                m.dirty = true;
            }
        }
    }
    fn remove_dirty_files(&mut self, dirty: &[PathBuf]) {
        for path in dirty {
            self.invalidate_file_impl(path);
        }
    }
    fn invalidate_file_impl(&mut self, path: &PathBuf) {
        if let Some(m) = self.modules.get_mut(path) {
            m.dirty = true;
        }
        if let Some(m) = self.simple_files.get_mut(path) {
            m.dirty = true;
        }
    }
    pub fn invalidate_file(&mut self, path: &(impl AsRef<Path> + ?Sized)) {
        let path = path.as_ref().canonicalize().unwrap();
        self.invalidate_file_impl(&path);
    }
    pub fn poll(&mut self) -> PollResult {
        let mut new_density_functions = None;
        let mut files = Vec::new();
        loop {
            match self.receiver.try_recv() {
                Ok(event) => match event {
                    AsyncEvent::FilesChanged(changed) => files.extend(changed),
                    AsyncEvent::NewFunction { density, gradient } => {
                        new_density_functions = Some((density, gradient))
                    }
                    AsyncEvent::Exit => return PollResult::Exit,
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Sender threads exited, this is an error")
                }
            }
        }

        let mut status = PollResult::Ok;
        if let Some(funs) = new_density_functions {
            self.density_fn_changed(funs);
            status = PollResult::Recreate;
        } else if self.density_function.is_none() {
            self.density_fn_changed(math_into_glsl(&"0.0").unwrap());
        }

        // if self.density_function.is_none() {
        //     return PollResult::Skip;
        // }

        if !files.is_empty() {
            status = PollResult::Recreate;
            files.sort();
            files.dedup();

            self.remove_dirty_files(&files);
        }

        status
    }
    pub fn retrieve_simple(&mut self, path: impl AsRef<Path>) -> u32 {
        let path = path.as_ref().canonicalize().unwrap();
        match self.simple_files.entry(path.clone()) {
            Entry::Occupied(mut exists) => {
                let meta = exists.get_mut();
                if meta.dirty {
                    meta.generation += 1;
                    meta.dirty = false;
                }
                meta.generation
            }
            Entry::Vacant(no) => {
                no.insert(SimpleFileEntry {
                    generation: 0,
                    dirty: false,
                });

                assert!(path.exists());
                self.watcher
                    .watch(&path, notify::RecursiveMode::NonRecursive);

                0
            }
        }
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
            let needs_density_fn =
                source.contains(DENSITY_FUNCTION_MAGIC) || source.contains(GRADIENT_FUNCTION_MAGIC);

            if needs_density_fn {
                let Some((density, gradient)) = self.density_function.as_ref() else {
                    panic!()
                };

                source = source.replace(DENSITY_FUNCTION_MAGIC, density);
                source = source.replace(GRADIENT_FUNCTION_MAGIC, gradient);
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

fn send_fun(line: &str, sender: &Sender<AsyncEvent>) -> bool {
    match math_into_glsl(line) {
        Ok((density, gradient)) => {
            // send the valid function to be used for graphing
            // if the main thread exit, we'll exit too
            match sender.send(AsyncEvent::NewFunction { density, gradient }) {
                Ok(_) => false,
                Err(_) => true,
            }
        }
        Err(e) => {
            let error = graph::util::debug_callback::Colored(graph::tracing::Severity::Error, &e);
            let mut stdout = std::io::stdout().lock();
            write!(stdout, "\r{error}\n❯ ");
            stdout.flush();
            false
        }
    }
}

pub enum AsyncEvent {
    FilesChanged(Vec<PathBuf>),
    NewFunction { density: String, gradient: String },
    Exit,
}

pub fn rust_target_dir() -> Cow<'static, str> {
    std::env::var("CARGO_TARGET_DIR")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("target"))
}
