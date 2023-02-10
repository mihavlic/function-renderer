crate::dispatchable_handle!(
    SurfaceKHR, SURFACE_KHR, "VkSurfaceKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html)"
);
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSurfaceCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: crate::vk10::Extent2D,
    pub min_image_extent: crate::vk10::Extent2D,
    pub max_image_extent: crate::vk10::Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: crate::vk10::ImageUsageFlags,
}
impl Default for SurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self {
            min_image_count: Default::default(),
            max_image_count: Default::default(),
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: Default::default(),
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSurfaceFormatKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html)
pub struct SurfaceFormatKHR {
    pub format: crate::vk10::Format,
    pub color_space: ColorSpaceKHR,
}
impl Default for SurfaceFormatKHR {
    fn default() -> Self {
        Self {
            format: Default::default(),
            color_space: Default::default(),
        }
    }
}
#[doc(alias = "VkPresentModeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PresentModeKHR(pub i32);
impl PresentModeKHR {
    pub const IMMEDIATE: Self = Self(0);
    pub const MAILBOX: Self = Self(1);
    pub const FIFO: Self = Self(2);
    pub const FIFO_RELAXED: Self = Self(3);
}
crate::enum_impl! {
    PresentModeKHR : i32, IMMEDIATE, MAILBOX, FIFO, FIFO_RELAXED
}
#[doc(alias = "VkColorSpaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ColorSpaceKHR(pub i32);
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR: Self = Self(0);
}
crate::enum_impl! {
    ColorSpaceKHR : i32, SRGB_NONLINEAR
}
#[doc(alias = "VkCompositeAlphaFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CompositeAlphaFlagsKHR(pub u32);
impl CompositeAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(1 << 0);
    pub const PRE_MULTIPLIED: Self = Self(1 << 1);
    pub const POST_MULTIPLIED: Self = Self(1 << 2);
    pub const INHERIT: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    CompositeAlphaFlagsKHR : u32, 0xf, OPAQUE, PRE_MULTIPLIED, POST_MULTIPLIED, INHERIT
}
#[doc(alias = "VkSurfaceTransformFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SurfaceTransformFlagsKHR(pub u32);
impl SurfaceTransformFlagsKHR {
    pub const IDENTITY: Self = Self(1 << 0);
    pub const ROTATE_90: Self = Self(1 << 1);
    pub const ROTATE_180: Self = Self(1 << 2);
    pub const ROTATE_270: Self = Self(1 << 3);
    pub const HORIZONTAL_MIRROR: Self = Self(1 << 4);
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(1 << 5);
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = Self(1 << 6);
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = Self(1 << 7);
    pub const INHERIT: Self = Self(1 << 8);
}
crate::bitflags_impl! {
    SurfaceTransformFlagsKHR : u32, 0x1ff, IDENTITY, ROTATE_90, ROTATE_180, ROTATE_270,
    HORIZONTAL_MIRROR, HORIZONTAL_MIRROR_ROTATE_90, HORIZONTAL_MIRROR_ROTATE_180,
    HORIZONTAL_MIRROR_ROTATE_270, INHERIT
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroySurfaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
pub unsafe fn destroy_surface_khr(
    instance: crate::vk10::Instance,
    surface: SurfaceKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .destroy_surface_khr
        .unwrap())(instance, surface, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroySurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
    pub unsafe fn destroy_surface_khr(
        &self,
        surface: SurfaceKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_surface_khr = (*self.table).destroy_surface_khr.unwrap();
        destroy_surface_khr(
            self.handle,
            surface,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
pub unsafe fn get_physical_device_surface_support_khr(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut crate::vk10::Bool32,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_support_khr
        .unwrap())(physical_device, queue_family_index, surface, p_supported)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
    ) -> crate::VulkanResult<crate::vk10::Bool32> {
        let get_physical_device_surface_support_khr = (*self.table)
            .get_physical_device_surface_support_khr
            .unwrap();
        let mut supported = Default::default();
        let result = get_physical_device_surface_support_khr(
            physical_device,
            queue_family_index as _,
            surface,
            &mut supported,
        );
        crate::new_result(supported, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
pub unsafe fn get_physical_device_surface_capabilities_khr(
    physical_device: crate::vk10::PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_capabilities_khr
        .unwrap())(physical_device, surface, p_surface_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::VulkanResult<SurfaceCapabilitiesKHR> {
        let get_physical_device_surface_capabilities_khr = (*self.table)
            .get_physical_device_surface_capabilities_khr
            .unwrap();
        let mut surface_capabilities = Default::default();
        let result = get_physical_device_surface_capabilities_khr(
            physical_device,
            surface,
            &mut surface_capabilities,
        );
        crate::new_result(surface_capabilities, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
pub unsafe fn get_physical_device_surface_formats_khr(
    physical_device: crate::vk10::PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_formats_khr
        .unwrap())(physical_device, surface, p_surface_format_count, p_surface_formats)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: SurfaceKHR,
        surface_format_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<SurfaceFormatKHR>, crate::vk10::Result)> {
        let get_physical_device_surface_formats_khr = (*self.table)
            .get_physical_device_surface_formats_khr
            .unwrap();
        let mut surface_format_count = match surface_format_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_surface_formats_khr(
                    physical_device,
                    surface,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut surface_formats = vec![
            Default::default(); surface_format_count as usize
        ];
        let result = get_physical_device_surface_formats_khr(
            physical_device,
            surface,
            &mut surface_format_count,
            surface_formats.as_mut_ptr(),
        );
        crate::new_result((surface_formats, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
pub unsafe fn get_physical_device_surface_present_modes_khr(
    physical_device: crate::vk10::PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_present_modes_khr
        .unwrap())(physical_device, surface, p_present_mode_count, p_present_modes)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: SurfaceKHR,
        present_mode_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<PresentModeKHR>, crate::vk10::Result)> {
        let get_physical_device_surface_present_modes_khr = (*self.table)
            .get_physical_device_surface_present_modes_khr
            .unwrap();
        let mut present_mode_count = match present_mode_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_surface_present_modes_khr(
                    physical_device,
                    surface,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut present_modes = vec![Default::default(); present_mode_count as usize];
        let result = get_physical_device_surface_present_modes_khr(
            physical_device,
            surface,
            &mut present_mode_count,
            present_modes.as_mut_ptr(),
        );
        crate::new_result((present_modes, result), result)
    }
}
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const KHR_SURFACE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!("VK_KHR_surface");
