#![allow(unreachable_code, dead_code)]

pub mod embed;
pub mod gui;
pub mod hotreaload;
pub mod parse;
pub mod passes;
pub mod recomputation;
pub mod stl;
pub mod yawpitch;

use dolly::prelude::{Arm, Position, RightHanded, Smooth};
use dolly::rig::CameraRig;
use dolly::transform::Transform;
use glam::Vec3;
use graph::device::reflection::{ReflectedLayout, SpirvModule};
use graph::device::{self, Device, DeviceCreateInfo, QueueFamilySelection};
use graph::graph::compile::{GraphCompiler, ImageKindCreateInfo};
use graph::graph::descriptors::{DescBuffer, DescImage, DescSetBuilder, DescriptorData};
use graph::graph::execute::{CompiledGraph, GraphExecutor, GraphRunConfig, GraphRunStatus};
use graph::graph::task::UnsafeSendSync;
use graph::graph::GraphImage;
use graph::instance::{Instance, InstanceCreateInfo};
use graph::object::{self, PipelineStage, SwapchainCreateInfo};
use graph::smallvec::{smallvec, SmallVec};
use graph::tracing::tracing_subscriber::install_tracing_subscriber;
use graph::vma;
use gui::{GuiControl, GuiOutput, WindowState};
use hotreaload::{PollResult, ShaderModules, ShaderModulesConfig};
use pumice::{util::ApiLoadConfig, vk};
use recomputation::RecomputationCache;
use std::cell::RefCell;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::println;
use std::sync::mpsc::{self};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use stl::write_stl;
use winit::event::{Event, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use yawpitch::YawPitchZUp;

use crate::embed::{maybe_embed_file, MaybeFile};
use crate::gui::{PaintConfig, RendererConfig};
use crate::passes::{LambdaPass, MeshPass};

/// Internal configuration option for the msaa sample count for the function mesh pass.
pub const MSAA_SAMPLE_COUNT: vk::SampleCountFlags = vk::SampleCountFlags::C1;
/// Internal configuration option for the msaa sample count for egui render pass.
pub const EGUI_MSAA_SAMPLE_COUNT: vk::SampleCountFlags = vk::SampleCountFlags::C4;

enum ApplicationEvent {
    MeshVerticesReady(Vec<glam::Vec3>),
    MeshIndicesReady(Vec<u32>),
}

/// The application state which is updated by [`main()`] and is read by some of the graph render passes.
pub struct ApplicationState {
    /// The current camera transform of the *camera*. The model to world transform is the inverse of this.
    camera: Transform<RightHanded>,
    /// Egui primitives
    primitives: Vec<egui::ClippedPrimitive>,
    /// Egui requested texture modifications
    textures_delta: egui::TexturesDelta,
    /// Physical pixels (physical display elements) / logical pixels (hardware independent size, like css pixels), essentially dpi scaling - "normal" (96 dpi) is 1.0 in `pixels_per_point`
    pixels_per_point: f32,
    /// The min corner of the rendered function interval
    rect_min: Vec3,
    /// The max corner of the rendered function interval
    rect_max: Vec3,
    /// The time since the start of the application
    time: f32,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            camera: Transform::IDENTITY,
            primitives: Default::default(),
            textures_delta: Default::default(),
            pixels_per_point: 1.0,
            rect_min: Vec3::splat(0.0),
            rect_max: Vec3::splat(63.0),
            time: 0.0,
        }
    }
}

