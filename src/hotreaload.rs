use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::{Cursor, Seek};
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver};
use std::thread::JoinHandle;
use std::{io, slice};

use crate::write::{compile_glsl_to_spirv, make_density_function};
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
    needs_eval_fn: bool,
    module: Rc<ModuleEntry>,
    dirty: bool,
}

#[derive(PartialEq, Eq)]
pub enum PollResult {
    Recreate,
    Continue,
    Exit,
}

pub struct ShaderModules {
    eval_fn: String,
    modules: HashMap<PathBuf, MetaModuleEntry>,
    watcher: notify::RecommendedWatcher,
    receiver: Receiver<AsyncEvent>,
    thread: JoinHandle<()>,
}

impl ShaderModules {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let sender1 = sender.clone();
        let watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(ok) => match ok.kind {
                    notify::EventKind::Modify(_) => {
                        sender1.send(AsyncEvent::FilesChanged(ok.paths));
                    }
                    _ => {}
                },
                Err(e) => panic!("{:?}", e),
            },
        )
        .unwrap();

        let thread = std::thread::spawn(move || {
            use std::io::Write;

            let path = PathBuf::from_str(&rust_target_dir())
                .unwrap()
                .join("history.txt")
                .canonicalize()
                .unwrap();

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
        });

        Self {
            eval_fn: make_density_function(
                // "sin(0.25 * sqrt((x-32.0)*(x-32.0) + (y-32.0)*(y-32.0))) * 10 + 40 - z",
                "1/(0.01*sqrt((x-32.0)*(x-32.0) + (y-32.0)*(y-32.0))) - z",
                // "0.0",
                // "sin(sqrt(x*x+y*y+z*z))",
            ),
            modules: Default::default(),
            watcher,
            receiver,
            thread,
        }
    }
    fn eval_fn_changed(&mut self, new: String) {
        self.eval_fn = new;
        for m in self.modules.values_mut() {
            if m.needs_eval_fn {
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
        let mut new_eval_expr = String::new();
        let mut files = Vec::new();
        loop {
            match self.receiver.try_recv() {
                Ok(event) => match event {
                    AsyncEvent::FilesChanged(changed) => files.extend(changed),
                    AsyncEvent::StdinLine(line) => new_eval_expr = line,
                    AsyncEvent::Exit => return PollResult::Exit,
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => panic!("Channel senders disconnected"),
            }
        }
        let mut status = PollResult::Continue;
        if !new_eval_expr.is_empty() {
            let result = std::panic::catch_unwind(|| make_glsl_math(&new_eval_expr));
            match result {
                Ok(glsl_math) => {
                    let glsl = make_density_function(&glsl_math);
                    self.eval_fn_changed(glsl);
                    status = PollResult::Recreate;

                    // eprintln!("New expression: {glsl_math}");
                }
                Err(err) => {
                    eprintln!("Error parsing expression")
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
        needs_eval_fn: bool,
        device: &device::Device,
    ) -> Result<Rc<ModuleEntry>, Box<dyn Error>> {
        let path = path.as_ref().canonicalize().unwrap();

        let make_new = |generation: u32| -> Result<Rc<ModuleEntry>, Box<dyn Error>> {
            let source = std::fs::read(&path)?;
            let mut source = String::from_utf8(source)?;

            if needs_eval_fn {
                assert!(!self.eval_fn.is_empty());

                source.push('\n');
                source.push_str(&self.eval_fn);
            }

            let spirv = compile_glsl_to_spirv(
                &source,
                rust_target_dir().as_ref().as_ref(),
                std::str::from_utf8(path.extension().unwrap().as_bytes()).unwrap(),
            )?;

            let module = device.create_shader_module_spirv(&spirv)?;

            Ok(Rc::new(ModuleEntry {
                module,
                spirv,
                generation,
            }))
        };

        match self.modules.entry(path.clone()) {
            Entry::Occupied(mut exists) => {
                let meta = exists.get_mut();
                if meta.dirty {
                    let next_generation = meta.module.generation + 1;
                    let new = make_new(next_generation)?;
                    meta.module = new;
                    meta.dirty = false;
                }

                Ok(meta.module.clone())
            }
            Entry::Vacant(no) => {
                let new = make_new(0)?;

                no.insert(MetaModuleEntry {
                    needs_eval_fn,
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
