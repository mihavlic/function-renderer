/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/IDirectFB.html)
pub type IDirectFB = std::os::raw::c_void;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/IDirectFBSurface.html)
pub type IDirectFBSurface = std::os::raw::c_void;
#[doc(alias = "VkDirectFBSurfaceCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DirectFBSurfaceCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    DirectFBSurfaceCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html)
pub struct DirectFBSurfaceCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut IDirectFB,
    pub surface: *mut IDirectFBSurface,
}
impl Default for DirectFBSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dfb: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
pub unsafe fn create_direct_fbsurface_ext(
    instance: crate::vk10::Instance,
    p_create_info: *const DirectFBSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_direct_fbsurface_ext
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
    pub unsafe fn create_direct_fbsurface_ext(
        &self,
        create_info: &DirectFBSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_direct_fbsurface_ext = (*self.table)
            .create_direct_fbsurface_ext
            .unwrap();
        let mut surface = Default::default();
        let result = create_direct_fbsurface_ext(
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
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
pub unsafe fn get_physical_device_direct_fbpresentation_support_ext(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    dfb: *mut IDirectFB,
) -> crate::vk10::Bool32 {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_direct_fbpresentation_support_ext
        .unwrap())(physical_device, queue_family_index, dfb)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
    pub unsafe fn get_physical_device_direct_fbpresentation_support_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) {
        let get_physical_device_direct_fbpresentation_support_ext = (*self.table)
            .get_physical_device_direct_fbpresentation_support_ext
            .unwrap();
        get_physical_device_direct_fbpresentation_support_ext(
            physical_device,
            queue_family_index as _,
            dfb,
        );
    }
}
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_directfb_surface"
);