/// The function which ties it all together. There really is a lot of code in here which should probably be separated.
pub fn main() {
    unsafe {
        // start the logging runtime, currently it really just redirects to stdout
        install_tracing_subscriber(None);
        // the winit event loop
        let event_loop = EventLoop::new();

        // create the window
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0f32, 1024.0f32))
            .with_decorations(false)
            .with_transparent(true)
            .with_maximized(true)
            .build(&event_loop)
            .unwrap();

        // create the vulkan window object and its application state
        let (surface, device, queue) = make_device(&window);
        let swapchain = make_swapchain(&window, surface, &device);
        let override_scaling = std::env::var("SCALING_OVERRIDE")
            .ok()
            .and_then(|s| s.parse::<f32>().ok());

        if let Some(scaling) = override_scaling {
            println!("Overriding scaling: {scaling}");
        }

        let window_state = WindowState::new(window, override_scaling, &event_loop);

        // this object tracks and reloads shader modules
        let modules = ShaderModules::new(
            // ShaderModulesConfig::WatchStdin,
            // ShaderModulesConfig::Static("sin(2 sqrt(x*x+y*y) / pi) * 25 + 30 - z"),
            // ShaderModulesConfig::Static("sin(2sqrt(x*x+y*y+z*z)/pi)"),
            ShaderModulesConfig::None,
            true,
            &[
                // register files which will be included by other shaders
                (
                    "compute_descriptors.glsl",
                    maybe_embed_file!("shaders/compute_descriptors.glsl"),
                ),
            ],
        );
        // this caches some function evaluations and improves screen resize speed in particular
        let cache = RecomputationCache::new();
        // compiler for rendergraphs - schedules and synchronizes gpu work
        let mut compiler = GraphCompiler::new();

        // the virtual camera which rotates around a single point, always loking at the center
        let mut camera: CameraRig = CameraRig::builder()
            .with(Position::new(Vec3::splat(31.5)))
            .with(YawPitchZUp::new().pitch_degrees(35.0).yaw_degrees(0.0))
            .with(Smooth::new_rotation(0.25))
            .with(Arm::new(Vec3::Z * 120.0))
            .build();

        let (tx, rx) = mpsc::channel::<ApplicationEvent>();
        let mut mesh_data = (None, None);
        let app_data: Arc<Mutex<ApplicationState>> =
            Arc::new(Mutex::new(ApplicationState::default()));

        let mut prev = std::time::Instant::now();
        let mut graph: Option<GraphOutput> = None;

        // the title bar gui state
        let mut gui = GuiControl::new(
            modules.event_sender(),
            app_data.clone(),
            // prefill with some initial history
            &[
                // "y - z",
                // "sin(x)",
                // "cos(abs(x)) + cos(abs(y)) + cos(abs(z)) - cos(x^2+y^2+z^2)",
                // "|x| + |y| - z",
                // "-(e^sqrt((x*x + y*y)) + z*100000)",
                "z - 8*sin(sqrt(x^2 + y^2) / 2pi)",
                "z - 128/sqrt(xx + yy)",
                "z - 2000/(x y)",
                "-sin(2sqrt(x^2+y^2+z^2)/pi)",
                "0.0",
            ],
        );

        // wrap the objects in options to allow us to control drop order from the event loop
        let mut device_option = Some(device);
        let mut swapchain_option = Some(swapchain);
        let mut modules_option = Some(modules);
        let mut cache_option = Some(cache);

        // the current size of the viewport where the function is rendered
        let mut inner_size = None;

        event_loop.run(move |event, _, control_flow| {
            let device = device_option.as_mut().unwrap();
            let swapchain = swapchain_option.as_mut().unwrap();
            let modules = modules_option.as_mut().unwrap();
            let cache = cache_option.as_mut().unwrap();

            // feed the event to the window state
            window_state.process_event(event);
            let window = window_state.window();

            // handle the mesh export - since reading gpu memory is inherently asynchronous, the current abstractions
            // are given a callback to call when the memory is ready, this callback copies the memory and sends it here
            // since the mesh is composed of both a vertex and an index buffer, we need to wait for both to complete
            // only then can we continue saving the mesh to disk
            {
                // receive the mesh buffers
                while let Ok(event) = rx.try_recv() {
                    match event {
                        ApplicationEvent::MeshVerticesReady(a) => mesh_data.0 = Some(a),
                        ApplicationEvent::MeshIndicesReady(a) => mesh_data.1 = Some(a),
                    }
                }

                // if we've collected both of them, remove them from the variable and open the file dialog
                if mesh_data.0.is_some() && mesh_data.1.is_some() {
                    let (Some(vertices), Some(indices)) = std::mem::take(&mut mesh_data) else {
                        unreachable!();
                    };

                    // the dialog is opened on another thread to keep the main thread running
                    device.threadpool().spawn(move || {
                        if let Some(path) = rfd::FileDialog::new().add_filter("stl", &["stl"]).save_file() {
                            if let Ok(file) = OpenOptions::new().read(false).write(true).truncate(true).create(true).open(&path) {
                                let writer = BufWriter::new(file);
                                if let Err(e) = write_stl(writer, &vertices, &indices) {
                                    eprintln!("Error saving mesh to '{path:?}': {e}");
                                }
                            } else {
                                eprintln!("Failed to open {path:?}");
                            }
                        } else {
                            eprintln!("FileDialog returned None");
                        }
                    });
                }
            }

            // process all winit events collected do far
            for event in window_state.drain_events() {
                match event {
                    Event::WindowEvent { event, window_id } => match event {
                        WindowEvent::CloseRequested if window_id == window.id() => {
                            control_flow.set_exit();
                        }
                        WindowEvent::Resized(size) => {
                            let extent = vk::Extent2D {
                                width: size.width,
                                height: size.height,
                            };
                            swapchain.surface_resized(extent);
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let delta = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y,
                                MouseScrollDelta::PixelDelta(p) => p.y as f32,
                            };
                            // zoom the camera
                            camera.driver_mut::<Arm>().offset.z -= delta * 2.0;
                        }
                        _ => {}
                    },
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }
                    // start rendering!
                    Event::RedrawRequested(_) => {
                        let dt = prev.elapsed().as_secs_f32();
                        camera.update(dt);

                        prev = std::time::Instant::now();

                        let mut exit = false;
                        let mut rebuild_graph = false;

                        match modules.poll() {
                            PollResult::Recreate => rebuild_graph = true,
                            PollResult::Ok if graph.is_none() => rebuild_graph = true,
                            PollResult::Ok => {}
                            PollResult::Exit => exit = true,
                        }

                        let size = window.inner_size();
                        if let Some(graph) = graph.as_mut() {
                            if !(window.is_visible() == Some(false)
                                || size.width == 0
                                || size.height == 0)
                            {
                                let output = window_state.gui_frame(|window| gui.ui(window));

                                if output.exit_requested {
                                    exit = true;
                                }

                                let GuiOutput {
                                    inner_size: new_inner_size,
                                    drag_delta,
                                    save_requested,
                                } = output.inner;

                                if Some(new_inner_size) != inner_size {
                                    inner_size = Some(new_inner_size);
                                    rebuild_graph = true;
                                }

                                if window_state.mouse_state().left {
                                    let m = 0.3;
                                    // rotate the camera
                                    camera
                                        .driver_mut::<YawPitchZUp>()
                                        .rotate_yaw_pitch(drag_delta.x * m, drag_delta.y * m);
                                }

                                // see the comment about saving the mesh at the start of the event loop
                                if save_requested {
                                    device.write_multiple(|manager| {
                                        let mut read = |buf: &object::Buffer, kind: fn(*const u8, usize) -> ApplicationEvent| {
                                            let size = buf.get_create_info().size as usize;
                                            let tx = tx.clone();
                                            manager.read_buffer(
                                                buf,
                                                0,
                                                std::alloc::Layout::from_size_align(size, 4)
                                                    .unwrap(),
                                                move |ptr| {
                                                    let len = *ptr.add(4).cast::<u32>();
                                                    // skip the size and offset fields at the start of the buffer
                                                    let ptr = ptr.add(8);

                                                    let _ = tx.send(kind(ptr, len as usize));
                                                },
                                                device,
                                            );
                                        };

                                        read(&graph.vertices, |ptr, size| {
                                            let vec = std::slice::from_raw_parts(ptr.cast::<glam::Vec3>(), size);
                                            ApplicationEvent::MeshVerticesReady(vec.to_owned())
                                        });
                                        read(&graph.indices, |ptr, size| {
                                            let vec = std::slice::from_raw_parts(ptr.cast::<u32>(), size);
                                            ApplicationEvent::MeshIndicesReady(vec.to_owned())
                                        });

                                        manager.flush(device);
                                    });
                                }

                                // update the app state
                                {
                                    let mut lock = app_data.lock().unwrap();
                                    lock.camera = camera.final_transform;
                                    lock.primitives = output.primitives;
                                    lock.textures_delta.append(output.textures_delta);
                                    lock.pixels_per_point = window_state.pixels_per_point();
                                }

                                let result = graph.graph.run(GraphRunConfig {
                                    swapchain_acquire_timeout_ns: 1_000_000_000 / 60,
                                    acquire_swapchain_with_fence: false,
                                    // we need to recreate some images if the swapchain size changed
                                    return_after_swapchain_recreate: true,
                                });

                                match result {
                                    GraphRunStatus::Ok => {}
                                    GraphRunStatus::SwapchainAcquireTimeout => {}
                                    GraphRunStatus::SwapchainRecreated => {
                                        rebuild_graph = true;
                                    }
                                }

                                // sleep the remainder of 16 milliseconds to achieve 60 Hz
                                if let Some(remaining) =
                                    Duration::from_millis(1000 / 60).checked_sub(prev.elapsed())
                                {
                                    if !rebuild_graph {
                                        std::thread::sleep(remaining);
                                    }
                                }
                            }
                        }

                        // poll the vulkan abstraction to perform some cleanup
                        device.idle_cleanup_poll();

                        if rebuild_graph && !exit {
                            // yees run the graph
                            let result = make_graph(
                                &swapchain,
                                inner_size.unwrap_or([512; 2]),
                                queue,
                                app_data.clone(),
                                modules,
                                &mut compiler,
                                cache,
                                &device,
                            );

                            match result {
                                Ok(ok) => {
                                    graph = Some(ok);
                                }
                                Err(err) => {
                                    eprintln!("Graph compilation error:\n{err}");
                                }
                            };
                        }

                        if exit {
                            control_flow.set_exit();
                        }
                    }
                    // winit guarantees that this is the last event we'll receive
                    // now, we have all the code to perform a clean teardown
                    // but for some reason the shaderc compiler takes way too long to clean up
                    // and the swapchain semaphore takes like 300ms (??) to be destroyed 
                    // 
                    // this combined makes the application take too long to exit, the simple solution
                    // is to end the process and leave the os to deal with it
                    Event::LoopDestroyed => {
                        // gotta go fast
                        std::process::exit(0);

                        drop(graph.take());
                        drop(swapchain_option.take());
                        drop(modules_option.take());
                        drop(cache_option.take());
                        let device = device_option.take().unwrap();
                        device.drain_work();
                        device.attempt_destroy().unwrap();
                    }
                    _ => (),
                }
            }
        });
    }
}

