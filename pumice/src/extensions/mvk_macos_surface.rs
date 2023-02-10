#[doc(alias = "VkMacOSSurfaceCreateFlagsMVK")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MacOSSurfaceCreateFlagsMVK(pub u32);
crate::bitflags_impl! {
    MacOSSurfaceCreateFlagsMVK : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html)
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const std::os::raw::c_void,
}
impl Default for MacOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MACOS_SURFACE_CREATE_INFO_MVK,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_view: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
pub unsafe fn create_mac_ossurface_mvk(
    instance: crate::vk10::Instance,
    p_create_info: *const MacOSSurfaceCreateInfoMVK,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_mac_ossurface_mvk
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
    pub unsafe fn create_mac_ossurface_mvk(
        &self,
        create_info: &MacOSSurfaceCreateInfoMVK,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_mac_ossurface_mvk = (*self.table).create_mac_ossurface_mvk.unwrap();
        let mut surface = Default::default();
        let result = create_mac_ossurface_mvk(
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
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_MVK_macos_surface"
);
