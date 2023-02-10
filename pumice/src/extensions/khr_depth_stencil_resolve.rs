#[doc(alias = "VkResolveModeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ResolveModeFlagsKHR(pub u32);
impl ResolveModeFlagsKHR {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO: Self = Self(1 << 0);
    pub const AVERAGE: Self = Self(1 << 1);
    pub const MIN: Self = Self(1 << 2);
    pub const MAX: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    ResolveModeFlagsKHR : u32, 0xf, NONE, SAMPLE_ZERO, AVERAGE, MIN, MAX
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDepthStencilResolvePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html)
pub struct PhysicalDeviceDepthStencilResolvePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub supported_depth_resolve_modes: ResolveModeFlagsKHR,
    pub supported_stencil_resolve_modes: ResolveModeFlagsKHR,
    pub independent_resolve_none: crate::vk10::Bool32,
    pub independent_resolve: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDepthStencilResolvePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: Default::default(),
            independent_resolve: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDescriptionDepthStencilResolveKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolveKHR.html)
pub struct SubpassDescriptionDepthStencilResolveKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub depth_resolve_mode: ResolveModeFlagsKHR,
    pub stencil_resolve_mode: ResolveModeFlagsKHR,
    pub p_depth_stencil_resolve_attachment: *const crate::extensions::khr_create_renderpass2::AttachmentReference2KHR,
}
impl Default for SubpassDescriptionDepthStencilResolveKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
            p_next: std::ptr::null(),
            depth_resolve_mode: Default::default(),
            stencil_resolve_mode: Default::default(),
            p_depth_stencil_resolve_attachment: std::ptr::null(),
        }
    }
}
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
pub const KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_depth_stencil_resolve"
);