/// Data from [`make_graph()`] needed by [`main()`]
struct GraphOutput {
    /// The vulkan buffer with vertices
    vertices: object::Buffer,
    /// The vulkan buffer with indices
    indices: object::Buffer,
    graph: CompiledGraph,
}

/// Creates the render graph which executes all Vulkan work. It must be recreated when screen size changes or the viewport extent changes.
unsafe fn make_graph(
    swapchain: &object::Swapchain,
    function_extent: [u32; 2],
    queue: device::submission::Queue,
    state: Arc<Mutex<ApplicationState>>,
    modules: &mut ShaderModules,
    compiler: &mut GraphCompiler,
    cache: &mut RecomputationCache,
    device: &device::OwnedDevice,
) -> Result<GraphOutput, Box<dyn Error>> {
    /// Hash all of the provided expressions, used with [`crate::recomputation::RecomputationCache::compute_located()`]
    macro_rules! args {
        ($($arg:expr),*) => {
            |_state| {
                $(
                    std::hash::Hash::hash(&$arg, _state);
                )*
            }
        };
    }

    let cache = RefCell::new(cache);

    let mut create_pipeline = |path: MaybeFile| -> Result<_, Box<dyn Error>> {
        let mut cache = cache.borrow_mut();
        let module = modules.retrieve(path, device)?;

        let all_layout =
            cache.get_or_insert_named::<object::PipelineLayout, _>("all layout", || {
                ReflectedLayout::new(&[SpirvModule {
                    spirv: &module.spirv,
                    entry_points: &["main"],
                    dynamic_uniform_buffers: true,
                    dynamic_storage_buffers: false,
                    include_unused_descriptors: true,
                }])
                .create(device, vk::DescriptorSetLayoutCreateFlags::empty())
                .unwrap()
            });

        let pipeline = cache.compute_located(args!(path.as_str()), args!(module), || {
            let pipeline_info = object::ComputePipelineCreateInfo {
                flags: vk::PipelineCreateFlags::empty(),
                stage: PipelineStage {
                    flags: vk::PipelineShaderStageCreateFlags::empty(),
                    stage: vk::ShaderStageFlags::COMPUTE,
                    module: module.module.clone(),
                    name: "main".into(),
                    specialization_info: None,
                },
                layout: all_layout.inner,
                base_pipeline: object::BasePipeline::None,
            };

            device.create_compute_pipeline(pipeline_info).unwrap()
        });

        Ok(pipeline.inner)
    };

    let create_image = |format: vk::Format,
                        samples: vk::SampleCountFlags,
                        size: object::Extent,
                        usage: vk::ImageUsageFlags,
                        label: &'static str| {
        let mut cache = cache.borrow_mut();
        cache
            .compute_named(label, args!(format, size), || {
                device
                    .create_image(
                        object::ImageCreateInfo {
                            flags: vk::ImageCreateFlags::empty(),
                            size,
                            format,
                            samples,
                            mip_levels: 1,
                            array_layers: 1,
                            tiling: vk::ImageTiling::OPTIMAL,
                            usage,
                            sharing_mode_concurrent: false,
                            initial_layout: vk::ImageLayout::UNDEFINED,
                            label: Some(label.into()),
                        },
                        vma::AllocationCreateInfo {
                            flags: vma::AllocationCreateFlags::empty(),
                            usage: vma::MemoryUsage::AutoPreferDevice,
                            ..Default::default()
                        },
                    )
                    .unwrap()
            })
            .inner
    };

    let create_buffer = |usage: vk::BufferUsageFlags, size: u64, label: &'static str| {
        let mut cache = cache.borrow_mut();
        cache
            .compute_named(label, args!(usage, size), || {
                device
                    .create_buffer(
                        object::BufferCreateInfo {
                            flags: vk::BufferCreateFlags::empty(),
                            size,
                            usage,
                            sharing_mode_concurrent: false,
                            label: Some(label.into()),
                        },
                        vma::AllocationCreateInfo {
                            flags: vma::AllocationCreateFlags::HOST_ACCESS_ALLOW_TRANSFER_INSTEAD
                                | vma::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                            usage: vma::MemoryUsage::AutoPreferDevice,
                            ..Default::default()
                        },
                    )
                    .unwrap()
            })
            .inner
    };

    let populate_grid = create_pipeline(maybe_embed_file!("shaders/populate_grid.comp"))?;
    let intersect = create_pipeline(maybe_embed_file!("shaders/intersect.comp"))?;
    let compute_vertex = create_pipeline(maybe_embed_file!("shaders/compute_vertex.comp"))?;
    let emit_triangles = create_pipeline(maybe_embed_file!("shaders/emit_triangles.comp"))?;

    let depth_image = create_image(
        vk::Format::D32_SFLOAT,
        MSAA_SAMPLE_COUNT,
        object::Extent::D2(function_extent[0], function_extent[1]),
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        "depth",
    );
    let color_image = create_image(
        vk::Format::B8G8R8A8_UNORM,
        MSAA_SAMPLE_COUNT,
        object::Extent::D2(function_extent[0], function_extent[1]),
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
        "color",
    );
    let resolve_image = (MSAA_SAMPLE_COUNT != vk::SampleCountFlags::C1).then(|| {
        create_image(
            vk::Format::B8G8R8A8_UNORM,
            vk::SampleCountFlags::C1,
            object::Extent::D2(function_extent[0], function_extent[1]),
            vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
            "resolve",
        )
    });
    let vk::Extent2D { width, height } = swapchain.get_extent();
    let window_color_image = create_image(
        vk::Format::B8G8R8A8_UNORM,
        EGUI_MSAA_SAMPLE_COUNT,
        object::Extent::D2(width, height),
        vk::ImageUsageFlags::COLOR_ATTACHMENT,
        "resolve",
    );

    let function_values = create_image(
        vk::Format::R16_SFLOAT,
        vk::SampleCountFlags::C1,
        object::Extent::D3(64, 64, 64),
        vk::ImageUsageFlags::STORAGE,
        "function_values",
    );
    let intersections = create_image(
        vk::Format::R32G32B32A32_SFLOAT,
        vk::SampleCountFlags::C1,
        object::Extent::D3(64 * 3, 64, 64),
        vk::ImageUsageFlags::STORAGE,
        "intersections",
    );
    let vertex_indices = create_image(
        vk::Format::R32_UINT,
        vk::SampleCountFlags::C1,
        object::Extent::D3(64, 64, 64),
        vk::ImageUsageFlags::STORAGE,
        "vertex_indices",
    );

    const INDEX_CAPACITY: u32 = 1024 * 512 * 3;
    const VERTEX_CAPACITY: u32 = 1024 * 512;

    let vertices = create_buffer(
        vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::VERTEX_BUFFER,
        (VERTEX_CAPACITY * 3 * 4) as u64 + 8,
        "vertices",
    );
    let indices = create_buffer(
        vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::INDEX_BUFFER,
        (INDEX_CAPACITY * 4) as u64 + 8,
        "indices",
    );

    let vert_module = modules.retrieve(maybe_embed_file!("shaders/mesh.vert"), device)?;
    let frag_module = modules.retrieve(maybe_embed_file!("shaders/mesh.frag"), device)?;

    let pipeline = cache
        .borrow_mut()
        .compute_located(args!(), args!(vert_module, frag_module), || {
            let layout = ReflectedLayout::new(&[
                SpirvModule {
                    spirv: &vert_module.spirv,
                    entry_points: &["main"],
                    dynamic_uniform_buffers: true,
                    dynamic_storage_buffers: false,
                    include_unused_descriptors: false,
                },
                SpirvModule {
                    spirv: &frag_module.spirv,
                    entry_points: &["main"],
                    dynamic_uniform_buffers: true,
                    dynamic_storage_buffers: false,
                    include_unused_descriptors: false,
                },
            ])
            .create(&device, vk::DescriptorSetLayoutCreateFlags::empty())
            .unwrap();

            let pipeline_info = object::GraphicsPipelineCreateInfo::builder()
                .stages([
                    PipelineStage {
                        flags: vk::PipelineShaderStageCreateFlags::empty(),
                        stage: vk::ShaderStageFlags::VERTEX,
                        module: vert_module.module.clone(),
                        name: "main".into(),
                        specialization_info: None,
                    },
                    PipelineStage {
                        flags: vk::PipelineShaderStageCreateFlags::empty(),
                        stage: vk::ShaderStageFlags::FRAGMENT,
                        module: frag_module.module.clone(),
                        name: "main".into(),
                        specialization_info: None,
                    },
                ])
                .input_assembly(object::state::InputAssembly {
                    topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                    primitive_restart_enable: false,
                })
                .viewport(object::state::Viewport {
                    // the actual contents are ignored, it is just important to have one for each
                    viewports: smallvec![Default::default()],
                    scissors: smallvec![Default::default()],
                })
                .vertex_input(object::state::VertexInput {
                    vertex_bindings: [object::state::InputBinding {
                        binding: 0,
                        // 3 floats for position
                        stride: 3 * 4,
                        input_rate: vk::VertexInputRate::VERTEX,
                    }]
                    .to_vec(),
                    vertex_attributes: [object::state::InputAttribute {
                        location: 0,
                        binding: 0,
                        format: vk::Format::R32G32B32_SFLOAT,
                        offset: 0,
                    }]
                    .to_vec(),
                })
                .rasterization(object::state::Rasterization {
                    depth_clamp_enable: false,
                    rasterizer_discard_enable: false,
                    polygon_mode: vk::PolygonMode::FILL,
                    cull_mode: vk::CullModeFlags::NONE,
                    front_face: vk::FrontFace::COUNTER_CLOCKWISE,
                    line_width: 1.0,
                    ..Default::default()
                })
                .multisample(object::state::Multisample {
                    rasterization_samples: MSAA_SAMPLE_COUNT,
                    sample_shading_enable: false,
                    ..Default::default()
                })
                .depth_stencil(object::state::DepthStencil {
                    depth_test_enable: true,
                    depth_write_enable: true,
                    depth_compare_op: vk::CompareOp::LESS,
                    depth_bounds_test_enable: false,
                    stencil_test_enable: false,
                    ..Default::default()
                })
                .color_blend(object::state::ColorBlend {
                    attachments: vec![object::state::Attachment {
                        color_write_mask: vk::ColorComponentFlags::all(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
                .dynamic_state([vk::DynamicState::SCISSOR, vk::DynamicState::VIEWPORT])
                .layout(layout)
                .finish();

            device.create_delayed_graphics_pipeline(pipeline_info)
        })
        .inner;

    let egui_pass = cache
        .borrow_mut()
        .compute_located(args!(), args!(), || {
            let config = RendererConfig {
                output_attachment_is_unorm_nonlinear: true,
                format: vk::Format::B8G8R8A8_UNORM,
                samples: EGUI_MSAA_SAMPLE_COUNT,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::DONT_CARE,
                color_src_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                color_src_stages: vk::PipelineStageFlags::empty(),
                color_src_access: vk::AccessFlags::empty(),
                color_final_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                resolve_enable: true,
                resolve_load_op: vk::AttachmentLoadOp::DONT_CARE,
                resolve_store_op: vk::AttachmentStoreOp::STORE,
                resolve_src_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                resolve_src_stages: vk::PipelineStageFlags::empty(),
                resolve_src_access: vk::AccessFlags::empty(),
                resolve_final_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
            };
            Arc::new(Mutex::new(UnsafeSendSync::new(gui::Renderer::new(
                &config, device,
            ))))
        })
        .inner;

    let whole_descriptor_pipeline_layout = cache
        .borrow()
        .get_named::<object::PipelineLayout>("all layout")
        .expect("Compute pipelines have already been compiled, the all layout must be available")
        .inner;

    let graph = compiler.compile(device.clone(), |b| {
        let queue = b.import_queue(queue);
        let swapchain_image = b.acquire_swapchain(swapchain.clone());

        let depth = b.import_image((depth_image, "depth"));
        let color = b.import_image((color_image.clone(), "color"));
        let window_color = b.import_image((window_color_image, "window_color"));
        let resolve = resolve_image
            .clone()
            .map(|resolve| b.import_image((resolve, "resolve")));

        let function_values = b.import_image((function_values, "function_values"));
        let intersections = b.import_image((intersections, "intersections"));
        let vertex_indices = b.import_image((vertex_indices, "vertex_indices"));

        let vertices = b.import_buffer((vertices.clone(), "vertices"));
        let indices = b.import_buffer((indices.clone(), "indices"));

        struct GlobalDescriptorData {
            set: vk::DescriptorSet,
            dynamic_offsets: SmallVec<[u32; 4]>,
        }

        let whole_descriptor_pipeline_layout_copy = whole_descriptor_pipeline_layout.clone();
        let state_copy = state.clone();
        let get_or_create_descriptor = move |e: &GraphExecutor| -> GlobalDescriptorData {
            #[repr(C)]
            struct FunctionData {
                rect_min: Vec3,
                rect_max: Vec3,
                time: f32,
            }

            let data = {
                let state = state_copy.lock().unwrap();
                FunctionData {
                    rect_min: state.rect_min,
                    rect_max: state.rect_max,
                    time: state.time,
                }
            };

            let uniform = e.allocate_uniform_element(&data);

            let (set, dynamic_offsets) = DescSetBuilder::new(
                &whole_descriptor_pipeline_layout_copy.get_descriptor_set_layouts()[0],
            )
            .update_whole(&[
                DescriptorData::Image(DescImage {
                    view: e.get_default_image_view(function_values),
                    layout: vk::ImageLayout::GENERAL,
                    ..Default::default()
                }),
                DescriptorData::Image(DescImage {
                    view: e.get_default_image_view(intersections),
                    layout: vk::ImageLayout::GENERAL,
                    ..Default::default()
                }),
                DescriptorData::Image(DescImage {
                    view: e.get_default_image_view(vertex_indices),
                    layout: vk::ImageLayout::GENERAL,
                    ..Default::default()
                }),
                DescriptorData::Buffer(DescBuffer {
                    buffer: e.get_buffer(vertices),
                    ..Default::default()
                }),
                DescriptorData::Buffer(DescBuffer {
                    buffer: e.get_buffer(indices),
                    ..Default::default()
                }),
                DescriptorData::Buffer(uniform.as_desc_dynamic()),
            ])
            .finish(e)
            .into_raw();

            GlobalDescriptorData {
                set,
                dynamic_offsets,
            }
        };

        let bind_descriptor = move |e: &GraphExecutor, device: &Device| {
            let mut blackboard = e.execution_blackboard_mut();
            let data = blackboard
                .get_or_insert_with::<GlobalDescriptorData, _>(|| get_or_create_descriptor(e));

            device.device().cmd_bind_descriptor_sets(
                e.command_buffer(),
                vk::PipelineBindPoint::COMPUTE,
                whole_descriptor_pipeline_layout.raw(),
                0,
                &[data.set],
                &data.dynamic_offsets,
            )
        };

        let draw_parameter_buffer = b.create_buffer(
            object::BufferCreateInfo {
                flags: vk::BufferCreateFlags::empty(),
                size: std::mem::size_of::<vk::DrawIndexedIndirectCommand>() as u64,
                usage: vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDIRECT_BUFFER,
                sharing_mode_concurrent: false,
                label: Some("draw_parameter_buffer".into()),
            },
            vma::AllocationCreateInfo {
                flags: vma::AllocationCreateFlags::empty(),
                usage: vma::MemoryUsage::Unknown,
                required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ..Default::default()
            },
        );

        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_buffer(
                        vertices,
                        vk::BufferUsageFlags::TRANSFER_DST,
                        vk::PipelineStageFlags2KHR::TRANSFER,
                        vk::AccessFlags2KHR::TRANSFER_WRITE,
                    );
                    builder.use_buffer(
                        indices,
                        vk::BufferUsageFlags::TRANSFER_DST,
                        vk::PipelineStageFlags2KHR::TRANSFER,
                        vk::AccessFlags2KHR::TRANSFER_WRITE,
                    );
                },
                move |e, d| {
                    let (d, cmd) = (d.device(), e.command_buffer());
                    // the layout of the allocator:
                    //  u32 capacity
                    //  u32 current offset
                    let vertices_buffer = e.get_buffer(vertices);
                    d.cmd_update_buffer(
                        cmd,
                        vertices_buffer,
                        0,
                        4,
                        [VERTEX_CAPACITY].as_ptr().cast(),
                    );
                    d.cmd_fill_buffer(cmd, vertices_buffer, 4, vk::WHOLE_SIZE, 0);

                    let indices_buffer = e.get_buffer(indices);
                    d.cmd_update_buffer(
                        cmd,
                        indices_buffer,
                        0,
                        4,
                        [INDEX_CAPACITY].as_ptr().cast(),
                    );
                    d.cmd_fill_buffer(cmd, indices_buffer, 4, vk::WHOLE_SIZE, 0);
                },
            ),
            "Reset buffer bump allocators",
        );

        let bind_descriptor_copy = bind_descriptor.clone();
        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_image(
                        function_values,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                },
                move |e, device| {
                    let (d, cmd) = (device.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, populate_grid.raw());

                    bind_descriptor_copy(e, device);

                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count, count, count);
                },
            ),
            "Populate grid",
        );

        let bind_descriptor_copy = bind_descriptor.clone();
        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_image(
                        function_values,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_READ,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                    builder.use_image(
                        intersections,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                },
                move |e, device| {
                    let (d, cmd) = (device.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, intersect.raw());

                    bind_descriptor_copy(e, device);

                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count * 3, count, count);
                },
            ),
            "Intersect cell edges",
        );

        let bind_descriptor_copy = bind_descriptor.clone();
        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_image(
                        intersections,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_READ,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                    builder.use_image(
                        vertex_indices,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                    builder.use_buffer(
                        vertices,
                        vk::BufferUsageFlags::STORAGE_BUFFER,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
                    );
                },
                move |e, device| {
                    let (d, cmd) = (device.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, compute_vertex.raw());

                    bind_descriptor_copy(e, device);

                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count, count, count);
                },
            ),
            "Compute cell vertices",
        );

        let bind_descriptor_copy = bind_descriptor.clone();
        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_image(
                        intersections,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_READ,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                    builder.use_image(
                        vertex_indices,
                        vk::ImageUsageFlags::STORAGE,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_READ,
                        vk::ImageLayout::GENERAL,
                        None,
                    );
                    builder.use_buffer(
                        vertices,
                        vk::BufferUsageFlags::STORAGE_BUFFER,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_READ,
                    );
                    builder.use_buffer(
                        indices,
                        vk::BufferUsageFlags::STORAGE_BUFFER,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
                    );
                },
                move |e, device| {
                    let (d, cmd) = (device.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, emit_triangles.raw());

                    bind_descriptor_copy(e, device);

                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count, count, count);
                },
            ),
            "Emit triangles",
        );

        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_buffer(
                        indices,
                        vk::BufferUsageFlags::TRANSFER_SRC,
                        vk::PipelineStageFlags2KHR::TRANSFER,
                        vk::AccessFlags2KHR::TRANSFER_READ,
                    );
                    builder.use_buffer(
                        draw_parameter_buffer,
                        vk::BufferUsageFlags::TRANSFER_DST,
                        vk::PipelineStageFlags2KHR::TRANSFER,
                        vk::AccessFlags2KHR::TRANSFER_WRITE,
                    );
                },
                move |e, d| {
                    let (d, cmd) = (d.device(), e.command_buffer());
                    // vk::DrawIndexedIndirectCommand {
                    //     index_count: u32,      variable
                    //     instance_count: u32,   1
                    //     first_index: u32,      0
                    //     vertex_offset: i32,    0
                    //     first_instance: u32,   0
                    // }
                    d.cmd_copy_buffer(
                        cmd,
                        e.get_buffer(indices),
                        e.get_buffer(draw_parameter_buffer),
                        &[vk::BufferCopy {
                            src_offset: 4,
                            dst_offset: 0,
                            size: 4,
                        }],
                    );
                    let data = [1u32, 0, 0, 0];
                    d.cmd_update_buffer(
                        cmd,
                        e.get_buffer(draw_parameter_buffer),
                        4,
                        data.len() as u64 * 4,
                        data.as_ptr().cast(),
                    );
                },
            ),
            "Prepare indirect draw",
        );

        let (attachments, resolve_attachments) = if MSAA_SAMPLE_COUNT == vk::SampleCountFlags::C1 {
            (vec![color], vec![])
        } else {
            (vec![color], vec![resolve.unwrap()])
        };

        b.add_pass(
            queue,
            MeshPass {
                pipeline,
                attachments,
                resolve_attachments,
                depth: Some(depth),
                vertices,
                indices,
                draw_parameter_buffer,
                transform: state.clone(),
            },
            "Draw triangles",
        );

        let egui_pass_copy = egui_pass.clone();
        b.add_pass(
            queue,
            LambdaPass(
                move |builder| {
                    builder.use_image(
                        window_color,
                        vk::ImageUsageFlags::COLOR_ATTACHMENT,
                        vk::PipelineStageFlags2KHR::COLOR_ATTACHMENT_OUTPUT,
                        vk::AccessFlags2KHR::COLOR_ATTACHMENT_WRITE,
                        vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                        None,
                    );
                    builder.use_image(
                        swapchain_image,
                        vk::ImageUsageFlags::COLOR_ATTACHMENT,
                        vk::PipelineStageFlags2KHR::COLOR_ATTACHMENT_OUTPUT,
                        vk::AccessFlags2KHR::COLOR_ATTACHMENT_WRITE,
                        vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                        None,
                    );

                    let function_texture = if MSAA_SAMPLE_COUNT != vk::SampleCountFlags::C1 {
                        resolve.unwrap()
                    } else {
                        color
                    };
                    builder.use_image(
                        function_texture,
                        vk::ImageUsageFlags::SAMPLED,
                        vk::PipelineStageFlags2KHR::FRAGMENT_SHADER,
                        vk::AccessFlags2KHR::SHADER_SAMPLED_READ,
                        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                        None,
                    );
                },
                move |e, d| {
                    let get_image_info = |image: GraphImage| {
                        let view = e.get_default_image_view(image);
                        let (create_flags, usage, format) = match e.get_image_create_info(image) {
                            ImageKindCreateInfo::ImageRef(i) => (Some(i.flags), i.usage, i.format),
                            ImageKindCreateInfo::Image(i) => (Some(i.flags), i.usage, i.format),
                            ImageKindCreateInfo::Swapchain(i) => (None, i.usage, i.format),
                        };
                        let (width, height) = e.get_image_extent(image).get_2d().unwrap();

                        (view, create_flags, usage, format, width, height)
                    };

                    let (
                        swapchain_view,
                        swapchain_flags,
                        swapchain_usage,
                        swapchain_format,
                        _swapchain_width,
                        _swapchain_height,
                    ) = get_image_info(swapchain_image);
                    let formats = &[swapchain_format];
                    let mut data = state.lock().unwrap();

                    let clear = vk::ClearColorValue { float_32: [0.0; 4] };
                    let (width, height) = e.get_image_extent(swapchain_image).get_2d().unwrap();
                    let config = {
                        let (
                            color_view,
                            color_flags,
                            color_usage,
                            _color_format,
                            color_width,
                            color_height,
                        ) = get_image_info(window_color);
                        assert_eq!([width, height], [color_width, color_height]);

                        PaintConfig {
                            command_buffer: e.command_buffer(),
                            queue_family: e.get_current_queue().family(),
                            submission: e.get_current_submission(),
                            color_view,
                            color_flags: color_flags.unwrap(),
                            color_usage: color_usage,
                            color_view_formats: formats,
                            resolve_view: swapchain_view,
                            resolve_flags: swapchain_flags.unwrap_or_default(),
                            resolve_usage: swapchain_usage,
                            resolve_view_formats: formats,
                            clear: Some(clear),
                            pixels_per_point: data.pixels_per_point,
                            primitives: &data.primitives,
                            textures_delta: &data.textures_delta,
                            size: [width, height],
                        }
                    };

                    let mut renderer = egui_pass_copy.lock().unwrap();
                    let tex = if MSAA_SAMPLE_COUNT == vk::SampleCountFlags::C1 {
                        color_image.clone()
                    } else {
                        resolve_image.clone().unwrap()
                    };
                    renderer.add_texture(
                        egui::TextureId::User(0),
                        tex,
                        egui::TextureOptions::NEAREST,
                        vk::ComponentMapping::default(),
                        d,
                    );
                    let copy_submissions = renderer.paint(&config, d);

                    for s in copy_submissions {
                        e.add_extra_submission_dependency(s);
                    }

                    data.primitives.clear();
                    data.textures_delta.clear();
                },
            ),
            "Draw egui",
        );
    });

    Ok(GraphOutput {
        vertices,
        indices,
        graph,
    })
}

