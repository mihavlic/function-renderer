/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/_screen_context.html)
pub type _screen_context = std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/_screen_window.html)
pub type _screen_window = std::os::raw::c_void;
#[doc(alias = "VkScreenSurfaceCreateFlagsQNX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ScreenSurfaceCreateFlagsQNX(pub u32);
crate::bitflags_impl! {
    ScreenSurfaceCreateFlagsQNX : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkScreenSurfaceCreateInfoQNX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html)
pub struct ScreenSurfaceCreateInfoQNX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ScreenSurfaceCreateFlagsQNX,
    pub context: *mut _screen_context,
    pub window: *mut _screen_window,
}
impl Default for ScreenSurfaceCreateInfoQNX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SCREEN_SURFACE_CREATE_INFO_QNX,
            p_next: std::ptr::null(),
            flags: Default::default(),
            context: std::ptr::null_mut(),
            window: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateScreenSurfaceQNX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html)
pub unsafe fn create_screen_surface_qnx(
    instance: crate::vk10::Instance,
    p_create_info: *const ScreenSurfaceCreateInfoQNX,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_screen_surface_qnx
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html)
    pub unsafe fn create_screen_surface_qnx(
        &self,
        create_info: &ScreenSurfaceCreateInfoQNX,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_screen_surface_qnx = (*self.table).create_screen_surface_qnx.unwrap();
        let mut surface = Default::default();
        let result = create_screen_surface_qnx(
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
#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
pub unsafe fn get_physical_device_screen_presentation_support_qnx(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> crate::vk10::Bool32 {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_screen_presentation_support_qnx
        .unwrap())(physical_device, queue_family_index, window)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) {
        let get_physical_device_screen_presentation_support_qnx = (*self.table)
            .get_physical_device_screen_presentation_support_qnx
            .unwrap();
        get_physical_device_screen_presentation_support_qnx(
            physical_device,
            queue_family_index as _,
            window,
        );
    }
}
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
pub const QNX_SCREEN_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_QNX_screen_surface"
);
