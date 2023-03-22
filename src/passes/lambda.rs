use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
    time::Instant,
};

use graph::{
    device::Device,
    graph::{
        descriptors::{DescImage, DescSetBuilder},
        execute::GraphExecutor,
        record::GraphPassBuilder,
        GraphImage,
    },
    object::{self},
    passes::{CreatePass, RenderPass},
};
use pumice::{util::ObjectHandle, vk};

pub struct LambdaPass<
    C: FnMut(&mut GraphPassBuilder) + Send + 'static,
    F: FnMut(&GraphExecutor, &Device) + Send + 'static,
>(pub C, pub F);

impl<
        C: FnMut(&mut GraphPassBuilder) + Send + 'static,
        F: FnMut(&GraphExecutor, &Device) + Send + 'static,
    > CreatePass for LambdaPass<C, F>
{
    type PreparedData = ();
    fn prepare(&mut self, builder: &mut GraphPassBuilder) -> Self::PreparedData {
        (self.0)(builder);
    }
    fn create(
        self,
        prepared: Self::PreparedData,
        ctx: &mut graph::graph::compile::GraphContext,
    ) -> Box<dyn RenderPass + Send> {
        Box::new(self.1)
    }
}
