#[doc(alias = "VkPhysicalDeviceFeatures2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2KHR.html)
pub type PhysicalDeviceFeatures2KHR = crate::vk11::PhysicalDeviceFeatures2;
#[doc(alias = "VkPhysicalDeviceProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2KHR.html)
pub type PhysicalDeviceProperties2KHR = crate::vk11::PhysicalDeviceProperties2;
#[doc(alias = "VkFormatProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2KHR.html)
pub type FormatProperties2KHR = crate::vk11::FormatProperties2;
#[doc(alias = "VkImageFormatProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2KHR.html)
pub type ImageFormatProperties2KHR = crate::vk11::ImageFormatProperties2;
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2KHR.html)
pub type PhysicalDeviceImageFormatInfo2KHR = crate::vk11::PhysicalDeviceImageFormatInfo2;
#[doc(alias = "VkQueueFamilyProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2KHR.html)
pub type QueueFamilyProperties2KHR = crate::vk11::QueueFamilyProperties2;
#[doc(alias = "VkPhysicalDeviceMemoryProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2KHR.html)
pub type PhysicalDeviceMemoryProperties2KHR = crate::vk11::PhysicalDeviceMemoryProperties2;
#[doc(alias = "VkSparseImageFormatProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2KHR.html)
pub type SparseImageFormatProperties2KHR = crate::vk11::SparseImageFormatProperties2;
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2KHR.html)
pub type PhysicalDeviceSparseImageFormatInfo2KHR = crate::vk11::PhysicalDeviceSparseImageFormatInfo2;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html)
pub unsafe fn get_physical_device_features_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_features_2
        .unwrap())(physical_device, p_features)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html)
    pub unsafe fn get_physical_device_features_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
    ) -> crate::vk11::PhysicalDeviceFeatures2 {
        let get_physical_device_features_2_khr = (*self.table)
            .get_physical_device_features_2_khr
            .unwrap();
        let mut features = Default::default();
        get_physical_device_features_2_khr(physical_device, &mut features);
        features
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html)
pub unsafe fn get_physical_device_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_properties_2
        .unwrap())(physical_device, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html)
    pub unsafe fn get_physical_device_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
    ) -> crate::vk11::PhysicalDeviceProperties2 {
        let get_physical_device_properties_2_khr = (*self.table)
            .get_physical_device_properties_2_khr
            .unwrap();
        let mut properties = Default::default();
        get_physical_device_properties_2_khr(physical_device, &mut properties);
        properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html)
pub unsafe fn get_physical_device_format_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    format: crate::vk10::Format,
    p_format_properties: *mut crate::vk11::FormatProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_format_properties_2
        .unwrap())(physical_device, format, p_format_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
    ) -> crate::vk11::FormatProperties2 {
        let get_physical_device_format_properties_2_khr = (*self.table)
            .get_physical_device_format_properties_2_khr
            .unwrap();
        let mut format_properties = Default::default();
        get_physical_device_format_properties_2_khr(
            physical_device,
            format as _,
            &mut format_properties,
        );
        format_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
pub unsafe fn get_physical_device_image_format_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_image_format_properties_2
        .unwrap())(physical_device, p_image_format_info, p_image_format_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_image_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        image_format_info: &crate::vk11::PhysicalDeviceImageFormatInfo2,
    ) -> crate::VulkanResult<crate::vk11::ImageFormatProperties2> {
        let get_physical_device_image_format_properties_2_khr = (*self.table)
            .get_physical_device_image_format_properties_2_khr
            .unwrap();
        let mut image_format_properties = Default::default();
        let result = get_physical_device_image_format_properties_2_khr(
            physical_device,
            image_format_info as _,
            &mut image_format_properties,
        );
        crate::new_result(image_format_properties, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
pub unsafe fn get_physical_device_queue_family_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_queue_family_properties_2
        .unwrap())(
        physical_device,
        p_queue_family_property_count,
        p_queue_family_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
    pub unsafe fn get_physical_device_queue_family_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_property_count: Option<u32>,
        mut queue_family_properties_callback: impl FnMut(
            &mut Vec<crate::vk11::QueueFamilyProperties2>,
        ),
    ) -> Vec<crate::vk11::QueueFamilyProperties2> {
        let get_physical_device_queue_family_properties_2_khr = (*self.table)
            .get_physical_device_queue_family_properties_2_khr
            .unwrap();
        let mut queue_family_property_count = match queue_family_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_queue_family_properties_2_khr(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut queue_family_properties = vec![
            Default::default(); queue_family_property_count as usize
        ];
        queue_family_properties_callback(&mut queue_family_properties);
        get_physical_device_queue_family_properties_2_khr(
            physical_device,
            &mut queue_family_property_count,
            queue_family_properties.as_mut_ptr(),
        );
        queue_family_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html)
pub unsafe fn get_physical_device_memory_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_memory_properties_2
        .unwrap())(physical_device, p_memory_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html)
    pub unsafe fn get_physical_device_memory_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
    ) -> crate::vk11::PhysicalDeviceMemoryProperties2 {
        let get_physical_device_memory_properties_2_khr = (*self.table)
            .get_physical_device_memory_properties_2_khr
            .unwrap();
        let mut memory_properties = Default::default();
        get_physical_device_memory_properties_2_khr(
            physical_device,
            &mut memory_properties,
        );
        memory_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut crate::vk11::SparseImageFormatProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_sparse_image_format_properties_2
        .unwrap())(physical_device, p_format_info, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format_info: &crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
        mut properties_callback: impl FnMut(
            &mut Vec<crate::vk11::SparseImageFormatProperties2>,
        ),
    ) -> Vec<crate::vk11::SparseImageFormatProperties2> {
        let get_physical_device_sparse_image_format_properties_2_khr = (*self.table)
            .get_physical_device_sparse_image_format_properties_2_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_sparse_image_format_properties_2_khr(
                    physical_device,
                    format_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        properties_callback(&mut properties);
        get_physical_device_sparse_image_format_properties_2_khr(
            physical_device,
            format_info as _,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        properties
    }
}
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_get_physical_device_properties2"
);
