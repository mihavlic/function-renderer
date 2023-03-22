#[doc(alias = "VkDebugUtilsMessengerCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessengerCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    DebugUtilsMessengerCreateFlagsEXT : u32, 0x0,
}
#[doc(alias = "VkDebugUtilsMessengerCallbackDataFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT(pub u32);
crate::bitflags_impl! {
    DebugUtilsMessengerCallbackDataFlagsEXT : u32, 0x0,
}
crate::dispatchable_handle!(
    DebugUtilsMessengerEXT, DEBUG_UTILS_MESSENGER_EXT, "VkDebugUtilsMessengerEXT",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html)"
);
#[doc(alias = "PFN_vkDebugUtilsMessengerCallbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html)
pub type PfnDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::os::raw::c_void,
) -> crate::vk10::Bool32;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html)
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub object_type: crate::vk10::ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const std::os::raw::c_char,
}
impl Default for DebugUtilsObjectNameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            p_object_name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html)
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub object_type: crate::vk10::ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const std::os::raw::c_void,
}
impl Default for DebugUtilsObjectTagInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            tag_name: Default::default(),
            tag_size: Default::default(),
            p_tag: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html)
pub struct DebugUtilsLabelEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_label_name: *const std::os::raw::c_char,
    pub color: [std::os::raw::c_float; 4],
}
impl Default for DebugUtilsLabelEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_UTILS_LABEL_EXT,
            p_next: std::ptr::null(),
            p_label_name: std::ptr::null(),
            color: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugUtilsMessengerCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html)
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: Option<PfnDebugUtilsMessengerCallbackEXT>,
    pub p_user_data: *mut std::os::raw::c_void,
}
impl Default for DebugUtilsMessengerCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            message_severity: Default::default(),
            message_type: Default::default(),
            pfn_user_callback: None,
            p_user_data: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html)
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const std::os::raw::c_char,
    pub message_id_number: i32,
    pub p_message: *const std::os::raw::c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT,
}
impl Default for DebugUtilsMessengerCallbackDataEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_message_id_name: std::ptr::null(),
            message_id_number: Default::default(),
            p_message: std::ptr::null(),
            queue_label_count: Default::default(),
            p_queue_labels: std::ptr::null(),
            cmd_buf_label_count: Default::default(),
            p_cmd_buf_labels: std::ptr::null(),
            object_count: Default::default(),
            p_objects: std::ptr::null(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub u32);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE: Self = Self(1 << 0);
    pub const INFO: Self = Self(1 << 4);
    pub const WARNING: Self = Self(1 << 8);
    pub const ERROR: Self = Self(1 << 12);
}
crate::bitflags_impl! {
    DebugUtilsMessageSeverityFlagsEXT : u32, 0x1111, VERBOSE, INFO, WARNING, ERROR
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessageTypeFlagsEXT(pub u32);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL: Self = Self(1 << 0);
    pub const VALIDATION: Self = Self(1 << 1);
    pub const PERFORMANCE: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    DebugUtilsMessageTypeFlagsEXT : u32, 0x7, GENERAL, VALIDATION, PERFORMANCE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
pub unsafe fn set_debug_utils_object_name_ext(
    device: crate::vk10::Device,
    p_name_info: *const DebugUtilsObjectNameInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_debug_utils_object_name_ext
        .unwrap())(device, p_name_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> crate::VulkanResult<()> {
        let set_debug_utils_object_name_ext = (*self.table)
            .set_debug_utils_object_name_ext
            .unwrap();
        let result = set_debug_utils_object_name_ext(self.handle, name_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
pub unsafe fn set_debug_utils_object_tag_ext(
    device: crate::vk10::Device,
    p_tag_info: *const DebugUtilsObjectTagInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_debug_utils_object_tag_ext
        .unwrap())(device, p_tag_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> crate::VulkanResult<()> {
        let set_debug_utils_object_tag_ext = (*self.table)
            .set_debug_utils_object_tag_ext
            .unwrap();
        let result = set_debug_utils_object_tag_ext(self.handle, tag_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
pub unsafe fn queue_begin_debug_utils_label_ext(
    queue: crate::vk10::Queue,
    p_label_info: *const DebugUtilsLabelEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_begin_debug_utils_label_ext
        .unwrap())(queue, p_label_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: crate::vk10::Queue,
        label_info: &DebugUtilsLabelEXT,
    ) {
        let queue_begin_debug_utils_label_ext = (*self.table)
            .queue_begin_debug_utils_label_ext
            .unwrap();
        queue_begin_debug_utils_label_ext(queue, label_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
pub unsafe fn queue_end_debug_utils_label_ext(queue: crate::vk10::Queue) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_end_debug_utils_label_ext
        .unwrap())(queue)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: crate::vk10::Queue) {
        let queue_end_debug_utils_label_ext = (*self.table)
            .queue_end_debug_utils_label_ext
            .unwrap();
        queue_end_debug_utils_label_ext(queue);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
pub unsafe fn queue_insert_debug_utils_label_ext(
    queue: crate::vk10::Queue,
    p_label_info: *const DebugUtilsLabelEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_insert_debug_utils_label_ext
        .unwrap())(queue, p_label_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: crate::vk10::Queue,
        label_info: &DebugUtilsLabelEXT,
    ) {
        let queue_insert_debug_utils_label_ext = (*self.table)
            .queue_insert_debug_utils_label_ext
            .unwrap();
        queue_insert_debug_utils_label_ext(queue, label_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
pub unsafe fn cmd_begin_debug_utils_label_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_debug_utils_label_ext
        .unwrap())(command_buffer, p_label_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        label_info: &DebugUtilsLabelEXT,
    ) {
        let cmd_begin_debug_utils_label_ext = (*self.table)
            .cmd_begin_debug_utils_label_ext
            .unwrap();
        cmd_begin_debug_utils_label_ext(command_buffer, label_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
pub unsafe fn cmd_end_debug_utils_label_ext(command_buffer: crate::vk10::CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_debug_utils_label_ext
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_end_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        let cmd_end_debug_utils_label_ext = (*self.table)
            .cmd_end_debug_utils_label_ext
            .unwrap();
        cmd_end_debug_utils_label_ext(command_buffer);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
pub unsafe fn cmd_insert_debug_utils_label_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_insert_debug_utils_label_ext
        .unwrap())(command_buffer, p_label_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        label_info: &DebugUtilsLabelEXT,
    ) {
        let cmd_insert_debug_utils_label_ext = (*self.table)
            .cmd_insert_debug_utils_label_ext
            .unwrap();
        cmd_insert_debug_utils_label_ext(command_buffer, label_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
pub unsafe fn create_debug_utils_messenger_ext(
    instance: crate::vk10::Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_debug_utils_messenger_ext
        .unwrap())(instance, p_create_info, p_allocator, p_messenger)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<DebugUtilsMessengerEXT> {
        let create_debug_utils_messenger_ext = (*self.table)
            .create_debug_utils_messenger_ext
            .unwrap();
        let mut messenger = Default::default();
        let result = create_debug_utils_messenger_ext(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut messenger,
        );
        crate::new_result(messenger, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
pub unsafe fn destroy_debug_utils_messenger_ext(
    instance: crate::vk10::Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .destroy_debug_utils_messenger_ext
        .unwrap())(instance, messenger, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_debug_utils_messenger_ext = (*self.table)
            .destroy_debug_utils_messenger_ext
            .unwrap();
        destroy_debug_utils_messenger_ext(
            self.handle,
            messenger,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
pub unsafe fn submit_debug_utils_message_ext(
    instance: crate::vk10::Instance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .submit_debug_utils_message_ext
        .unwrap())(instance, message_severity, message_types, p_callback_data)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) {
        let submit_debug_utils_message_ext = (*self.table)
            .submit_debug_utils_message_ext
            .unwrap();
        submit_debug_utils_message_ext(
            self.handle,
            message_severity as _,
            message_types as _,
            callback_data as _,
        );
    }
}
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_debug_utils"
);
