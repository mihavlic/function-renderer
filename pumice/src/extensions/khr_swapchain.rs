crate::dispatchable_handle!(
    SwapchainKHR, SWAPCHAIN_KHR, "VkSwapchainKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSwapchainCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html)
pub struct SwapchainCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: crate::extensions::khr_surface::SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: crate::vk10::Format,
    pub image_color_space: crate::extensions::khr_surface::ColorSpaceKHR,
    pub image_extent: crate::vk10::Extent2D,
    pub image_array_layers: u32,
    pub image_usage: crate::vk10::ImageUsageFlags,
    pub image_sharing_mode: crate::vk10::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    pub present_mode: crate::extensions::khr_surface::PresentModeKHR,
    pub clipped: crate::vk10::Bool32,
    pub old_swapchain: SwapchainKHR,
}
impl Default for SwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            min_image_count: Default::default(),
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: Default::default(),
            image_usage: Default::default(),
            image_sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            present_mode: Default::default(),
            clipped: Default::default(),
            old_swapchain: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPresentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html)
pub struct PresentInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const crate::vk10::Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut crate::vk10::Result,
}
impl Default for PresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphores: std::ptr::null(),
            swapchain_count: Default::default(),
            p_swapchains: std::ptr::null(),
            p_image_indices: std::ptr::null(),
            p_results: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html)
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub present_mask: [u32; crate::vk11::MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupPresentCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            present_mask: unsafe { std::mem::zeroed() },
            modes: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html)
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain: SwapchainKHR,
}
impl Default for ImageSwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html)
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: u32,
}
impl Default for BindImageMemorySwapchainInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            image_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAcquireNextImageInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html)
pub struct AcquireNextImageInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: crate::vk10::Semaphore,
    pub fence: crate::vk10::Fence,
    pub device_mask: u32,
}
impl Default for AcquireNextImageInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            timeout: Default::default(),
            semaphore: Default::default(),
            fence: Default::default(),
            device_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html)
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupPresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_device_masks: std::ptr::null(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html)
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupSwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            modes: Default::default(),
        }
    }
}
#[doc(alias = "VkDeviceGroupPresentModeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagsKHR(pub u32);
impl DeviceGroupPresentModeFlagsKHR {
    pub const LOCAL: Self = Self(1 << 0);
    pub const REMOTE: Self = Self(1 << 1);
    pub const SUM: Self = Self(1 << 2);
    pub const LOCAL_MULTI_DEVICE: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    DeviceGroupPresentModeFlagsKHR : u32, 0xf, LOCAL, REMOTE, SUM, LOCAL_MULTI_DEVICE
}
#[doc(alias = "VkSwapchainCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SwapchainCreateFlagsKHR(pub u32);
impl SwapchainCreateFlagsKHR {
    /// khr_swapchain
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1 << 0);
    pub const PROTECTED: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    SwapchainCreateFlagsKHR : u32, 0x3, SPLIT_INSTANCE_BIND_REGIONS, PROTECTED
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateSwapchainKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
pub unsafe fn create_swapchain_khr(
    device: crate::vk10::Device,
    p_create_info: *const SwapchainCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_swapchain: *mut SwapchainKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_swapchain_khr
        .unwrap())(device, p_create_info, p_allocator, p_swapchain)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateSwapchainKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
    pub unsafe fn create_swapchain_khr(
        &self,
        create_info: &SwapchainCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<SwapchainKHR> {
        let create_swapchain_khr = (*self.table).create_swapchain_khr.unwrap();
        let mut swapchain = Default::default();
        let result = create_swapchain_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut swapchain,
        );
        crate::new_result(swapchain, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroySwapchainKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
pub unsafe fn destroy_swapchain_khr(
    device: crate::vk10::Device,
    swapchain: SwapchainKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_swapchain_khr
        .unwrap())(device, swapchain, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroySwapchainKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
    pub unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: SwapchainKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_swapchain_khr = (*self.table).destroy_swapchain_khr.unwrap();
        destroy_swapchain_khr(
            self.handle,
            swapchain,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSwapchainImagesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
pub unsafe fn get_swapchain_images_khr(
    device: crate::vk10::Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut crate::vk10::Image,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_swapchain_images_khr
        .unwrap())(device, swapchain, p_swapchain_image_count, p_swapchain_images)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
    pub unsafe fn get_swapchain_images_khr(
        &self,
        swapchain: SwapchainKHR,
        swapchain_image_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<crate::vk10::Image>, crate::vk10::Result)> {
        let get_swapchain_images_khr = (*self.table).get_swapchain_images_khr.unwrap();
        let mut swapchain_image_count = match swapchain_image_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_swapchain_images_khr(
                    self.handle,
                    swapchain,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut swapchain_images = vec![
            Default::default(); swapchain_image_count as usize
        ];
        let result = get_swapchain_images_khr(
            self.handle,
            swapchain,
            &mut swapchain_image_count,
            swapchain_images.as_mut_ptr(),
        );
        crate::new_result((swapchain_images, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireNextImageKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
pub unsafe fn acquire_next_image_khr(
    device: crate::vk10::Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: crate::vk10::Semaphore,
    fence: crate::vk10::Fence,
    p_image_index: *mut u32,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .acquire_next_image_khr
        .unwrap())(device, swapchain, timeout, semaphore, fence, p_image_index)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireNextImageKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: crate::vk10::Semaphore,
        fence: crate::vk10::Fence,
    ) -> crate::VulkanResult<(u32, crate::vk10::Result)> {
        let acquire_next_image_khr = (*self.table).acquire_next_image_khr.unwrap();
        let mut image_index = Default::default();
        let result = acquire_next_image_khr(
            self.handle,
            swapchain,
            timeout as _,
            semaphore,
            fence,
            &mut image_index,
        );
        crate::new_result((image_index, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueuePresentKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
pub unsafe fn queue_present_khr(
    queue: crate::vk10::Queue,
    p_present_info: *const PresentInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_present_khr
        .unwrap())(queue, p_present_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueuePresentKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
    pub unsafe fn queue_present_khr(
        &self,
        queue: crate::vk10::Queue,
        present_info: &PresentInfoKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let queue_present_khr = (*self.table).queue_present_khr.unwrap();
        let result = queue_present_khr(queue, present_info as _);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
pub unsafe fn get_device_group_present_capabilities_khr(
    device: crate::vk10::Device,
    p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_group_present_capabilities_khr
        .unwrap())(device, p_device_group_present_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
    ) -> crate::VulkanResult<DeviceGroupPresentCapabilitiesKHR> {
        let get_device_group_present_capabilities_khr = (*self.table)
            .get_device_group_present_capabilities_khr
            .unwrap();
        let mut device_group_present_capabilities = Default::default();
        let result = get_device_group_present_capabilities_khr(
            self.handle,
            &mut device_group_present_capabilities,
        );
        crate::new_result(device_group_present_capabilities, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
pub unsafe fn get_device_group_surface_present_modes_khr(
    device: crate::vk10::Device,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_group_surface_present_modes_khr
        .unwrap())(device, surface, p_modes)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        modes: &mut DeviceGroupPresentModeFlagsKHR,
    ) -> crate::VulkanResult<()> {
        let get_device_group_surface_present_modes_khr = (*self.table)
            .get_device_group_surface_present_modes_khr
            .unwrap();
        let result = get_device_group_surface_present_modes_khr(
            self.handle,
            surface,
            modes as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireNextImage2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)
pub unsafe fn acquire_next_image_2_khr(
    device: crate::vk10::Device,
    p_acquire_info: *const AcquireNextImageInfoKHR,
    p_image_index: *mut u32,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .acquire_next_image_2_khr
        .unwrap())(device, p_acquire_info, p_image_index)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireNextImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)
    pub unsafe fn acquire_next_image_2_khr(
        &self,
        acquire_info: &AcquireNextImageInfoKHR,
    ) -> crate::VulkanResult<(u32, crate::vk10::Result)> {
        let acquire_next_image_2_khr = (*self.table).acquire_next_image_2_khr.unwrap();
        let mut image_index = Default::default();
        let result = acquire_next_image_2_khr(
            self.handle,
            acquire_info as _,
            &mut image_index,
        );
        crate::new_result((image_index, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
pub unsafe fn get_physical_device_present_rectangles_khr(
    physical_device: crate::vk10::PhysicalDevice,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut crate::vk10::Rect2D,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_present_rectangles_khr
        .unwrap())(physical_device, surface, p_rect_count, p_rects)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        rect_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<crate::vk10::Rect2D>, crate::vk10::Result)> {
        let get_physical_device_present_rectangles_khr = (*self.table)
            .get_physical_device_present_rectangles_khr
            .unwrap();
        let mut rect_count = match rect_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_present_rectangles_khr(
                    physical_device,
                    surface,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut rects = vec![Default::default(); rect_count as usize];
        let result = get_physical_device_present_rectangles_khr(
            physical_device,
            surface,
            &mut rect_count,
            rects.as_mut_ptr(),
        );
        crate::new_result((rects, result), result)
    }
}
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_swapchain"
);
