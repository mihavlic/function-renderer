//! The renderpass which draw the egui triangle list.
use egui::{Color32, TextureId, TexturesDelta};
use graph::{
    device::{
        batch::GenerationId, maybe_attach_debug_label, staging::ImageRegion,
        submission::QueueSubmission, Device, LazyDisplay,
    },
    graph::descriptors::DescriptorSetAllocator,
    object::{self, ObjRef},
    smallvec::{smallvec, SmallVec},
    storage::DefaultAhashRandomstate,
    util::ffi_ptr::AsFFiPtr,
};
use pumice::{util::ObjectHandle, vk};
use pumice_vma as vma;
use slice_group_by::GroupByMut;
use std::{
    collections::{hash_map::Entry, HashMap},
    ffi::c_void,
};

const VERTEX_BUFFER_COUNT: usize = 1024 * 1024 * 4;
const INDEX_BUFFER_COUNT: usize = 1024 * 1024 * 2;

const VERTEX_BUFFER_BYTES: u64 =
    (VERTEX_BUFFER_COUNT * std::mem::size_of::<egui::epaint::Vertex>()) as u64;
const INDEX_BUFFER_BYTES: u64 = (INDEX_BUFFER_COUNT * 4) as u64;

/// Vulkan push constants.
#[repr(C)]
pub struct PushConstants {
    screen_size: [f32; 2],
}

/// An entry which an image registered with the gui.
pub struct TextureImage {
    pub image: object::Image,
    /// The color channel swizzle.
    pub components: vk::ComponentMapping,
    pub set: vk::DescriptorSet,
    pub view: vk::ImageView,
    pub options: egui::TextureOptions,
}

/// The persistent state of the rendeerer.
pub struct Renderer {
    resolve_attachment: bool,

    render_pass: object::RenderPass,
    framebuffer: Option<object::Framebuffer>,

    set_allocator: EguiDescriptorSetAllocator,
    desc_layout: object::DescriptorSetLayout,
    pipeline: object::ConcreteGraphicsPipeline,

    vertex_buffer: (object::Buffer, *mut egui::epaint::Vertex),
    index_buffer: (object::Buffer, *mut u32),

    pending_retired_sets: Vec<vk::DescriptorSet>,
    pending_retired_textures: Vec<egui::TextureId>,

    samplers: HashMap<egui::TextureOptions, object::Sampler, DefaultAhashRandomstate>,
    texture_images: HashMap<egui::TextureId, TextureImage, DefaultAhashRandomstate>,
}

pub struct RendererConfig {
    pub output_attachment_is_unorm_nonlinear: bool,
    pub format: vk::Format,

    pub samples: vk::SampleCountFlags,

    pub color_load_op: vk::AttachmentLoadOp,
    pub color_store_op: vk::AttachmentStoreOp,

    pub color_src_layout: vk::ImageLayout,
    pub color_src_stages: vk::PipelineStageFlags,
    pub color_src_access: vk::AccessFlags,
    pub color_final_layout: vk::ImageLayout,

    /// Whether to do a resolve operation, if this is false all resolve_* arguments are ignored.
    pub resolve_enable: bool,

    pub resolve_load_op: vk::AttachmentLoadOp,
    pub resolve_store_op: vk::AttachmentStoreOp,

    pub resolve_src_layout: vk::ImageLayout,
    pub resolve_src_stages: vk::PipelineStageFlags,
    pub resolve_src_access: vk::AccessFlags,
    pub resolve_final_layout: vk::ImageLayout,
}

pub struct PaintConfig<'a> {
    /// The command buffer into which the paint will be recorded.
    pub command_buffer: vk::CommandBuffer,
    /// The queue family into which the paint will be recorded.
    pub queue_family: u32,
    /// The submission into which the paint will be recorded.
    pub submission: QueueSubmission,

    pub color_view: vk::ImageView,
    pub color_flags: vk::ImageCreateFlags,
    pub color_usage: vk::ImageUsageFlags,
    pub color_view_formats: &'a [vk::Format],

    pub resolve_view: vk::ImageView,
    pub resolve_flags: vk::ImageCreateFlags,
    pub resolve_usage: vk::ImageUsageFlags,
    pub resolve_view_formats: &'a [vk::Format],

    pub clear: Option<vk::ClearColorValue>,
    pub pixels_per_point: f32,

    /// Primitives to draw.
    pub primitives: &'a [egui::ClippedPrimitive],
    /// Texture updates before draw.
    pub textures_delta: &'a TexturesDelta,
    pub size: [u32; 2],
}

