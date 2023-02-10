#[doc(alias = "VkRenderingFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RenderingFlagsKHR(pub u32);
impl RenderingFlagsKHR {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1 << 0);
    pub const SUSPENDING: Self = Self(1 << 1);
    pub const RESUMING: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    RenderingFlagsKHR : u32, 0x7, CONTENTS_SECONDARY_COMMAND_BUFFERS, SUSPENDING,
    RESUMING
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRenderingCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfoKHR.html)
pub struct PipelineRenderingCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const crate::vk10::Format,
    pub depth_attachment_format: crate::vk10::Format,
    pub stencil_attachment_format: crate::vk10::Format,
}
impl Default for PipelineRenderingCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RENDERING_CREATE_INFO,
            p_next: std::ptr::null(),
            view_mask: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachment_formats: std::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderingInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingInfoKHR.html)
pub struct RenderingInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: RenderingFlagsKHR,
    pub render_area: crate::vk10::Rect2D,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const RenderingAttachmentInfoKHR,
    pub p_depth_attachment: *const RenderingAttachmentInfoKHR,
    pub p_stencil_attachment: *const RenderingAttachmentInfoKHR,
}
impl Default for RenderingInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDERING_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            render_area: Default::default(),
            layer_count: Default::default(),
            view_mask: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachments: std::ptr::null(),
            p_depth_attachment: std::ptr::null(),
            p_stencil_attachment: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderingAttachmentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfoKHR.html)
pub struct RenderingAttachmentInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_view: crate::vk10::ImageView,
    pub image_layout: crate::vk10::ImageLayout,
    pub resolve_mode: crate::extensions::khr_depth_stencil_resolve::ResolveModeFlagsKHR,
    pub resolve_image_view: crate::vk10::ImageView,
    pub resolve_image_layout: crate::vk10::ImageLayout,
    pub load_op: crate::vk10::AttachmentLoadOp,
    pub store_op: crate::vk10::AttachmentStoreOp,
    pub clear_value: crate::vk10::ClearValue,
}
impl Default for RenderingAttachmentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDERING_ATTACHMENT_INFO,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            resolve_mode: Default::default(),
            resolve_image_view: Default::default(),
            resolve_image_layout: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            clear_value: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeaturesKHR.html)
pub struct PhysicalDeviceDynamicRenderingFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub dynamic_rendering: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDynamicRenderingFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
            p_next: std::ptr::null_mut(),
            dynamic_rendering: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferInheritanceRenderingInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfoKHR.html)
pub struct CommandBufferInheritanceRenderingInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: RenderingFlagsKHR,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const crate::vk10::Format,
    pub depth_attachment_format: crate::vk10::Format,
    pub stencil_attachment_format: crate::vk10::Format,
    pub rasterization_samples: crate::vk10::SampleCountFlags,
}
impl Default for CommandBufferInheritanceRenderingInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            view_mask: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachment_formats: std::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
            rasterization_samples: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRenderingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html)
pub unsafe fn cmd_begin_rendering_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_rendering_info: *const RenderingInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_rendering_khr
        .unwrap())(command_buffer, p_rendering_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html)
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rendering_info: &RenderingInfoKHR,
    ) {
        let cmd_begin_rendering_khr = (*self.table).cmd_begin_rendering_khr.unwrap();
        cmd_begin_rendering_khr(command_buffer, rendering_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRenderingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html)
pub unsafe fn cmd_end_rendering_khr(command_buffer: crate::vk10::CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_rendering_khr
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html)
    pub unsafe fn cmd_end_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        let cmd_end_rendering_khr = (*self.table).cmd_end_rendering_khr.unwrap();
        cmd_end_rendering_khr(command_buffer);
    }
}
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
pub const KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_dynamic_rendering"
);
