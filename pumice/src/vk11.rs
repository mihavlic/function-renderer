pub const API_VERSION_1_1: u32 = crate::vk10::make_api_version(0, 1, 1, 0);
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateCreateFlags(pub u32);
crate::bitflags_impl! {
    DescriptorUpdateTemplateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkCommandPoolTrimFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandPoolTrimFlags(pub u32);
crate::bitflags_impl! {
    CommandPoolTrimFlags : u32, 0x0,
}
crate::dispatchable_handle!(
    DescriptorUpdateTemplate, DESCRIPTOR_UPDATE_TEMPLATE, "VkDescriptorUpdateTemplate",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html)"
);
crate::dispatchable_handle!(
    SamplerYcbcrConversion, SAMPLER_YCBCR_CONVERSION, "VkSamplerYcbcrConversion",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversion.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFeatures2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html)
pub struct PhysicalDeviceFeatures2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub features: crate::vk10::PhysicalDeviceFeatures,
}
impl Default for PhysicalDeviceFeatures2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FEATURES_2,
            p_next: std::ptr::null_mut(),
            features: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html)
pub struct PhysicalDeviceProperties2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub properties: crate::vk10::PhysicalDeviceProperties,
}
impl Default for PhysicalDeviceProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFormatProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2.html)
pub struct FormatProperties2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub format_properties: crate::vk10::FormatProperties,
}
impl Default for FormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            format_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageFormatProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2.html)
pub struct ImageFormatProperties2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_format_properties: crate::vk10::ImageFormatProperties,
}
impl Default for ImageFormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            image_format_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html)
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub format: crate::vk10::Format,
    pub kind: crate::vk10::ImageType,
    pub tiling: crate::vk10::ImageTiling,
    pub usage: crate::vk10::ImageUsageFlags,
    pub flags: crate::vk10::ImageCreateFlags,
}
impl Default for PhysicalDeviceImageFormatInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
            p_next: std::ptr::null(),
            format: Default::default(),
            kind: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html)
pub struct QueueFamilyProperties2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub queue_family_properties: crate::vk10::QueueFamilyProperties,
}
impl Default for QueueFamilyProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUEUE_FAMILY_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            queue_family_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html)
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_properties: crate::vk10::PhysicalDeviceMemoryProperties,
}
impl Default for PhysicalDeviceMemoryProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            memory_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSparseImageFormatProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2.html)
pub struct SparseImageFormatProperties2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub properties: crate::vk10::SparseImageFormatProperties,
}
impl Default for SparseImageFormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html)
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub format: crate::vk10::Format,
    pub kind: crate::vk10::ImageType,
    pub samples: crate::vk10::SampleCountFlags,
    pub usage: crate::vk10::ImageUsageFlags,
    pub tiling: crate::vk10::ImageTiling,
}
impl Default for PhysicalDeviceSparseImageFormatInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
            p_next: std::ptr::null(),
            format: Default::default(),
            kind: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html)
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub variable_pointers_storage_buffer: crate::vk10::Bool32,
    pub variable_pointers: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVariablePointersFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
            p_next: std::ptr::null_mut(),
            variable_pointers_storage_buffer: Default::default(),
            variable_pointers: Default::default(),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVariablePointerFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointerFeatures.html)
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkExternalMemoryProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryProperties.html)
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryProperties {
    fn default() -> Self {
        Self {
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html)
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
impl Default for PhysicalDeviceExternalImageFormatInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalImageFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatProperties.html)
pub struct ExternalImageFormatProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
impl Default for ExternalImageFormatProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html)
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::BufferCreateFlags,
    pub usage: crate::vk10::BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
impl Default for PhysicalDeviceExternalBufferInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalBufferProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalBufferProperties.html)
pub struct ExternalBufferProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
impl Default for ExternalBufferProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_BUFFER_PROPERTIES,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceIDProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDProperties.html)
pub struct PhysicalDeviceIDProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_uuid: [u8; crate::vk10::UUID_SIZE as usize],
    pub driver_uuid: [u8; crate::vk10::UUID_SIZE as usize],
    pub device_luid: [u8; LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luidvalid: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceIDProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ID_PROPERTIES,
            p_next: std::ptr::null_mut(),
            device_uuid: unsafe { std::mem::zeroed() },
            driver_uuid: unsafe { std::mem::zeroed() },
            device_luid: unsafe { std::mem::zeroed() },
            device_node_mask: Default::default(),
            device_luidvalid: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html)
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryImageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfo.html)
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryBufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMemoryAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html)
pub struct ExportMemoryAllocateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExportMemoryAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_MEMORY_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html)
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
impl Default for PhysicalDeviceExternalSemaphoreInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalSemaphoreProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreProperties.html)
pub struct ExternalSemaphoreProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
impl Default for ExternalSemaphoreProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_SEMAPHORE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_semaphore_features: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportSemaphoreCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html)
pub struct ExportSemaphoreCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
impl Default for ExportSemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_SEMAPHORE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html)
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
impl Default for PhysicalDeviceExternalFenceInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalFenceProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceProperties.html)
pub struct ExternalFenceProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}
impl Default for ExternalFenceProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_FENCE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_fence_features: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportFenceCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html)
pub struct ExportFenceCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}
impl Default for ExportFenceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html)
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub multiview: crate::vk10::Bool32,
    pub multiview_geometry_shader: crate::vk10::Bool32,
    pub multiview_tessellation_shader: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMultiviewFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
            p_next: std::ptr::null_mut(),
            multiview: Default::default(),
            multiview_geometry_shader: Default::default(),
            multiview_tessellation_shader: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html)
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
impl Default for PhysicalDeviceMultiviewProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_multiview_view_count: Default::default(),
            max_multiview_instance_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassMultiviewCreateInfo.html)
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
impl Default for RenderPassMultiviewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO,
            p_next: std::ptr::null(),
            subpass_count: Default::default(),
            p_view_masks: std::ptr::null(),
            dependency_count: Default::default(),
            p_view_offsets: std::ptr::null(),
            correlation_mask_count: Default::default(),
            p_correlation_masks: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupProperties.html)
pub struct PhysicalDeviceGroupProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub physical_device_count: u32,
    pub physical_devices: [crate::vk10::PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    pub subset_allocation: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceGroupProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES,
            p_next: std::ptr::null_mut(),
            physical_device_count: Default::default(),
            physical_devices: unsafe { std::mem::zeroed() },
            subset_allocation: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html)
pub struct MemoryAllocateFlagsInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}
impl Default for MemoryAllocateFlagsInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_ALLOCATE_FLAGS_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            device_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindBufferMemoryInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfo.html)
pub struct BindBufferMemoryInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub buffer: crate::vk10::Buffer,
    pub memory: crate::vk10::DeviceMemory,
    pub memory_offset: crate::vk10::DeviceSize,
}
impl Default for BindBufferMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_BUFFER_MEMORY_INFO,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html)
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl Default for BindBufferMemoryDeviceGroupInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
            p_next: std::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindImageMemoryInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html)
pub struct BindImageMemoryInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
    pub memory: crate::vk10::DeviceMemory,
    pub memory_offset: crate::vk10::DeviceSize,
}
impl Default for BindImageMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_IMAGE_MEMORY_INFO,
            p_next: std::ptr::null(),
            image: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html)
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const crate::vk10::Rect2D,
}
impl Default for BindImageMemoryDeviceGroupInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
            p_next: std::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
            split_instance_bind_region_count: Default::default(),
            p_split_instance_bind_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html)
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const crate::vk10::Rect2D,
}
impl Default for DeviceGroupRenderPassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            device_mask: Default::default(),
            device_render_area_count: Default::default(),
            p_device_render_areas: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html)
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub device_mask: u32,
}
impl Default for DeviceGroupCommandBufferBeginInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            device_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupSubmitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfo.html)
pub struct DeviceGroupSubmitInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
impl Default for DeviceGroupSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphore_device_indices: std::ptr::null(),
            command_buffer_count: Default::default(),
            p_command_buffer_device_masks: std::ptr::null(),
            signal_semaphore_count: Default::default(),
            p_signal_semaphore_device_indices: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfo.html)
pub struct DeviceGroupBindSparseInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
impl Default for DeviceGroupBindSparseInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_BIND_SPARSE_INFO,
            p_next: std::ptr::null(),
            resource_device_index: Default::default(),
            memory_device_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html)
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const crate::vk10::PhysicalDevice,
}
impl Default for DeviceGroupDeviceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            physical_device_count: Default::default(),
            p_physical_devices: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDescriptorUpdateTemplateEntry")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntry.html)
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: crate::vk10::DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
impl Default for DescriptorUpdateTemplateEntry {
    fn default() -> Self {
        Self {
            dst_binding: Default::default(),
            dst_array_element: Default::default(),
            descriptor_count: Default::default(),
            descriptor_type: Default::default(),
            offset: Default::default(),
            stride: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html)
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: crate::vk10::DescriptorSetLayout,
    pub pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pub pipeline_layout: crate::vk10::PipelineLayout,
    pub set: u32,
}
impl Default for DescriptorUpdateTemplateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            descriptor_update_entry_count: Default::default(),
            p_descriptor_update_entries: std::ptr::null(),
            template_type: Default::default(),
            descriptor_set_layout: Default::default(),
            pipeline_bind_point: Default::default(),
            pipeline_layout: Default::default(),
            set: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkInputAttachmentAspectReference")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html)
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: crate::vk10::ImageAspectFlags,
}
impl Default for InputAttachmentAspectReference {
    fn default() -> Self {
        Self {
            subpass: Default::default(),
            input_attachment_index: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html)
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}
impl Default for RenderPassInputAttachmentAspectCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
            p_next: std::ptr::null(),
            aspect_reference_count: Default::default(),
            p_aspect_references: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html)
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub storage_buffer_16_bit_access: crate::vk10::Bool32,
    pub uniform_and_storage_buffer_16_bit_access: crate::vk10::Bool32,
    pub storage_push_constant_16: crate::vk10::Bool32,
    pub storage_input_output_16: crate::vk10::Bool32,
}
impl Default for PhysicalDevice16BitStorageFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer_16_bit_access: Default::default(),
            uniform_and_storage_buffer_16_bit_access: Default::default(),
            storage_push_constant_16: Default::default(),
            storage_input_output_16: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html)
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub subgroup_size: u32,
    pub supported_stages: crate::vk10::ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSubgroupProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
            p_next: std::ptr::null_mut(),
            subgroup_size: Default::default(),
            supported_stages: Default::default(),
            supported_operations: Default::default(),
            quad_operations_in_all_stages: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryRequirementsInfo2.html)
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub buffer: crate::vk10::Buffer,
}
impl Default for BufferMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2.html)
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
}
impl Default for ImageMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html)
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
}
impl Default for ImageSparseMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryRequirements2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html)
pub struct MemoryRequirements2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_requirements: crate::vk10::MemoryRequirements,
}
impl Default for MemoryRequirements2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_REQUIREMENTS_2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSparseImageMemoryRequirements2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2.html)
pub struct SparseImageMemoryRequirements2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_requirements: crate::vk10::SparseImageMemoryRequirements,
}
impl Default for SparseImageMemoryRequirements2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePointClippingProperties.html)
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}
impl Default for PhysicalDevicePointClippingProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
            p_next: std::ptr::null_mut(),
            point_clipping_behavior: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryDedicatedRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedRequirements.html)
pub struct MemoryDedicatedRequirements {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub prefers_dedicated_allocation: crate::vk10::Bool32,
    pub requires_dedicated_allocation: crate::vk10::Bool32,
}
impl Default for MemoryDedicatedRequirements {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_DEDICATED_REQUIREMENTS,
            p_next: std::ptr::null_mut(),
            prefers_dedicated_allocation: Default::default(),
            requires_dedicated_allocation: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html)
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
    pub buffer: crate::vk10::Buffer,
}
impl Default for MemoryDedicatedAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewUsageCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html)
pub struct ImageViewUsageCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub usage: crate::vk10::ImageUsageFlags,
}
impl Default for ImageViewUsageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_VIEW_USAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            usage: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html)
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub domain_origin: TessellationDomainOrigin,
}
impl Default for PipelineTessellationDomainOriginStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            domain_origin: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html)
pub struct SamplerYcbcrConversionInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub conversion: SamplerYcbcrConversion,
}
impl Default for SamplerYcbcrConversionInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_INFO,
            p_next: std::ptr::null(),
            conversion: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html)
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub format: crate::vk10::Format,
    pub ycbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: crate::vk10::ComponentMapping,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: crate::vk10::Filter,
    pub force_explicit_reconstruction: crate::vk10::Bool32,
}
impl Default for SamplerYcbcrConversionCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
            p_next: std::ptr::null(),
            format: Default::default(),
            ycbcr_model: Default::default(),
            ycbcr_range: Default::default(),
            components: Default::default(),
            x_chroma_offset: Default::default(),
            y_chroma_offset: Default::default(),
            chroma_filter: Default::default(),
            force_explicit_reconstruction: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImagePlaneMemoryInfo.html)
pub struct BindImagePlaneMemoryInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub plane_aspect: crate::vk10::ImageAspectFlags,
}
impl Default for BindImagePlaneMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_IMAGE_PLANE_MEMORY_INFO,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html)
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub plane_aspect: crate::vk10::ImageAspectFlags,
}
impl Default for ImagePlaneMemoryRequirementsInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html)
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub sampler_ycbcr_conversion: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
            p_next: std::ptr::null_mut(),
            sampler_ycbcr_conversion: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html)
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub combined_image_sampler_descriptor_count: u32,
}
impl Default for SamplerYcbcrConversionImageFormatProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            combined_image_sampler_descriptor_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkProtectedSubmitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProtectedSubmitInfo.html)
pub struct ProtectedSubmitInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub protected_submit: crate::vk10::Bool32,
}
impl Default for ProtectedSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PROTECTED_SUBMIT_INFO,
            p_next: std::ptr::null(),
            protected_submit: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html)
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub protected_memory: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceProtectedMemoryFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
            p_next: std::ptr::null_mut(),
            protected_memory: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html)
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub protected_no_fault: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceProtectedMemoryProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
            p_next: std::ptr::null_mut(),
            protected_no_fault: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceQueueInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html)