impl Renderer {
    pub unsafe fn new(config: &RendererConfig, device: &Device) -> Renderer {
        let &RendererConfig {
            output_attachment_is_unorm_nonlinear,
            format,
            samples,
            color_load_op,
            color_store_op,
            color_src_layout,
            color_src_stages,
            color_src_access,
            color_final_layout,
            resolve_enable,
            resolve_load_op,
            resolve_store_op,
            resolve_src_layout,
            resolve_src_stages,
            resolve_src_access,
            resolve_final_layout,
        } = config;

        if resolve_enable {
            assert!(samples != vk::SampleCountFlags::C1);
        }

        let render_pass = {
            let mut attachments = Vec::new();
            let mut dependencies = Vec::new();

            // color
            let color_ref = vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
            };
            attachments.push(vk::AttachmentDescription {
                format,
                samples,
                load_op: color_load_op,
                store_op: color_store_op,
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                initial_layout: color_src_layout,
                final_layout: color_final_layout,
                ..Default::default()
            });
            dependencies.push(vk::SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                dst_subpass: 0,
                src_stage_mask: color_src_stages,
                dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                src_access_mask: color_src_access,
                dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                ..Default::default()
            });

            // color resolve
            let mut resolve_ref = None;
            if resolve_enable {
                resolve_ref = Some(vk::AttachmentReference {
                    attachment: 1,
                    layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                });
                attachments.push(vk::AttachmentDescription {
                    format,
                    samples: vk::SampleCountFlags::C1,
                    load_op: resolve_load_op,
                    store_op: resolve_store_op,
                    stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                    stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                    initial_layout: resolve_src_layout,
                    final_layout: resolve_final_layout,
                    ..Default::default()
                });
                dependencies.push(vk::SubpassDependency {
                    src_subpass: vk::SUBPASS_EXTERNAL,
                    dst_subpass: 0,
                    src_stage_mask: resolve_src_stages,
                    dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    src_access_mask: resolve_src_access,
                    dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                    ..Default::default()
                });
            }

            let subpass = object::SubpassDescription {
                color_attachments: vec![color_ref],
                resolve_attachments: resolve_ref.into_iter().collect(),
                ..Default::default()
            };

