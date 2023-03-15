use std::sync::{Arc, Mutex};

use dolly::{prelude::RightHanded, rig::CameraRig, transform::Transform};
use glam::{EulerRot, Mat4, Quat, Vec3, Vec3Swizzles};
use graph::{
    graph::{
        compile::GraphContext, execute::GraphExecutor, record::GraphPassBuilder,
        task::GraphicsPipelinePromise, GraphBuffer, GraphImage,
    },
    object::{self, ConcreteGraphicsPipeline, Extent, GraphicsPipeline, RenderPassMode},
    passes::{CreatePass, RenderPass},
    smallvec::SmallVec,
    util::ffi_ptr::AsFFiPtr,
};
use pumice::{util::ObjectHandle, vk};

use crate::{arcball::ArcballCamera, FrameData};

pub struct SimpleShader {
    pub pipeline: GraphicsPipeline,
    pub attachments: Vec<GraphImage>,
    pub resolve_attachments: Vec<GraphImage>,
    pub depth: Option<GraphImage>,
    pub vertices: GraphBuffer,
    pub indices: GraphBuffer,
    pub draw_parameter_buffer: GraphBuffer,
    pub transform: Arc<Mutex<FrameData>>,
}

impl CreatePass for SimpleShader {
    type PreparedData = GraphicsPipelinePromise;
    fn prepare(&mut self, builder: &mut GraphPassBuilder) -> Self::PreparedData {
        for &image in &self.attachments {
            builder.use_image(
                image,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
                vk::PipelineStageFlags2KHR::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags2KHR::COLOR_ATTACHMENT_WRITE,
                vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                None,
            );
        }
        for &image in &self.resolve_attachments {
            builder.use_image(
                image,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
                vk::PipelineStageFlags2KHR::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags2KHR::COLOR_ATTACHMENT_WRITE,
                vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                None,
            );
        }
        if let Some(depth) = self.depth {
            builder.use_image(
                depth,
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                vk::PipelineStageFlags2KHR::EARLY_FRAGMENT_TESTS,
                vk::AccessFlags2KHR::DEPTH_STENCIL_ATTACHMENT_WRITE,
                vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                None,
            );
        }
        builder.use_buffer(
            self.draw_parameter_buffer,
            vk::BufferUsageFlags::INDIRECT_BUFFER,
            vk::PipelineStageFlags2KHR::DRAW_INDIRECT,
            vk::AccessFlags2KHR::INDIRECT_COMMAND_READ,
        );
        builder.use_buffer(
            self.indices,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::PipelineStageFlags2KHR::INDEX_INPUT,
            vk::AccessFlags2KHR::INDEX_READ,
        );
        builder.use_buffer(
            self.vertices,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::PipelineStageFlags2KHR::VERTEX_INPUT,
            vk::AccessFlags2KHR::VERTEX_ATTRIBUTE_READ,
        );
        let mode = RenderPassMode::Dynamic {
            view_mask: 0,
            colors: self
                .attachments
                .iter()
                .map(|i| builder.get_image_format(*i))
                .collect(),
            depth: self
                .depth
                .map(|image| builder.get_image_format(image))
                .unwrap_or(vk::Format::UNDEFINED),
            stencil: vk::Format::UNDEFINED,
        };

        builder.compile_graphics_pipeline(&self.pipeline, &mode)
    }

    fn create(
        self,
        prepared: Self::PreparedData,
        ctx: &mut GraphContext,
    ) -> Box<dyn RenderPass + Send> {
        let pipeline = ctx.resolve_graphics_pipeline(prepared);
        Box::new(SimpleShaderPass {
            info: self,
            pipeline,
        })
    }
}

struct SimpleShaderPass {
    info: SimpleShader,
    pipeline: ConcreteGraphicsPipeline,
}

