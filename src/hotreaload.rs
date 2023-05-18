//! Watches and reloads shaders.
//!
//! This module implements all of the shader reloading logic. A [`notify::Watcher`] is used to watch files on disk and invalidates all that changes.
//! Additionally, this also receives commands from the gui about changes to the function, processes it, and templates into the shaders.
use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::Seek;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

use crate::embed::MaybeFile;
use crate::parse::{self, math_into_glsl, parse_math, GlslCompiler};
use graph::device;
use graph::object;
use notify::Watcher;
use rustyline::Editor;

/// The stored shader module
pub struct ModuleEntry {
    /// The actual vulkan object
    pub module: object::ShaderModule,
    /// Spirv bytecode from the compiled shader, it is needed for descriptor reflection
    pub spirv: Vec<u32>,
    /// A generation which is incremented every time the module is reloaded, this is useful for [`crate::recomputation`] functions
    pub generation: u32,
}

impl Hash for ModuleEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.generation.hash(state);
    }
}

/// A [`ModuleEntry`] with some metadata
struct MetaModuleEntry {
    needs_density_fn: bool,
    module: Rc<ModuleEntry>,
    dirty: bool,
}

/// Like a [`ModuleEntry`] but doesn't result in a vulkan module.
///
/// It is still watched on disk and has a generation. The use case for this is shader includes, it can then be used with [crate::recomputation] to invalidate other modules (I admit it's hacky).
struct SimpleFileEntry {
    generation: u32,
    dirty: bool,
}

/// The overall result of processing all the changed files and [`AsyncEvent`]s.
#[derive(PartialEq, Eq, Debug)]
pub enum PollResult {
    /// Some modules changed, the vulkan Pipelines must be recreated.
    Recreate,
    /// Nothing changed, continue rendering.
    Ok,
    /// An application exit was requested.
    Exit,
}

/// A collection of strings derived from the submitted implicit function.
struct DensityFunctionData {
    /// The original implicit function.
    original: String,
    /// A glsl function which evaluates the function value.
    function: String,
    /// A glsl function which evaluates the function value and gradient with automatic reverse differentiation.
    gradient: String,
}

/// The central object which manages shader modules.
pub struct ShaderModules {
    /// Whether to force the edges of the function volume positive to make the resulting shape always solid.
    thickness: bool,
    invert: bool,
    density_function: Option<DensityFunctionData>,
    modules: HashMap<PathBuf, MetaModuleEntry>,
    simple_files: HashMap<PathBuf, SimpleFileEntry>,
    watcher: notify::RecommendedWatcher,
    sender: Sender<AsyncEvent>,
    receiver: Receiver<AsyncEvent>,
    stdin: Option<JoinHandle<()>>,
    compiler: GlslCompiler,
}

/// The string which lets us know that the shader wants the density function, it is replaced with [`DensityFunctionData::function`]
const DENSITY_FUNCTION_MAGIC: &str = "float density(vec4 d, vec3 n);";
/// The string which lets us know that the shader wants the gradient and density function, it is replaced with [`DensityFunctionData::gradient`]
const GRADIENT_FUNCTION_MAGIC: &str = "vec4 gradient_density(vec4 d, vec3 n);";

/// The initial config of [`ShaderModules`]
pub enum ShaderModulesConfig<'a> {
    /// Only display a static function, it may be changed later by a AsyncEvent::NewFunction
    Static(&'a str),
    /// Start a new thread which read stdin and provides [rustyline] interface
    WatchStdin,
    /// Do nothing
    None,
}

