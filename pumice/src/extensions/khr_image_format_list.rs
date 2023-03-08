#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageFormatListCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfoKHR.html)
pub struct ImageFormatListCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const crate::vk10::Format,
}
impl Default for ImageFormatListCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO,
            p_next: std::ptr::null(),
            view_format_count: Default::default(),
            p_view_formats: std::ptr::null(),
        }
    }
}
pub const KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;
pub const KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_image_format_list"
);