            device
                .create_render_pass(object::RenderPassCreateInfo {
                    attachments,
                    subpasses: vec![subpass],
                    dependencies,
                    ..Default::default()
                })
                .unwrap()
        };

        let (pipeline, desc_layout) = {
            const VS_BYTES: &[u8] = include_bytes!("../../shaders/gui/vertex.spv");
            const FS_BYTES: &[u8] = include_bytes!("../../shaders/gui/fragment.spv");

            let vs = device
                .create_shader_module_spirv_unaligned(VS_BYTES)
                .unwrap();
            let fs = device
                .create_shader_module_spirv_unaligned(FS_BYTES)
                .unwrap();

            let set_layout = device
                .create_descriptor_set_layout(object::DescriptorSetLayoutCreateInfo {
                    flags: vk::DescriptorSetLayoutCreateFlags::empty(),
                    bindings: vec![object::DescriptorBinding {
                        binding: 0,
                        count: 1,
                        kind: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                        stages: vk::ShaderStageFlags::FRAGMENT,
                        immutable_samplers: smallvec![],
                    }],
                })
                .unwrap();

            let layout = device
                .create_pipeline_layout(object::PipelineLayoutCreateInfo {
                    set_layouts: vec![set_layout.clone()],
                    push_constants: vec![vk::PushConstantRange {
                        stage_flags: vk::ShaderStageFlags::VERTEX,
                        offset: 0,
                        size: std::mem::size_of::<PushConstants>() as u32,
                    }],
                })
                .unwrap();

            let specialization = {
                let srgb_textures = false;
                let output_srgb_fragment = output_attachment_is_unorm_nonlinear;
                let data = [srgb_textures as u32, output_srgb_fragment as u32];
                let bytes = std::slice::from_raw_parts(data.as_ptr().cast::<u8>(), 8);

                object::SpecializationInfo {
                    map_entries: vec![
                        vk::SpecializationMapEntry {
                            constant_id: 0,
                            offset: 0,
                            size: 4,
                        },
                        vk::SpecializationMapEntry {
                            constant_id: 1,
                            offset: 4,
                            size: 4,
                        },
                    ],
                    data: bytes.to_vec(),
                }
            };

            let info = object::GraphicsPipelineCreateInfo::builder()
                .stages([
                    object::PipelineStage {
                        flags: vk::PipelineShaderStageCreateFlags::empty(),
                        stage: vk::ShaderStageFlags::VERTEX,
                        module: vs,
                        name: "main".into(),
                        specialization_info: None,
                    },
                    object::PipelineStage {
                        flags: vk::PipelineShaderStageCreateFlags::empty(),
                        stage: vk::ShaderStageFlags::FRAGMENT,
                        module: fs,
                        name: "main".into(),
                        specialization_info: Some(specialization),
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
                        stride: std::mem::size_of::<egui::epaint::Vertex>() as u32,
                        input_rate: vk::VertexInputRate::VERTEX,
                    }]
                    .to_vec(),
                    vertex_attributes: [
                        object::state::InputAttribute {
                            location: 0,
                            binding: 0,
                            format: vk::Format::R32G32_SFLOAT,
                            offset: 0,
                        },
                        object::state::InputAttribute {
                            location: 1,
                            binding: 0,
                            format: vk::Format::R32G32_SFLOAT,
                            offset: 8,
                        },
                        object::state::InputAttribute {
                            location: 2,
                            binding: 0,
                            format: vk::Format::R8G8B8A8_UNORM,
                            offset: 16,
                        },
                    ]
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
                    rasterization_samples: samples,
                    sample_shading_enable: false,
                    ..Default::default()
                })
                .depth_stencil(object::state::DepthStencil::default())
                .color_blend(object::state::ColorBlend {
                    attachments: [object::state::Attachment {
                        blend_enable: vk::TRUE,
                        color_write_mask: vk::ColorComponentFlags::all(),
                        src_color_blend_factor: vk::BlendFactor::ONE,
                        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                        color_blend_op: vk::BlendOp::ADD,
                        src_alpha_blend_factor: vk::BlendFactor::SRC_ALPHA,
                        dst_alpha_blend_factor: vk::BlendFactor::DST_ALPHA,
                        alpha_blend_op: vk::BlendOp::ADD,
                    }]
                    .to_vec(),
                    ..Default::default()
                })
                .dynamic_state([vk::DynamicState::SCISSOR, vk::DynamicState::VIEWPORT])
                .render_pass(object::RenderPassMode::Normal {
                    subpass: 0,
                    render_pass: render_pass.clone(),
                })
                .layout(layout)
                .finish();

            (device.create_graphics_pipeline(info).unwrap(), set_layout)
        };

        let vertex_buffer = Self::create_buffer(
            VERTEX_BUFFER_BYTES,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            "Egui vertex buffer",
            device,
        );

        let index_buffer = Self::create_buffer(
            INDEX_BUFFER_BYTES,
            vk::BufferUsageFlags::INDEX_BUFFER,
            "Egui index buffer",
            device,
        );

        Renderer {
            resolve_attachment: resolve_enable,
            render_pass,
            framebuffer: None,
            set_allocator: EguiDescriptorSetAllocator::new(),
            desc_layout,
            pipeline,
            vertex_buffer,
            index_buffer,
            pending_retired_sets: Vec::new(),
            pending_retired_textures: Vec::new(),
            samplers: Default::default(),
            texture_images: Default::default(),
        }
    }

    /// Allocate an internal buffer.
    unsafe fn create_buffer<T>(
        size: u64,
        usage: vk::BufferUsageFlags,
        label: &'static str,
        device: &Device,
    ) -> (object::Buffer, *mut T) {
        let buffer = device
            .create_buffer(
                object::BufferCreateInfo {
                    flags: vk::BufferCreateFlags::empty(),
                    size,
                    usage,
                    sharing_mode_concurrent: false,
                    label: Some(label.into()),
                },
                vma::AllocationCreateInfo {
                    flags: vma::AllocationCreateFlags::MAPPED
                        | vma::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                    required_flags: vk::MemoryPropertyFlags::HOST_VISIBLE,
                    usage: vma::MemoryUsage::AutoPreferDevice,
                    ..Default::default()
                },
            )
            .unwrap();

        let info = device
            .allocator()
            .get_allocation_info(buffer.get_allocation());

        let align = std::mem::align_of::<T>();
        assert!(
            (info.mapped_data as usize & (align - 1)) == 0,
            "Pointer is not properly aligned"
        );

        (buffer, info.mapped_data.cast())
    }

    /// Get or create a sampler according to [`egui::TextureOptions`]
    unsafe fn get_sampler(
        samplers: &mut HashMap<egui::TextureOptions, object::Sampler, DefaultAhashRandomstate>,
        options: egui::TextureOptions,
        device: &Device,
    ) -> vk::Sampler {
        let to_vk = |filter: egui::TextureFilter| match filter {
            egui::TextureFilter::Nearest => vk::Filter::NEAREST,
            egui::TextureFilter::Linear => vk::Filter::LINEAR,
        };

        samplers
            .entry(options)
            .or_insert_with(|| {
                device
                    .create_descriptor_sampler(object::SamplerCreateInfo {
                        mag_filter: to_vk(options.magnification),
                        min_filter: to_vk(options.minification),
                        mipmap_mode: vk::SamplerMipmapMode::NEAREST,
                        address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                        address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                        ..Default::default()
                    })
                    .unwrap()
            })
            .get_handle()
    }

    /// Update the registered textures with a [`TexturesDelta`]
    #[must_use]
    unsafe fn update_textures(
        &mut self,
        delta: &TexturesDelta,
        cmd: vk::CommandBuffer,
        queue_family: u32,
        submission: QueueSubmission,
        device: &Device,
    ) -> SmallVec<[QueueSubmission; 4]> {
        for tex in self.pending_retired_textures.drain(..) {
            let entry = self.texture_images.remove(&tex).unwrap();
            self.set_allocator.free_set(entry.set);
        }

        self.pending_retired_textures
            .extend(delta.free.iter().copied());

        let mut texture_deltas: Vec<_> = delta.set.iter().cloned().collect();
        texture_deltas.sort_by_key(|d| d.0);

        struct ImageBlitRegion {
            offset: [i32; 2],
            extent: [u32; 2],
            data: egui::ImageData,
        }
        let mut output_blits: Vec<(TextureId, bool, Vec<ImageBlitRegion>)> = Vec::new();

        for mut image_blits in texture_deltas.binary_group_by_key_mut(|d| d.0) {
            let texture_id = image_blits[0].0;
            let mut regions = Vec::new();
            let mut image_is_rgba = None;

            // if there are multiple full-texture-rewrites, discard all but the last one
            if let Some(found_full) = image_blits.iter().rposition(|(_, blit)| blit.pos.is_none()) {
                image_blits = &mut image_blits[found_full..];
            }

            for (_, blit) in image_blits {
                let width = blit.image.width() as u32;
                let height = blit.image.height() as u32;

                if width == 0 || height == 0 {
                    continue;
                }

                assert_eq!(
                    [width as usize, height as usize],
                    blit.image.size(),
                    "Mismatch between texture size and texel count"
                );

                let is_rgba = match blit.image {
                    egui::ImageData::Color(_) => true,
                    egui::ImageData::Font(_) => false,
                };

                match image_is_rgba {
                    Some(prev_is_rgba) => assert_eq!(prev_is_rgba, is_rgba),
                    None => image_is_rgba = Some(is_rgba),
                }

                let entry = self.texture_images.entry(texture_id);

                'handle_blit: {
                    // this means that a new image is to be created with the size of this delta
                    if let Some(pos) = blit.pos {
                        let Entry::Occupied(mut v) = entry else {
                           panic!("Updating a not yet initialized image");
                        };

                        let texture = &mut v.get_mut();
                        let (texture_width, texture_height) =
                            texture.image.get_create_info().size.get_2d().unwrap();

                        if (pos[0] as u32 + width > texture_width)
                            || (pos[1] as u32 + height > texture_height)
                        {
                            panic!("Write out of bounds for texture {:?}", texture_id);
                        }

                        assert_eq!(
                            texture.options, blit.options,
                            "Cannot change texture filtering through a partial delta"
                        );
                    } else {
                        let components = if is_rgba {
                            vk::ComponentMapping {
                                r: vk::ComponentSwizzle::IDENTITY,
                                g: vk::ComponentSwizzle::IDENTITY,
                                b: vk::ComponentSwizzle::IDENTITY,
                                a: vk::ComponentSwizzle::IDENTITY,
                            }
                        } else {
                            vk::ComponentMapping {
                                r: vk::ComponentSwizzle::R,
                                g: vk::ComponentSwizzle::R,
                                b: vk::ComponentSwizzle::R,
                                a: vk::ComponentSwizzle::R,
                            }
                        };

                        // we may get creation requests multiple times (at least for the font texture)
                        if let Entry::Occupied(mut v) = entry {
                            let old = v.get();
                            let (old_width, old_height) =
                                old.image.get_create_info().size.get_2d().unwrap();

                            if width == old_width
                                && height == old_height
                                && old.components == components
                            {
                                let texture = v.get_mut();
                                if texture.options != blit.options {
                                    let sampler =
                                        Self::get_sampler(&mut self.samplers, blit.options, device);
                                    let new_set = self.set_allocator.allocate(
                                        texture.view,
                                        sampler,
                                        &self.desc_layout,
                                        device,
                                    );

                                    self.pending_retired_sets.push(texture.set);
                                    texture.options = blit.options;
                                    texture.set = new_set;
                                };

                                break 'handle_blit;
                            } else {
                                let old = v.remove();
                                self.pending_retired_sets.push(old.set);
                                // the rest is either not owned by the image or is cleaned up automatically
                            }
                        };

                        let (character, index) = match texture_id {
                            egui::TextureId::Managed(i) => ('M', i),
                            egui::TextureId::User(i) => ('U', i),
                        };

                        let format = if is_rgba {
                            vk::Format::R8G8B8A8_UNORM
                        } else {
                            vk::Format::R8_UNORM
                        };

                        let image = device
                            .create_image(
                                object::ImageCreateInfo {
                                    flags: vk::ImageCreateFlags::empty(),
                                    size: object::Extent::D2(width, height),
                                    // we explicitly want a UNORM format
                                    // because we want the fragment shader to not
                                    // convert our colors to linear space
                                    format,
                                    samples: vk::SampleCountFlags::C1,
                                    mip_levels: 1,
                                    array_layers: 1,
                                    tiling: vk::ImageTiling::OPTIMAL,
                                    usage: vk::ImageUsageFlags::SAMPLED
                                        | vk::ImageUsageFlags::TRANSFER_DST,
                                    sharing_mode_concurrent: false,
                                    initial_layout: vk::ImageLayout::UNDEFINED,
                                    label: Some(format!("Egui texture {character}#{index}").into()),
                                },
                                vma::AllocationCreateInfo {
                                    flags: vma::AllocationCreateFlags::empty(),
                                    preferred_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                                    usage: vma::MemoryUsage::AutoPreferDevice,
                                    ..Default::default()
                                },
                            )
                            .unwrap();

                        self.add_texture(texture_id, image, blit.options, components, device);
                    }
                }

                let region = ImageBlitRegion {
                    offset: blit
                        .pos
                        .map(|[x, y]| [x as i32, y as i32])
                        .unwrap_or_default(),
                    extent: [width, height],
                    data: std::mem::replace(
                        &mut blit.image,
                        egui::ImageData::Color(egui::ColorImage::default()),
                    ),
                };

                regions.push(region);
            }

            output_blits.push((texture_id, image_is_rgba.unwrap(), regions));
        }

        if output_blits.is_empty() {
            return SmallVec::new();
        }

        let submissions = device.write_multiple(|staging| {
            let mut submissions = SmallVec::new();
            for (image, is_rgba, blits) in output_blits {
                let image = self.texture_images.get(&image).unwrap();
                // TODO create a better api that doesn't require a vector
                let regions = blits
                    .iter()
                    .map(|b| ImageRegion {
                        buffer_row_length: 0,
                        buffer_image_height: 0,
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        image_offset: vk::Offset3D {
                            x: b.offset[0],
                            y: b.offset[1],
                            z: 0,
                        },
                        image_extent: vk::Extent3D {
                            width: b.extent[0],
                            height: b.extent[1],
                            depth: 1,
                        },
                    })
                    .collect();

                let layout = if is_rgba {
                    std::alloc::Layout::new::<Color32>()
                } else {
                    std::alloc::Layout::new::<u8>()
                };

                let wait_for = staging.write_image_in_cmd(
                    &image.image,
                    Some(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL),
                    layout,
                    regions,
                    cmd,
                    queue_family,
                    submission,
                    device,
                    move |ptr, i, _| match &blits[i].data {
                        egui::ImageData::Color(image) => {
                            ptr.copy_from_nonoverlapping(
                                image.pixels.as_ptr().cast::<u8>(),
                                image.width() * image.height() * 4,
                            );
                        }
                        egui::ImageData::Font(image) => {
                            let mut ptr = ptr;
                            for coverage in image.pixels.iter() {
                                let gamma = 0.55;
                                let alpha = coverage.powf(gamma);
                                let quantized = (alpha * 255.0 + 0.5).floor() as u8;
                                *ptr = quantized;
                                ptr = ptr.add(1);
                            }
                        }
                    },
                );
                submissions.extend(wait_for);

                // we do the barrier ourselves because we'll be sampling from the image next
                let barrier = vk::ImageMemoryBarrier2KHR {
                    src_stage_mask: vk::PipelineStageFlags2KHR::TRANSFER,
                    src_access_mask: vk::AccessFlags2KHR::TRANSFER_WRITE,
                    old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    dst_stage_mask: vk::PipelineStageFlags2KHR::FRAGMENT_SHADER,
                    dst_access_mask: vk::AccessFlags2KHR::SHADER_SAMPLED_READ,
                    image: image.image.get_handle(),
                    subresource_range: image.image.get_whole_subresource_range(),
                    ..Default::default()
                };
                let dependency_info = vk::DependencyInfoKHR {
                    image_memory_barrier_count: 1,
                    p_image_memory_barriers: &barrier,
                    ..Default::default()
                };
                device
                    .device()
                    .cmd_pipeline_barrier_2_khr(cmd, &dependency_info);
            }
            submissions
        });

        submissions
    }

    /// Record paint commands.
    pub unsafe fn paint(
        &mut self,
        config: &PaintConfig,
        device: &Device,
    ) -> SmallVec<[QueueSubmission; 4]> {
        let &PaintConfig {
            command_buffer: cmd,
            queue_family,
            submission,
            color_view,
            color_flags,
            color_usage,
            color_view_formats,
            resolve_view,
            resolve_flags,
            resolve_usage,
            resolve_view_formats,
            clear,
            pixels_per_point,
            primitives,
            textures_delta,
            size: [width, height],
        } = config;

        if self.resolve_attachment {
            assert!(resolve_view != vk::ImageView::null());
        }

        let wait_submission =
            self.update_textures(textures_delta, cmd, queue_family, submission, device);

        let framebuffer_size_differs = || {
            let info = self.framebuffer.as_ref().unwrap().get_create_info();
            width != info.width || height != info.height
        };
        if self.framebuffer.is_none() || framebuffer_size_differs() {
            drop(self.framebuffer.take());

            let mut attachments = Vec::new();
            attachments.push(object::ImagelessAttachment {
                flags: color_flags,
                usage: color_usage,
                width,
                height,
                layer_count: 1,
                view_formats: color_view_formats.into(),
            });
            if self.resolve_attachment {
                attachments.push(object::ImagelessAttachment {
                    flags: resolve_flags,
                    usage: resolve_usage,
                    width,
                    height,
                    layer_count: 1,
                    view_formats: resolve_view_formats.into(),
                });
            }

            self.framebuffer = Some(
                device
                    .create_framebuffer(object::FramebufferCreateInfo {
                        flags: vk::FramebufferCreateFlags::IMAGELESS_KHR,
                        render_pass: self.render_pass.clone(),
                        attachments: object::AttachmentMode::Imageless(attachments),
                        width,
                        height,
                        layers: 1,
                    })
                    .unwrap(),
            );
        }

        let framebuffer = self.framebuffer.as_deref().unwrap();
        let d = device.device();

        {
            let mut clear_value = None;
            let mut clear_value_count = 0;
            if let Some(color) = clear {
                clear_value = Some(vk::ClearValue { color });
                clear_value_count = 1;
            }

            let attachments = [color_view, resolve_view];

            let attachment_info = vk::RenderPassAttachmentBeginInfoKHR {
                attachment_count: 1 + self.resolve_attachment as u32,
                p_attachments: attachments.as_ptr(),
                ..Default::default()
            };

            let begin = vk::RenderPassBeginInfo {
                p_next: &attachment_info as *const _ as *const _,
                render_pass: self.render_pass.get_handle(),
                framebuffer: framebuffer.get_handle(),
                render_area: vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: vk::Extent2D { width, height },
                },
                clear_value_count,
                p_clear_values: clear_value.as_ffi_ptr(),
                ..Default::default()
            };
            d.cmd_begin_render_pass(cmd, &begin, vk::SubpassContents::INLINE);
        }

        let pipeline_layout = &*self.pipeline.get_object().get_create_info().layout;

        // prepare pipeline state
        {
            d.cmd_bind_pipeline(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.get_handle(),
            );
            d.cmd_bind_vertex_buffers(cmd, 0, &[self.vertex_buffer.0.get_handle()], &[0]);
            d.cmd_bind_index_buffer(
                cmd,
                self.index_buffer.0.get_handle(),
                0,
                vk::IndexType::UINT32,
            );
            d.cmd_set_viewport(
                cmd,
                0,
                &[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: width as f32,
                    height: height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }],
            );
            let width_in_points = width as f32 / pixels_per_point;
            let height_in_points = height as f32 / pixels_per_point;
            d.cmd_push_constants(
                cmd,
                pipeline_layout.get_handle(),
                vk::ShaderStageFlags::VERTEX,
                0,
                std::mem::size_of::<PushConstants>() as u32,
                &PushConstants {
                    screen_size: [width_in_points, height_in_points],
                } as *const _ as *const c_void,
            );
        }

        let mut vertex_buffer_ptr = self.vertex_buffer.1;
        let vertex_buffer_ptr_end = vertex_buffer_ptr.add(VERTEX_BUFFER_COUNT);

        let mut index_buffer_ptr = self.index_buffer.1;
        let index_buffer_ptr_end = index_buffer_ptr.add(INDEX_BUFFER_COUNT);

        let mut vertex_offset = 0;
        let mut index_offset = 0;

        let mut current_texture = None;
        let mut current_scissor = None;

        // draw meshes
        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in primitives
        {
            let egui::epaint::Primitive::Mesh(mesh) = primitive else {
                panic!("Callback primitive is Unsupported");
            };

            if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                continue;
            }

            if let Some(texture) = self.texture_images.get(&mesh.texture_id) {
                if current_texture != Some(mesh.texture_id) {
                    d.cmd_bind_descriptor_sets(
                        cmd,
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline_layout.get_handle(),
                        0,
                        &[texture.set],
                        &[],
                    );
                    current_texture = Some(mesh.texture_id);
                }
            } else {
                eprintln!(
                    "Attempt to draw mesh with invalid TextureID: {:?}",
                    mesh.texture_id
                );
                // we continue drawing with the previous texture, perhaps the obviously wrong output
                // will make developers fix stuff quicker
            };

            let v_end = vertex_buffer_ptr.add(mesh.vertices.len());
            let i_end = index_buffer_ptr.add(mesh.indices.len());

            if v_end > vertex_buffer_ptr_end || i_end > index_buffer_ptr_end {
                panic!("Ran out of memory for mesh buffers");
            }

            vertex_buffer_ptr.copy_from_nonoverlapping(mesh.vertices.as_ptr(), mesh.vertices.len());
            index_buffer_ptr.copy_from_nonoverlapping(mesh.indices.as_ptr(), mesh.indices.len());

            if Some(clip_rect) != current_scissor {
                current_scissor = Some(clip_rect);
                // Transform clip rect to physical pixels:
                let clip_min_x = pixels_per_point * clip_rect.min.x;
                let clip_min_y = pixels_per_point * clip_rect.min.y;
                let clip_max_x = pixels_per_point * clip_rect.max.x;
                let clip_max_y = pixels_per_point * clip_rect.max.y;

                // Round to integer:
                let clip_min_x = clip_min_x.round() as i32;
                let clip_min_y = clip_min_y.round() as i32;
                let clip_max_x = clip_max_x.round() as i32;
                let clip_max_y = clip_max_y.round() as i32;

                // Clamp:
                let clip_min_x = clip_min_x.clamp(0, width as i32);
                let clip_min_y = clip_min_y.clamp(0, height as i32);
                let clip_max_x = clip_max_x.clamp(clip_min_x, width as i32);
                let clip_max_y = clip_max_y.clamp(clip_min_y, height as i32);

                d.cmd_set_scissor(
                    cmd,
                    0,
                    &[vk::Rect2D {
                        offset: vk::Offset2D {
                            x: clip_min_x,
                            y: clip_min_y,
                        },
                        extent: vk::Extent2D {
                            width: (clip_max_x - clip_min_x).max(0) as u32,
                            height: (clip_max_y - clip_min_y).max(0) as u32,
                        },
                    }],
                );
            }

            d.cmd_draw_indexed(
                cmd,
                mesh.indices.len() as u32,
                1,
                index_offset,
                vertex_offset,
                0,
            );

            vertex_buffer_ptr = v_end;
            index_buffer_ptr = i_end;

            index_offset += mesh.indices.len() as u32;
            vertex_offset += mesh.vertices.len() as i32;
        }

        d.cmd_end_render_pass(cmd);

        wait_submission
    }

    /// Get registered texture.
    pub fn get_texture(&self, id: egui::TextureId) -> Option<&TextureImage> {
        self.texture_images.get(&id)
    }

    /// Register a user texture, may be called multiple times for the same id, potentially recreating it.
    pub unsafe fn add_texture(
        &mut self,
        id: egui::TextureId,
        image: object::Image,
        texture_options: egui::TextureOptions,
        components: vk::ComponentMapping,
        device: &Device,
    ) {
        let Self {
            set_allocator,
            desc_layout,
            pending_retired_sets,
            samplers,
            texture_images,
            ..
        } = self;

        let components_copy = components.clone();
        let remake = |image: object::Image| {
            let sampler = Self::get_sampler(samplers, texture_options, device);
            let view = image
                .get_view(
                    &object::ImageViewCreateInfo {
                        view_type: vk::ImageViewType::T2D,
                        format: image.get_create_info().format,
                        components: components_copy.clone(),
                        subresource_range: image.get_whole_subresource_range(),
                    },
                    GenerationId::NEVER,
                )
                .unwrap();
            let set = set_allocator.allocate(view, sampler, &desc_layout, device);

            TextureImage {
                image,
                components: components_copy,
                set,
                view,
                options: texture_options,
            }
        };

        match texture_images.entry(id) {
            Entry::Occupied(mut o) => {
                let v = o.get_mut();
                if image.get_pointer_identity() == v.image.get_pointer_identity()
                    && components == v.components
                    && texture_options == v.options
                {
                    return;
                } else {
                    pending_retired_sets.push(v.set);
                    std::ptr::drop_in_place(v);
                    std::ptr::write(v, remake(image))
                }
            }
            Entry::Vacant(v) => {
                let value = remake(image);
                v.insert(value);
            }
        }
    }
}