impl ShaderModules {
    pub fn new(config: ShaderModulesConfig, solid: bool, includes: &[(&str, MaybeFile)]) -> Self {
        let (sender, receiver) = mpsc::channel();

        let sender_copy = sender.clone();
        let watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(ok) => match ok.kind {
                    notify::EventKind::Modify(_) => {
                        let _ = sender_copy.send(AsyncEvent::FilesChanged(ok.paths));
                    }
                    _ => {}
                },
                Err(e) => panic!("{:?}", e),
            },
        )
        .unwrap();

        let sender_copy = sender.clone();
        let stdin_watcher = move || {
            use std::io::Write;

            let path = PathBuf::from_str(&rust_target_dir())
                .unwrap()
                .join("history.txt");

            let mut rl = Editor::<()>::new().unwrap();
            let _ = rl.load_history(&path);

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

            // we manually create the history file since we need to append an entry immediately after a line is read
            // since this thread otherwise blocks on stdin the rest of the time and the main thread ending will terminate it immediately
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
                        let _ = sender_copy.send(AsyncEvent::Exit);
                        break;
                    }
                }
            }
        };

        let mut s = Self {
            thickness: solid,
            invert: false,
            density_function: None,
            modules: Default::default(),
            simple_files: Default::default(),
            watcher,
            receiver,
            sender,
            stdin: None,
            compiler: GlslCompiler::new(includes),
        };

        match config {
            ShaderModulesConfig::Static(str) => {
                s.density_fn_changed(str.to_owned()).unwrap();
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
    fn density_fn_changed(&mut self, new: String) -> parse::Result<()> {
        let (function, gradient) = math_into_glsl(&new, self.thickness, self.invert)?;
        self.density_function = Some(DensityFunctionData {
            original: new,
            function,
            gradient,
        });
        self.invalidate_all_with_density();
        Ok(())
    }
    pub fn invalidate_all_with_density(&mut self) {
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
    pub fn invalidate_file(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref().canonicalize().unwrap();
        self.invalidate_file_impl(&path);
    }
    /// Process all new [`AsyncEvent`]s
    pub fn poll(&mut self) -> PollResult {
        let mut new_density_function = None;
        let mut files = Vec::new();
        let mut status = PollResult::Ok;
        loop {
            match self.receiver.try_recv() {
                Ok(event) => match event {
                    AsyncEvent::FilesChanged(changed) => files.extend(changed),
                    AsyncEvent::NewFunction(expr) => new_density_function = Some(expr),
                    AsyncEvent::GenerateThickness(thickness) => {
                        if thickness != self.thickness {
                            self.thickness = thickness;
                            status = PollResult::Recreate;
                            if let Some(data) = &self.density_function {
                                new_density_function = Some(data.original.to_owned());
                            }
                        }
                    }
                    AsyncEvent::Invert(invert) => {
                        if invert != self.invert {
                            self.invert = invert;
                            status = PollResult::Recreate;
                            if let Some(data) = &self.density_function {
                                new_density_function = Some(data.original.to_owned());
                            }
                        }
                    }
                    AsyncEvent::Exit => return PollResult::Exit,
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Sender threads exited, this is an error")
                }
            }
        }

        if let Some(new) = new_density_function {
            self.density_fn_changed(new)
                .expect("Expressions should be verified before being sent");
            status = PollResult::Recreate;
        } else if self.density_function.is_none() {
            self.density_fn_changed("0.0".to_owned()).unwrap();
        }

        if !files.is_empty() {
            status = PollResult::Recreate;
            files.sort();
            files.dedup();

            self.remove_dirty_files(&files);
        }

        status
    }
    /// Retrieve the generation of a SimpleFileEntry
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
                    .watch(&path, notify::RecursiveMode::NonRecursive)
                    .unwrap();

                0
            }
        }
    }
    /// Retrieve or possibly recompiled a shader module
    pub unsafe fn retrieve(
        &mut self,
        file: MaybeFile,
        device: &device::Device,
    ) -> Result<Rc<ModuleEntry>, Box<dyn Error>> {
        let path = file.path();

        let make_new = |generation: u32| -> Result<(Rc<ModuleEntry>, bool), Box<dyn Error>> {
            let source = file.read()?;
            let mut source = std::str::from_utf8(&source)?.to_owned();

            // I would've used the preprocessor to insert the function but then the function isn't
            // available in the sourcecode which makes the vscode plugin unhappy and I can't make
            // the linter use some dummy definition for the macro because its arguments are given
            // just as a string so nothing with whitespace can be defined }and quotes don't work for
            // some reason
            let needs_density_fn =
                source.contains(DENSITY_FUNCTION_MAGIC) || source.contains(GRADIENT_FUNCTION_MAGIC);

            if needs_density_fn {
                let Some(DensityFunctionData { function, gradient, .. }) = self.density_function.as_ref() else {
                    panic!()
                };

                source = source.replace(DENSITY_FUNCTION_MAGIC, function);
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

        match self.modules.entry(path.to_owned()) {
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

                if !file.is_embedded() {
                    assert!(file.path().exists());
                    self.watcher
                        .watch(&path, notify::RecursiveMode::NonRecursive)
                        .unwrap();
                }

                Ok(new)
            }
        }
    }
}

fn send_fun(expr: &str, sender: &Sender<AsyncEvent>) -> bool {
    match parse_math(expr) {
        Ok(_) => {
            // send the valid function to be used for graphing
            // if the main thread exit, we'll exit too
            match sender.send(AsyncEvent::NewFunction(expr.to_owned())) {
                Ok(_) => false,
                Err(_) => true,
            }
        }
        Err(e) => {
            use std::io::Write;

            let error = graph::util::debug_callback::Colored(graph::tracing::Severity::Error, &e);
            let mut stdout = std::io::stdout().lock();
            let _ = write!(stdout, "\r{error}\n❯ ");
            let _ = stdout.flush();
            false
        }
    }
}

pub enum AsyncEvent {
    FilesChanged(Vec<PathBuf>),
    NewFunction(String),
    GenerateThickness(bool),
    Invert(bool),
    Exit,
}

pub fn rust_target_dir() -> Cow<'static, str> {
    std::env::var("CARGO_TARGET_DIR")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("target"))
}
