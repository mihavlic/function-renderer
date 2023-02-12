use graph::{
    graph::{
        compile::GraphContext, execute::GraphExecutor, record::GraphPassBuilder,
        task::GraphicsPipelinePromise, GraphBuffer, GraphImage,
    },
    object::{self, ConcreteGraphicsPipeline, Extent, GraphicsPipeline, RenderPassMode},
    passes::{CreatePass, RenderPass},
    smallvec::SmallVec,
};
use pumice::vk;

pub struct SimpleShader {
    pub pipeline: GraphicsPipeline,
    pub attachments: Vec<GraphImage>,
    pub depth: Option<GraphImage>,
    pub vertices: GraphBuffer,
    pub indices: GraphBuffer,
    pub draw_parameter_buffer: GraphBuffer,
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
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                None,
            );
            if let Some(depth) = self.depth {
                builder.use_image(
                    depth,
                    vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                    vk::PipelineStageFlags2KHR::EARLY_FRAGMENT_TESTS,
                    vk::AccessFlags2KHR::DEPTH_STENCIL_ATTACHMENT_WRITE,
                    vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    None,
                );
            }
            builder.use_buffer(
                self.draw_parameter_buffer,
                vk::BufferUsageFlags::INDIRECT_BUFFER,
                vk::PipelineStageFlags2KHR::DRAW_INDIRECT,
                vk::AccessFlags2KHR::INDIRECT_COMMAND_READ,
            );
        }
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

        let attachments = self
            .info
            .attachments
            .iter()
            .map(|&image| vk::RenderingAttachmentInfoKHR {
                image_view: executor.get_default_image_view(image, vk::ImageAspectFlags::COLOR),
                image_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlagsKHR::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float_32: [0.0, 0.0, 0.0, 1.0],
                    },
                },
                ..Default::default()
            })
            .collect::<SmallVec<[_; 8]>>();

        let (width, height) = match executor.get_image_extent(self.info.attachments[0]) {
            Extent::D2(w, h) => (w, h),
            Extent::D1(_) | Extent::D3(_, _, _) => panic!("Attachments must be 2D images"),
        };

        let mut depth = vk::RenderingAttachmentInfoKHR::default();
        if let Some(image) = self.info.depth {
            depth = vk::RenderingAttachmentInfoKHR {
                image_view: executor.get_default_image_view(image, vk::ImageAspectFlags::DEPTH),
                image_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR,
                resolve_mode: vk::ResolveModeFlagsKHR::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float_32: [0.0, 0.0, 0.0, 1.0],
                    },
                },
                ..Default::default()
            };
        }

        let info = vk::RenderingInfoKHR {
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D { width, height },
            },
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: attachments.len() as u32,
            p_color_attachments: attachments.as_ptr(),
            p_depth_attachment: self
                .info
                .depth
                .map(|_| &depth as *const _)
                .unwrap_or(std::ptr::null()),
            p_stencil_attachment: std::ptr::null(),
            ..Default::default()
        };

        d.cmd_begin_rendering_khr(cmd, &info);

        d.cmd_bind_pipeline(
            cmd,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline.get_handle(),
        );

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

        d.cmd_bind_vertex_buffers(cmd, 0, &[executor.get_buffer(self.info.vertices)], &[0]);
        d.cmd_bind_index_buffer(
            cmd,
            executor.get_buffer(self.info.indices),
            0,
            vk::IndexType::UINT32,
        );

        d.cmd_draw_indexed_indirect(
            cmd,
            executor.get_buffer(self.info.draw_parameter_buffer),
            0,
            1,
            1,
        );

        d.cmd_end_rendering_khr(cmd);

        Ok(())
    }
}
