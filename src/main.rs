#![allow(unused)]

mod hotreaload;
mod mesh_pass;
mod parser;
mod pass;
mod write;

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::{io, slice};

use graph::device::reflection::ReflectedLayout;
use graph::device::{self, read_spirv, Device, DeviceCreateInfo, QueueFamilySelection};
use graph::graph::compile::GraphCompiler;
use graph::graph::descriptors::{DescBuffer, DescImage, DescSetBuilder};
use graph::graph::execute::GraphExecutor;
use graph::graph::record::GraphPassBuilder;
use graph::instance::{Instance, InstanceCreateInfo, OwnedInstance};
use graph::object::{self, ImageCreateInfo, PipelineStage, SwapchainCreateInfo};
use graph::passes::{self, ClearImage, SimpleShader};
use graph::smallvec::{smallvec, SmallVec};
use graph::tracing::tracing_subscriber::install_tracing_subscriber;
use graph::vma::{self};
use hotreaload::ShaderModules;
use notify::event::{DataChange, ModifyKind};
use notify::Watcher;
use pass::LambdaPass;
use pumice::util::ObjectHandle;
use pumice::VulkanResult;
use pumice::{util::ApiLoadConfig, vk};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;
use write::{compile_glsl_to_spirv, make_density_function};

use crate::write::make_glsl_math;

pub struct ViewState {
    zoom: f32,
    x_shift: f32,
    y_shift: f32,
}

fn main() {
    unsafe {
        install_tracing_subscriber(None);
        let mut event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(512.0f32, 512.0f32))
            .with_always_on_top(true)
            .build(&event_loop)
            .unwrap();

        let (surface, device, queue) = make_device(&window);
        let swapchain = make_swapchain(&window, surface, &device);

        let mut modules = ShaderModules::new();
        let mut compiler = GraphCompiler::new();

        let state = Arc::new(Mutex::new(ViewState {
            zoom: 1.0,
            x_shift: 0.0,
            y_shift: 0.0,
        }));

        let mut graph = make_graph(
            &swapchain,
            queue,
            state.clone(),
            &mut modules,
            &mut compiler,
            &device,
        )
        .unwrap();

        let mut prev = std::time::Instant::now();
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut w_pressed = false;
        let mut s_pressed = false;
        let mut r_pressed = false;

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

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
                    WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => {
                        let mut lock = state.lock().unwrap();

                        if let Some(a) = input.virtual_keycode {
                            let pressed = input.state == ElementState::Pressed;
                            match a {
                                VirtualKeyCode::Up => up_pressed = pressed,
                                VirtualKeyCode::Down => down_pressed = pressed,
                                VirtualKeyCode::Left => left_pressed = pressed,
                                VirtualKeyCode::Right => right_pressed = pressed,
                                VirtualKeyCode::W => w_pressed = pressed,
                                VirtualKeyCode::S => s_pressed = pressed,
                                VirtualKeyCode::R => r_pressed = pressed,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    match modules.poll() {
                        hotreaload::PollResult::Recreate => {
                            let result = make_graph(
                                &swapchain,
                                queue,
                                state.clone(),
                                &mut modules,
                                &mut compiler,
                                &device,
                            );

                            match result {
                                Ok(ok) => graph = ok,
                                Err(err) => {
                                    eprintln!("Compilation error: {err}");
                                }
                            }
                        }
                        hotreaload::PollResult::Continue => {}
                        hotreaload::PollResult::Exit => {
                            control_flow.set_exit();
                        }
                    }

                    window.request_redraw();
                }
                Event::RedrawRequested(req) => {
                    let dt = prev.elapsed().as_secs_f32().min(0.5);
                    prev = std::time::Instant::now();

                    let mut lock = state.lock().unwrap();
                    let scale = 700.0 * lock.zoom * dt;

                    if up_pressed {
                        lock.y_shift += scale;
                    }
                    if down_pressed {
                        lock.y_shift -= scale;
                    }
                    if left_pressed {
                        lock.x_shift += scale;
                    }
                    if right_pressed {
                        lock.x_shift -= scale;
                    }
                    if w_pressed {
                        let f = 0.999f32.powf(dt.recip());
                        lock.zoom *= f;
                    }
                    if s_pressed {
                        let f = 0.999f32.powf(dt.recip());
                        lock.zoom /= f;
                    }
                    if r_pressed {
                        lock.zoom = 1.0;
                        lock.x_shift = 0.0;
                        lock.y_shift = 0.0;
                    }

                    lock.x_shift = lock.x_shift.round();
                    lock.y_shift = lock.y_shift.round();

                    lock.zoom = lock.zoom.clamp(0.001, 999.0);

                    drop(lock);

                    let size = window.inner_size();
                    if !(window.is_visible() == Some(false) || size.width == 0 || size.height == 0)
                    {
                        graph.run();
                    }
                    device.idle_cleanup_poll();
                }

                _ => (),
            }
        });
    }
}