impl RenderPass for SimpleShaderPass {
    unsafe fn execute(
        &mut self,
        executor: &GraphExecutor,
        device: &graph::device::Device,
    ) -> pumice::VulkanResult<()> {
        let d = device.device();
        let cmd = executor.command_buffer();

        let attachments_len = self.info.attachments.len();
        let resolve_attachments_len = self.info.resolve_attachments.len();
        assert!(attachments_len == resolve_attachments_len || resolve_attachments_len == 0);

        let attachments = self
            .info
            .attachments
            .iter()
            .zip(
                self.info
                    .resolve_attachments
                    .iter()
                    .copied()
                    .map(Some)
                    .chain(std::iter::repeat(None)),
            )
            .map(|(&image, resolve)| {
                let (resolve_mode, resolve_image_view, resolve_image_layout, store_op) =
                    match resolve {
                        Some(resolve) => (
                            vk::ResolveModeFlagsKHR::AVERAGE,
                            executor.get_default_image_view(resolve),
                            vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                            vk::AttachmentStoreOp::STORE,
                        ),
                        None => (
                            vk::ResolveModeFlagsKHR::NONE,
                            vk::ImageView::null(),
                            vk::ImageLayout::UNDEFINED,
                            vk::AttachmentStoreOp::STORE,
                        ),
                    };

                vk::RenderingAttachmentInfoKHR {
                    image_view: executor.get_default_image_view(image),
                    image_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                    resolve_mode,
                    resolve_image_view,
                    resolve_image_layout,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op,
                    clear_value: vk::ClearValue {
                        color: vk::ClearColorValue {
                            float_32: [0.0, 0.0, 0.0, 1.0],
                        },
                    },
                    ..Default::default()
                }
            })
            .collect::<SmallVec<[_; 8]>>();

        let (width, height) = match executor.get_image_extent(self.info.attachments[0]) {
            Extent::D2(w, h) => (w, h),
            Extent::D1(_) | Extent::D3(_, _, _) => panic!("Attachments must be 2D images"),
        };

        let mut depth = None;
        if let Some(image) = self.info.depth {
            depth = Some(vk::RenderingAttachmentInfoKHR {
                image_view: executor.get_default_image_view(image),
                image_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                resolve_mode: vk::ResolveModeFlagsKHR::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
                ..Default::default()
            });
        }

        let info = vk::RenderingInfoKHR {
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D { width, height },
            },
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: attachments.len() as u32,
            p_color_attachments: attachments.as_ffi_ptr(),
            p_depth_attachment: depth.as_ffi_ptr(),
            p_stencil_attachment: std::ptr::null(),
            ..Default::default()
        };

        d.cmd_begin_rendering_khr(cmd, &info);

        let pipeline_info = self.pipeline.get_object().get_create_info();
        if let Some(state) = &pipeline_info.dynamic_state {
            // TODO this kinda sucks, we should probably include a bitmask of dynamic states or something
            if state.dynamic_states.contains(&vk::DynamicState::VIEWPORT) {
                let viewport = vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: width as f32,
                    height: height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                };
                d.cmd_set_viewport(cmd, 0, std::slice::from_ref(&viewport));
            }

            if state.dynamic_states.contains(&vk::DynamicState::SCISSOR) {
                let rect = vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: vk::Extent2D { width, height },
                };
                d.cmd_set_scissor(cmd, 0, std::slice::from_ref(&rect));
            }
        }

        d.cmd_bind_pipeline(
            cmd,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline.get_handle(),
        );

        let perspective = Mat4::perspective_rh(
            1.2 * std::f32::consts::FRAC_PI_4,
            (width as f32) / (height as f32),
            0.2,
            512.0,
        );
        let world_to_view = {
            let transform = self.info.transform.lock().unwrap().camera;
            let ro = Mat4::from_quat(transform.rotation.conjugate());
            let tr = Mat4::from_translation(-transform.position);
            ro * tr
        };

        let matrices = [world_to_view, perspective];

        d.cmd_push_constants(
            cmd,
            self.pipeline.get_object().get_create_info().layout.raw(),
            vk::ShaderStageFlags::VERTEX,
            0,
            std::mem::size_of_val(&matrices) as u32,
            matrices.as_ptr().cast(),
        );

        d.cmd_bind_vertex_buffers(cmd, 0, &[executor.get_buffer(self.info.vertices)], &[8]);
        d.cmd_bind_index_buffer(
            cmd,
            executor.get_buffer(self.info.indices),
            8,
            vk::IndexType::UINT32,
        );

        d.cmd_draw_indexed_indirect(
            cmd,
            executor.get_buffer(self.info.draw_parameter_buffer),
            0,
            1,
            0,
        );

        d.cmd_end_rendering_khr(cmd);

        Ok(())
    }
}

pub struct ArcBallAngles {
    pub horizontal: f32,
    pub vertical: f32,
    pub distance: f32,
}
