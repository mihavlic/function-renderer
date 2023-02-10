#[doc(alias = "VkSemaphoreWaitFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreWaitFlagsKHR(pub u32);
impl SemaphoreWaitFlagsKHR {
    pub const ANY: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    SemaphoreWaitFlagsKHR : u32, 0x1, ANY
}
#[doc(alias = "VkSemaphoreTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreTypeKHR(pub i32);
impl SemaphoreTypeKHR {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}
crate::enum_impl! {
    SemaphoreTypeKHR : i32, BINARY, TIMELINE
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeaturesKHR.html)
pub struct PhysicalDeviceTimelineSemaphoreFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub timeline_semaphore: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTimelineSemaphoreFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
            p_next: std::ptr::null_mut(),
            timeline_semaphore: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphorePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphorePropertiesKHR.html)
pub struct PhysicalDeviceTimelineSemaphorePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_timeline_semaphore_value_difference: u64,
}
impl Default for PhysicalDeviceTimelineSemaphorePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_timeline_semaphore_value_difference: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreTypeCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfoKHR.html)
pub struct SemaphoreTypeCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore_type: SemaphoreTypeKHR,
    pub initial_value: u64,
}
impl Default for SemaphoreTypeCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_TYPE_CREATE_INFO,
            p_next: std::ptr::null(),
            semaphore_type: Default::default(),
            initial_value: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkTimelineSemaphoreSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfoKHR.html)
pub struct TimelineSemaphoreSubmitInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
impl Default for TimelineSemaphoreSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_value_count: Default::default(),
            p_wait_semaphore_values: std::ptr::null(),
            signal_semaphore_value_count: Default::default(),
            p_signal_semaphore_values: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreWaitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfoKHR.html)
pub struct SemaphoreWaitInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SemaphoreWaitFlagsKHR,
    pub semaphore_count: u32,
    pub p_semaphores: *const crate::vk10::Semaphore,
    pub p_values: *const u64,
}
impl Default for SemaphoreWaitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_WAIT_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            semaphore_count: Default::default(),
            p_semaphores: std::ptr::null(),
            p_values: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreSignalInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfoKHR.html)
pub struct SemaphoreSignalInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub value: u64,
}
impl Default for SemaphoreSignalInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_SIGNAL_INFO,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html)
pub unsafe fn get_semaphore_counter_value_khr(
    device: crate::vk10::Device,
    semaphore: crate::vk10::Semaphore,
    p_value: *mut u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_semaphore_counter_value_khr
        .unwrap())(device, semaphore, p_value)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html)
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        semaphore: crate::vk10::Semaphore,
    ) -> crate::VulkanResult<u64> {
        let get_semaphore_counter_value_khr = (*self.table)
            .get_semaphore_counter_value_khr
            .unwrap();
        let mut value = Default::default();
        let result = get_semaphore_counter_value_khr(self.handle, semaphore, &mut value);
        crate::new_result(value, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWaitSemaphoresKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html)
pub unsafe fn wait_semaphores_khr(
    device: crate::vk10::Device,
    p_wait_info: *const SemaphoreWaitInfoKHR,
    timeout: u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .wait_semaphores_khr
        .unwrap())(device, p_wait_info, timeout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html)
    pub unsafe fn wait_semaphores_khr(
        &self,
        wait_info: &SemaphoreWaitInfoKHR,
        timeout: u64,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let wait_semaphores_khr = (*self.table).wait_semaphores_khr.unwrap();
        let result = wait_semaphores_khr(self.handle, wait_info as _, timeout as _);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSignalSemaphoreKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html)
pub unsafe fn signal_semaphore_khr(
    device: crate::vk10::Device,
    p_signal_info: *const SemaphoreSignalInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .signal_semaphore_khr
        .unwrap())(device, p_signal_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html)
    pub unsafe fn signal_semaphore_khr(
        &self,
        signal_info: &SemaphoreSignalInfoKHR,
    ) -> crate::VulkanResult<()> {
        let signal_semaphore_khr = (*self.table).signal_semaphore_khr.unwrap();
        let result = signal_semaphore_khr(self.handle, signal_info as _);
        crate::new_result((), result)
    }
}
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_timeline_semaphore"
);
