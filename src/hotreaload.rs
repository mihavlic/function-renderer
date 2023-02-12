use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
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
    pub needs_eval_fn: bool,
    pub spirv: Vec<u32>,
}

#[derive(PartialEq, Eq)]
pub enum PollResult {
    Recreate,
    Continue,
    Exit,
}

pub struct ShaderModules {
    eval_fn: String,
    modules: HashMap<PathBuf, Rc<ModuleEntry>>,
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

            let mut history_file = File::options().append(true).open(&path).unwrap();

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
            eval_fn: make_density_function("1.0"),
            modules: Default::default(),
            watcher,
            receiver,
            thread,
        }
    }
    fn eval_fn_changed(&mut self, new: String) {
        self.eval_fn = new;
        self.modules.retain(|_, entry| !entry.needs_eval_fn);
    }
    fn remove_dirty_files(&mut self, dirty: &[PathBuf]) {
        for path in dirty {
            self.modules.remove(path);
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

        match self.modules.entry(path.clone()) {
            Entry::Occupied(exists) => Ok(exists.get().clone()),
            Entry::Vacant(no) => {
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

                let value = Rc::new(ModuleEntry {
                    module,
                    needs_eval_fn,
                    spirv,
                });

                no.insert(value.clone());

                assert!(&path.exists());
                self.watcher
                    .watch(&path, notify::RecursiveMode::NonRecursive);

                Ok(value)
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