unsafe fn make_graph(
    swapchain: &object::Swapchain,
    queue: device::submission::Queue,
    state: Arc<Mutex<ViewState>>,
    modules: &mut ShaderModules,
    compiler: &mut GraphCompiler,
    device: &device::OwnedDevice,
) -> Result<graph::graph::execute::CompiledGraph, Box<dyn Error>> {
    let mut create_pipeline = |path: &str, needs_eval_fn: bool| {
        let comp_module = modules.retrieve(path, needs_eval_fn, device).unwrap();

        let pipeline_layout = ReflectedLayout::new(&[(&comp_module.spirv, &["main"], false)])
            .create(device, vk::DescriptorSetLayoutCreateFlags::empty())
            .unwrap();

        let pipeline_info = object::ComputePipelineCreateInfo {
            flags: vk::PipelineCreateFlags::empty(),
            stage: PipelineStage {
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                stage: vk::ShaderStageFlags::COMPUTE,
                module: comp_module.module.clone(),
                name: "main".into(),
                specialization_info: None,
            },
            layout: pipeline_layout,
            base_pipeline: object::BasePipeline::None,
        };

        device.create_compute_pipeline(pipeline_info).unwrap()
    };

    let create_image = |format: vk::Format, size: object::Extent| {
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
                },
                vma::AllocationCreateInfo {
                    flags: vma::AllocationCreateFlags::empty(),
                    usage: vma::MemoryUsage::AutoPreferDevice,
                    ..Default::default()
                },
            )
            .unwrap()
    };

    let create_buffer = |usage: vk::BufferUsageFlags, size: u64| {
        device
            .create_buffer(
                object::BufferCreateInfo {
                    flags: vk::BufferCreateFlags::empty(),
                    size,
                    usage,
                    sharing_mode_concurrent: false,
                },
                vma::AllocationCreateInfo {
                    flags: vma::AllocationCreateFlags::empty(),
                    usage: vma::MemoryUsage::AutoPreferDevice,
                    ..Default::default()
                },
            )
            .unwrap()
    };

    let populate_grid = create_pipeline("shaders/populate_grid.comp", true);
    let intersect = create_pipeline("shaders/intersect.comp", true);
    let compute_vertex = create_pipeline("shaders/compute_vertex.comp", false);
    let emit_triangles = create_pipeline("shaders/emit_triangles.comp", false);

    let function_values = create_image(vk::Format::R16_SFLOAT, object::Extent::D3(64, 64, 64));
    let intersections = create_image(
        vk::Format::R8G8B8A8_UNORM,
        object::Extent::D3(64 * 3, 64, 64),
    );
    let vertex_indices = create_image(vk::Format::R32_UINT, object::Extent::D3(64, 64, 64));

    let vertices = create_buffer(
        vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::VERTEX_BUFFER,
        3 * 4 * 1024 * 512,
    );
    let indices = create_buffer(
        vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::INDEX_BUFFER,
        4 * 1024 * 1024,
    );

    let vert_module = modules
        .retrieve("shaders/mesh.vert", false, device)
        .unwrap();
    let frag_module = modules
        .retrieve("shaders/mesh.frag", false, device)
        .unwrap();

    let pipeline_layout = ReflectedLayout::new(&[
        (&vert_module.spirv, &["main"], false),
        (&frag_module.spirv, &["main"], false),
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
            front_face: vk::FrontFace::CLOCKWISE,
            line_width: 1.0,
            ..Default::default()
        })
        .multisample(object::state::Multisample {
            rasterization_samples: vk::SampleCountFlags::C1,
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
        .layout(pipeline_layout.clone())
        .finish();

    let pipeline = device.create_delayed_graphics_pipeline(pipeline_info);

    let graph = compiler.compile(device.clone(), |b| {
        let queue = b.import_queue(queue);
        // let swapchain = b.acquire_swapchain(swapchain.clone());

        let function_values = b.import_image((function_values, "function_values"));
        let intersections = b.import_image((intersections, "intersections"));
        let vertex_indices = b.import_image((vertex_indices, "vertex_indices"));

        let vertices = b.import_buffer((vertices, "vertices"));
        let indices = b.import_buffer((indices, "indices"));

        let swapchain_size = swapchain.get_extent();

        let depth = b.create_image(
            object::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                size: object::Extent::D2(swapchain_size.width, swapchain_size.height),
                format: vk::Format::D16_UNORM,
                samples: vk::SampleCountFlags::C1,
                mip_levels: 1,
                array_layers: 1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                sharing_mode_concurrent: false,
                initial_layout: vk::ImageLayout::UNDEFINED,
            },
            vma::AllocationCreateInfo {
                flags: vma::AllocationCreateFlags::empty(),
                usage: vma::MemoryUsage::Unknown,
                required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ..Default::default()
            },
            "depth",
        );

        let draw_parameter_buffer = b.create_buffer(
            object::BufferCreateInfo {
                flags: vk::BufferCreateFlags::empty(),
                size: std::mem::size_of::<vk::DrawIndexedIndirectCommand>() as u64,
                usage: vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDIRECT_BUFFER,
                sharing_mode_concurrent: false,
            },
            vma::AllocationCreateInfo {
                flags: vma::AllocationCreateFlags::empty(),
                usage: vma::MemoryUsage::Unknown,
                required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ..Default::default()
            },
            "draw_parameter_buffer",
        );
        let swapchain = b.acquire_swapchain(swapchain.clone());

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
                    d.cmd_update_buffer(
                        cmd,
                        e.get_buffer(vertices),
                        0,
                        8,
                        [1024 * 512, 0].as_ptr().cast(),
                    );
                    d.cmd_update_buffer(
                        cmd,
                        e.get_buffer(indices),
                        0,
                        8,
                        [1024 * 1024, 0].as_ptr().cast(),
                    );
                },
            ),
            "Reset buffer bump allocators",
        );

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
                move |e, d| {
                    let (d, cmd) = (d.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, populate_grid.raw());

                    let pipeline_layout = populate_grid.get_pipeline_layout();
                    let set_layout = &pipeline_layout.get_descriptor_set_layouts()[0];

                    DescSetBuilder::new(set_layout)
                        .update_image_binding(
                            0,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    function_values,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .finish(e)
                        .bind(vk::PipelineBindPoint::COMPUTE, pipeline_layout, e);

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
                        time: 0.0,
                    };

                    d.cmd_push_constants(
                        cmd,
                        pipeline_layout.raw(),
                        vk::ShaderStageFlags::COMPUTE,
                        0,
                        std::mem::size_of::<PushConstant>() as u32,
                        (&data as *const PushConstant).cast(),
                    );
                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count, count, count);
                },
            ),
            "Populate grid",
        );

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
                move |e, d| {
                    let (d, cmd) = (d.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, intersect.raw());

                    let pipeline_layout = intersect.get_pipeline_layout();
                    let set_layout = &pipeline_layout.get_descriptor_set_layouts()[0];

                    DescSetBuilder::new(set_layout)
                        .update_image_binding(
                            0,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    function_values,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .update_image_binding(
                            1,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    intersections,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .finish(e)
                        .bind(vk::PipelineBindPoint::COMPUTE, pipeline_layout, e);

                    struct PushConstant {
                        time: f32,
                    }

                    let data = PushConstant { time: 0.0 };

                    d.cmd_push_constants(
                        cmd,
                        pipeline_layout.raw(),
                        vk::ShaderStageFlags::COMPUTE,
                        0,
                        std::mem::size_of::<PushConstant>() as u32,
                        (&data as *const PushConstant).cast(),
                    );
                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count * 3, count, count);
                },
            ),
            "Intersect cell edges",
        );

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
                        indices,
                        vk::BufferUsageFlags::STORAGE_BUFFER,
                        vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
                        vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
                    );
                },
                move |e, d| {
                    let (d, cmd) = (d.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, compute_vertex.raw());

                    let pipeline_layout = compute_vertex.get_pipeline_layout();
                    let set_layout = &pipeline_layout.get_descriptor_set_layouts()[0];

                    DescSetBuilder::new(set_layout)
                        .update_image_binding(
                            0,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    intersections,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .update_image_binding(
                            1,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    vertex_indices,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .update_buffer_binding(
                            2,
                            0,
                            &DescBuffer {
                                buffer: e.get_buffer(indices),
                                ..Default::default()
                            },
                        )
                        .finish(e)
                        .bind(vk::PipelineBindPoint::COMPUTE, pipeline_layout, e);

                    let count = 64 / 4;
                    d.cmd_dispatch(cmd, count, count, count);
                },
            ),
            "Compute cell vertices",
        );

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
                move |e, d| {
                    let (d, cmd) = (d.device(), e.command_buffer());
                    d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, emit_triangles.raw());

                    let pipeline_layout = emit_triangles.get_pipeline_layout();
                    let set_layout = &pipeline_layout.get_descriptor_set_layouts()[0];

                    DescSetBuilder::new(set_layout)
                        .update_image_binding(
                            0,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    function_values,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .update_image_binding(
                            1,
                            0,
                            &DescImage {
                                sampler: vk::Sampler::null(),
                                view: e.get_default_image_view(
                                    vertex_indices,
                                    vk::ImageAspectFlags::COLOR,
                                ),
                                layout: vk::ImageLayout::GENERAL,
                            },
                        )
                        .update_buffer_binding(
                            2,
                            0,
                            &DescBuffer {
                                buffer: e.get_buffer(indices),
                                ..Default::default()
                            },
                        )
                        .finish(e)
                        .bind(vk::PipelineBindPoint::COMPUTE, pipeline_layout, e);

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

        b.add_pass(
            queue,
            mesh_pass::SimpleShader {
                pipeline,
                attachments: vec![swapchain],
                depth: Some(depth),
                vertices,
                indices,
                draw_parameter_buffer,
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
    conf.add_extension(vk::KHR_WAYLAND_SURFACE_EXTENSION_NAME);

    conf.fill_in_extensions().unwrap();

    let info = InstanceCreateInfo {
        config: &mut conf,
        validation_layers: &[
            pumice::cstr!("VK_LAYER_KHRONOS_validation"),
            // pumice::cstr!("VK_LAYER_LUNARG_api_dump"),
        ],
        enable_debug_callback: true,
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

    // TODO swapchain configuration fallback for formats, present modes, and color spaces
    let info = SwapchainCreateInfo {
        surface: surface.into_raw(),
        flags: vk::SwapchainCreateFlagsKHR::empty(),
        min_image_count: 2,
        format: vk::Format::B8G8R8A8_UNORM,
        color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        extent,
        array_layers: 1,
        usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: vk::PresentModeKHR::FIFO,
        clipped: false,
    };

    device.create_swapchain(info).unwrap()
}
