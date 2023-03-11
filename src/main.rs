#![allow(unused)]

mod arcball;
mod hotreaload;
mod mesh_pass;
mod parser;
mod pass;
pub mod recomputation;
mod write;
mod yawpitch;

use dolly::prelude::{Arm, Position, RightHanded, Smooth};
use dolly::rig::CameraRig;
use dolly::transform::Transform;
use glam::Vec3;
use graph::device::reflection::{ReflectedLayout, SpirvModule};
use graph::device::{self, Device, DeviceCreateInfo, QueueFamilySelection};
use graph::graph::compile::GraphCompiler;
use graph::graph::descriptors::{DescBuffer, DescImage, DescSetBuilder, DescriptorData};
use graph::graph::execute::{CompiledGraph, GraphExecutor, GraphRunConfig, GraphRunStatus};
use graph::instance::{Instance, InstanceCreateInfo};
use graph::object::{self, PipelineStage, SwapchainCreateInfo};
use graph::smallvec::{smallvec, SmallVec};
use graph::tracing::tracing_subscriber::install_tracing_subscriber;
use graph::vma;
use hotreaload::{PollResult, ShaderModules, ShaderModulesConfig};
use pass::LambdaPass;
use pumice::{util::ApiLoadConfig, vk};
use recomputation::RecomputationCache;
use std::cell::RefCell;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use yawpitch::YawPitchZUp;

pub const MSAA_SAMPLE_COUNT: vk::SampleCountFlags = vk::SampleCountFlags::C1;