pub struct DeviceQueueInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
impl Default for DeviceQueueInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_QUEUE_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: Default::default(),
            queue_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html)
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: crate::vk10::DeviceSize,
}
impl Default for PhysicalDeviceMaintenance3Properties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_per_set_descriptors: Default::default(),
            max_memory_allocation_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetLayoutSupport")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupport.html)
pub struct DescriptorSetLayoutSupport {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub supported: crate::vk10::Bool32,
}
impl Default for DescriptorSetLayoutSupport {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT,
            p_next: std::ptr::null_mut(),
            supported: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html)
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_draw_parameters: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderDrawParametersFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_draw_parameters: Default::default(),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderDrawParameterFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDrawParameterFeatures.html)
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
pub const LUID_SIZE: u32 = 8;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
#[doc(alias = "VkDescriptorUpdateTemplateType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateType(pub i32);
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
}
crate::enum_impl! {
    DescriptorUpdateTemplateType : i32, DESCRIPTOR_SET
}
#[doc(alias = "VkSubgroupFeatureFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubgroupFeatureFlags(pub u32);
impl SubgroupFeatureFlags {
    pub const BASIC: Self = Self(1 << 0);
    pub const VOTE: Self = Self(1 << 1);
    pub const ARITHMETIC: Self = Self(1 << 2);
    pub const BALLOT: Self = Self(1 << 3);
    pub const SHUFFLE: Self = Self(1 << 4);
    pub const SHUFFLE_RELATIVE: Self = Self(1 << 5);
    pub const CLUSTERED: Self = Self(1 << 6);
    pub const QUAD: Self = Self(1 << 7);
}
crate::bitflags_impl! {
    SubgroupFeatureFlags : u32, 0xff, BASIC, VOTE, ARITHMETIC, BALLOT, SHUFFLE,
    SHUFFLE_RELATIVE, CLUSTERED, QUAD
}
#[doc(alias = "VkExternalMemoryHandleTypeFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlags(pub u32);
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const D3D11_TEXTURE: Self = Self(1 << 3);
    pub const D3D11_TEXTURE_KMT: Self = Self(1 << 4);
    pub const D3D12_HEAP: Self = Self(1 << 5);
    pub const D3D12_RESOURCE: Self = Self(1 << 6);
}
crate::bitflags_impl! {
    ExternalMemoryHandleTypeFlags : u32, 0x7f, OPAQUE_FD, OPAQUE_WIN32, OPAQUE_WIN32_KMT,
    D3D11_TEXTURE, D3D11_TEXTURE_KMT, D3D12_HEAP, D3D12_RESOURCE
}
#[doc(alias = "VkExternalMemoryFeatureFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlags(pub u32);
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY: Self = Self(1 << 0);
    pub const EXPORTABLE: Self = Self(1 << 1);
    pub const IMPORTABLE: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    ExternalMemoryFeatureFlags : u32, 0x7, DEDICATED_ONLY, EXPORTABLE, IMPORTABLE
}
#[doc(alias = "VkExternalSemaphoreHandleTypeFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalSemaphoreHandleTypeFlags(pub u32);
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const D3D12_FENCE: Self = Self(1 << 3);
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    pub const SYNC_FD: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    ExternalSemaphoreHandleTypeFlags : u32, 0x1f, OPAQUE_FD, OPAQUE_WIN32,
    OPAQUE_WIN32_KMT, D3D12_FENCE, SYNC_FD
}
#[doc(alias = "VkExternalSemaphoreFeatureFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalSemaphoreFeatureFlags(pub u32);
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE: Self = Self(1 << 0);
    pub const IMPORTABLE: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    ExternalSemaphoreFeatureFlags : u32, 0x3, EXPORTABLE, IMPORTABLE
}
#[doc(alias = "VkSemaphoreImportFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreImportFlags(pub u32);
impl SemaphoreImportFlags {
    pub const TEMPORARY: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    SemaphoreImportFlags : u32, 0x1, TEMPORARY
}
#[doc(alias = "VkExternalFenceHandleTypeFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalFenceHandleTypeFlags(pub u32);
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const SYNC_FD: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    ExternalFenceHandleTypeFlags : u32, 0xf, OPAQUE_FD, OPAQUE_WIN32, OPAQUE_WIN32_KMT,
    SYNC_FD
}
#[doc(alias = "VkExternalFenceFeatureFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalFenceFeatureFlags(pub u32);
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE: Self = Self(1 << 0);
    pub const IMPORTABLE: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    ExternalFenceFeatureFlags : u32, 0x3, EXPORTABLE, IMPORTABLE
}
#[doc(alias = "VkFenceImportFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FenceImportFlags(pub u32);
impl FenceImportFlags {
    pub const TEMPORARY: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    FenceImportFlags : u32, 0x1, TEMPORARY
}
#[doc(alias = "VkPeerMemoryFeatureFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PeerMemoryFeatureFlags(pub u32);
impl PeerMemoryFeatureFlags {
    pub const COPY_SRC: Self = Self(1 << 0);
    pub const COPY_DST: Self = Self(1 << 1);
    pub const GENERIC_SRC: Self = Self(1 << 2);
    pub const GENERIC_DST: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    PeerMemoryFeatureFlags : u32, 0xf, COPY_SRC, COPY_DST, GENERIC_SRC, GENERIC_DST
}
#[doc(alias = "VkMemoryAllocateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryAllocateFlags(pub u32);
impl MemoryAllocateFlags {
    pub const DEVICE_MASK: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    MemoryAllocateFlags : u32, 0x1, DEVICE_MASK
}
#[doc(alias = "VkPointClippingBehavior")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehavior.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PointClippingBehavior(pub i32);
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
crate::enum_impl! {
    PointClippingBehavior : i32, ALL_CLIP_PLANES, USER_CLIP_PLANES_ONLY
}
#[doc(alias = "VkTessellationDomainOrigin")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOrigin.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TessellationDomainOrigin(pub i32);
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
crate::enum_impl! {
    TessellationDomainOrigin : i32, UPPER_LEFT, LOWER_LEFT
}
#[doc(alias = "VkSamplerYcbcrModelConversion")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversion.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrModelConversion(pub i32);
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
}
crate::enum_impl! {
    SamplerYcbcrModelConversion : i32, RGB_IDENTITY, YCBCR_IDENTITY, YCBCR_709,
    YCBCR_601, YCBCR_2020
}
#[doc(alias = "VkSamplerYcbcrRange")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRange.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrRange(pub i32);
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
}
crate::enum_impl! {
    SamplerYcbcrRange : i32, ITU_FULL, ITU_NARROW
}
#[doc(alias = "VkChromaLocation")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkChromaLocation.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ChromaLocation(pub i32);
impl ChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}
crate::enum_impl! {
    ChromaLocation : i32, COSITED_EVEN, MIDPOINT
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumerateInstanceVersion")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
pub unsafe fn enumerate_instance_version(
    p_api_version: *mut u32,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_ENTRY_TABLE
        .enumerate_instance_version
        .unwrap())(p_api_version)
}
#[cfg(feature = "wrappers")]
impl crate::EntryWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
    pub unsafe fn enumerate_instance_version(&self) -> crate::VulkanResult<u32> {
        let enumerate_instance_version = (*self.table)
            .enumerate_instance_version
            .unwrap();
        let mut api_version = Default::default();
        let result = enumerate_instance_version(&mut api_version);
        crate::new_result(api_version, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
pub unsafe fn get_physical_device_features_2(
    physical_device: crate::vk10::PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_features_2
        .unwrap())(physical_device, p_features)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
    pub unsafe fn get_physical_device_features_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
    ) -> PhysicalDeviceFeatures2 {
        let get_physical_device_features_2 = (*self.table)
            .get_physical_device_features_2
            .unwrap();
        let mut features = Default::default();
        get_physical_device_features_2(physical_device, &mut features);
        features
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
pub unsafe fn get_physical_device_properties_2(
    physical_device: crate::vk10::PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_properties_2
        .unwrap())(physical_device, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
    pub unsafe fn get_physical_device_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
    ) -> PhysicalDeviceProperties2 {
        let get_physical_device_properties_2 = (*self.table)
            .get_physical_device_properties_2
            .unwrap();
        let mut properties = Default::default();
        get_physical_device_properties_2(physical_device, &mut properties);
        properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
pub unsafe fn get_physical_device_format_properties_2(
    physical_device: crate::vk10::PhysicalDevice,
    format: crate::vk10::Format,
    p_format_properties: *mut FormatProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_format_properties_2
        .unwrap())(physical_device, format, p_format_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
    pub unsafe fn get_physical_device_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
    ) -> FormatProperties2 {
        let get_physical_device_format_properties_2 = (*self.table)
            .get_physical_device_format_properties_2
            .unwrap();
        let mut format_properties = Default::default();
        get_physical_device_format_properties_2(
            physical_device,
            format as _,
            &mut format_properties,
        );
        format_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
pub unsafe fn get_physical_device_image_format_properties_2(
    physical_device: crate::vk10::PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut ImageFormatProperties2,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_image_format_properties_2
        .unwrap())(physical_device, p_image_format_info, p_image_format_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
    pub unsafe fn get_physical_device_image_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2,
    ) -> crate::VulkanResult<ImageFormatProperties2> {
        let get_physical_device_image_format_properties_2 = (*self.table)
            .get_physical_device_image_format_properties_2
            .unwrap();
        let mut image_format_properties = Default::default();
        let result = get_physical_device_image_format_properties_2(
            physical_device,
            image_format_info as _,
            &mut image_format_properties,
        );
        crate::new_result(image_format_properties, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
pub unsafe fn get_physical_device_queue_family_properties_2(
    physical_device: crate::vk10::PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2,
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
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
    pub unsafe fn get_physical_device_queue_family_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_property_count: Option<u32>,
        mut queue_family_properties_callback: impl FnMut(
            &mut Vec<QueueFamilyProperties2>,
        ),
    ) -> Vec<QueueFamilyProperties2> {
        let get_physical_device_queue_family_properties_2 = (*self.table)
            .get_physical_device_queue_family_properties_2
            .unwrap();
        let mut queue_family_property_count = match queue_family_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_queue_family_properties_2(
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
        get_physical_device_queue_family_properties_2(
            physical_device,
            &mut queue_family_property_count,
            queue_family_properties.as_mut_ptr(),
        );
        queue_family_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
pub unsafe fn get_physical_device_memory_properties_2(
    physical_device: crate::vk10::PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_memory_properties_2
        .unwrap())(physical_device, p_memory_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
    pub unsafe fn get_physical_device_memory_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties2 {
        let get_physical_device_memory_properties_2 = (*self.table)
            .get_physical_device_memory_properties_2
            .unwrap();
        let mut memory_properties = Default::default();
        get_physical_device_memory_properties_2(physical_device, &mut memory_properties);
        memory_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
pub unsafe fn get_physical_device_sparse_image_format_properties_2(
    physical_device: crate::vk10::PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_sparse_image_format_properties_2
        .unwrap())(physical_device, p_format_info, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<SparseImageFormatProperties2>),
    ) -> Vec<SparseImageFormatProperties2> {
        let get_physical_device_sparse_image_format_properties_2 = (*self.table)
            .get_physical_device_sparse_image_format_properties_2
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_sparse_image_format_properties_2(
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
        get_physical_device_sparse_image_format_properties_2(
            physical_device,
            format_info as _,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkTrimCommandPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html)
pub unsafe fn trim_command_pool(
    device: crate::vk10::Device,
    command_pool: crate::vk10::CommandPool,
    flags: CommandPoolTrimFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .trim_command_pool
        .unwrap())(device, command_pool, flags)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkTrimCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html)
    pub unsafe fn trim_command_pool(
        &self,
        command_pool: crate::vk10::CommandPool,
        flags: Option<CommandPoolTrimFlags>,
    ) {
        let trim_command_pool = (*self.table).trim_command_pool.unwrap();
        trim_command_pool(
            self.handle,
            command_pool,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
pub unsafe fn get_physical_device_external_buffer_properties(
    physical_device: crate::vk10::PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut ExternalBufferProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_buffer_properties
        .unwrap())(physical_device, p_external_buffer_info, p_external_buffer_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo,
    ) -> ExternalBufferProperties {
        let get_physical_device_external_buffer_properties = (*self.table)
            .get_physical_device_external_buffer_properties
            .unwrap();
        let mut external_buffer_properties = Default::default();
        get_physical_device_external_buffer_properties(
            physical_device,
            external_buffer_info as _,
            &mut external_buffer_properties,
        );
        external_buffer_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
pub unsafe fn get_physical_device_external_semaphore_properties(
    physical_device: crate::vk10::PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_semaphore_properties
        .unwrap())(
        physical_device,
        p_external_semaphore_info,
        p_external_semaphore_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
    ) -> ExternalSemaphoreProperties {
        let get_physical_device_external_semaphore_properties = (*self.table)
            .get_physical_device_external_semaphore_properties
            .unwrap();
        let mut external_semaphore_properties = Default::default();
        get_physical_device_external_semaphore_properties(
            physical_device,
            external_semaphore_info as _,
            &mut external_semaphore_properties,
        );
        external_semaphore_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
pub unsafe fn get_physical_device_external_fence_properties(
    physical_device: crate::vk10::PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut ExternalFenceProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_fence_properties
        .unwrap())(physical_device, p_external_fence_info, p_external_fence_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo,
    ) -> ExternalFenceProperties {
        let get_physical_device_external_fence_properties = (*self.table)
            .get_physical_device_external_fence_properties
            .unwrap();
        let mut external_fence_properties = Default::default();
        get_physical_device_external_fence_properties(
            physical_device,
            external_fence_info as _,
            &mut external_fence_properties,
        );
        external_fence_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
pub unsafe fn enumerate_physical_device_groups(
    instance: crate::vk10::Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .enumerate_physical_device_groups
        .unwrap())(
        instance,
        p_physical_device_group_count,
        p_physical_device_group_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        physical_device_group_count: Option<u32>,
        mut physical_device_group_properties_callback: impl FnMut(
            &mut Vec<PhysicalDeviceGroupProperties>,
        ),
    ) -> crate::VulkanResult<(Vec<PhysicalDeviceGroupProperties>, crate::vk10::Result)> {
        let enumerate_physical_device_groups = (*self.table)
            .enumerate_physical_device_groups
            .unwrap();
        let mut physical_device_group_count = match physical_device_group_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_physical_device_groups(
                    self.handle,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut physical_device_group_properties = vec![
            Default::default(); physical_device_group_count as usize
        ];
        physical_device_group_properties_callback(&mut physical_device_group_properties);
        let result = enumerate_physical_device_groups(
            self.handle,
            &mut physical_device_group_count,
            physical_device_group_properties.as_mut_ptr(),
        );
        crate::new_result((physical_device_group_properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
pub unsafe fn get_device_group_peer_memory_features(
    device: crate::vk10::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_group_peer_memory_features
        .unwrap())(
        device,
        heap_index,
        local_device_index,
        remote_device_index,
        p_peer_memory_features,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        let get_device_group_peer_memory_features = (*self.table)
            .get_device_group_peer_memory_features
            .unwrap();
        let mut peer_memory_features = Default::default();
        get_device_group_peer_memory_features(
            self.handle,
            heap_index as _,
            local_device_index as _,
            remote_device_index as _,
            &mut peer_memory_features,
        );
        peer_memory_features
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindBufferMemory2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)
pub unsafe fn bind_buffer_memory_2(
    device: crate::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_buffer_memory_2
        .unwrap())(device, bind_info_count, p_bind_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindBufferMemory2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)
    pub unsafe fn bind_buffer_memory_2(
        &self,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> crate::VulkanResult<()> {
        let bind_buffer_memory_2 = (*self.table).bind_buffer_memory_2.unwrap();
        let bind_info_count = bind_infos.len();
        let result = bind_buffer_memory_2(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindImageMemory2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)
pub unsafe fn bind_image_memory_2(
    device: crate::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_image_memory_2
        .unwrap())(device, bind_info_count, p_bind_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindImageMemory2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)
    pub unsafe fn bind_image_memory_2(
        &self,
        bind_infos: &[BindImageMemoryInfo],
    ) -> crate::VulkanResult<()> {
        let bind_image_memory_2 = (*self.table).bind_image_memory_2.unwrap();
        let bind_info_count = bind_infos.len();
        let result = bind_image_memory_2(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDeviceMask")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html)
pub unsafe fn cmd_set_device_mask(
    command_buffer: crate::vk10::CommandBuffer,
    device_mask: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_device_mask
        .unwrap())(command_buffer, device_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDeviceMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html)
    pub unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        device_mask: u32,
    ) {
        let cmd_set_device_mask = (*self.table).cmd_set_device_mask.unwrap();
        cmd_set_device_mask(command_buffer, device_mask as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDispatchBase")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html)
pub unsafe fn cmd_dispatch_base(
    command_buffer: crate::vk10::CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_dispatch_base
        .unwrap())(
        command_buffer,
        base_group_x,
        base_group_y,
        base_group_z,
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDispatchBase")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html)
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let cmd_dispatch_base = (*self.table).cmd_dispatch_base.unwrap();
        cmd_dispatch_base(
            command_buffer,
            base_group_x as _,
            base_group_y as _,
            base_group_z as _,
            group_count_x as _,
            group_count_y as _,
            group_count_z as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
pub unsafe fn create_descriptor_update_template(
    device: crate::vk10::Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_descriptor_update_template
        .unwrap())(device, p_create_info, p_allocator, p_descriptor_update_template)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
    pub unsafe fn create_descriptor_update_template(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<DescriptorUpdateTemplate> {
        let create_descriptor_update_template = (*self.table)
            .create_descriptor_update_template
            .unwrap();
        let mut descriptor_update_template = Default::default();
        let result = create_descriptor_update_template(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut descriptor_update_template,
        );
        crate::new_result(descriptor_update_template, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
pub unsafe fn destroy_descriptor_update_template(
    device: crate::vk10::Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_descriptor_update_template
        .unwrap())(device, descriptor_update_template, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_descriptor_update_template = (*self.table)
            .destroy_descriptor_update_template
            .unwrap();
        destroy_descriptor_update_template(
            self.handle,
            descriptor_update_template,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
pub unsafe fn update_descriptor_set_with_template(
    device: crate::vk10::Device,
    descriptor_set: crate::vk10::DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .update_descriptor_set_with_template
        .unwrap())(device, descriptor_set, descriptor_update_template, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: crate::vk10::DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: *const std::os::raw::c_void,
    ) {
        let update_descriptor_set_with_template = (*self.table)
            .update_descriptor_set_with_template
            .unwrap();
        update_descriptor_set_with_template(
            self.handle,
            descriptor_set,
            descriptor_update_template,
            data,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferMemoryRequirements2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html)
pub unsafe fn get_buffer_memory_requirements_2(
    device: crate::vk10::Device,
    p_info: *const BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_memory_requirements_2
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html)
    pub unsafe fn get_buffer_memory_requirements_2(
        &self,
        info: &BufferMemoryRequirementsInfo2,
    ) -> MemoryRequirements2 {
        let get_buffer_memory_requirements_2 = (*self.table)
            .get_buffer_memory_requirements_2
            .unwrap();
        let mut memory_requirements = Default::default();
        get_buffer_memory_requirements_2(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageMemoryRequirements2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html)
pub unsafe fn get_image_memory_requirements_2(
    device: crate::vk10::Device,
    p_info: *const ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_memory_requirements_2
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html)
    pub unsafe fn get_image_memory_requirements_2(
        &self,
        info: &ImageMemoryRequirementsInfo2,
    ) -> MemoryRequirements2 {
        let get_image_memory_requirements_2 = (*self.table)
            .get_image_memory_requirements_2
            .unwrap();
        let mut memory_requirements = Default::default();
        get_image_memory_requirements_2(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
pub unsafe fn get_image_sparse_memory_requirements_2(
    device: crate::vk10::Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_sparse_memory_requirements_2
        .unwrap())(
        device,
        p_info,
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
    pub unsafe fn get_image_sparse_memory_requirements_2(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
        mut sparse_memory_requirements_callback: impl FnMut(
            &mut Vec<SparseImageMemoryRequirements2>,
        ),
    ) -> Vec<SparseImageMemoryRequirements2> {
        let get_image_sparse_memory_requirements_2 = (*self.table)
            .get_image_sparse_memory_requirements_2
            .unwrap();
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_image_sparse_memory_requirements_2(
                    self.handle,
                    info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut sparse_memory_requirements = vec![
            Default::default(); sparse_memory_requirement_count as usize
        ];
        sparse_memory_requirements_callback(&mut sparse_memory_requirements);
        get_image_sparse_memory_requirements_2(
            self.handle,
            info as _,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateSamplerYcbcrConversion")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
pub unsafe fn create_sampler_ycbcr_conversion(
    device: crate::vk10::Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_sampler_ycbcr_conversion
        .unwrap())(device, p_create_info, p_allocator, p_ycbcr_conversion)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<SamplerYcbcrConversion> {
        let create_sampler_ycbcr_conversion = (*self.table)
            .create_sampler_ycbcr_conversion
            .unwrap();
        let mut ycbcr_conversion = Default::default();
        let result = create_sampler_ycbcr_conversion(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut ycbcr_conversion,
        );
        crate::new_result(ycbcr_conversion, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
pub unsafe fn destroy_sampler_ycbcr_conversion(
    device: crate::vk10::Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_sampler_ycbcr_conversion
        .unwrap())(device, ycbcr_conversion, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_sampler_ycbcr_conversion = (*self.table)
            .destroy_sampler_ycbcr_conversion
            .unwrap();
        destroy_sampler_ycbcr_conversion(
            self.handle,
            ycbcr_conversion,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceQueue2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)
pub unsafe fn get_device_queue_2(
    device: crate::vk10::Device,
    p_queue_info: *const DeviceQueueInfo2,
    p_queue: *mut crate::vk10::Queue,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_queue_2
        .unwrap())(device, p_queue_info, p_queue)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceQueue2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)
    pub unsafe fn get_device_queue_2(
        &self,
        queue_info: &DeviceQueueInfo2,
    ) -> crate::vk10::Queue {
        let get_device_queue_2 = (*self.table).get_device_queue_2.unwrap();
        let mut queue = Default::default();
        get_device_queue_2(self.handle, queue_info as _, &mut queue);
        queue
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
pub unsafe fn get_descriptor_set_layout_support(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
    p_support: *mut DescriptorSetLayoutSupport,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_descriptor_set_layout_support
        .unwrap())(device, p_create_info, p_support)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &crate::vk10::DescriptorSetLayoutCreateInfo,
    ) -> DescriptorSetLayoutSupport {
        let get_descriptor_set_layout_support = (*self.table)
            .get_descriptor_set_layout_support
            .unwrap();
        let mut support = Default::default();
        get_descriptor_set_layout_support(self.handle, create_info as _, &mut support);
        support
    }
}