/// Creates the vulkan context with the required extension and selects a device
unsafe fn make_device(
    window: &winit::window::Window,
) -> (
    object::Surface,
    device::OwnedDevice,
    device::submission::Queue,
) {
    let mut conf = ApiLoadConfig::new(vk::API_VERSION_1_1);
    conf.enable_surface_extensions(window);
    conf.add_extension(vk::KHR_SWAPCHAIN_EXTENSION_NAME);
    conf.add_extension(vk::KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME);
    conf.add_extension(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME);
    conf.add_extension(vk::KHR_SYNCHRONIZATION_2_EXTENSION_NAME);
    conf.add_extension(vk::KHR_DYNAMIC_RENDERING_EXTENSION_NAME);
    conf.add_extension(vk::EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME);
    conf.add_extension(vk::EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME);
    conf.add_extension(vk::KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME);
    conf.add_extension(vk::EXT_DEBUG_UTILS_EXTENSION_NAME);
    conf.add_extension(vk::KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME);

    conf.fill_in_extensions().unwrap();

    let info = InstanceCreateInfo {
        config: &mut conf,
        validation_layers: &[
            // pumice::cstr!("VK_LAYER_KHRONOS_validation"),
            // pumice::cstr!("VK_LAYER_LUNARG_api_dump"),
        ],
        enable_debug_callback: true,
        debug_labeling: true,
        app_name: "test application".to_owned(),
        verbose: false,
    };

    let instance = Instance::new(info);

    let surface = instance.create_surface(window).unwrap();

    let mut scalar_layout = vk::PhysicalDeviceScalarBlockLayoutFeaturesEXT {
        scalar_block_layout: vk::TRUE,
        ..Default::default()
    };
    let mut sync = vk::PhysicalDeviceSynchronization2FeaturesKHR {
        synchronization_2: vk::TRUE,
        p_next: (&mut scalar_layout) as *mut _ as *mut _,
        ..Default::default()
    };
    let mut timeline = vk::PhysicalDeviceTimelineSemaphoreFeaturesKHR {
        timeline_semaphore: vk::TRUE,
        p_next: (&mut sync) as *mut _ as *mut _,
        ..Default::default()
    };
    let mut dynamic = vk::PhysicalDeviceDynamicRenderingFeaturesKHR {
        dynamic_rendering: vk::TRUE,
        p_next: (&mut timeline) as *mut _ as *mut _,
        ..Default::default()
    };
    let imageless = vk::PhysicalDeviceImagelessFramebufferFeaturesKHR {
        imageless_framebuffer: vk::TRUE,
        p_next: (&mut dynamic) as *mut _ as *mut _,
        ..Default::default()
    };

    let info = DeviceCreateInfo {
        instance,
        config: &mut conf,
        device_features: vk::PhysicalDeviceFeatures {
            ..Default::default()
        },
        queue_family_selection: &[QueueFamilySelection {
            mask: vk::QueueFlags::GRAPHICS,
            count: 1,
            priority: 1.0,
            exact: false,
            attempt_dedicated: false,
            coalesce: true,
            support_surfaces: &[&surface],
        }],
        staging_transfer_queue: (0, 0),
        device_substrings: &["NVIDIA"],
        verbose: false,
        p_next: (&imageless) as *const _ as *const _,
    };

    let device = device::Device::new(info);
    let queue = device.get_queue_bundle(0, 0).unwrap();
    (surface, device, queue)
}

