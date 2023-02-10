#[doc(alias = "VkIOSSurfaceCreateFlagsMVK")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct IOSSurfaceCreateFlagsMVK(pub u32);
crate::bitflags_impl! {
    IOSSurfaceCreateFlagsMVK : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkIOSSurfaceCreateInfoMVK")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html)
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub p_view: *const std::os::raw::c_void,
}
impl Default for IOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IOS_SURFACE_CREATE_INFO_MVK,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_view: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateIOSSurfaceMVK")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html)
pub unsafe fn create_iossurface_mvk(
    instance: crate::vk10::Instance,
    p_create_info: *const IOSSurfaceCreateInfoMVK,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_iossurface_mvk
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html)
    pub unsafe fn create_iossurface_mvk(
        &self,
        create_info: &IOSSurfaceCreateInfoMVK,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_iossurface_mvk = (*self.table).create_iossurface_mvk.unwrap();
        let mut surface = Default::default();
        let result = create_iossurface_mvk(
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
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
pub const MVK_IOS_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_MVK_ios_surface"
);
