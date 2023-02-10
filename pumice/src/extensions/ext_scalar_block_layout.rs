#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeaturesEXT.html)
pub struct PhysicalDeviceScalarBlockLayoutFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub scalar_block_layout: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceScalarBlockLayoutFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES,
            p_next: std::ptr::null_mut(),
            scalar_block_layout: Default::default(),
        }
    }
}
pub const EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
pub const EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_scalar_block_layout"
);
