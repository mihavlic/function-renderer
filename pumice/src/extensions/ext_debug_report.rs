crate::dispatchable_handle!(
    DebugReportCallbackEXT, DEBUG_REPORT_CALLBACK_EXT, "VkDebugReportCallbackEXT",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackEXT.html)"
);
#[doc(alias = "PFN_vkDebugReportCallbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugReportCallbackEXT.html)
pub type PfnDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const std::os::raw::c_char,
    p_message: *const std::os::raw::c_char,
    p_user_data: *mut std::os::raw::c_void,
) -> crate::vk10::Bool32;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugReportCallbackCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html)
pub struct DebugReportCallbackCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DebugReportFlagsEXT,
    pub pfn_callback: Option<PfnDebugReportCallbackEXT>,
    pub p_user_data: *mut std::os::raw::c_void,
}
impl Default for DebugReportCallbackCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pfn_callback: None,
            p_user_data: std::ptr::null_mut(),
        }
    }
}
#[doc(alias = "VkDebugReportFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugReportFlagsEXT(pub u32);
impl DebugReportFlagsEXT {
    pub const INFORMATION: Self = Self(1 << 0);
    pub const WARNING: Self = Self(1 << 1);
    pub const PERFORMANCE_WARNING: Self = Self(1 << 2);
    pub const ERROR: Self = Self(1 << 3);
    pub const DEBUG: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    DebugReportFlagsEXT : u32, 0x1f, INFORMATION, WARNING, PERFORMANCE_WARNING, ERROR,
    DEBUG
}
#[doc(alias = "VkDebugReportObjectTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportObjectTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugReportObjectTypeEXT(pub i32);
impl DebugReportObjectTypeEXT {
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
    pub const SURFACE_KHR: Self = Self(26);
    pub const SWAPCHAIN_KHR: Self = Self(27);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(28);
    pub const DISPLAY_KHR: Self = Self(29);
    pub const DISPLAY_MODE_KHR: Self = Self(30);
    pub const VALIDATION_CACHE_EXT: Self = Self(33);
    /// ext_debug_report
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
}
crate::enum_impl! {
    DebugReportObjectTypeEXT : i32, UNKNOWN, INSTANCE, PHYSICAL_DEVICE, DEVICE, QUEUE,
    SEMAPHORE, COMMAND_BUFFER, FENCE, DEVICE_MEMORY, BUFFER, IMAGE, EVENT, QUERY_POOL,
    BUFFER_VIEW, IMAGE_VIEW, SHADER_MODULE, PIPELINE_CACHE, PIPELINE_LAYOUT, RENDER_PASS,
    PIPELINE, DESCRIPTOR_SET_LAYOUT, SAMPLER, DESCRIPTOR_POOL, DESCRIPTOR_SET,
    FRAMEBUFFER, COMMAND_POOL, SURFACE_KHR, SWAPCHAIN_KHR, DEBUG_REPORT_CALLBACK_EXT,
    DISPLAY_KHR, DISPLAY_MODE_KHR, VALIDATION_CACHE_EXT, SAMPLER_YCBCR_CONVERSION,
    DESCRIPTOR_UPDATE_TEMPLATE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
pub unsafe fn create_debug_report_callback_ext(
    instance: crate::vk10::Instance,
    p_create_info: *const DebugReportCallbackCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_callback: *mut DebugReportCallbackEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_debug_report_callback_ext
        .unwrap())(instance, p_create_info, p_allocator, p_callback)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        create_info: &DebugReportCallbackCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<DebugReportCallbackEXT> {
        let create_debug_report_callback_ext = (*self.table)
            .create_debug_report_callback_ext
            .unwrap();
        let mut callback = Default::default();
        let result = create_debug_report_callback_ext(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut callback,
        );
        crate::new_result(callback, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
pub unsafe fn destroy_debug_report_callback_ext(
    instance: crate::vk10::Instance,
    callback: DebugReportCallbackEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .destroy_debug_report_callback_ext
        .unwrap())(instance, callback, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        callback: DebugReportCallbackEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_debug_report_callback_ext = (*self.table)
            .destroy_debug_report_callback_ext
            .unwrap();
        destroy_debug_report_callback_ext(
            self.handle,
            callback,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDebugReportMessageEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html)
pub unsafe fn debug_report_message_ext(
    instance: crate::vk10::Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const std::os::raw::c_char,
    p_message: *const std::os::raw::c_char,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .debug_report_message_ext
        .unwrap())(
        instance,
        flags,
        object_type,
        object,
        location,
        message_code,
        p_layer_prefix,
        p_message,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkDebugReportMessageEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html)
    pub unsafe fn debug_report_message_ext(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: Option<&std::ffi::CStr>,
        message: Option<&std::ffi::CStr>,
    ) {
        let debug_report_message_ext = (*self.table).debug_report_message_ext.unwrap();
        debug_report_message_ext(
            self.handle,
            flags as _,
            object_type as _,
            object as _,
            location as _,
            message_code as _,
            match layer_prefix {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
            match message {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
        );
    }
}
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_debug_report"
);
