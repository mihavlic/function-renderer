#[doc(alias = "VkViSurfaceCreateFlagsNN")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateFlagsNN.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ViSurfaceCreateFlagsNN(pub u32);
crate::bitflags_impl! {
    ViSurfaceCreateFlagsNN : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkViSurfaceCreateInfoNN")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateInfoNN.html)
pub struct ViSurfaceCreateInfoNN {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *mut std::os::raw::c_void,
}
impl Default for ViSurfaceCreateInfoNN {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VI_SURFACE_CREATE_INFO_NN,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateViSurfaceNN")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html)
pub unsafe fn create_vi_surface_nn(
    instance: crate::vk10::Instance,
    p_create_info: *const ViSurfaceCreateInfoNN,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_vi_surface_nn
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateViSurfaceNN")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html)
    pub unsafe fn create_vi_surface_nn(
        &self,
        create_info: &ViSurfaceCreateInfoNN,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_vi_surface_nn = (*self.table).create_vi_surface_nn.unwrap();
        let mut surface = Default::default();
        let result = create_vi_surface_nn(
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
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
pub const NN_VI_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NN_vi_surface"
);
