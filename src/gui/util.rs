use graph::{
    device::{maybe_attach_debug_label, Device, LazyDisplay},
    graph::descriptors::DescriptorSetAllocator,
    object::{self, ObjRef},
};
use pumice::vk;

const DESCRIPTOR_SET_SIZES: &[vk::DescriptorPoolSize] = graph::desc_set_sizes!(64 * SAMPLED_IMAGE);

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
