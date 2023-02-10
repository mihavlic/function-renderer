#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentDescription2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2KHR.html)
pub struct AttachmentDescription2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::AttachmentDescriptionFlags,
    pub format: crate::vk10::Format,
    pub samples: crate::vk10::SampleCountFlags,
    pub load_op: crate::vk10::AttachmentLoadOp,
    pub store_op: crate::vk10::AttachmentStoreOp,
    pub stencil_load_op: crate::vk10::AttachmentLoadOp,
    pub stencil_store_op: crate::vk10::AttachmentStoreOp,
    pub initial_layout: crate::vk10::ImageLayout,
    pub final_layout: crate::vk10::ImageLayout,
}
impl Default for AttachmentDescription2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_DESCRIPTION_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            format: Default::default(),
            samples: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            stencil_load_op: Default::default(),
            stencil_store_op: Default::default(),
            initial_layout: Default::default(),
            final_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentReference2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2KHR.html)
pub struct AttachmentReference2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment: u32,
    pub layout: crate::vk10::ImageLayout,
    pub aspect_mask: crate::vk10::ImageAspectFlags,
}
impl Default for AttachmentReference2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_REFERENCE_2,
            p_next: std::ptr::null(),
            attachment: Default::default(),
            layout: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDescription2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2KHR.html)
pub struct SubpassDescription2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::SubpassDescriptionFlags,
    pub pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference2KHR,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference2KHR,
    pub p_resolve_attachments: *const AttachmentReference2KHR,
    pub p_depth_stencil_attachment: *const AttachmentReference2KHR,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
impl Default for SubpassDescription2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_DESCRIPTION_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            view_mask: Default::default(),
            input_attachment_count: Default::default(),
            p_input_attachments: std::ptr::null(),
            color_attachment_count: Default::default(),
            p_color_attachments: std::ptr::null(),
            p_resolve_attachments: std::ptr::null(),
            p_depth_stencil_attachment: std::ptr::null(),
            preserve_attachment_count: Default::default(),
            p_preserve_attachments: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDependency2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2KHR.html)
pub struct SubpassDependency2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: crate::vk10::PipelineStageFlags,
    pub dst_stage_mask: crate::vk10::PipelineStageFlags,
    pub src_access_mask: crate::vk10::AccessFlags,
    pub dst_access_mask: crate::vk10::AccessFlags,
    pub dependency_flags: crate::vk10::DependencyFlags,
    pub view_offset: i32,
}
impl Default for SubpassDependency2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_DEPENDENCY_2,
            p_next: std::ptr::null(),
            src_subpass: Default::default(),
            dst_subpass: Default::default(),
            src_stage_mask: Default::default(),
            dst_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            dependency_flags: Default::default(),
            view_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassCreateInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2KHR.html)
pub struct RenderPassCreateInfo2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription2KHR,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription2KHR,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency2KHR,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}
impl Default for RenderPassCreateInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_CREATE_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
            subpass_count: Default::default(),
            p_subpasses: std::ptr::null(),
            dependency_count: Default::default(),
            p_dependencies: std::ptr::null(),
            correlated_view_mask_count: Default::default(),
            p_correlated_view_masks: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassBeginInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfoKHR.html)
pub struct SubpassBeginInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub contents: crate::vk10::SubpassContents,
}
impl Default for SubpassBeginInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            contents: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassEndInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfoKHR.html)
pub struct SubpassEndInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
}
impl Default for SubpassEndInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_END_INFO,
            p_next: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateRenderPass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html)
pub unsafe fn create_render_pass_2_khr(
    device: crate::vk10::Device,
    p_create_info: *const RenderPassCreateInfo2KHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_render_pass: *mut crate::vk10::RenderPass,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_render_pass_2_khr
        .unwrap())(device, p_create_info, p_allocator, p_render_pass)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html)
    pub unsafe fn create_render_pass_2_khr(
        &self,
        create_info: &RenderPassCreateInfo2KHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk10::RenderPass> {
        let create_render_pass_2_khr = (*self.table).create_render_pass_2_khr.unwrap();
        let mut render_pass = Default::default();
        let result = create_render_pass_2_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut render_pass,
        );
        crate::new_result(render_pass, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html)
pub unsafe fn cmd_begin_render_pass_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_render_pass_2_khr
        .unwrap())(command_buffer, p_render_pass_begin, p_subpass_begin_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html)
    pub unsafe fn cmd_begin_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        render_pass_begin: &crate::vk10::RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfoKHR,
    ) {
        let cmd_begin_render_pass_2_khr = (*self.table)
            .cmd_begin_render_pass_2_khr
            .unwrap();
        cmd_begin_render_pass_2_khr(
            command_buffer,
            render_pass_begin as _,
            subpass_begin_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdNextSubpass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html)
pub unsafe fn cmd_next_subpass_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfoKHR,
    p_subpass_end_info: *const SubpassEndInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_next_subpass_2_khr
        .unwrap())(command_buffer, p_subpass_begin_info, p_subpass_end_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html)
    pub unsafe fn cmd_next_subpass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        subpass_begin_info: &SubpassBeginInfoKHR,
        subpass_end_info: &SubpassEndInfoKHR,
    ) {
        let cmd_next_subpass_2_khr = (*self.table).cmd_next_subpass_2_khr.unwrap();
        cmd_next_subpass_2_khr(
            command_buffer,
            subpass_begin_info as _,
            subpass_end_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRenderPass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html)
pub unsafe fn cmd_end_render_pass_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_subpass_end_info: *const SubpassEndInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_render_pass_2_khr
        .unwrap())(command_buffer, p_subpass_end_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html)
    pub unsafe fn cmd_end_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        subpass_end_info: &SubpassEndInfoKHR,
    ) {
        let cmd_end_render_pass_2_khr = (*self.table).cmd_end_render_pass_2_khr.unwrap();
        cmd_end_render_pass_2_khr(command_buffer, subpass_end_info as _);
    }
}
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
pub const KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_create_renderpass2"
);