fn main() {
    unsafe {
        install_tracing_subscriber(None);
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(512.0f32, 512.0f32))
            .build(&event_loop)
            .unwrap();

        let (surface, device, queue) = make_device(&window);
        let swapchain = make_swapchain(&window, surface, &device);

        let modules = ShaderModules::new(
            ShaderModulesConfig::WatchStdin,
            // ShaderModulesConfig::Static("sin(2 sqrt(x*x+y*y) / pi) * 25 + 30 - z"),
            // ShaderModulesConfig::Static("30 - z"),
        );
        let cache = RecomputationCache::new();
        let mut compiler = GraphCompiler::new();

        let mut camera: CameraRig = CameraRig::builder()
            .with(Position::new(Vec3::splat(32.0)))
            .with(YawPitchZUp::new().pitch_degrees(25.0).yaw_degrees(90.0))
            .with(Smooth::new_rotation(0.25))
            .with(Arm::new(Vec3::Z * 120.0))
            .build();

        let transform: Arc<Mutex<Transform<RightHanded>>> =
            Arc::new(Mutex::new(Transform::IDENTITY));

        let mut prev = std::time::Instant::now();
        let mut mouse_left_pressed = false;
        let mut mouse_right_pressed = false;

        let mut graph: Option<CompiledGraph> = None;

        let mut device_option = Some(device);
        let mut swapchain_option = Some(swapchain);
        let mut modules_option = Some(modules);
        let mut cache_option = Some(cache);

        event_loop.run(move |event, _, control_flow| {
            let mut device = device_option.as_mut().unwrap();
            let mut swapchain = swapchain_option.as_mut().unwrap();
            let mut modules = modules_option.as_mut().unwrap();
            let mut cache = cache_option.as_mut().unwrap();

            match event {
                Event::DeviceEvent { device_id, event } => match event {
                    DeviceEvent::MouseMotion { delta: (x, y) } => {
                        let m = 0.3;
                        if mouse_left_pressed {
                            camera
                                .driver_mut::<YawPitchZUp>()
                                .rotate_yaw_pitch(x as f32 * m, y as f32 * m);
                        }
                    }
                    _ => {}
                },
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
                        camera.driver_mut::<Arm>().offset.z -= delta * 2.0;
                    }
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        let pressed = state == ElementState::Pressed;
                        match button {
                            MouseButton::Left => mouse_left_pressed = pressed,
                            MouseButton::Right => mouse_right_pressed = pressed,
                            _ => {}
                        }
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    control_flow.set_poll();

                    let dt = prev.elapsed().as_secs_f32();
                    {
                        *transform.lock().unwrap() = camera.update(dt);
                    }

                    prev = std::time::Instant::now();

                    let mut exit = false;
                    let mut rebuild_graph = false;

                    match modules.poll() {
                        PollResult::Recreate => rebuild_graph = true,
                        PollResult::Skip => return,
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
                            let result = graph.run(GraphRunConfig {
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

                            if let Some(remaining) =
                                Duration::from_millis(1000 / 60).checked_sub(prev.elapsed())
                            {
                                if !rebuild_graph {
                                    std::thread::sleep(remaining);
                                }
                            }
                        }
                    }

                    device.idle_cleanup_poll();

                    if rebuild_graph && !exit {
                        let result = make_graph(
                            &swapchain,
                            queue,
                            transform.clone(),
                            &mut modules,
                            &mut compiler,
                            &mut cache,
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
                Event::LoopDestroyed => {
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
        });
    }
}

unsafe fn make_graph(
    swapchain: &object::Swapchain,
    queue: device::submission::Queue,
    state: Arc<Mutex<Transform<RightHanded>>>,
    modules: &mut ShaderModules,
    compiler: &mut GraphCompiler,
    cache: &mut RecomputationCache,
    device: &device::OwnedDevice,
) -> Result<CompiledGraph, Box<dyn Error>> {
    macro_rules! args {
        ($($arg:expr),*) => {
            |state| {
                use std::hash::Hash;
                $(
                    $arg.hash(state);
                )*
            }
        };
    }

    let common = modules.retrieve_simple("shaders/common.h");

    let cache = RefCell::new(cache);

    let mut create_pipeline = |path: &str, needs_eval_fn: bool| -> Result<_, Box<dyn Error>> {
        let mut cache = cache.borrow_mut();

        // this is getting criminal
        cache.compute_located(args!(path), args!(common), || {
            modules.invalidate_file(path);
        });
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

        let pipeline = cache.compute_located(args!(path), args!(common, module), || {
            let pipeline_info = object::ComputePipelineCreateInfo {
                flags: vk::PipelineCreateFlags::empty(),
                stage: PipelineStage {
                    flags: vk::PipelineShaderStageCreateFlags::empty(),
                    stage: vk::ShaderStageFlags::COMPUTE,
                    module: module.module.clone(),
                    name: "main".into(),
                    specialization_info: None,
                },
                layout: all_layout.into_inner(),
                base_pipeline: object::BasePipeline::None,
            };

            device.create_compute_pipeline(pipeline_info).unwrap()
        });

        Ok(pipeline.into_inner())
    };

    let create_image = |format: vk::Format, size: object::Extent, label: &'static str| {
        let mut cache = cache.borrow_mut();
        cache
            .compute_named(label, args!(format, size), || {
                device
                    .create_image(
                        object::ImageCreateInfo {
                            flags: vk::ImageCreateFlags::empty(),
                            size,
                            format,
                            samples: vk::SampleCountFlags::C1,
                            mip_levels: 1,
                            array_layers: 1,
                            tiling: vk::ImageTiling::OPTIMAL,
                            usage: vk::ImageUsageFlags::STORAGE,
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
            .into_inner()
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
                            flags: vma::AllocationCreateFlags::empty(),
                            usage: vma::MemoryUsage::AutoPreferDevice,
                            ..Default::default()
                        },
                    )
                    .unwrap()
            })
            .into_inner()
    };

    let populate_grid = create_pipeline("shaders/populate_grid.comp", true)?;
    let intersect = create_pipeline("shaders/intersect.comp", true)?;
    let compute_vertex = create_pipeline("shaders/compute_vertex.comp", true)?;
    let emit_triangles = create_pipeline("shaders/emit_triangles.comp", false)?;

    let function_values = create_image(
        vk::Format::R16_SFLOAT,
        object::Extent::D3(64, 64, 64),
        "function_values",
    );
    let intersections = create_image(
        vk::Format::R16G16B16A16_SNORM,
        object::Extent::D3(64 * 3, 64, 64),
        "intersections",
    );
    let vertex_indices = create_image(
        vk::Format::R32_UINT,
        object::Extent::D3(64, 64, 64),
        "vertex_indices",
    );

    const INDEX_CAPACITY: u32 = 1024 * 1024;
    const VERTEX_CAPACITY: u32 = 1024 * 512;

    let vertices = create_buffer(
        vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::VERTEX_BUFFER,
        (VERTEX_CAPACITY * 3 * 4) as u64,
        "vertices",
    );
    let indices = create_buffer(
        vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::INDEX_BUFFER,
        (INDEX_CAPACITY * 4) as u64,
        "indices",
    );

    let vert_module = modules.retrieve("shaders/mesh.vert", device)?;
    let frag_module = modules.retrieve("shaders/mesh.frag", device)?;

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
        .into_inner();

    let whole_descriptor_pipeline_layout = cache
        .borrow()
        .get_named::<object::PipelineLayout>("all layout")
        .expect("Compute pipelines have already been compiled, the all layout must be available")
        .into_inner();

    let graph = compiler.compile(device.clone(), |b| {
        let queue = b.import_queue(queue);
        let swapchain_image = b.acquire_swapchain(swapchain.clone());

        let function_values = b.import_image((function_values, "function_values"));
        let intersections = b.import_image((intersections, "intersections"));
        let vertex_indices = b.import_image((vertex_indices, "vertex_indices"));

        let vertices = b.import_buffer((vertices, "vertices"));
        let indices = b.import_buffer((indices, "indices"));

        struct GlobalDescriptorData {
            set: vk::DescriptorSet,
            dynamic_offsets: SmallVec<[u32; 4]>,
        }

        let whole_descriptor_pipeline_layout_copy = whole_descriptor_pipeline_layout.clone();
        let get_or_create_descriptor =
            move |e: &GraphExecutor, device: &Device| -> GlobalDescriptorData {
                // layout(binding = 0, r16) uniform image3D function_values;
                // layout(binding = 1, rgba8) uniform image3D intersections;
                // layout(binding = 2, r32ui) uniform uimage3D vertex_indices;
                // layout(binding = 3, scalar) buffer VertexData {
                //     uint size;
                //     uint offset;
                // 	Vertex vertices[];
                // } vertices;
                // layout(binding = 4, scalar) buffer IndexData {
                //     uint size;
                //     uint offset;
                // 	uint indices[];
                // } indices;
                // layout(binding = 5, scalar) uniform FunctionData {
                // 	uvec3 size;
                //     vec3 offset;
                //     float scale;
                //     float time;
                // } function_data;

                struct PushConstant {
                    size: [u32; 3],
                    offset: [f32; 3],
                    scale: f32,
                    time: f32,
                }

                let data = PushConstant {
                    size: [64; 3],
                    offset: [0.0; 3],
                    scale: 1.0,
                    // TODO plumb around a common elapsed time and delta
                    time: 0.0,
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
                    DescriptorData::Buffer(DescBuffer {
                        buffer: uniform.buffer,
                        dynamic_offset: Some(uniform.dynamic_offset),
                        ..Default::default()
                    }),
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
            let data = blackboard.get_or_insert_with::<GlobalDescriptorData, _>(|| {
                get_or_create_descriptor(e, device)
            });

            device.device().cmd_bind_descriptor_sets(
                e.command_buffer(),
                vk::PipelineBindPoint::COMPUTE,
                whole_descriptor_pipeline_layout.raw(),
                0,
                &[data.set],
                &data.dynamic_offsets,
            )
        };

        let swapchain_size = swapchain.get_extent();
        let depth = b.create_image(
            object::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                size: object::Extent::D2(swapchain_size.width, swapchain_size.height),
                format: vk::Format::D32_SFLOAT,
                samples: MSAA_SAMPLE_COUNT,
                mip_levels: 1,
                array_layers: 1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                sharing_mode_concurrent: false,
                initial_layout: vk::ImageLayout::UNDEFINED,
                label: Some("depth".into()),
            },
            vma::AllocationCreateInfo {
                flags: vma::AllocationCreateFlags::empty(),
                usage: vma::MemoryUsage::Unknown,
                required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ..Default::default()
            },
        );
        let color = b.create_image(
            object::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                size: object::Extent::D2(swapchain_size.width, swapchain_size.height),
                format: vk::Format::B8G8R8A8_UNORM,
                samples: MSAA_SAMPLE_COUNT,
                mip_levels: 1,
                array_layers: 1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
                sharing_mode_concurrent: false,
                initial_layout: vk::ImageLayout::UNDEFINED,
                label: Some("color".into()),
            },
            vma::AllocationCreateInfo {
                flags: vma::AllocationCreateFlags::empty(),
                usage: vma::MemoryUsage::Unknown,
                required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ..Default::default()
            },
        );

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
            (vec![swapchain_image], vec![])
        } else {
            (vec![color], vec![swapchain_image])
        };

        b.add_pass(
            queue,
            mesh_pass::SimpleShader {
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
    });

    Ok(graph)
}

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
    let dynamic = vk::PhysicalDeviceDynamicRenderingFeaturesKHR {
        dynamic_rendering: vk::TRUE,
        p_next: (&mut timeline) as *mut _ as *mut _,
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
        device_substrings: &["NVIDIA"],
        verbose: false,
        p_next: (&dynamic) as *const _ as *const _,
    };

    let device = device::Device::new(info);
    let queue = device.get_queue_bundle(0, 0).unwrap();
    (surface, device, queue)
}

unsafe fn make_swapchain(
    window: &winit::window::Window,
    surface: object::Surface,
    device: &device::OwnedDevice,
) -> object::Swapchain {
    let extent = {
        let size = window.inner_size();
        vk::Extent2D {
            width: size.width,
            height: size.height,
        }
    };

    let (present_modes, result) = device
        .instance()
        .handle()
        .get_physical_device_surface_present_modes_khr(
            device.physical_device(),
            surface.handle(),
            None,
        )
        .unwrap();
    assert_eq!(result, vk::Result::SUCCESS);

    let mut present_mode = vk::PresentModeKHR::FIFO;
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
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode,
        clipped: false,
    };

    device.create_swapchain(info).unwrap()
}
