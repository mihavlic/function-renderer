#[doc(alias = "VkHeadlessSurfaceCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct HeadlessSurfaceCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    HeadlessSurfaceCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkHeadlessSurfaceCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html)
pub struct HeadlessSurfaceCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: HeadlessSurfaceCreateFlagsEXT,
}
impl Default for HeadlessSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
pub unsafe fn create_headless_surface_ext(
    instance: crate::vk10::Instance,
    p_create_info: *const HeadlessSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_headless_surface_ext
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
    pub unsafe fn create_headless_surface_ext(
        &self,
        create_info: &HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_headless_surface_ext = (*self.table)
            .create_headless_surface_ext
            .unwrap();
        let mut surface = Default::default();
        let result = create_headless_surface_ext(
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
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
pub const EXT_HEADLESS_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_headless_surface"
);
