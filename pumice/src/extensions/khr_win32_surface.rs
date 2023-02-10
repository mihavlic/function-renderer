pub type HINSTANCE = *mut std::os::raw::c_void;
pub type HWND = *mut std::os::raw::c_void;
pub type HMONITOR = *mut std::os::raw::c_void;
pub type HANDLE = *mut std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/SECURITY_ATTRIBUTES.html)
pub type SECURITY_ATTRIBUTES = std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/DWORD.html)
pub type DWORD = u32;
pub type LPCWSTR = *const u16;
#[doc(alias = "VkWin32SurfaceCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Win32SurfaceCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    Win32SurfaceCreateFlagsKHR : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWin32SurfaceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html)
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
impl Default for Win32SurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            hinstance: std::ptr::null_mut(),
            hwnd: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateWin32SurfaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)
pub unsafe fn create_win_32_surface_khr(
    instance: crate::vk10::Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_win_32_surface_khr
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)
    pub unsafe fn create_win_32_surface_khr(
        &self,
        create_info: &Win32SurfaceCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_win_32_surface_khr = (*self.table).create_win_32_surface_khr.unwrap();
        let mut surface = Default::default();
        let result = create_win_32_surface_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut surface,
        );
        crate::new_result(surface, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
pub unsafe fn get_physical_device_win_32_presentation_support_khr(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
) -> crate::vk10::Bool32 {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_win_32_presentation_support_khr
        .unwrap())(physical_device, queue_family_index)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
    pub unsafe fn get_physical_device_win_32_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
    ) {
        let get_physical_device_win_32_presentation_support_khr = (*self.table)
            .get_physical_device_win_32_presentation_support_khr
            .unwrap();
        get_physical_device_win_32_presentation_support_khr(
            physical_device,
            queue_family_index as _,
        );
    }
}
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_win32_surface"
);
