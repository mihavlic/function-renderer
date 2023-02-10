#[doc(alias = "VkXlibSurfaceCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XlibSurfaceCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    XlibSurfaceCreateFlagsKHR : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkXlibSurfaceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html)
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut crate::extensions::khr_xcb_surface::Display,
    pub window: crate::extensions::khr_xcb_surface::Window,
}
impl Default for XlibSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dpy: std::ptr::null_mut(),
            window: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateXlibSurfaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html)
pub unsafe fn create_xlib_surface_khr(
    instance: crate::vk10::Instance,
    p_create_info: *const XlibSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_xlib_surface_khr
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html)
    pub unsafe fn create_xlib_surface_khr(
        &self,
        create_info: &XlibSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_xlib_surface_khr = (*self.table).create_xlib_surface_khr.unwrap();
        let mut surface = Default::default();
        let result = create_xlib_surface_khr(
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
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
pub unsafe fn get_physical_device_xlib_presentation_support_khr(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut crate::extensions::khr_xcb_surface::Display,
    visual_id: crate::extensions::khr_xcb_surface::VisualID,
) -> crate::vk10::Bool32 {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_xlib_presentation_support_khr
        .unwrap())(physical_device, queue_family_index, dpy, visual_id)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        visual_id: crate::extensions::khr_xcb_surface::VisualID,
    ) {
        let get_physical_device_xlib_presentation_support_khr = (*self.table)
            .get_physical_device_xlib_presentation_support_khr
            .unwrap();
        get_physical_device_xlib_presentation_support_khr(
            physical_device,
            queue_family_index as _,
            dpy,
            visual_id as _,
        );
    }
}
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_xlib_surface"
);
