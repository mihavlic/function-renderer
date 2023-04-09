use graph::{
    device::Device,
    graph::{execute::GraphExecutor, record::GraphPassBuilder},
    passes::{CreatePass, RenderPass},
};

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
        _prepared: Self::PreparedData,
        _ctx: &mut graph::graph::compile::GraphContext,
    ) -> Box<dyn RenderPass + Send> {
        Box::new(self.1)
    }
}
