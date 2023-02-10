pub type zx_handle_t = *mut std::os::raw::c_void;
#[doc(alias = "VkImagePipeSurfaceCreateFlagsFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(pub u32);
crate::bitflags_impl! {
    ImagePipeSurfaceCreateFlagsFUCHSIA : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html)
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    pub image_pipe_handle: zx_handle_t,
}
impl Default for ImagePipeSurfaceCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            flags: Default::default(),
            image_pipe_handle: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
pub unsafe fn create_image_pipe_surface_fuchsia(
    instance: crate::vk10::Instance,
    p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_image_pipe_surface_fuchsia
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_image_pipe_surface_fuchsia = (*self.table)
            .create_image_pipe_surface_fuchsia
            .unwrap();
        let mut surface = Default::default();
        let result = create_image_pipe_surface_fuchsia(
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
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
pub const FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_FUCHSIA_imagepipe_surface"
);
