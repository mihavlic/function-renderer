/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/GgpStreamDescriptor.html)
pub type GgpStreamDescriptor = u32;
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/GgpFrameToken.html)
pub type GgpFrameToken = u64;
#[doc(alias = "VkStreamDescriptorSurfaceCreateFlagsGGP")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StreamDescriptorSurfaceCreateFlagsGGP(pub u32);
crate::bitflags_impl! {
    StreamDescriptorSurfaceCreateFlagsGGP : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkStreamDescriptorSurfaceCreateInfoGGP")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html)
pub struct StreamDescriptorSurfaceCreateInfoGGP {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: GgpStreamDescriptor,
}
impl Default for StreamDescriptorSurfaceCreateInfoGGP {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stream_descriptor: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html)
pub unsafe fn create_stream_descriptor_surface_ggp(
    instance: crate::vk10::Instance,
    p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_stream_descriptor_surface_ggp
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html)
    pub unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        create_info: &StreamDescriptorSurfaceCreateInfoGGP,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_stream_descriptor_surface_ggp = (*self.table)
            .create_stream_descriptor_surface_ggp
            .unwrap();
        let mut surface = Default::default();
        let result = create_stream_descriptor_surface_ggp(
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
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
pub const GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_GGP_stream_descriptor_surface"
);
