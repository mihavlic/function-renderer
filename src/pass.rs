use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
    time::Instant,
};

use graph::{
    graph::{
        descriptors::{DescImage, DescSetBuilder},
        GraphImage,
    },
    object::{self},
    passes::{CreatePass, RenderPass},
};
use pumice::{util::ObjectHandle, vk};

use crate::ViewState;

pub struct ComputePass {
    pub image: GraphImage,
    pub pipeline: object::ComputePipeline,
    pub time: Instant,
    pub state: Arc<Mutex<ViewState>>,
}

impl CreatePass for ComputePass {
    type PreparedData = ();
    fn prepare(
        &mut self,
        builder: &mut graph::graph::record::GraphPassBuilder,
    ) -> Self::PreparedData {
        builder.use_image(
            self.image,
            vk::ImageUsageFlags::STORAGE,
            vk::PipelineStageFlags2KHR::COMPUTE_SHADER,
            vk::AccessFlags2KHR::SHADER_STORAGE_WRITE,
            vk::ImageLayout::GENERAL,
            None,
        );
    }

    fn create(
        self,
        prepared: Self::PreparedData,
        ctx: &mut graph::graph::compile::GraphContext,
    ) -> Box<dyn graph::passes::RenderPass + Send> {
        Box::new(self)
    }
}

struct PushConstantData {
    width: u32,
    height: u32,
    offset_x: f32,
    offset_y: f32,
    scale: f32,
    time: f32,
}

impl RenderPass for ComputePass {
    unsafe fn execute(
        &mut self,
        executor: &graph::graph::execute::GraphExecutor,
        device: &graph::device::Device,
    ) -> pumice::VulkanResult<()> {
        let d = device.device();
        let cmd = executor.command_buffer();
        let (w, h) = executor.get_image_extent(self.image).get_2d().unwrap();

        d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, self.pipeline.raw());

        let time = self.time.elapsed().as_secs_f32();

        let pipeline_layout = self.pipeline.get_pipeline_layout();
        let set_layout = &pipeline_layout.get_descriptor_set_layouts()[0];

        DescSetBuilder::new(set_layout)
            .update_image_binding(
                0,
                0,
                &DescImage {
                    sampler: vk::Sampler::null(),
                    view: executor.get_default_image_view(self.image, vk::ImageAspectFlags::COLOR),
                    layout: vk::ImageLayout::GENERAL,
                },
            )
            .finish(executor)
            .bind(vk::PipelineBindPoint::COMPUTE, pipeline_layout, executor);

        let mut state = self.state.lock().unwrap();

        let data = PushConstantData {
            width: w,
            height: h,
            offset_x: state.x_shift,
            offset_y: state.y_shift,
            scale: state.zoom,
            time,
        };

        drop(state);

        d.cmd_push_constants(
            cmd,
            pipeline_layout.raw(),
            vk::ShaderStageFlags::COMPUTE,
            0,
            std::mem::size_of::<PushConstantData>() as u32,
            (&data) as *const PushConstantData as *const c_void,
        );
        d.cmd_dispatch(cmd, (w + 7) / 8, (h + 7) / 8, 1);

        Ok(())
    }
}
