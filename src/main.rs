#![allow(unused)]

mod hotreaload;
mod parser;
mod pass;
mod write;

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
use graph::device::{self, read_spirv, DeviceCreateInfo, QueueFamilySelection};
use graph::graph::compile::GraphCompiler;
use graph::instance::{Instance, InstanceCreateInfo, OwnedInstance};
use graph::object::{self, ImageCreateInfo, PipelineStage, SwapchainCreateInfo};
use graph::passes::{self, ClearImage, SimpleShader};
use graph::smallvec::{smallvec, SmallVec};
use graph::tracing::tracing_subscriber::install_tracing_subscriber;
use graph::vma::{AllocationCreateFlags, AllocationCreateInfo};
use hotreaload::ShaderModules;
use notify::event::{DataChange, ModifyKind};
use notify::Watcher;
use pumice::VulkanResult;
use pumice::{util::ApiLoadConfig, vk};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;
use write::{compile_glsl_to_spirv, make_density_function};

use crate::pass::ComputePass;
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
    let comp_module = modules.retrieve("shaders/compute.comp", true, device)?;

    let pipeline_layout = ReflectedLayout::new(&[(&comp_module.spirv, &["main"], true)])
        .create(device, vk::DescriptorSetLayoutCreateFlags::empty())
        .unwrap();

    let set_layout = pipeline_layout.get_descriptor_set_layouts()[0].clone();

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

    let pipeline = device.create_compute_pipeline(pipeline_info).unwrap();

    let graph = compiler.compile(device.clone(), |b| {
        let queue = b.import_queue(queue);
        let swapchain = b.acquire_swapchain(swapchain.clone());
        b.add_pass(
            queue,
            ComputePass {
                image: swapchain,
                pipeline,
                time: std::time::Instant::now(),
                state,
            },
            "",
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
    let mut conf = ApiLoadConfig::new(vk::API_VERSION_1_0);
    conf.add_extensions_iter(
        pumice::surface::enumerate_required_extensions(window)
            .unwrap()
            .into_iter()
            .cloned(),
    );
    conf.add_extension(vk::KHR_SWAPCHAIN_EXTENSION_NAME);
    conf.add_extension(vk::KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME);
    conf.add_extension(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME);
    conf.add_extension(vk::KHR_SYNCHRONIZATION_2_EXTENSION_NAME);
    conf.add_extension(vk::KHR_DYNAMIC_RENDERING_EXTENSION_NAME);
    conf.add_extension(vk::EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME);

    conf.fill_in_extensions();

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
        usage: vk::ImageUsageFlags::STORAGE,
        pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: vk::PresentModeKHR::FIFO,
        clipped: false,
    };

    device.create_swapchain(info).unwrap()
}
