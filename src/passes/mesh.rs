use crate::ApplicationState;
use glam::{Mat3, Mat4, Vec3};
use graph::{
    graph::{
        compile::GraphContext,
        descriptors::{DescSetBuilder, DescriptorData},
        execute::GraphExecutor,
        record::GraphPassBuilder,
        task::GraphicsPipelinePromise,
        GraphBuffer, GraphImage,
    },
    object::{ConcreteGraphicsPipeline, Extent, GraphicsPipeline, RenderPassMode},
    passes::{CreatePass, RenderPass},
    smallvec::SmallVec,
    util::ffi_ptr::AsFFiPtr,
};
use pumice::{util::ObjectHandle, vk};
use std::sync::{Arc, Mutex};

/// The render pass which draws the function mesh
pub struct MeshPass {
    pub pipeline: GraphicsPipeline,
    pub attachments: Vec<GraphImage>,
    pub resolve_attachments: Vec<GraphImage>,
    pub depth: Option<GraphImage>,
    pub vertices: GraphBuffer,
    pub indices: GraphBuffer,
    pub draw_parameter_buffer: GraphBuffer,
    pub transform: Arc<Mutex<ApplicationState>>,
}

impl CreatePass for MeshPass {
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
                .map(|depth| builder.get_image_format(depth))
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
        Box::new(CreatedMeshPass {
            info: self,
            pipeline,
        })
    }
}

struct CreatedMeshPass {
    info: MeshPass,
    pipeline: ConcreteGraphicsPipeline,
}

impl RenderPass for CreatedMeshPass {
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

        #[repr(C, align(16))]
        struct PushConstants {
            model_matrix: Mat4,
            projection_matrix: Mat4,
            rect_min: Vec3,
            rect_max: Vec3,
            time: f32,
            camera_dir: Vec3,
        }

        let push = {
            let state = self.info.transform.lock().unwrap();
            let camera = state.camera;

            let perspective = Mat4::perspective_rh(
                1.2 * std::f32::consts::FRAC_PI_4,
                (width as f32) / (height as f32),
                0.2,
                512.0,
            );
            let world_to_view = {
                let ro = Mat4::from_quat(camera.rotation.conjugate());
                let tr = Mat4::from_translation(-camera.position);
                ro * tr
            };

            PushConstants {
                model_matrix: world_to_view,
                projection_matrix: perspective,
                rect_min: state.rect_min,
                rect_max: state.rect_max,
                time: state.time,
                camera_dir: (Mat3::from_quat(camera.rotation.normalize()) * (Vec3::Z)).normalize(),
            }
        };

        let uniform = executor.allocate_uniform_element::<PushConstants>(&push);

        let layout = &self.pipeline.get_object().get_descriptor_set_layouts()[0];
        DescSetBuilder::new(layout)
            .update_whole(&[DescriptorData::Buffer(uniform.as_desc_dynamic())])
            .finish(executor)
            .bind(
                vk::PipelineBindPoint::GRAPHICS,
                &self.pipeline.get_object().get_create_info().layout,
                executor,
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
