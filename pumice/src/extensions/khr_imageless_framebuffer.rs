#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeaturesKHR.html)
pub struct PhysicalDeviceImagelessFramebufferFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub imageless_framebuffer: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImagelessFramebufferFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES,
            p_next: std::ptr::null_mut(),
            imageless_framebuffer: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFramebufferAttachmentsCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfoKHR.html)
pub struct FramebufferAttachmentsCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const FramebufferAttachmentImageInfoKHR,
}
impl Default for FramebufferAttachmentsCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
            p_next: std::ptr::null(),
            attachment_image_info_count: Default::default(),
            p_attachment_image_infos: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFramebufferAttachmentImageInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfoKHR.html)
pub struct FramebufferAttachmentImageInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::ImageCreateFlags,
    pub usage: crate::vk10::ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const crate::vk10::Format,
}
impl Default for FramebufferAttachmentImageInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            width: Default::default(),
            height: Default::default(),
            layer_count: Default::default(),
            view_format_count: Default::default(),
            p_view_formats: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassAttachmentBeginInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfoKHR.html)
pub struct RenderPassAttachmentBeginInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk10::ImageView,
}
impl Default for RenderPassAttachmentBeginInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO,
            p_next: std::ptr::null(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
        }
    }
}
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
pub const KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_imageless_framebuffer"
);
