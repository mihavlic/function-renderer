/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html)
pub type CAMetalLayer = std::os::raw::c_void;
#[doc(alias = "VkMetalSurfaceCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MetalSurfaceCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    MetalSurfaceCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMetalSurfaceCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html)
pub struct MetalSurfaceCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: MetalSurfaceCreateFlagsEXT,
    pub p_layer: *const CAMetalLayer,
}
impl Default for MetalSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::METAL_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_layer: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateMetalSurfaceEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html)
pub unsafe fn create_metal_surface_ext(
    instance: crate::vk10::Instance,
    p_create_info: *const MetalSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_metal_surface_ext
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html)
    pub unsafe fn create_metal_surface_ext(
        &self,
        create_info: &MetalSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_metal_surface_ext = (*self.table).create_metal_surface_ext.unwrap();
        let mut surface = Default::default();
        let result = create_metal_surface_ext(
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
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
pub const EXT_METAL_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_metal_surface"
);