/// Creates the vulkan swapchain which allows rendering to a window. The object returned wraps [`vk::SwapchainKHR`] and automatically recreates it.
unsafe fn make_swapchain(
    window: &winit::window::Window,
    surface: object::Surface,
    device: &device::Device,
) -> object::Swapchain {
    let extent = {
        let size = window.inner_size();
        vk::Extent2D {
            width: size.width,
            height: size.height,
        }
    };

    // let (present_modes, result) = device
    //     .instance()
    //     .handle()
    //     .get_physical_device_surface_present_modes_khr(
    //         device.physical_device(),
    //         surface.handle(),
    //         None,
    //     )
    //     .unwrap();
    // assert_eq!(result, vk::Result::SUCCESS);

    let present_mode = vk::PresentModeKHR::FIFO;
    // for mode in present_modes {
    //     if mode == vk::PresentModeKHR::MAILBOX {
    //         present_mode = vk::PresentModeKHR::MAILBOX;
    //         break;
    //     }
    // }

    // TODO swapchain configuration fallback for formats, present modes, and color spaces
    let info = SwapchainCreateInfo {
        surface: surface.into_raw(),
        flags: vk::SwapchainCreateFlagsKHR::empty(),
        min_image_count: 3,
        format: vk::Format::B8G8R8A8_UNORM,
        color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        extent,
        array_layers: 1,
        usage: vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST,
        pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
        composite_alpha: vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
        present_mode,
        clipped: true,
    };

    device.create_swapchain(info).unwrap()
}
