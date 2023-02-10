/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/Display.html)
pub type Display = std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VisualID.html)
pub type VisualID = u64;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/Window.html)
pub type Window = u64;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/RROutput.html)
pub type RROutput = u64;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/xcb_connection_t.html)
pub type xcb_connection_t = std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/xcb_visualid_t.html)
pub type xcb_visualid_t = u32;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/xcb_window_t.html)
pub type xcb_window_t = u32;
#[doc(alias = "VkXcbSurfaceCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XcbSurfaceCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    XcbSurfaceCreateFlagsKHR : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkXcbSurfaceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html)
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}
impl Default for XcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::XCB_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            connection: std::ptr::null_mut(),
            window: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateXcbSurfaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html)
pub unsafe fn create_xcb_surface_khr(
    instance: crate::vk10::Instance,
    p_create_info: *const XcbSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_xcb_surface_khr
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html)
    pub unsafe fn create_xcb_surface_khr(
        &self,
        create_info: &XcbSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_xcb_surface_khr = (*self.table).create_xcb_surface_khr.unwrap();
        let mut surface = Default::default();
        let result = create_xcb_surface_khr(
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
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
pub unsafe fn get_physical_device_xcb_presentation_support_khr(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> crate::vk10::Bool32 {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_xcb_presentation_support_khr
        .unwrap())(physical_device, queue_family_index, connection, visual_id)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) {
        let get_physical_device_xcb_presentation_support_khr = (*self.table)
            .get_physical_device_xcb_presentation_support_khr
            .unwrap();
        get_physical_device_xcb_presentation_support_khr(
            physical_device,
            queue_family_index as _,
            connection,
            visual_id as _,
        );
    }
}
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
pub const KHR_XCB_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_xcb_surface"
);