const DESCRIPTOR_SET_SIZES: &[vk::DescriptorPoolSize] = graph::desc_set_sizes!(64 * SAMPLED_IMAGE);

/// A vulkan descriptor set allocator which recycles unused ones.
pub(crate) struct EguiDescriptorSetAllocator {
    allocator: DescriptorSetAllocator,
    free_sets: Vec<vk::DescriptorSet>,
}

impl EguiDescriptorSetAllocator {
    pub(crate) fn new() -> Self {
        Self {
            allocator: DescriptorSetAllocator::new(DESCRIPTOR_SET_SIZES),
            free_sets: Vec::new(),
        }
    }
    /// Allocate a new descriptor set, either uses a recycled one or creates a new one.
    pub(crate) unsafe fn allocate(
        &mut self,
        image_view: vk::ImageView,
        sampler: vk::Sampler,
        layout: &ObjRef<object::DescriptorSetLayout>,
        device: &Device,
    ) -> vk::DescriptorSet {
        let set = self
            .free_sets
            .pop()
            .unwrap_or_else(|| self.allocator.allocate_set(layout, device));

        let label = LazyDisplay(|f| write!(f, "Egui descriptor {:p}", set));
        maybe_attach_debug_label(set, &label, device);

        let image_info = vk::DescriptorImageInfo {
            sampler,
            image_view,
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        };

        let write = vk::WriteDescriptorSet {
            dst_set: set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_image_info: &image_info,
            ..Default::default()
        };

        device.device().update_descriptor_sets(&[write], &[]);

        set
    }
    pub fn free_set(&mut self, set: vk::DescriptorSet) {
        self.free_sets.push(set);
    }
}
