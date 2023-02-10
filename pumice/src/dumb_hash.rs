use std::{
    borrow::Borrow,
    ffi::CStr,
    hash::{self, Hasher},
    os::raw::{c_char, c_void},
};

pub trait DumbHash {
    fn hash<H: Hasher>(&self, state: &mut H);
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for element in data {
            DumbHash::hash(element, state);
        }
    }
    unsafe fn wrap(self) -> DumbWrap<Self>
    where
        Self: Sized,
    {
        DumbWrap(self)
    }
    unsafe fn wrap_ref(&self) -> &DumbWrap<Self> {
        // ok because we have repr(transparent)
        // https://stackoverflow.com/questions/61441303/is-it-safe-to-cast-references-to-unsized-transparent-types
        std::mem::transmute(self)
    }
}

#[repr(transparent)]
pub struct DumbWrap<T: DumbHash + ?Sized>(T);

impl<T: DumbHash> DumbWrap<T> {
    pub unsafe fn new(what: T) -> Self {
        Self(what)
    }
}

impl<T: DumbHash + ?Sized> DumbWrap<T> {
    pub unsafe fn from_ref(what: &T) -> &Self {
        // ok because we have repr(transparent)
        std::mem::transmute(what)
    }
}

impl<T: DumbHash> hash::Hash for DumbWrap<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DumbHash::hash(&self.0, state);
    }
}

unsafe fn hash_ptr<T: DumbHash, H: Hasher>(what: *const T, state: &mut H) {
    if let Some(what) = what.as_ref() {
        what.hash(state);
    }
}

unsafe fn hash_raw_arr<T: DumbHash, H: Hasher>(what: *const T, len: usize, state: &mut H) {
    if !what.is_null() {
        let slice = std::slice::from_raw_parts(what, len);
        DumbHash::hash_slice(slice, state);
    }
}

unsafe fn hash_cstr<H: Hasher>(ptr: *const c_char, state: &mut H) {
    if !ptr.is_null() {
        hash::Hash::hash(&CStr::from_ptr(ptr), state);
    }
}

#[track_caller]
unsafe fn hash_pnext<H: Hasher>(mut pnext: *const c_void, state: &mut H) {
    while !pnext.is_null() {
        #[rustfmt::skip]
        crate::pnext_visit!(
            pnext, stype, object,
            {
                // the usefulness of hashing the stype is dubious, however following the same logic as `Hasher::write_length_prefix` it eliminates a class of hash collisions
                hash::Hash::hash(&stype, state);
                (&&*object).get_or_die().hash(state);
            }
        );
    }
}

// hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

struct NeverInstantiated;

impl DumbHash for NeverInstantiated {
    fn hash<H: Hasher>(&self, _: &mut H) {
        unreachable!()
    }
}

trait ViaPanic {
    fn get_or_die(&self) -> &NeverInstantiated {
        panic!("{:?} does not implement DumbHash, this is likely because it contains a void pointer without any size information which cannot be manipulated and thus no implementation could be generated.", std::any::type_name::<Self>());
    }
}
impl<T> ViaPanic for &&T {}

trait ViaActual<T>: Borrow<T> {
    fn get_or_die(&self) -> &T {
        self.borrow()
    }
}
impl<T: DumbHash> ViaActual<T> for &T {}

impl DumbHash for f32 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.to_bits(), state);
    }
}

impl DumbHash for f64 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.to_bits(), state);
    }
}

impl<T: DumbHash, const S: usize> DumbHash for [T; S] {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DumbHash::hash_slice(self, state);
    }
}

impl<T: DumbHash> DumbHash for [T] {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DumbHash::hash_slice(self, state);
    }
}

impl<T> DumbHash for *const T {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state)
    }
}

impl<T> DumbHash for *mut T {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state)
    }
}

