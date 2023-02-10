/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/wl_display.html)
pub type wl_display = std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/wl_surface.html)
pub type wl_surface = std::os::raw::c_void;
#[doc(alias = "VkWaylandSurfaceCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WaylandSurfaceCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    WaylandSurfaceCreateFlagsKHR : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWaylandSurfaceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html)
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
}
impl Default for WaylandSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            display: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
pub unsafe fn create_wayland_surface_khr(
    instance: crate::vk10::Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_wayland_surface_khr
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
    pub unsafe fn create_wayland_surface_khr(
        &self,
        create_info: &WaylandSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_wayland_surface_khr = (*self.table)
            .create_wayland_surface_khr
            .unwrap();
        let mut surface = Default::default();
        let result = create_wayland_surface_khr(
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
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
pub unsafe fn get_physical_device_wayland_presentation_support_khr(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
) -> crate::vk10::Bool32 {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_wayland_presentation_support_khr
        .unwrap())(physical_device, queue_family_index, display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) {
        let get_physical_device_wayland_presentation_support_khr = (*self.table)
            .get_physical_device_wayland_presentation_support_khr
            .unwrap();
        get_physical_device_wayland_presentation_support_khr(
            physical_device,
            queue_family_index as _,
            display,
        );
    }
}
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_wayland_surface"
);