macro_rules! dumb_hash_passthrough_impl {
    ($($name:path),+) => {
        $(
            impl DumbHash for $name {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    hash::Hash::hash(self, state);
                }
            }
        )+
    };
}
impl DumbHash for crate::vk10::BaseOutStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_ptr(self.p_next, state);
        }
    }
}
impl DumbHash for crate::vk10::BaseInStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_ptr(self.p_next, state);
        }
    }
}
impl DumbHash for crate::vk10::Viewport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.x.hash(state);
            self.y.hash(state);
            self.width.hash(state);
            self.height.hash(state);
            self.min_depth.hash(state);
            self.max_depth.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PhysicalDeviceProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.api_version.hash(state);
            self.driver_version.hash(state);
            self.vendor_id.hash(state);
            self.device_id.hash(state);
            self.device_type.hash(state);
            self.device_name.hash(state);
            self.pipeline_cache_uuid.hash(state);
            self.limits.hash(state);
            self.sparse_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ApplicationInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_cstr(self.p_application_name, state);
            self.application_version.hash(state);
            hash_cstr(self.p_engine_name, state);
            self.engine_version.hash(state);
            self.api_version.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::AllocationCallbacks {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.p_user_data.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_allocation),
                state,
            );
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_reallocation),
                state,
            );
            std::ptr::hash(std::mem::transmute::<_, *const ()>(self.pfn_free), state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_internal_allocation),
                state,
            );
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_internal_free),
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::DeviceQueueCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_family_index.hash(state);
            self.queue_count.hash(state);
            hash_raw_arr(self.p_queue_priorities, (self.queue_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DeviceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_create_info_count.hash(state);
            hash_raw_arr(
                self.p_queue_create_infos,
                (self.queue_create_info_count) as usize,
                state,
            );
            self.enabled_layer_count.hash(state);
            let len = (self.enabled_layer_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                hash_cstr(ptr, state);
            }
            self.enabled_extension_count.hash(state);
            let len = (self.enabled_extension_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                hash_cstr(ptr, state);
            }
            hash_ptr(self.p_enabled_features, state);
        }
    }
}
impl DumbHash for crate::vk10::InstanceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            hash_ptr(self.p_application_info, state);
            self.enabled_layer_count.hash(state);
            let len = (self.enabled_layer_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                hash_cstr(ptr, state);
            }
            self.enabled_extension_count.hash(state);
            let len = (self.enabled_extension_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                hash_cstr(ptr, state);
            }
        }
    }
}
impl DumbHash for crate::vk10::MemoryAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.allocation_size.hash(state);
            self.memory_type_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::MappedMemoryRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::WriteDescriptorSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dst_set.hash(state);
            self.dst_binding.hash(state);
            self.dst_array_element.hash(state);
            self.descriptor_count.hash(state);
            self.descriptor_type.hash(state);
            hash_raw_arr(self.p_image_info, (self.descriptor_count) as usize, state);
            hash_raw_arr(self.p_buffer_info, (self.descriptor_count) as usize, state);
            hash_raw_arr(
                self.p_texel_buffer_view,
                (self.descriptor_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::CopyDescriptorSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_set.hash(state);
            self.src_binding.hash(state);
            self.src_array_element.hash(state);
            self.dst_set.hash(state);
            self.dst_binding.hash(state);
            self.dst_array_element.hash(state);
            self.descriptor_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::BufferCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.size.hash(state);
            self.usage.hash(state);
            self.sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::BufferViewCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.buffer.hash(state);
            self.format.hash(state);
            self.offset.hash(state);
            self.range.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::MemoryBarrier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::BufferMemoryBarrier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ImageMemoryBarrier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.old_layout.hash(state);
            self.new_layout.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.image.hash(state);
            self.subresource_range.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ImageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.image_type.hash(state);
            self.format.hash(state);
            self.extent.hash(state);
            self.mip_levels.hash(state);
            self.array_layers.hash(state);
            self.samples.hash(state);
            self.tiling.hash(state);
            self.usage.hash(state);
            self.sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
            self.initial_layout.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ImageViewCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.image.hash(state);
            self.view_type.hash(state);
            self.format.hash(state);
            self.components.hash(state);
            self.subresource_range.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SparseBufferMemoryBindInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.buffer.hash(state);
            self.bind_count.hash(state);
            hash_raw_arr(self.p_binds, (self.bind_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::SparseImageOpaqueMemoryBindInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.image.hash(state);
            self.bind_count.hash(state);
            hash_raw_arr(self.p_binds, (self.bind_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::SparseImageMemoryBindInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.image.hash(state);
            self.bind_count.hash(state);
            hash_raw_arr(self.p_binds, (self.bind_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::BindSparseInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphores,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.buffer_bind_count.hash(state);
            hash_raw_arr(self.p_buffer_binds, (self.buffer_bind_count) as usize, state);
            self.image_opaque_bind_count.hash(state);
            hash_raw_arr(
                self.p_image_opaque_binds,
                (self.image_opaque_bind_count) as usize,
                state,
            );
            self.image_bind_count.hash(state);
            hash_raw_arr(self.p_image_binds, (self.image_bind_count) as usize, state);
            self.signal_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::ShaderModuleCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.code_size.hash(state);
            hash_raw_arr(self.p_code, (self.code_size / 4) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DescriptorSetLayoutBinding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.binding.hash(state);
            self.descriptor_type.hash(state);
            self.descriptor_count.hash(state);
            self.stage_flags.hash(state);
            hash_raw_arr(
                self.p_immutable_samplers,
                (self.descriptor_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::DescriptorSetLayoutCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.binding_count.hash(state);
            hash_raw_arr(self.p_bindings, (self.binding_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DescriptorPoolCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.max_sets.hash(state);
            self.pool_size_count.hash(state);
            hash_raw_arr(self.p_pool_sizes, (self.pool_size_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DescriptorSetAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.descriptor_pool.hash(state);
            self.descriptor_set_count.hash(state);
            hash_raw_arr(
                self.p_set_layouts,
                (self.descriptor_set_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::SpecializationInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.map_entry_count.hash(state);
            hash_raw_arr(self.p_map_entries, (self.map_entry_count) as usize, state);
            self.data_size.hash(state);
            hash_raw_arr(self.p_data.cast::<u8>(), (self.data_size) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineShaderStageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage.hash(state);
            self.module.hash(state);
            hash_cstr(self.p_name, state);
            hash_ptr(self.p_specialization_info, state);
        }
    }
}
impl DumbHash for crate::vk10::ComputePipelineCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage.hash(state);
            self.layout.hash(state);
            self.base_pipeline_handle.hash(state);
            self.base_pipeline_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineVertexInputStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.vertex_binding_description_count.hash(state);
            hash_raw_arr(
                self.p_vertex_binding_descriptions,
                (self.vertex_binding_description_count) as usize,
                state,
            );
            self.vertex_attribute_description_count.hash(state);
            hash_raw_arr(
                self.p_vertex_attribute_descriptions,
                (self.vertex_attribute_description_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::PipelineInputAssemblyStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.topology.hash(state);
            self.primitive_restart_enable.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineTessellationStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.patch_control_points.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineViewportStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.viewport_count.hash(state);
            hash_raw_arr(self.p_viewports, (self.viewport_count) as usize, state);
            self.scissor_count.hash(state);
            hash_raw_arr(self.p_scissors, (self.scissor_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineRasterizationStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.depth_clamp_enable.hash(state);
            self.rasterizer_discard_enable.hash(state);
            self.polygon_mode.hash(state);
            self.cull_mode.hash(state);
            self.front_face.hash(state);
            self.depth_bias_enable.hash(state);
            self.depth_bias_constant_factor.hash(state);
            self.depth_bias_clamp.hash(state);
            self.depth_bias_slope_factor.hash(state);
            self.line_width.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineMultisampleStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.rasterization_samples.hash(state);
            self.sample_shading_enable.hash(state);
            self.min_sample_shading.hash(state);
            hash_raw_arr(
                self.p_sample_mask,
                ((self.rasterization_samples.0 + 31) / 32) as usize,
                state,
            );
            self.alpha_to_coverage_enable.hash(state);
            self.alpha_to_one_enable.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineColorBlendStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.logic_op_enable.hash(state);
            self.logic_op.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.blend_constants.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineDynamicStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.dynamic_state_count.hash(state);
            hash_raw_arr(
                self.p_dynamic_states,
                (self.dynamic_state_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::PipelineDepthStencilStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.depth_test_enable.hash(state);
            self.depth_write_enable.hash(state);
            self.depth_compare_op.hash(state);
            self.depth_bounds_test_enable.hash(state);
            self.stencil_test_enable.hash(state);
            self.front.hash(state);
            self.back.hash(state);
            self.min_depth_bounds.hash(state);
            self.max_depth_bounds.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::GraphicsPipelineCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage_count.hash(state);
            hash_raw_arr(self.p_stages, (self.stage_count) as usize, state);
            hash_ptr(self.p_vertex_input_state, state);
            hash_ptr(self.p_input_assembly_state, state);
            hash_ptr(self.p_tessellation_state, state);
            hash_ptr(self.p_viewport_state, state);
            hash_ptr(self.p_rasterization_state, state);
            hash_ptr(self.p_multisample_state, state);
            hash_ptr(self.p_depth_stencil_state, state);
            hash_ptr(self.p_color_blend_state, state);
            hash_ptr(self.p_dynamic_state, state);
            self.layout.hash(state);
            self.render_pass.hash(state);
            self.subpass.hash(state);
            self.base_pipeline_handle.hash(state);
            self.base_pipeline_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineCacheCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.initial_data_size.hash(state);
            hash_raw_arr(
                self.p_initial_data.cast::<u8>(),
                (self.initial_data_size) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::PipelineLayoutCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.set_layout_count.hash(state);
            hash_raw_arr(self.p_set_layouts, (self.set_layout_count) as usize, state);
            self.push_constant_range_count.hash(state);
            hash_raw_arr(
                self.p_push_constant_ranges,
                (self.push_constant_range_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::SamplerCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.mag_filter.hash(state);
            self.min_filter.hash(state);
            self.mipmap_mode.hash(state);
            self.address_mode_u.hash(state);
            self.address_mode_v.hash(state);
            self.address_mode_w.hash(state);
            self.mip_lod_bias.hash(state);
            self.anisotropy_enable.hash(state);
            self.max_anisotropy.hash(state);
            self.compare_enable.hash(state);
            self.compare_op.hash(state);
            self.min_lod.hash(state);
            self.max_lod.hash(state);
            self.border_color.hash(state);
            self.unnormalized_coordinates.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandPoolCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_family_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandBufferAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.command_pool.hash(state);
            self.level.hash(state);
            self.command_buffer_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandBufferInheritanceInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.render_pass.hash(state);
            self.subpass.hash(state);
            self.framebuffer.hash(state);
            self.occlusion_query_enable.hash(state);
            self.query_flags.hash(state);
            self.pipeline_statistics.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandBufferBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            hash_ptr(self.p_inheritance_info, state);
        }
    }
}
impl DumbHash for crate::vk10::RenderPassBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.render_pass.hash(state);
            self.framebuffer.hash(state);
            self.render_area.hash(state);
            self.clear_value_count.hash(state);
            hash_raw_arr(self.p_clear_values, (self.clear_value_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::ClearColorValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ClearDepthStencilValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.depth.hash(state);
            self.stencil.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ClearValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ClearAttachment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.aspect_mask.hash(state);
            self.color_attachment.hash(state);
            self.clear_value.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SubpassDescription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.pipeline_bind_point.hash(state);
            self.input_attachment_count.hash(state);
            hash_raw_arr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
                state,
            );
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_raw_arr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_ptr(self.p_depth_stencil_attachment, state);
            self.preserve_attachment_count.hash(state);
            hash_raw_arr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::RenderPassCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.subpass_count.hash(state);
            hash_raw_arr(self.p_subpasses, (self.subpass_count) as usize, state);
            self.dependency_count.hash(state);
            hash_raw_arr(self.p_dependencies, (self.dependency_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::EventCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::FenceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PhysicalDeviceLimits {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.max_image_dimension_1_d.hash(state);
            self.max_image_dimension_2_d.hash(state);
            self.max_image_dimension_3_d.hash(state);
            self.max_image_dimension_cube.hash(state);
            self.max_image_array_layers.hash(state);
            self.max_texel_buffer_elements.hash(state);
            self.max_uniform_buffer_range.hash(state);
            self.max_storage_buffer_range.hash(state);
            self.max_push_constants_size.hash(state);
            self.max_memory_allocation_count.hash(state);
            self.max_sampler_allocation_count.hash(state);
            self.buffer_image_granularity.hash(state);
            self.sparse_address_space_size.hash(state);
            self.max_bound_descriptor_sets.hash(state);
            self.max_per_stage_descriptor_samplers.hash(state);
            self.max_per_stage_descriptor_uniform_buffers.hash(state);
            self.max_per_stage_descriptor_storage_buffers.hash(state);
            self.max_per_stage_descriptor_sampled_images.hash(state);
            self.max_per_stage_descriptor_storage_images.hash(state);
            self.max_per_stage_descriptor_input_attachments.hash(state);
            self.max_per_stage_resources.hash(state);
            self.max_descriptor_set_samplers.hash(state);
            self.max_descriptor_set_uniform_buffers.hash(state);
            self.max_descriptor_set_uniform_buffers_dynamic.hash(state);
            self.max_descriptor_set_storage_buffers.hash(state);
            self.max_descriptor_set_storage_buffers_dynamic.hash(state);
            self.max_descriptor_set_sampled_images.hash(state);
            self.max_descriptor_set_storage_images.hash(state);
            self.max_descriptor_set_input_attachments.hash(state);
            self.max_vertex_input_attributes.hash(state);
            self.max_vertex_input_bindings.hash(state);
            self.max_vertex_input_attribute_offset.hash(state);
            self.max_vertex_input_binding_stride.hash(state);
            self.max_vertex_output_components.hash(state);
            self.max_tessellation_generation_level.hash(state);
            self.max_tessellation_patch_size.hash(state);
            self.max_tessellation_control_per_vertex_input_components.hash(state);
            self.max_tessellation_control_per_vertex_output_components.hash(state);
            self.max_tessellation_control_per_patch_output_components.hash(state);
            self.max_tessellation_control_total_output_components.hash(state);
            self.max_tessellation_evaluation_input_components.hash(state);
            self.max_tessellation_evaluation_output_components.hash(state);
            self.max_geometry_shader_invocations.hash(state);
            self.max_geometry_input_components.hash(state);
            self.max_geometry_output_components.hash(state);
            self.max_geometry_output_vertices.hash(state);
            self.max_geometry_total_output_components.hash(state);
            self.max_fragment_input_components.hash(state);
            self.max_fragment_output_attachments.hash(state);
            self.max_fragment_dual_src_attachments.hash(state);
            self.max_fragment_combined_output_resources.hash(state);
            self.max_compute_shared_memory_size.hash(state);
            self.max_compute_work_group_count.hash(state);
            self.max_compute_work_group_invocations.hash(state);
            self.max_compute_work_group_size.hash(state);
            self.sub_pixel_precision_bits.hash(state);
            self.sub_texel_precision_bits.hash(state);
            self.mipmap_precision_bits.hash(state);
            self.max_draw_indexed_index_value.hash(state);
            self.max_draw_indirect_count.hash(state);
            self.max_sampler_lod_bias.hash(state);
            self.max_sampler_anisotropy.hash(state);
            self.max_viewports.hash(state);
            self.max_viewport_dimensions.hash(state);
            self.viewport_bounds_range.hash(state);
            self.viewport_sub_pixel_bits.hash(state);
            self.min_memory_map_alignment.hash(state);
            self.min_texel_buffer_offset_alignment.hash(state);
            self.min_uniform_buffer_offset_alignment.hash(state);
            self.min_storage_buffer_offset_alignment.hash(state);
            self.min_texel_offset.hash(state);
            self.max_texel_offset.hash(state);
            self.min_texel_gather_offset.hash(state);
            self.max_texel_gather_offset.hash(state);
            self.min_interpolation_offset.hash(state);
            self.max_interpolation_offset.hash(state);
            self.sub_pixel_interpolation_offset_bits.hash(state);
            self.max_framebuffer_width.hash(state);
            self.max_framebuffer_height.hash(state);
            self.max_framebuffer_layers.hash(state);
            self.framebuffer_color_sample_counts.hash(state);
            self.framebuffer_depth_sample_counts.hash(state);
            self.framebuffer_stencil_sample_counts.hash(state);
            self.framebuffer_no_attachments_sample_counts.hash(state);
            self.max_color_attachments.hash(state);
            self.sampled_image_color_sample_counts.hash(state);
            self.sampled_image_integer_sample_counts.hash(state);
            self.sampled_image_depth_sample_counts.hash(state);
            self.sampled_image_stencil_sample_counts.hash(state);
            self.storage_image_sample_counts.hash(state);
            self.max_sample_mask_words.hash(state);
            self.timestamp_compute_and_graphics.hash(state);
            self.timestamp_period.hash(state);
            self.max_clip_distances.hash(state);
            self.max_cull_distances.hash(state);
            self.max_combined_clip_and_cull_distances.hash(state);
            self.discrete_queue_priorities.hash(state);
            self.point_size_range.hash(state);
            self.line_width_range.hash(state);
            self.point_size_granularity.hash(state);
            self.line_width_granularity.hash(state);
            self.strict_lines.hash(state);
            self.standard_sample_locations.hash(state);
            self.optimal_buffer_copy_offset_alignment.hash(state);
            self.optimal_buffer_copy_row_pitch_alignment.hash(state);
            self.non_coherent_atom_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SemaphoreCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::QueryPoolCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.query_type.hash(state);
            self.query_count.hash(state);
            self.pipeline_statistics.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::FramebufferCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.render_pass.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.width.hash(state);
            self.height.hash(state);
            self.layers.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphores,
                (self.wait_semaphore_count) as usize,
                state,
            );
            hash_raw_arr(
                self.p_wait_dst_stage_mask,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.command_buffer_count.hash(state);
            hash_raw_arr(
                self.p_command_buffers,
                (self.command_buffer_count) as usize,
                state,
            );
            self.signal_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.display.hash(state);
            self.surface.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.hinstance.hash(state);
            self.hwnd.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.dpy.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.connection.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.dfb.hash(state);
            self.surface.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.image_pipe_handle.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stream_descriptor.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.context.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.surface.hash(state);
            self.min_image_count.hash(state);
            self.image_format.hash(state);
            self.image_color_space.hash(state);
            self.image_extent.hash(state);
            self.image_array_layers.hash(state);
            self.image_usage.hash(state);
            self.image_sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
            self.pre_transform.hash(state);
            self.composite_alpha.hash(state);
            self.present_mode.hash(state);
            self.clipped.hash(state);
            self.old_swapchain.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::PresentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphores,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_swapchains, (self.swapchain_count) as usize, state);
            hash_raw_arr(self.p_image_indices, (self.swapchain_count) as usize, state);
            hash_raw_arr(self.p_results, (self.swapchain_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_callback),
                state,
            );
            self.p_user_data.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceFeatures2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.features.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::FormatProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageFormatProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_format_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceImageFormatInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.kind.hash(state);
            self.tiling.hash(state);
            self.usage.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::QueueFamilyProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.queue_family_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMemoryProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SparseImageFormatProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceSparseImageFormatInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.kind.hash(state);
            self.samples.hash(state);
            self.usage.hash(state);
            self.tiling.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceVariablePointersFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.variable_pointers_storage_buffer.hash(state);
            self.variable_pointers.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalImageFormatInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalImageFormatProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.external_memory_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalBufferInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.usage.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalBufferProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.external_memory_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceIDProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_uuid.hash(state);
            self.driver_uuid.hash(state);
            self.device_luid.hash(state);
            self.device_node_mask.hash(state);
            self.device_luidvalid.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalMemoryImageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalMemoryBufferCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExportMemoryAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalSemaphoreInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalSemaphoreProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.export_from_imported_handle_types.hash(state);
            self.compatible_handle_types.hash(state);
            self.external_semaphore_features.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExportSemaphoreCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalFenceInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalFenceProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.export_from_imported_handle_types.hash(state);
            self.compatible_handle_types.hash(state);
            self.external_fence_features.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExportFenceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMultiviewFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.multiview.hash(state);
            self.multiview_geometry_shader.hash(state);
            self.multiview_tessellation_shader.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMultiviewProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_multiview_view_count.hash(state);
            self.max_multiview_instance_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::RenderPassMultiviewCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subpass_count.hash(state);
            hash_raw_arr(self.p_view_masks, (self.subpass_count) as usize, state);
            self.dependency_count.hash(state);
            hash_raw_arr(self.p_view_offsets, (self.dependency_count) as usize, state);
            self.correlation_mask_count.hash(state);
            hash_raw_arr(
                self.p_correlation_masks,
                (self.correlation_mask_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceGroupProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.physical_device_count.hash(state);
            self.physical_devices.hash(state);
            self.subset_allocation.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryAllocateFlagsInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindBufferMemoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
            self.memory.hash(state);
            self.memory_offset.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindBufferMemoryDeviceGroupInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_index_count.hash(state);
            hash_raw_arr(
                self.p_device_indices,
                (self.device_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::BindImageMemoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.memory.hash(state);
            self.memory_offset.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindImageMemoryDeviceGroupInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_index_count.hash(state);
            hash_raw_arr(
                self.p_device_indices,
                (self.device_index_count) as usize,
                state,
            );
            self.split_instance_bind_region_count.hash(state);
            hash_raw_arr(
                self.p_split_instance_bind_regions,
                (self.split_instance_bind_region_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupRenderPassBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_mask.hash(state);
            self.device_render_area_count.hash(state);
            hash_raw_arr(
                self.p_device_render_areas,
                (self.device_render_area_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupCommandBufferBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_device_indices,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.command_buffer_count.hash(state);
            hash_raw_arr(
                self.p_command_buffer_device_masks,
                (self.command_buffer_count) as usize,
                state,
            );
            self.signal_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_device_indices,
                (self.signal_semaphore_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupBindSparseInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.resource_device_index.hash(state);
            self.memory_device_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_mask.hash(state);
            self.modes.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::ImageSwapchainCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain.hash(state);
            self.image_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::AcquireNextImageInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain.hash(state);
            self.timeout.hash(state);
            self.semaphore.hash(state);
            self.fence.hash(state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::DeviceGroupPresentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_device_masks, (self.swapchain_count) as usize, state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupDeviceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.physical_device_count.hash(state);
            hash_raw_arr(
                self.p_physical_devices,
                (self.physical_device_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.modes.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DescriptorUpdateTemplateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.descriptor_update_entry_count.hash(state);
            hash_raw_arr(
                self.p_descriptor_update_entries,
                (self.descriptor_update_entry_count) as usize,
                state,
            );
            self.template_type.hash(state);
            self.descriptor_set_layout.hash(state);
            self.pipeline_bind_point.hash(state);
            self.pipeline_layout.hash(state);
            self.set.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.p_view.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.p_view.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.p_layer.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::RenderPassInputAttachmentAspectCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.aspect_reference_count.hash(state);
            hash_raw_arr(
                self.p_aspect_references,
                (self.aspect_reference_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDevice16BitStorageFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.storage_buffer_16_bit_access.hash(state);
            self.uniform_and_storage_buffer_16_bit_access.hash(state);
            self.storage_push_constant_16.hash(state);
            self.storage_input_output_16.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceSubgroupProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subgroup_size.hash(state);
            self.supported_stages.hash(state);
            self.supported_operations.hash(state);
            self.quad_operations_in_all_stages.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BufferMemoryRequirementsInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageMemoryRequirementsInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageSparseMemoryRequirementsInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryRequirements2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_requirements.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SparseImageMemoryRequirements2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_requirements.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDevicePointClippingProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.point_clipping_behavior.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryDedicatedRequirements {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.prefers_dedicated_allocation.hash(state);
            self.requires_dedicated_allocation.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryDedicatedAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageViewUsageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.usage.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PipelineTessellationDomainOriginStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.domain_origin.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SamplerYcbcrConversionInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.conversion.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SamplerYcbcrConversionCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.ycbcr_model.hash(state);
            self.ycbcr_range.hash(state);
            self.components.hash(state);
            self.x_chroma_offset.hash(state);
            self.y_chroma_offset.hash(state);
            self.chroma_filter.hash(state);
            self.force_explicit_reconstruction.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindImagePlaneMemoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.plane_aspect.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImagePlaneMemoryRequirementsInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.plane_aspect.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sampler_ycbcr_conversion.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SamplerYcbcrConversionImageFormatProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.combined_image_sampler_descriptor_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ProtectedSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.protected_submit.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceProtectedMemoryFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.protected_memory.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceProtectedMemoryProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.protected_no_fault.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DeviceQueueInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_family_index.hash(state);
            self.queue_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMaintenance3Properties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_per_set_descriptors.hash(state);
            self.max_memory_allocation_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DescriptorSetLayoutSupport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supported.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceShaderDrawParametersFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_draw_parameters.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.object_type.hash(state);
            self.object_handle.hash(state);
            hash_cstr(self.p_object_name, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.object_type.hash(state);
            self.object_handle.hash(state);
            self.tag_name.hash(state);
            self.tag_size.hash(state);
            hash_raw_arr(self.p_tag.cast::<u8>(), (self.tag_size) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsLabelEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_cstr(self.p_label_name, state);
            self.color.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.message_severity.hash(state);
            self.message_type.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_user_callback),
                state,
            );
            self.p_user_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            hash_cstr(self.p_message_id_name, state);
            self.message_id_number.hash(state);
            hash_cstr(self.p_message, state);
            self.queue_label_count.hash(state);
            hash_raw_arr(self.p_queue_labels, (self.queue_label_count) as usize, state);
            self.cmd_buf_label_count.hash(state);
            hash_raw_arr(
                self.p_cmd_buf_labels,
                (self.cmd_buf_label_count) as usize,
                state,
            );
            self.object_count.hash(state);
            hash_raw_arr(self.p_objects, (self.object_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::AttachmentDescription2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.format.hash(state);
            self.samples.hash(state);
            self.load_op.hash(state);
            self.store_op.hash(state);
            self.stencil_load_op.hash(state);
            self.stencil_store_op.hash(state);
            self.initial_layout.hash(state);
            self.final_layout.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::AttachmentReference2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment.hash(state);
            self.layout.hash(state);
            self.aspect_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::SubpassDescription2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.pipeline_bind_point.hash(state);
            self.view_mask.hash(state);
            self.input_attachment_count.hash(state);
            hash_raw_arr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
                state,
            );
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_raw_arr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_ptr(self.p_depth_stencil_attachment, state);
            self.preserve_attachment_count.hash(state);
            hash_raw_arr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::SubpassDependency2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_subpass.hash(state);
            self.dst_subpass.hash(state);
            self.src_stage_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.dependency_flags.hash(state);
            self.view_offset.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::RenderPassCreateInfo2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.subpass_count.hash(state);
            hash_raw_arr(self.p_subpasses, (self.subpass_count) as usize, state);
            self.dependency_count.hash(state);
            hash_raw_arr(self.p_dependencies, (self.dependency_count) as usize, state);
            self.correlated_view_mask_count.hash(state);
            hash_raw_arr(
                self.p_correlated_view_masks,
                (self.correlated_view_mask_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::SubpassBeginInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.contents.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_create_renderpass2::SubpassEndInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_timeline_semaphore::PhysicalDeviceTimelineSemaphoreFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.timeline_semaphore.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_timeline_semaphore::PhysicalDeviceTimelineSemaphorePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_timeline_semaphore_value_difference.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_timeline_semaphore::SemaphoreTypeCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore_type.hash(state);
            self.initial_value.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_timeline_semaphore::TimelineSemaphoreSubmitInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_value_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_values,
                (self.wait_semaphore_value_count) as usize,
                state,
            );
            self.signal_semaphore_value_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_values,
                (self.signal_semaphore_value_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_timeline_semaphore::SemaphoreWaitInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.semaphore_count.hash(state);
            hash_raw_arr(self.p_semaphores, (self.semaphore_count) as usize, state);
            hash_raw_arr(self.p_values, (self.semaphore_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_timeline_semaphore::SemaphoreSignalInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.value.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_depth_stencil_resolve::PhysicalDeviceDepthStencilResolvePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supported_depth_resolve_modes.hash(state);
            self.supported_stencil_resolve_modes.hash(state);
            self.independent_resolve_none.hash(state);
            self.independent_resolve.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_depth_stencil_resolve::SubpassDescriptionDepthStencilResolveKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.depth_resolve_mode.hash(state);
            self.stencil_resolve_mode.hash(state);
            hash_ptr(self.p_depth_stencil_resolve_attachment, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_scalar_block_layout::PhysicalDeviceScalarBlockLayoutFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.scalar_block_layout.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::MemoryBarrier2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.dst_access_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::ImageMemoryBarrier2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.old_layout.hash(state);
            self.new_layout.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.image.hash(state);
            self.subresource_range.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::BufferMemoryBarrier2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::DependencyInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dependency_flags.hash(state);
            self.memory_barrier_count.hash(state);
            hash_raw_arr(
                self.p_memory_barriers,
                (self.memory_barrier_count) as usize,
                state,
            );
            self.buffer_memory_barrier_count.hash(state);
            hash_raw_arr(
                self.p_buffer_memory_barriers,
                (self.buffer_memory_barrier_count) as usize,
                state,
            );
            self.image_memory_barrier_count.hash(state);
            hash_raw_arr(
                self.p_image_memory_barriers,
                (self.image_memory_barrier_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::SemaphoreSubmitInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.value.hash(state);
            self.stage_mask.hash(state);
            self.device_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::CommandBufferSubmitInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.command_buffer.hash(state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::SubmitInfo2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.wait_semaphore_info_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_infos,
                (self.wait_semaphore_info_count) as usize,
                state,
            );
            self.command_buffer_info_count.hash(state);
            hash_raw_arr(
                self.p_command_buffer_infos,
                (self.command_buffer_info_count) as usize,
                state,
            );
            self.signal_semaphore_info_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_infos,
                (self.signal_semaphore_info_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::khr_synchronization2::PhysicalDeviceSynchronization2FeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.synchronization_2.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::PipelineRenderingCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.view_mask.hash(state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
                state,
            );
            self.depth_attachment_format.hash(state);
            self.stencil_attachment_format.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_dynamic_rendering::RenderingInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.render_area.hash(state);
            self.layer_count.hash(state);
            self.view_mask.hash(state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_ptr(self.p_depth_attachment, state);
            hash_ptr(self.p_stencil_attachment, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_view.hash(state);
            self.image_layout.hash(state);
            self.resolve_mode.hash(state);
            self.resolve_image_view.hash(state);
            self.resolve_image_layout.hash(state);
            self.load_op.hash(state);
            self.store_op.hash(state);
            self.clear_value.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::PhysicalDeviceDynamicRenderingFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dynamic_rendering.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::CommandBufferInheritanceRenderingInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.view_mask.hash(state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
                state,
            );
            self.depth_attachment_format.hash(state);
            self.stencil_attachment_format.hash(state);
            self.rasterization_samples.hash(state);
        }
    }
}
dumb_hash_passthrough_impl! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, std::ffi::CStr, crate
    ::vk10::QueryPoolCreateFlags, crate ::vk10::PipelineDynamicStateCreateFlags, crate
    ::vk10::PipelineMultisampleStateCreateFlags, crate
    ::vk10::PipelineRasterizationStateCreateFlags, crate
    ::vk10::PipelineViewportStateCreateFlags, crate
    ::vk10::PipelineTessellationStateCreateFlags, crate
    ::vk10::PipelineInputAssemblyStateCreateFlags, crate
    ::vk10::PipelineVertexInputStateCreateFlags, crate ::vk10::BufferViewCreateFlags,
    crate ::vk10::DeviceCreateFlags, crate ::vk10::SemaphoreCreateFlags, crate
    ::vk10::ShaderModuleCreateFlags, crate ::vk10::MemoryMapFlags, crate
    ::vk10::DescriptorPoolResetFlags, crate ::vk11::DescriptorUpdateTemplateCreateFlags,
    crate ::extensions::khr_timeline_semaphore::SemaphoreWaitFlagsKHR, crate
    ::extensions::khr_synchronization2::AccessFlags2KHR, crate
    ::extensions::khr_synchronization2::PipelineStageFlags2KHR, crate
    ::extensions::khr_dynamic_rendering::RenderingFlagsKHR, crate
    ::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR, crate
    ::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN, crate
    ::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR, crate
    ::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR, crate
    ::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR, crate
    ::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR, crate
    ::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT, crate
    ::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK, crate
    ::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK, crate
    ::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT, crate
    ::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA, crate
    ::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    crate ::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT, crate
    ::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX, crate
    ::vk11::CommandPoolTrimFlags, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT, crate
    ::extensions::khr_depth_stencil_resolve::ResolveModeFlagsKHR, crate
    ::extensions::khr_synchronization2::SubmitFlagsKHR, crate ::vk10::Instance, crate
    ::vk10::PhysicalDevice, crate ::vk10::Device, crate ::vk10::Queue, crate
    ::vk10::CommandBuffer, crate ::vk10::DeviceMemory, crate ::vk10::CommandPool, crate
    ::vk10::Buffer, crate ::vk10::BufferView, crate ::vk10::Image, crate
    ::vk10::ImageView, crate ::vk10::ShaderModule, crate ::vk10::Pipeline, crate
    ::vk10::PipelineLayout, crate ::vk10::Sampler, crate ::vk10::DescriptorSet, crate
    ::vk10::DescriptorSetLayout, crate ::vk10::DescriptorPool, crate ::vk10::Fence, crate
    ::vk10::Semaphore, crate ::vk10::Event, crate ::vk10::QueryPool, crate
    ::vk10::Framebuffer, crate ::vk10::RenderPass, crate ::vk10::PipelineCache, crate
    ::vk11::DescriptorUpdateTemplate, crate ::vk11::SamplerYcbcrConversion, crate
    ::extensions::khr_surface::SurfaceKHR, crate
    ::extensions::khr_swapchain::SwapchainKHR, crate
    ::extensions::ext_debug_report::DebugReportCallbackEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerEXT, crate
    ::extensions::khr_timeline_semaphore::SemaphoreTypeKHR, crate ::vk10::Offset2D, crate
    ::vk10::Offset3D, crate ::vk10::Extent2D, crate ::vk10::Extent3D, crate
    ::vk10::Rect2D, crate ::vk10::ClearRect, crate ::vk10::ComponentMapping, crate
    ::vk10::ExtensionProperties, crate ::vk10::LayerProperties, crate
    ::vk10::QueueFamilyProperties, crate ::vk10::PhysicalDeviceMemoryProperties, crate
    ::vk10::MemoryRequirements, crate ::vk10::SparseImageFormatProperties, crate
    ::vk10::SparseImageMemoryRequirements, crate ::vk10::MemoryType, crate
    ::vk10::MemoryHeap, crate ::vk10::FormatProperties, crate
    ::vk10::ImageFormatProperties, crate ::vk10::DescriptorBufferInfo, crate
    ::vk10::DescriptorImageInfo, crate ::vk10::ImageSubresource, crate
    ::vk10::ImageSubresourceLayers, crate ::vk10::ImageSubresourceRange, crate
    ::vk10::SubresourceLayout, crate ::vk10::BufferCopy, crate ::vk10::SparseMemoryBind,
    crate ::vk10::SparseImageMemoryBind, crate ::vk10::ImageCopy, crate
    ::vk10::ImageBlit, crate ::vk10::BufferImageCopy, crate ::vk10::ImageResolve, crate
    ::vk10::DescriptorPoolSize, crate ::vk10::SpecializationMapEntry, crate
    ::vk10::VertexInputBindingDescription, crate ::vk10::VertexInputAttributeDescription,
    crate ::vk10::PipelineColorBlendAttachmentState, crate ::vk10::StencilOpState, crate
    ::vk10::PipelineCacheHeaderVersionOne, crate ::vk10::PushConstantRange, crate
    ::vk10::AttachmentDescription, crate ::vk10::AttachmentReference, crate
    ::vk10::SubpassDependency, crate ::vk10::PhysicalDeviceFeatures, crate
    ::vk10::PhysicalDeviceSparseProperties, crate ::vk10::DrawIndirectCommand, crate
    ::vk10::DrawIndexedIndirectCommand, crate ::vk10::DispatchIndirectCommand, crate
    ::extensions::khr_surface::SurfaceCapabilitiesKHR, crate
    ::extensions::khr_surface::SurfaceFormatKHR, crate ::vk11::ExternalMemoryProperties,
    crate ::vk11::DescriptorUpdateTemplateEntry, crate
    ::vk11::InputAttachmentAspectReference, crate ::vk10::ImageLayout, crate
    ::vk10::AttachmentLoadOp, crate ::vk10::AttachmentStoreOp, crate ::vk10::ImageType,
    crate ::vk10::ImageTiling, crate ::vk10::ImageViewType, crate
    ::vk10::CommandBufferLevel, crate ::vk10::ComponentSwizzle, crate
    ::vk10::DescriptorType, crate ::vk10::QueryType, crate ::vk10::BorderColor, crate
    ::vk10::PipelineBindPoint, crate ::vk10::PipelineCacheHeaderVersion, crate
    ::vk10::PipelineCacheCreateFlags, crate ::vk10::PrimitiveTopology, crate
    ::vk10::SharingMode, crate ::vk10::IndexType, crate ::vk10::Filter, crate
    ::vk10::SamplerMipmapMode, crate ::vk10::SamplerAddressMode, crate ::vk10::CompareOp,
    crate ::vk10::PolygonMode, crate ::vk10::FrontFace, crate ::vk10::BlendFactor, crate
    ::vk10::BlendOp, crate ::vk10::StencilOp, crate ::vk10::LogicOp, crate
    ::vk10::InternalAllocationType, crate ::vk10::SystemAllocationScope, crate
    ::vk10::PhysicalDeviceType, crate ::vk10::VertexInputRate, crate ::vk10::Format,
    crate ::vk10::StructureType, crate ::vk10::SubpassContents, crate ::vk10::Result,
    crate ::vk10::DynamicState, crate ::vk11::DescriptorUpdateTemplateType, crate
    ::vk10::ObjectType, crate ::vk10::QueueFlags, crate ::vk10::CullModeFlags, crate
    ::vk10::RenderPassCreateFlags, crate ::vk10::DeviceQueueCreateFlags, crate
    ::vk10::MemoryPropertyFlags, crate ::vk10::MemoryHeapFlags, crate
    ::vk10::AccessFlags, crate ::vk10::BufferUsageFlags, crate ::vk10::BufferCreateFlags,
    crate ::vk10::ShaderStageFlags, crate ::vk10::ImageUsageFlags, crate
    ::vk10::ImageCreateFlags, crate ::vk10::ImageViewCreateFlags, crate
    ::vk10::SamplerCreateFlags, crate ::vk10::PipelineCreateFlags, crate
    ::vk10::PipelineShaderStageCreateFlags, crate ::vk10::ColorComponentFlags, crate
    ::vk10::FenceCreateFlags, crate ::vk10::FormatFeatureFlags, crate
    ::vk10::QueryControlFlags, crate ::vk10::QueryResultFlags, crate
    ::vk10::CommandBufferUsageFlags, crate ::vk10::QueryPipelineStatisticFlags, crate
    ::vk10::ImageAspectFlags, crate ::vk10::SparseImageFormatFlags, crate
    ::vk10::SparseMemoryBindFlags, crate ::vk10::PipelineStageFlags, crate
    ::vk10::CommandPoolCreateFlags, crate ::vk10::CommandPoolResetFlags, crate
    ::vk10::CommandBufferResetFlags, crate ::vk10::SampleCountFlags, crate
    ::vk10::AttachmentDescriptionFlags, crate ::vk10::StencilFaceFlags, crate
    ::vk10::DescriptorPoolCreateFlags, crate ::vk10::DependencyFlags, crate
    ::extensions::khr_surface::PresentModeKHR, crate
    ::extensions::khr_surface::ColorSpaceKHR, crate
    ::extensions::khr_surface::CompositeAlphaFlagsKHR, crate
    ::extensions::khr_surface::SurfaceTransformFlagsKHR, crate
    ::extensions::ext_debug_report::DebugReportFlagsEXT, crate
    ::extensions::ext_debug_report::DebugReportObjectTypeEXT, crate
    ::vk11::SubgroupFeatureFlags, crate ::vk10::DescriptorSetLayoutCreateFlags, crate
    ::vk11::ExternalMemoryHandleTypeFlags, crate ::vk11::ExternalMemoryFeatureFlags,
    crate ::vk11::ExternalSemaphoreHandleTypeFlags, crate
    ::vk11::ExternalSemaphoreFeatureFlags, crate ::vk11::SemaphoreImportFlags, crate
    ::vk11::ExternalFenceHandleTypeFlags, crate ::vk11::ExternalFenceFeatureFlags, crate
    ::vk11::FenceImportFlags, crate ::vk11::PeerMemoryFeatureFlags, crate
    ::vk11::MemoryAllocateFlags, crate
    ::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR, crate
    ::extensions::khr_swapchain::SwapchainCreateFlagsKHR, crate
    ::vk10::SubpassDescriptionFlags, crate ::vk11::PointClippingBehavior, crate
    ::vk11::TessellationDomainOrigin, crate ::vk11::SamplerYcbcrModelConversion, crate
    ::vk11::SamplerYcbcrRange, crate ::vk11::ChromaLocation, crate
    ::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT, crate ::vk10::VendorId,
    crate ::vk10::FramebufferCreateFlags, crate ::vk10::EventCreateFlags, crate
    ::vk10::PipelineLayoutCreateFlags, crate ::vk10::PipelineColorBlendStateCreateFlags,
    crate ::vk10::PipelineDepthStencilStateCreateFlags, crate ::vk10::InstanceCreateFlags
}
