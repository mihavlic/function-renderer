pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}
pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}
pub const fn api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7f
}
pub const fn api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}
pub const fn api_version_patch(version: u32) -> u32 {
    version & 0xfff
}
pub const API_VERSION_1_0: u32 = crate::vk10::make_api_version(0, 1, 0, 0);
pub const HEADER_VERSION: u32 = 230;
pub const HEADER_VERSION_COMPLETE: u32 = crate::vk10::make_api_version(
    0,
    1,
    3,
    HEADER_VERSION,
);
#[doc(alias = "VkSampleMask")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleMask.html)
pub type SampleMask = u32;
#[doc(alias = "VkBool32")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBool32.html)
pub type Bool32 = u32;
#[doc(alias = "VkFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFlags.html)
pub type Flags = u32;
#[doc(alias = "VkDeviceSize")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceSize.html)
pub type DeviceSize = u64;
#[doc(alias = "VkDeviceAddress")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddress.html)
pub type DeviceAddress = u64;
#[doc(alias = "VkQueryPoolCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryPoolCreateFlags(pub u32);
crate::bitflags_impl! {
    QueryPoolCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineDynamicStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineDynamicStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineDynamicStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineMultisampleStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineMultisampleStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineMultisampleStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineRasterizationStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRasterizationStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineRasterizationStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineViewportStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineViewportStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineViewportStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineTessellationStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineTessellationStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineTessellationStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineInputAssemblyStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineInputAssemblyStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineInputAssemblyStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineVertexInputStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineVertexInputStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineVertexInputStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkBufferViewCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BufferViewCreateFlags(pub u32);
crate::bitflags_impl! {
    BufferViewCreateFlags : u32, 0x0,
}
#[doc(alias = "VkDeviceCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceCreateFlags(pub u32);
crate::bitflags_impl! {
    DeviceCreateFlags : u32, 0x0,
}
#[doc(alias = "VkSemaphoreCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreCreateFlags(pub u32);
crate::bitflags_impl! {
    SemaphoreCreateFlags : u32, 0x0,
}
#[doc(alias = "VkShaderModuleCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderModuleCreateFlags(pub u32);
crate::bitflags_impl! {
    ShaderModuleCreateFlags : u32, 0x0,
}
#[doc(alias = "VkMemoryMapFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryMapFlags(pub u32);
crate::bitflags_impl! {
    MemoryMapFlags : u32, 0x0,
}
#[doc(alias = "VkDescriptorPoolResetFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorPoolResetFlags(pub u32);
crate::bitflags_impl! {
    DescriptorPoolResetFlags : u32, 0x0,
}
crate::non_dispatchable_handle!(
    Instance, INSTANCE, "VkInstance",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstance.html)"
);
crate::non_dispatchable_handle!(
    PhysicalDevice, PHYSICAL_DEVICE, "VkPhysicalDevice",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html)"
);
crate::non_dispatchable_handle!(
    Device, DEVICE, "VkDevice",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevice.html)"
);
crate::non_dispatchable_handle!(
    Queue, QUEUE, "VkQueue",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueue.html)"
);
crate::non_dispatchable_handle!(
    CommandBuffer, COMMAND_BUFFER, "VkCommandBuffer",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html)"
);
crate::dispatchable_handle!(
    DeviceMemory, DEVICE_MEMORY, "VkDeviceMemory",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html)"
);
crate::dispatchable_handle!(
    CommandPool, COMMAND_POOL, "VkCommandPool",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html)"
);
crate::dispatchable_handle!(
    Buffer, BUFFER, "VkBuffer",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuffer.html)"
);
crate::dispatchable_handle!(
    BufferView, BUFFER_VIEW, "VkBufferView",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferView.html)"
);
crate::dispatchable_handle!(
    Image, IMAGE, "VkImage",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImage.html)"
);
crate::dispatchable_handle!(
    ImageView, IMAGE_VIEW, "VkImageView",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageView.html)"
);
crate::dispatchable_handle!(
    ShaderModule, SHADER_MODULE, "VkShaderModule",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html)"
);
crate::dispatchable_handle!(
    Pipeline, PIPELINE, "VkPipeline",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipeline.html)"
);
crate::dispatchable_handle!(
    PipelineLayout, PIPELINE_LAYOUT, "VkPipelineLayout",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html)"
);
crate::dispatchable_handle!(
    Sampler, SAMPLER, "VkSampler",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampler.html)"
);
crate::dispatchable_handle!(
    DescriptorSet, DESCRIPTOR_SET, "VkDescriptorSet",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html)"
);
crate::dispatchable_handle!(
    DescriptorSetLayout, DESCRIPTOR_SET_LAYOUT, "VkDescriptorSetLayout",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html)"
);
crate::dispatchable_handle!(
    DescriptorPool, DESCRIPTOR_POOL, "VkDescriptorPool",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html)"
);
crate::dispatchable_handle!(
    Fence, FENCE, "VkFence",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFence.html)"
);
crate::dispatchable_handle!(
    Semaphore, SEMAPHORE, "VkSemaphore",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html)"
);
crate::dispatchable_handle!(
    Event, EVENT, "VkEvent",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEvent.html)"
);
crate::dispatchable_handle!(
    QueryPool, QUERY_POOL, "VkQueryPool",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html)"
);
crate::dispatchable_handle!(
    Framebuffer, FRAMEBUFFER, "VkFramebuffer",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html)"
);
crate::dispatchable_handle!(
    RenderPass, RENDER_PASS, "VkRenderPass",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html)"
);
crate::dispatchable_handle!(
    PipelineCache, PIPELINE_CACHE, "VkPipelineCache",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html)"
);
#[doc(alias = "PFN_vkInternalAllocationNotification")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html)
pub type PfnInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut std::os::raw::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
#[doc(alias = "PFN_vkInternalFreeNotification")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalFreeNotification.html)
pub type PfnInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut std::os::raw::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
#[doc(alias = "PFN_vkReallocationFunction")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html)
pub type PfnReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut std::os::raw::c_void,
    p_original: *mut std::os::raw::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut std::os::raw::c_void;
#[doc(alias = "PFN_vkAllocationFunction")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html)
pub type PfnAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut std::os::raw::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut std::os::raw::c_void;
#[doc(alias = "PFN_vkFreeFunction")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html)
pub type PfnFreeFunction = unsafe extern "system" fn(
    p_user_data: *mut std::os::raw::c_void,
    p_memory: *mut std::os::raw::c_void,
);
#[doc(alias = "PFN_vkVoidFunction")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkVoidFunction.html)
pub type PfnVoidFunction = unsafe extern "system" fn();
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBaseOutStructure")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html)
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut BaseOutStructure,
}
impl Default for BaseOutStructure {
    fn default() -> Self {
        Self {
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBaseInStructure")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html)
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}
impl Default for BaseInStructure {
    fn default() -> Self {
        Self {
            s_type: Default::default(),
            p_next: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkOffset2D")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html)
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}
impl Default for Offset2D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkOffset3D")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html)
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Default for Offset3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkExtent2D")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html)
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}
impl Default for Extent2D {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkExtent3D")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html)
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl Default for Extent3D {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            depth: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkViewport")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewport.html)
pub struct Viewport {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub width: std::os::raw::c_float,
    pub height: std::os::raw::c_float,
    pub min_depth: std::os::raw::c_float,
    pub max_depth: std::os::raw::c_float,
}
impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            min_depth: Default::default(),
            max_depth: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkRect2D")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRect2D.html)
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}
impl Default for Rect2D {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            extent: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkClearRect")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearRect.html)
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
impl Default for ClearRect {
    fn default() -> Self {
        Self {
            rect: Default::default(),
            base_array_layer: Default::default(),
            layer_count: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkComponentMapping")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html)
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}
impl Default for ComponentMapping {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            a: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html)
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [std::os::raw::c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}
impl Default for PhysicalDeviceProperties {
    fn default() -> Self {
        Self {
            api_version: Default::default(),
            driver_version: Default::default(),
            vendor_id: Default::default(),
            device_id: Default::default(),
            device_type: Default::default(),
            device_name: unsafe { std::mem::zeroed() },
            pipeline_cache_uuid: unsafe { std::mem::zeroed() },
            limits: Default::default(),
            sparse_properties: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkExtensionProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html)
pub struct ExtensionProperties {
    pub extension_name: [std::os::raw::c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
}
impl Default for ExtensionProperties {
    fn default() -> Self {
        Self {
            extension_name: unsafe { std::mem::zeroed() },
            spec_version: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkLayerProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html)
pub struct LayerProperties {
    pub layer_name: [std::os::raw::c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [std::os::raw::c_char; MAX_DESCRIPTION_SIZE as usize],
}
impl Default for LayerProperties {
    fn default() -> Self {
        Self {
            layer_name: unsafe { std::mem::zeroed() },
            spec_version: Default::default(),
            implementation_version: Default::default(),
            description: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkApplicationInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html)
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_application_name: *const std::os::raw::c_char,
    pub application_version: u32,
    pub p_engine_name: *const std::os::raw::c_char,
    pub engine_version: u32,
    pub api_version: u32,
}
impl Default for ApplicationInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: std::ptr::null(),
            application_version: Default::default(),
            p_engine_name: std::ptr::null(),
            engine_version: Default::default(),
            api_version: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAllocationCallbacks")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html)
pub struct AllocationCallbacks {
    pub p_user_data: *mut std::os::raw::c_void,
    pub pfn_allocation: Option<PfnAllocationFunction>,
    pub pfn_reallocation: Option<PfnReallocationFunction>,
    pub pfn_free: Option<PfnFreeFunction>,
    pub pfn_internal_allocation: Option<PfnInternalAllocationNotification>,
    pub pfn_internal_free: Option<PfnInternalFreeNotification>,
}
impl Default for AllocationCallbacks {
    fn default() -> Self {
        Self {
            p_user_data: std::ptr::null_mut(),
            pfn_allocation: None,
            pfn_reallocation: None,
            pfn_free: None,
            pfn_internal_allocation: None,
            pfn_internal_free: None,
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceQueueCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html)
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const std::os::raw::c_float,
}
impl Default for DeviceQueueCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: Default::default(),
            queue_count: Default::default(),
            p_queue_priorities: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateInfo.html)
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const std::os::raw::c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const std::os::raw::c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}
impl Default for DeviceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_create_info_count: Default::default(),
            p_queue_create_infos: std::ptr::null(),
            enabled_layer_count: Default::default(),
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: Default::default(),
            pp_enabled_extension_names: std::ptr::null(),
            p_enabled_features: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkInstanceCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html)
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const std::os::raw::c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const std::os::raw::c_char,
}
impl Default for InstanceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_application_info: std::ptr::null(),
            enabled_layer_count: Default::default(),
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: Default::default(),
            pp_enabled_extension_names: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html)
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}
impl Default for QueueFamilyProperties {
    fn default() -> Self {
        Self {
            queue_flags: Default::default(),
            queue_count: Default::default(),
            timestamp_valid_bits: Default::default(),
            min_image_transfer_granularity: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMemoryProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html)
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES as usize],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS as usize],
}
impl Default for PhysicalDeviceMemoryProperties {
    fn default() -> Self {
        Self {
            memory_type_count: Default::default(),
            memory_types: unsafe { std::mem::zeroed() },
            memory_heap_count: Default::default(),
            memory_heaps: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html)
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}
impl Default for MemoryAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            allocation_size: Default::default(),
            memory_type_index: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html)
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}
impl Default for MemoryRequirements {
    fn default() -> Self {
        Self {
            size: Default::default(),
            alignment: Default::default(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSparseImageFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties.html)
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}
impl Default for SparseImageFormatProperties {
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            image_granularity: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSparseImageMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements.html)
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}
impl Default for SparseImageMemoryRequirements {
    fn default() -> Self {
        Self {
            format_properties: Default::default(),
            image_mip_tail_first_lod: Default::default(),
            image_mip_tail_size: Default::default(),
            image_mip_tail_offset: Default::default(),
            image_mip_tail_stride: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMemoryType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html)
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}
impl Default for MemoryType {
    fn default() -> Self {
        Self {
            property_flags: Default::default(),
            heap_index: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMemoryHeap")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html)
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}
impl Default for MemoryHeap {
    fn default() -> Self {
        Self {
            size: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMappedMemoryRange")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMappedMemoryRange.html)
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for MappedMemoryRange {
    fn default() -> Self {
        Self {
            s_type: StructureType::MAPPED_MEMORY_RANGE,
            p_next: std::ptr::null(),
            memory: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties.html)
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}
impl Default for FormatProperties {
    fn default() -> Self {
        Self {
            linear_tiling_features: Default::default(),
            optimal_tiling_features: Default::default(),
            buffer_features: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties.html)
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}
impl Default for ImageFormatProperties {
    fn default() -> Self {
        Self {
            max_extent: Default::default(),
            max_mip_levels: Default::default(),
            max_array_layers: Default::default(),
            sample_counts: Default::default(),
            max_resource_size: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDescriptorBufferInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferInfo.html)
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
impl Default for DescriptorBufferInfo {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: Default::default(),
            range: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDescriptorImageInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorImageInfo.html)
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}
impl Default for DescriptorImageInfo {
    fn default() -> Self {
        Self {
            sampler: Default::default(),
            image_view: Default::default(),
            image_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWriteDescriptorSet")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSet.html)
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}
impl Default for WriteDescriptorSet {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_DESCRIPTOR_SET,
            p_next: std::ptr::null(),
            dst_set: Default::default(),
            dst_binding: Default::default(),
            dst_array_element: Default::default(),
            descriptor_count: Default::default(),
            descriptor_type: Default::default(),
            p_image_info: std::ptr::null(),
            p_buffer_info: std::ptr::null(),
            p_texel_buffer_view: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyDescriptorSet")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyDescriptorSet.html)
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}
impl Default for CopyDescriptorSet {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_DESCRIPTOR_SET,
            p_next: std::ptr::null(),
            src_set: Default::default(),
            src_binding: Default::default(),
            src_array_element: Default::default(),
            dst_set: Default::default(),
            dst_binding: Default::default(),
            dst_array_element: Default::default(),
            descriptor_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html)
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
impl Default for BufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            size: Default::default(),
            usage: Default::default(),
            sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferViewCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateInfo.html)
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
impl Default for BufferViewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_VIEW_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            buffer: Default::default(),
            format: Default::default(),
            offset: Default::default(),
            range: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageSubresource")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html)
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}
impl Default for ImageSubresource {
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            mip_level: Default::default(),
            array_layer: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageSubresourceLayers")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html)
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
impl Default for ImageSubresourceLayers {
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            mip_level: Default::default(),
            base_array_layer: Default::default(),
            layer_count: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageSubresourceRange")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html)
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
impl Default for ImageSubresourceRange {
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            base_mip_level: Default::default(),
            level_count: Default::default(),
            base_array_layer: Default::default(),
            layer_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryBarrier")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html)
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}
impl Default for MemoryBarrier {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_BARRIER,
            p_next: std::ptr::null(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferMemoryBarrier")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html)
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for BufferMemoryBarrier {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_MEMORY_BARRIER,
            p_next: std::ptr::null(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            src_queue_family_index: Default::default(),
            dst_queue_family_index: Default::default(),
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageMemoryBarrier")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html)
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_MEMORY_BARRIER,
            p_next: std::ptr::null(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            old_layout: Default::default(),
            new_layout: Default::default(),
            src_queue_family_index: Default::default(),
            dst_queue_family_index: Default::default(),
            image: Default::default(),
            subresource_range: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateInfo.html)
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}
impl Default for ImageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            image_type: Default::default(),
            format: Default::default(),
            extent: Default::default(),
            mip_levels: Default::default(),
            array_layers: Default::default(),
            samples: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
            initial_layout: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSubresourceLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout.html)
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}
impl Default for SubresourceLayout {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            size: Default::default(),
            row_pitch: Default::default(),
            array_pitch: Default::default(),
            depth_pitch: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html)
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}
impl Default for ImageViewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            image: Default::default(),
            view_type: Default::default(),
            format: Default::default(),
            components: Default::default(),
            subresource_range: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkBufferCopy")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy.html)
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for BufferCopy {
    fn default() -> Self {
        Self {
            src_offset: Default::default(),
            dst_offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSparseMemoryBind")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBind.html)
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
impl Default for SparseMemoryBind {
    fn default() -> Self {
        Self {
            resource_offset: Default::default(),
            size: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSparseImageMemoryBind")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBind.html)
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
impl Default for SparseImageMemoryBind {
    fn default() -> Self {
        Self {
            subresource: Default::default(),
            offset: Default::default(),
            extent: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSparseBufferMemoryBindInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}
impl Default for SparseBufferMemoryBindInfo {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            bind_count: Default::default(),
            p_binds: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSparseImageOpaqueMemoryBindInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}
impl Default for SparseImageOpaqueMemoryBindInfo {
    fn default() -> Self {
        Self {
            image: Default::default(),
            bind_count: Default::default(),
            p_binds: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSparseImageMemoryBindInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html)
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
}
impl Default for SparseImageMemoryBindInfo {
    fn default() -> Self {
        Self {
            image: Default::default(),
            bind_count: Default::default(),
            p_binds: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindSparseInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindSparseInfo.html)
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}
impl Default for BindSparseInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_SPARSE_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphores: std::ptr::null(),
            buffer_bind_count: Default::default(),
            p_buffer_binds: std::ptr::null(),
            image_opaque_bind_count: Default::default(),
            p_image_opaque_binds: std::ptr::null(),
            image_bind_count: Default::default(),
            p_image_binds: std::ptr::null(),
            signal_semaphore_count: Default::default(),
            p_signal_semaphores: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageCopy")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy.html)
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
impl Default for ImageCopy {
    fn default() -> Self {
        Self {
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageBlit")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageBlit.html)
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}
impl Default for ImageBlit {
    fn default() -> Self {
        Self {
            src_subresource: Default::default(),
            src_offsets: unsafe { std::mem::zeroed() },
            dst_subresource: Default::default(),
            dst_offsets: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkBufferImageCopy")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy.html)
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
impl Default for BufferImageCopy {
    fn default() -> Self {
        Self {
            buffer_offset: Default::default(),
            buffer_row_length: Default::default(),
            buffer_image_height: Default::default(),
            image_subresource: Default::default(),
            image_offset: Default::default(),
            image_extent: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkImageResolve")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageResolve.html)
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
impl Default for ImageResolve {
    fn default() -> Self {
        Self {
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkShaderModuleCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateInfo.html)
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}
impl Default for ShaderModuleCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            code_size: Default::default(),
            p_code: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetLayoutBinding")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html)
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}
impl Default for DescriptorSetLayoutBinding {
    fn default() -> Self {
        Self {
            binding: Default::default(),
            descriptor_type: Default::default(),
            descriptor_count: Default::default(),
            stage_flags: Default::default(),
            p_immutable_samplers: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetLayoutCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html)
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}
impl Default for DescriptorSetLayoutCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            binding_count: Default::default(),
            p_bindings: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDescriptorPoolSize")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolSize.html)
pub struct DescriptorPoolSize {
    pub kind: DescriptorType,
    pub descriptor_count: u32,
}
impl Default for DescriptorPoolSize {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            descriptor_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorPoolCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateInfo.html)
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
}
impl Default for DescriptorPoolCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            max_sets: Default::default(),
            pool_size_count: Default::default(),
            p_pool_sizes: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetAllocateInfo.html)
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}
impl Default for DescriptorSetAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            descriptor_pool: Default::default(),
            descriptor_set_count: Default::default(),
            p_set_layouts: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSpecializationMapEntry")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html)
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}
impl Default for SpecializationMapEntry {
    fn default() -> Self {
        Self {
            constant_id: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSpecializationInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html)
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const std::os::raw::c_void,
}
impl Default for SpecializationInfo {
    fn default() -> Self {
        Self {
            map_entry_count: Default::default(),
            p_map_entries: std::ptr::null(),
            data_size: Default::default(),
            p_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineShaderStageCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateInfo.html)
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub p_name: *const std::os::raw::c_char,
    pub p_specialization_info: *const SpecializationInfo,
}
impl Default for PipelineShaderStageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            module: Default::default(),
            p_name: std::ptr::null(),
            p_specialization_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkComputePipelineCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComputePipelineCreateInfo.html)
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for ComputePipelineCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMPUTE_PIPELINE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkVertexInputBindingDescription")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription.html)
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}
impl Default for VertexInputBindingDescription {
    fn default() -> Self {
        Self {
            binding: Default::default(),
            stride: Default::default(),
            input_rate: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkVertexInputAttributeDescription")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription.html)
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
impl Default for VertexInputAttributeDescription {
    fn default() -> Self {
        Self {
            location: Default::default(),
            binding: Default::default(),
            format: Default::default(),
            offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineVertexInputStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}
impl Default for PipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            vertex_binding_description_count: Default::default(),
            p_vertex_binding_descriptions: std::ptr::null(),
            vertex_attribute_description_count: Default::default(),
            p_vertex_attribute_descriptions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineInputAssemblyStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html)
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}
impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            topology: Default::default(),
            primitive_restart_enable: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineTessellationStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateInfo.html)
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}
impl Default for PipelineTessellationStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            patch_control_points: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html)
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
}
impl Default for PipelineViewportStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            viewport_count: Default::default(),
            p_viewports: std::ptr::null(),
            scissor_count: Default::default(),
            p_scissors: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html)
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: std::os::raw::c_float,
    pub depth_bias_clamp: std::os::raw::c_float,
    pub depth_bias_slope_factor: std::os::raw::c_float,
    pub line_width: std::os::raw::c_float,
}
impl Default for PipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            depth_clamp_enable: Default::default(),
            rasterizer_discard_enable: Default::default(),
            polygon_mode: Default::default(),
            cull_mode: Default::default(),
            front_face: Default::default(),
            depth_bias_enable: Default::default(),
            depth_bias_constant_factor: Default::default(),
            depth_bias_clamp: Default::default(),
            depth_bias_slope_factor: Default::default(),
            line_width: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineMultisampleStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html)
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlags,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: std::os::raw::c_float,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}
impl Default for PipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rasterization_samples: Default::default(),
            sample_shading_enable: Default::default(),
            min_sample_shading: Default::default(),
            p_sample_mask: std::ptr::null(),
            alpha_to_coverage_enable: Default::default(),
            alpha_to_one_enable: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPipelineColorBlendAttachmentState")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAttachmentState.html)
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}
impl Default for PipelineColorBlendAttachmentState {
    fn default() -> Self {
        Self {
            blend_enable: Default::default(),
            src_color_blend_factor: Default::default(),
            dst_color_blend_factor: Default::default(),
            color_blend_op: Default::default(),
            src_alpha_blend_factor: Default::default(),
            dst_alpha_blend_factor: Default::default(),
            alpha_blend_op: Default::default(),
            color_write_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineColorBlendStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html)
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [std::os::raw::c_float; 4],
}
impl Default for PipelineColorBlendStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            logic_op_enable: Default::default(),
            logic_op: Default::default(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
            blend_constants: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineDynamicStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
}
impl Default for PipelineDynamicStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dynamic_state_count: Default::default(),
            p_dynamic_states: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkStencilOpState")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilOpState.html)
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}
impl Default for StencilOpState {
    fn default() -> Self {
        Self {
            fail_op: Default::default(),
            pass_op: Default::default(),
            depth_fail_op: Default::default(),
            compare_op: Default::default(),
            compare_mask: Default::default(),
            write_mask: Default::default(),
            reference: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineDepthStencilStateCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: std::os::raw::c_float,
    pub max_depth_bounds: std::os::raw::c_float,
}
impl Default for PipelineDepthStencilStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            depth_test_enable: Default::default(),
            depth_write_enable: Default::default(),
            depth_compare_op: Default::default(),
            depth_bounds_test_enable: Default::default(),
            stencil_test_enable: Default::default(),
            front: Default::default(),
            back: Default::default(),
            min_depth_bounds: Default::default(),
            max_depth_bounds: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGraphicsPipelineCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for GraphicsPipelineCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            p_vertex_input_state: std::ptr::null(),
            p_input_assembly_state: std::ptr::null(),
            p_tessellation_state: std::ptr::null(),
            p_viewport_state: std::ptr::null(),
            p_rasterization_state: std::ptr::null(),
            p_multisample_state: std::ptr::null(),
            p_depth_stencil_state: std::ptr::null(),
            p_color_blend_state: std::ptr::null(),
            p_dynamic_state: std::ptr::null(),
            layout: Default::default(),
            render_pass: Default::default(),
            subpass: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineCacheCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateInfo.html)
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const std::os::raw::c_void,
}
impl Default for PipelineCacheCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_CACHE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            initial_data_size: Default::default(),
            p_initial_data: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPipelineCacheHeaderVersionOne")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionOne.html)
pub struct PipelineCacheHeaderVersionOne {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub vendor_id: u32,
    pub device_id: u32,
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
}
impl Default for PipelineCacheHeaderVersionOne {
    fn default() -> Self {
        Self {
            header_size: Default::default(),
            header_version: Default::default(),
            vendor_id: Default::default(),
            device_id: Default::default(),
            pipeline_cache_uuid: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPushConstantRange")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html)
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
impl Default for PushConstantRange {
    fn default() -> Self {
        Self {
            stage_flags: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineLayoutCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html)
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}
impl Default for PipelineLayoutCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            set_layout_count: Default::default(),
            p_set_layouts: std::ptr::null(),
            push_constant_range_count: Default::default(),
            p_push_constant_ranges: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html)
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: std::os::raw::c_float,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: std::os::raw::c_float,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: std::os::raw::c_float,
    pub max_lod: std::os::raw::c_float,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}
impl Default for SamplerCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_mode: Default::default(),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mip_lod_bias: Default::default(),
            anisotropy_enable: Default::default(),
            max_anisotropy: Default::default(),
            compare_enable: Default::default(),
            compare_op: Default::default(),
            min_lod: Default::default(),
            max_lod: Default::default(),
            border_color: Default::default(),
            unnormalized_coordinates: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandPoolCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateInfo.html)
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}
impl Default for CommandPoolCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferAllocateInfo.html)
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}
impl Default for CommandBufferAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            command_pool: Default::default(),
            level: Default::default(),
            command_buffer_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferInheritanceInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html)
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl Default for CommandBufferInheritanceInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_INFO,
            p_next: std::ptr::null(),
            render_pass: Default::default(),
            subpass: Default::default(),
            framebuffer: Default::default(),
            occlusion_query_enable: Default::default(),
            query_flags: Default::default(),
            pipeline_statistics: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferBeginInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html)
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}
impl Default for CommandBufferBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_inheritance_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassBeginInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassBeginInfo.html)
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
}
impl Default for RenderPassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            render_pass: Default::default(),
            framebuffer: Default::default(),
            render_area: Default::default(),
            clear_value_count: Default::default(),
            p_clear_values: std::ptr::null(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkClearColorValue")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html)
pub union ClearColorValue {
    pub float_32: [std::os::raw::c_float; 4],
    pub int_32: [i32; 4],
    pub uint_32: [u32; 4],
}
impl Default for ClearColorValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkClearDepthStencilValue")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html)
pub struct ClearDepthStencilValue {
    pub depth: std::os::raw::c_float,
    pub stencil: u32,
}
impl Default for ClearDepthStencilValue {
    fn default() -> Self {
        Self {
            depth: Default::default(),
            stencil: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkClearValue")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearValue.html)
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
impl Default for ClearValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkClearAttachment")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html)
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}
impl Default for ClearAttachment {
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            color_attachment: Default::default(),
            clear_value: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkAttachmentDescription")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription.html)
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}
impl Default for AttachmentDescription {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            format: Default::default(),
            samples: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            stencil_load_op: Default::default(),
            stencil_store_op: Default::default(),
            initial_layout: Default::default(),
            final_layout: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkAttachmentReference")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference.html)
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}
impl Default for AttachmentReference {
    fn default() -> Self {
        Self {
            attachment: Default::default(),
            layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDescription")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription.html)
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
impl Default for SubpassDescription {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            input_attachment_count: Default::default(),
            p_input_attachments: std::ptr::null(),
            color_attachment_count: Default::default(),
            p_color_attachments: std::ptr::null(),
            p_resolve_attachments: std::ptr::null(),
            p_depth_stencil_attachment: std::ptr::null(),
            preserve_attachment_count: Default::default(),
            p_preserve_attachments: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSubpassDependency")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency.html)
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}
impl Default for SubpassDependency {
    fn default() -> Self {
        Self {
            src_subpass: Default::default(),
            dst_subpass: Default::default(),
            src_stage_mask: Default::default(),
            dst_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            dependency_flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo.html)
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
}
impl Default for RenderPassCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
            subpass_count: Default::default(),
            p_subpasses: std::ptr::null(),
            dependency_count: Default::default(),
            p_dependencies: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkEventCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEventCreateInfo.html)
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: EventCreateFlags,
}
impl Default for EventCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::EVENT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFenceCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceCreateInfo.html)
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: FenceCreateFlags,
}
impl Default for FenceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint_32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float_64: Bool32,
    pub shader_int_64: Bool32,
    pub shader_int_16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image_2_d: Bool32,
    pub sparse_residency_image_3_d: Bool32,
    pub sparse_residency_2_samples: Bool32,
    pub sparse_residency_4_samples: Bool32,
    pub sparse_residency_8_samples: Bool32,
    pub sparse_residency_16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}
impl Default for PhysicalDeviceFeatures {
    fn default() -> Self {
        Self {
            robust_buffer_access: Default::default(),
            full_draw_index_uint_32: Default::default(),
            image_cube_array: Default::default(),
            independent_blend: Default::default(),
            geometry_shader: Default::default(),
            tessellation_shader: Default::default(),
            sample_rate_shading: Default::default(),
            dual_src_blend: Default::default(),
            logic_op: Default::default(),
            multi_draw_indirect: Default::default(),
            draw_indirect_first_instance: Default::default(),
            depth_clamp: Default::default(),
            depth_bias_clamp: Default::default(),
            fill_mode_non_solid: Default::default(),
            depth_bounds: Default::default(),
            wide_lines: Default::default(),
            large_points: Default::default(),
            alpha_to_one: Default::default(),
            multi_viewport: Default::default(),
            sampler_anisotropy: Default::default(),
            texture_compression_etc2: Default::default(),
            texture_compression_astc_ldr: Default::default(),
            texture_compression_bc: Default::default(),
            occlusion_query_precise: Default::default(),
            pipeline_statistics_query: Default::default(),
            vertex_pipeline_stores_and_atomics: Default::default(),
            fragment_stores_and_atomics: Default::default(),
            shader_tessellation_and_geometry_point_size: Default::default(),
            shader_image_gather_extended: Default::default(),
            shader_storage_image_extended_formats: Default::default(),
            shader_storage_image_multisample: Default::default(),
            shader_storage_image_read_without_format: Default::default(),
            shader_storage_image_write_without_format: Default::default(),
            shader_uniform_buffer_array_dynamic_indexing: Default::default(),
            shader_sampled_image_array_dynamic_indexing: Default::default(),
            shader_storage_buffer_array_dynamic_indexing: Default::default(),
            shader_storage_image_array_dynamic_indexing: Default::default(),
            shader_clip_distance: Default::default(),
            shader_cull_distance: Default::default(),
            shader_float_64: Default::default(),
            shader_int_64: Default::default(),
            shader_int_16: Default::default(),
            shader_resource_residency: Default::default(),
            shader_resource_min_lod: Default::default(),
            sparse_binding: Default::default(),
            sparse_residency_buffer: Default::default(),
            sparse_residency_image_2_d: Default::default(),
            sparse_residency_image_3_d: Default::default(),
            sparse_residency_2_samples: Default::default(),
            sparse_residency_4_samples: Default::default(),
            sparse_residency_8_samples: Default::default(),
            sparse_residency_16_samples: Default::default(),
            sparse_residency_aliased: Default::default(),
            variable_multisample_rate: Default::default(),
            inherited_queries: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSparseProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html)
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2_dblock_shape: Bool32,
    pub residency_standard_2_dmultisample_block_shape: Bool32,
    pub residency_standard_3_dblock_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}
impl Default for PhysicalDeviceSparseProperties {
    fn default() -> Self {
        Self {
            residency_standard_2_dblock_shape: Default::default(),
            residency_standard_2_dmultisample_block_shape: Default::default(),
            residency_standard_3_dblock_shape: Default::default(),
            residency_aligned_mip_size: Default::default(),
            residency_non_resident_strict: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceLimits")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLimits.html)
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1_d: u32,
    pub max_image_dimension_2_d: u32,
    pub max_image_dimension_3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: std::os::raw::c_float,
    pub max_sampler_anisotropy: std::os::raw::c_float,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [std::os::raw::c_float; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: std::os::raw::c_float,
    pub max_interpolation_offset: std::os::raw::c_float,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: std::os::raw::c_float,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [std::os::raw::c_float; 2],
    pub line_width_range: [std::os::raw::c_float; 2],
    pub point_size_granularity: std::os::raw::c_float,
    pub line_width_granularity: std::os::raw::c_float,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}
impl Default for PhysicalDeviceLimits {
    fn default() -> Self {
        Self {
            max_image_dimension_1_d: Default::default(),
            max_image_dimension_2_d: Default::default(),
            max_image_dimension_3_d: Default::default(),
            max_image_dimension_cube: Default::default(),
            max_image_array_layers: Default::default(),
            max_texel_buffer_elements: Default::default(),
            max_uniform_buffer_range: Default::default(),
            max_storage_buffer_range: Default::default(),
            max_push_constants_size: Default::default(),
            max_memory_allocation_count: Default::default(),
            max_sampler_allocation_count: Default::default(),
            buffer_image_granularity: Default::default(),
            sparse_address_space_size: Default::default(),
            max_bound_descriptor_sets: Default::default(),
            max_per_stage_descriptor_samplers: Default::default(),
            max_per_stage_descriptor_uniform_buffers: Default::default(),
            max_per_stage_descriptor_storage_buffers: Default::default(),
            max_per_stage_descriptor_sampled_images: Default::default(),
            max_per_stage_descriptor_storage_images: Default::default(),
            max_per_stage_descriptor_input_attachments: Default::default(),
            max_per_stage_resources: Default::default(),
            max_descriptor_set_samplers: Default::default(),
            max_descriptor_set_uniform_buffers: Default::default(),
            max_descriptor_set_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_storage_buffers: Default::default(),
            max_descriptor_set_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_sampled_images: Default::default(),
            max_descriptor_set_storage_images: Default::default(),
            max_descriptor_set_input_attachments: Default::default(),
            max_vertex_input_attributes: Default::default(),
            max_vertex_input_bindings: Default::default(),
            max_vertex_input_attribute_offset: Default::default(),
            max_vertex_input_binding_stride: Default::default(),
            max_vertex_output_components: Default::default(),
            max_tessellation_generation_level: Default::default(),
            max_tessellation_patch_size: Default::default(),
            max_tessellation_control_per_vertex_input_components: Default::default(),
            max_tessellation_control_per_vertex_output_components: Default::default(),
            max_tessellation_control_per_patch_output_components: Default::default(),
            max_tessellation_control_total_output_components: Default::default(),
            max_tessellation_evaluation_input_components: Default::default(),
            max_tessellation_evaluation_output_components: Default::default(),
            max_geometry_shader_invocations: Default::default(),
            max_geometry_input_components: Default::default(),
            max_geometry_output_components: Default::default(),
            max_geometry_output_vertices: Default::default(),
            max_geometry_total_output_components: Default::default(),
            max_fragment_input_components: Default::default(),
            max_fragment_output_attachments: Default::default(),
            max_fragment_dual_src_attachments: Default::default(),
            max_fragment_combined_output_resources: Default::default(),
            max_compute_shared_memory_size: Default::default(),
            max_compute_work_group_count: unsafe { std::mem::zeroed() },
            max_compute_work_group_invocations: Default::default(),
            max_compute_work_group_size: unsafe { std::mem::zeroed() },
            sub_pixel_precision_bits: Default::default(),
            sub_texel_precision_bits: Default::default(),
            mipmap_precision_bits: Default::default(),
            max_draw_indexed_index_value: Default::default(),
            max_draw_indirect_count: Default::default(),
            max_sampler_lod_bias: Default::default(),
            max_sampler_anisotropy: Default::default(),
            max_viewports: Default::default(),
            max_viewport_dimensions: unsafe { std::mem::zeroed() },
            viewport_bounds_range: unsafe { std::mem::zeroed() },
            viewport_sub_pixel_bits: Default::default(),
            min_memory_map_alignment: Default::default(),
            min_texel_buffer_offset_alignment: Default::default(),
            min_uniform_buffer_offset_alignment: Default::default(),
            min_storage_buffer_offset_alignment: Default::default(),
            min_texel_offset: Default::default(),
            max_texel_offset: Default::default(),
            min_texel_gather_offset: Default::default(),
            max_texel_gather_offset: Default::default(),
            min_interpolation_offset: Default::default(),
            max_interpolation_offset: Default::default(),
            sub_pixel_interpolation_offset_bits: Default::default(),
            max_framebuffer_width: Default::default(),
            max_framebuffer_height: Default::default(),
            max_framebuffer_layers: Default::default(),
            framebuffer_color_sample_counts: Default::default(),
            framebuffer_depth_sample_counts: Default::default(),
            framebuffer_stencil_sample_counts: Default::default(),
            framebuffer_no_attachments_sample_counts: Default::default(),
            max_color_attachments: Default::default(),
            sampled_image_color_sample_counts: Default::default(),
            sampled_image_integer_sample_counts: Default::default(),
            sampled_image_depth_sample_counts: Default::default(),
            sampled_image_stencil_sample_counts: Default::default(),
            storage_image_sample_counts: Default::default(),
            max_sample_mask_words: Default::default(),
            timestamp_compute_and_graphics: Default::default(),
            timestamp_period: Default::default(),
            max_clip_distances: Default::default(),
            max_cull_distances: Default::default(),
            max_combined_clip_and_cull_distances: Default::default(),
            discrete_queue_priorities: Default::default(),
            point_size_range: unsafe { std::mem::zeroed() },
            line_width_range: unsafe { std::mem::zeroed() },
            point_size_granularity: Default::default(),
            line_width_granularity: Default::default(),
            strict_lines: Default::default(),
            standard_sample_locations: Default::default(),
            optimal_buffer_copy_offset_alignment: Default::default(),
            optimal_buffer_copy_row_pitch_alignment: Default::default(),
            non_coherent_atom_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateInfo.html)
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SemaphoreCreateFlags,
}
impl Default for SemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SEMAPHORE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueryPoolCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html)
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl Default for QueryPoolCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUERY_POOL_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            query_type: Default::default(),
            query_count: Default::default(),
            pipeline_statistics: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFramebufferCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateInfo.html)
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
impl Default for FramebufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            render_pass: Default::default(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
            width: Default::default(),
            height: Default::default(),
            layers: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDrawIndirectCommand")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawIndirectCommand.html)
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
impl Default for DrawIndirectCommand {
    fn default() -> Self {
        Self {
            vertex_count: Default::default(),
            instance_count: Default::default(),
            first_vertex: Default::default(),
            first_instance: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDrawIndexedIndirectCommand")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawIndexedIndirectCommand.html)
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}
impl Default for DrawIndexedIndirectCommand {
    fn default() -> Self {
        Self {
            index_count: Default::default(),
            instance_count: Default::default(),
            first_index: Default::default(),
            vertex_offset: Default::default(),
            first_instance: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDispatchIndirectCommand")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html)
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl Default for DispatchIndirectCommand {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubmitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo.html)
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}
impl Default for SubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphores: std::ptr::null(),
            p_wait_dst_stage_mask: std::ptr::null(),
            command_buffer_count: Default::default(),
            p_command_buffers: std::ptr::null(),
            signal_semaphore_count: Default::default(),
            p_signal_semaphores: std::ptr::null(),
        }
    }
}
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE: u32 = 16;
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
pub const MAX_MEMORY_TYPES: u32 = 32;
pub const MAX_MEMORY_HEAPS: u32 = 16;
pub const LOD_CLAMP_NONE: std::os::raw::c_float = 1000.0;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const WHOLE_SIZE: u64 = !0;
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const SUBPASS_EXTERNAL: u32 = !0;
#[doc(alias = "VkImageLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageLayout(pub i32);
impl ImageLayout {
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
    /// vk11
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
    /// khr_swapchain
    pub const PRESENT_SRC_KHR: Self = Self(1000001002);
    /// khr_synchronization2
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self(1000314000);
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self(1000314001);
}
crate::enum_impl! {
    ImageLayout : i32, UNDEFINED, GENERAL, COLOR_ATTACHMENT_OPTIMAL,
    DEPTH_STENCIL_ATTACHMENT_OPTIMAL, DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    SHADER_READ_ONLY_OPTIMAL, TRANSFER_SRC_OPTIMAL, TRANSFER_DST_OPTIMAL, PREINITIALIZED,
    DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
    DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL, PRESENT_SRC_KHR, READ_ONLY_OPTIMAL_KHR,
    ATTACHMENT_OPTIMAL_KHR
}
#[doc(alias = "VkAttachmentLoadOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentLoadOp(pub i32);
impl AttachmentLoadOp {
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
}
crate::enum_impl! {
    AttachmentLoadOp : i32, LOAD, CLEAR, DONT_CARE
}
#[doc(alias = "VkAttachmentStoreOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentStoreOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentStoreOp(pub i32);
impl AttachmentStoreOp {
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
    /// khr_dynamic_rendering
    pub const NONE_KHR: Self = Self(1000301000);
}
crate::enum_impl! {
    AttachmentStoreOp : i32, STORE, DONT_CARE, NONE_KHR
}
#[doc(alias = "VkImageType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageType(pub i32);
impl ImageType {
    pub const T1D: Self = Self(0);
    pub const T2D: Self = Self(1);
    pub const T3D: Self = Self(2);
}
crate::enum_impl! {
    ImageType : i32, T1D, T2D, T3D
}
#[doc(alias = "VkImageTiling")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageTiling(pub i32);
impl ImageTiling {
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
crate::enum_impl! {
    ImageTiling : i32, OPTIMAL, LINEAR
}
#[doc(alias = "VkImageViewType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageViewType(pub i32);
impl ImageViewType {
    pub const T1D: Self = Self(0);
    pub const T2D: Self = Self(1);
    pub const T3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const T1D_ARRAY: Self = Self(4);
    pub const T2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}
crate::enum_impl! {
    ImageViewType : i32, T1D, T2D, T3D, CUBE, T1D_ARRAY, T2D_ARRAY, CUBE_ARRAY
}
#[doc(alias = "VkCommandBufferLevel")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferLevel(pub i32);
impl CommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
crate::enum_impl! {
    CommandBufferLevel : i32, PRIMARY, SECONDARY
}
#[doc(alias = "VkComponentSwizzle")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ComponentSwizzle(pub i32);
impl ComponentSwizzle {
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}
crate::enum_impl! {
    ComponentSwizzle : i32, IDENTITY, ZERO, ONE, R, G, B, A
}
#[doc(alias = "VkDescriptorType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorType(pub i32);
impl DescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
}
crate::enum_impl! {
    DescriptorType : i32, SAMPLER, COMBINED_IMAGE_SAMPLER, SAMPLED_IMAGE, STORAGE_IMAGE,
    UNIFORM_TEXEL_BUFFER, STORAGE_TEXEL_BUFFER, UNIFORM_BUFFER, STORAGE_BUFFER,
    UNIFORM_BUFFER_DYNAMIC, STORAGE_BUFFER_DYNAMIC, INPUT_ATTACHMENT
}
#[doc(alias = "VkQueryType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryType(pub i32);
impl QueryType {
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
}
crate::enum_impl! {
    QueryType : i32, OCCLUSION, PIPELINE_STATISTICS, TIMESTAMP
}
#[doc(alias = "VkBorderColor")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BorderColor(pub i32);
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
}
crate::enum_impl! {
    BorderColor : i32, FLOAT_TRANSPARENT_BLACK, INT_TRANSPARENT_BLACK,
    FLOAT_OPAQUE_BLACK, INT_OPAQUE_BLACK, FLOAT_OPAQUE_WHITE, INT_OPAQUE_WHITE
}
#[doc(alias = "VkPipelineBindPoint")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineBindPoint(pub i32);
impl PipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
}
crate::enum_impl! {
    PipelineBindPoint : i32, GRAPHICS, COMPUTE
}
#[doc(alias = "VkPipelineCacheHeaderVersion")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersion.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCacheHeaderVersion(pub i32);
impl PipelineCacheHeaderVersion {
    pub const ONE: Self = Self(1);
}
crate::enum_impl! {
    PipelineCacheHeaderVersion : i32, ONE
}
#[doc(alias = "VkPipelineCacheCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCacheCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineCacheCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPrimitiveTopology")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PrimitiveTopology(pub i32);
impl PrimitiveTopology {
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
crate::enum_impl! {
    PrimitiveTopology : i32, POINT_LIST, LINE_LIST, LINE_STRIP, TRIANGLE_LIST,
    TRIANGLE_STRIP, TRIANGLE_FAN, LINE_LIST_WITH_ADJACENCY, LINE_STRIP_WITH_ADJACENCY,
    TRIANGLE_LIST_WITH_ADJACENCY, TRIANGLE_STRIP_WITH_ADJACENCY, PATCH_LIST
}
#[doc(alias = "VkSharingMode")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SharingMode(pub i32);
impl SharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
crate::enum_impl! {
    SharingMode : i32, EXCLUSIVE, CONCURRENT
}
#[doc(alias = "VkIndexType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndexType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct IndexType(pub i32);
impl IndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
}
crate::enum_impl! {
    IndexType : i32, UINT16, UINT32
}
#[doc(alias = "VkFilter")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilter.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Filter(pub i32);
impl Filter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
crate::enum_impl! {
    Filter : i32, NEAREST, LINEAR
}
#[doc(alias = "VkSamplerMipmapMode")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerMipmapMode(pub i32);
impl SamplerMipmapMode {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
crate::enum_impl! {
    SamplerMipmapMode : i32, NEAREST, LINEAR
}
#[doc(alias = "VkSamplerAddressMode")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerAddressMode(pub i32);
impl SamplerAddressMode {
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
}
crate::enum_impl! {
    SamplerAddressMode : i32, REPEAT, MIRRORED_REPEAT, CLAMP_TO_EDGE, CLAMP_TO_BORDER
}
#[doc(alias = "VkCompareOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CompareOp(pub i32);
impl CompareOp {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}
crate::enum_impl! {
    CompareOp : i32, NEVER, LESS, EQUAL, LESS_OR_EQUAL, GREATER, NOT_EQUAL,
    GREATER_OR_EQUAL, ALWAYS
}
#[doc(alias = "VkPolygonMode")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PolygonMode(pub i32);
impl PolygonMode {
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
}
crate::enum_impl! {
    PolygonMode : i32, FILL, LINE, POINT
}
#[doc(alias = "VkFrontFace")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FrontFace(pub i32);
impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}
crate::enum_impl! {
    FrontFace : i32, COUNTER_CLOCKWISE, CLOCKWISE
}
#[doc(alias = "VkBlendFactor")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BlendFactor(pub i32);
impl BlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}
crate::enum_impl! {
    BlendFactor : i32, ZERO, ONE, SRC_COLOR, ONE_MINUS_SRC_COLOR, DST_COLOR,
    ONE_MINUS_DST_COLOR, SRC_ALPHA, ONE_MINUS_SRC_ALPHA, DST_ALPHA, ONE_MINUS_DST_ALPHA,
    CONSTANT_COLOR, ONE_MINUS_CONSTANT_COLOR, CONSTANT_ALPHA, ONE_MINUS_CONSTANT_ALPHA,
    SRC_ALPHA_SATURATE, SRC1_COLOR, ONE_MINUS_SRC1_COLOR, SRC1_ALPHA,
    ONE_MINUS_SRC1_ALPHA
}
#[doc(alias = "VkBlendOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BlendOp(pub i32);
impl BlendOp {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
}
crate::enum_impl! {
    BlendOp : i32, ADD, SUBTRACT, REVERSE_SUBTRACT, MIN, MAX
}
#[doc(alias = "VkStencilOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StencilOp(pub i32);
impl StencilOp {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
crate::enum_impl! {
    StencilOp : i32, KEEP, ZERO, REPLACE, INCREMENT_AND_CLAMP, DECREMENT_AND_CLAMP,
    INVERT, INCREMENT_AND_WRAP, DECREMENT_AND_WRAP
}
#[doc(alias = "VkLogicOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct LogicOp(pub i32);
impl LogicOp {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
crate::enum_impl! {
    LogicOp : i32, CLEAR, AND, AND_REVERSE, COPY, AND_INVERTED, NO_OP, XOR, OR, NOR,
    EQUIVALENT, INVERT, OR_REVERSE, COPY_INVERTED, OR_INVERTED, NAND, SET
}
#[doc(alias = "VkInternalAllocationType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct InternalAllocationType(pub i32);
impl InternalAllocationType {
    pub const EXECUTABLE: Self = Self(0);
}
crate::enum_impl! {
    InternalAllocationType : i32, EXECUTABLE
}
#[doc(alias = "VkSystemAllocationScope")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SystemAllocationScope(pub i32);
impl SystemAllocationScope {
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}
crate::enum_impl! {
    SystemAllocationScope : i32, COMMAND, OBJECT, CACHE, DEVICE, INSTANCE
}
#[doc(alias = "VkPhysicalDeviceType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhysicalDeviceType(pub i32);
impl PhysicalDeviceType {
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}
crate::enum_impl! {
    PhysicalDeviceType : i32, OTHER, INTEGRATED_GPU, DISCRETE_GPU, VIRTUAL_GPU, CPU
}
#[doc(alias = "VkVertexInputRate")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VertexInputRate(pub i32);
impl VertexInputRate {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}
crate::enum_impl! {
    VertexInputRate : i32, VERTEX, INSTANCE
}
#[doc(alias = "VkFormat")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormat.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Format(pub i32);
impl Format {
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4x4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4x4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5x4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5x4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5x5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5x5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6x5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6x5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6x6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6x6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8x5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8x5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8x6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8x6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8x8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8x8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10x5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10x5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10x6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10x6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10x8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10x8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10x10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10x10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12x10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12x10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12x12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12x12_SRGB_BLOCK: Self = Self(184);
    /// vk11
    pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
}
crate::enum_impl! {
    Format : i32, UNDEFINED, R4G4_UNORM_PACK8, R4G4B4A4_UNORM_PACK16,
    B4G4R4A4_UNORM_PACK16, R5G6B5_UNORM_PACK16, B5G6R5_UNORM_PACK16,
    R5G5B5A1_UNORM_PACK16, B5G5R5A1_UNORM_PACK16, A1R5G5B5_UNORM_PACK16, R8_UNORM,
    R8_SNORM, R8_USCALED, R8_SSCALED, R8_UINT, R8_SINT, R8_SRGB, R8G8_UNORM, R8G8_SNORM,
    R8G8_USCALED, R8G8_SSCALED, R8G8_UINT, R8G8_SINT, R8G8_SRGB, R8G8B8_UNORM,
    R8G8B8_SNORM, R8G8B8_USCALED, R8G8B8_SSCALED, R8G8B8_UINT, R8G8B8_SINT, R8G8B8_SRGB,
    B8G8R8_UNORM, B8G8R8_SNORM, B8G8R8_USCALED, B8G8R8_SSCALED, B8G8R8_UINT, B8G8R8_SINT,
    B8G8R8_SRGB, R8G8B8A8_UNORM, R8G8B8A8_SNORM, R8G8B8A8_USCALED, R8G8B8A8_SSCALED,
    R8G8B8A8_UINT, R8G8B8A8_SINT, R8G8B8A8_SRGB, B8G8R8A8_UNORM, B8G8R8A8_SNORM,
    B8G8R8A8_USCALED, B8G8R8A8_SSCALED, B8G8R8A8_UINT, B8G8R8A8_SINT, B8G8R8A8_SRGB,
    A8B8G8R8_UNORM_PACK32, A8B8G8R8_SNORM_PACK32, A8B8G8R8_USCALED_PACK32,
    A8B8G8R8_SSCALED_PACK32, A8B8G8R8_UINT_PACK32, A8B8G8R8_SINT_PACK32,
    A8B8G8R8_SRGB_PACK32, A2R10G10B10_UNORM_PACK32, A2R10G10B10_SNORM_PACK32,
    A2R10G10B10_USCALED_PACK32, A2R10G10B10_SSCALED_PACK32, A2R10G10B10_UINT_PACK32,
    A2R10G10B10_SINT_PACK32, A2B10G10R10_UNORM_PACK32, A2B10G10R10_SNORM_PACK32,
    A2B10G10R10_USCALED_PACK32, A2B10G10R10_SSCALED_PACK32, A2B10G10R10_UINT_PACK32,
    A2B10G10R10_SINT_PACK32, R16_UNORM, R16_SNORM, R16_USCALED, R16_SSCALED, R16_UINT,
    R16_SINT, R16_SFLOAT, R16G16_UNORM, R16G16_SNORM, R16G16_USCALED, R16G16_SSCALED,
    R16G16_UINT, R16G16_SINT, R16G16_SFLOAT, R16G16B16_UNORM, R16G16B16_SNORM,
    R16G16B16_USCALED, R16G16B16_SSCALED, R16G16B16_UINT, R16G16B16_SINT,
    R16G16B16_SFLOAT, R16G16B16A16_UNORM, R16G16B16A16_SNORM, R16G16B16A16_USCALED,
    R16G16B16A16_SSCALED, R16G16B16A16_UINT, R16G16B16A16_SINT, R16G16B16A16_SFLOAT,
    R32_UINT, R32_SINT, R32_SFLOAT, R32G32_UINT, R32G32_SINT, R32G32_SFLOAT,
    R32G32B32_UINT, R32G32B32_SINT, R32G32B32_SFLOAT, R32G32B32A32_UINT,
    R32G32B32A32_SINT, R32G32B32A32_SFLOAT, R64_UINT, R64_SINT, R64_SFLOAT, R64G64_UINT,
    R64G64_SINT, R64G64_SFLOAT, R64G64B64_UINT, R64G64B64_SINT, R64G64B64_SFLOAT,
    R64G64B64A64_UINT, R64G64B64A64_SINT, R64G64B64A64_SFLOAT, B10G11R11_UFLOAT_PACK32,
    E5B9G9R9_UFLOAT_PACK32, D16_UNORM, X8_D24_UNORM_PACK32, D32_SFLOAT, S8_UINT,
    D16_UNORM_S8_UINT, D24_UNORM_S8_UINT, D32_SFLOAT_S8_UINT, BC1_RGB_UNORM_BLOCK,
    BC1_RGB_SRGB_BLOCK, BC1_RGBA_UNORM_BLOCK, BC1_RGBA_SRGB_BLOCK, BC2_UNORM_BLOCK,
    BC2_SRGB_BLOCK, BC3_UNORM_BLOCK, BC3_SRGB_BLOCK, BC4_UNORM_BLOCK, BC4_SNORM_BLOCK,
    BC5_UNORM_BLOCK, BC5_SNORM_BLOCK, BC6H_UFLOAT_BLOCK, BC6H_SFLOAT_BLOCK,
    BC7_UNORM_BLOCK, BC7_SRGB_BLOCK, ETC2_R8G8B8_UNORM_BLOCK, ETC2_R8G8B8_SRGB_BLOCK,
    ETC2_R8G8B8A1_UNORM_BLOCK, ETC2_R8G8B8A1_SRGB_BLOCK, ETC2_R8G8B8A8_UNORM_BLOCK,
    ETC2_R8G8B8A8_SRGB_BLOCK, EAC_R11_UNORM_BLOCK, EAC_R11_SNORM_BLOCK,
    EAC_R11G11_UNORM_BLOCK, EAC_R11G11_SNORM_BLOCK, ASTC_4x4_UNORM_BLOCK,
    ASTC_4x4_SRGB_BLOCK, ASTC_5x4_UNORM_BLOCK, ASTC_5x4_SRGB_BLOCK, ASTC_5x5_UNORM_BLOCK,
    ASTC_5x5_SRGB_BLOCK, ASTC_6x5_UNORM_BLOCK, ASTC_6x5_SRGB_BLOCK, ASTC_6x6_UNORM_BLOCK,
    ASTC_6x6_SRGB_BLOCK, ASTC_8x5_UNORM_BLOCK, ASTC_8x5_SRGB_BLOCK, ASTC_8x6_UNORM_BLOCK,
    ASTC_8x6_SRGB_BLOCK, ASTC_8x8_UNORM_BLOCK, ASTC_8x8_SRGB_BLOCK,
    ASTC_10x5_UNORM_BLOCK, ASTC_10x5_SRGB_BLOCK, ASTC_10x6_UNORM_BLOCK,
    ASTC_10x6_SRGB_BLOCK, ASTC_10x8_UNORM_BLOCK, ASTC_10x8_SRGB_BLOCK,
    ASTC_10x10_UNORM_BLOCK, ASTC_10x10_SRGB_BLOCK, ASTC_12x10_UNORM_BLOCK,
    ASTC_12x10_SRGB_BLOCK, ASTC_12x12_UNORM_BLOCK, ASTC_12x12_SRGB_BLOCK,
    G8B8G8R8_422_UNORM, B8G8R8G8_422_UNORM, G8_B8_R8_3PLANE_420_UNORM,
    G8_B8R8_2PLANE_420_UNORM, G8_B8_R8_3PLANE_422_UNORM, G8_B8R8_2PLANE_422_UNORM,
    G8_B8_R8_3PLANE_444_UNORM, R10X6_UNORM_PACK16, R10X6G10X6_UNORM_2PACK16,
    R10X6G10X6B10X6A10X6_UNORM_4PACK16, G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16, G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16, R12X4_UNORM_PACK16,
    R12X4G12X4_UNORM_2PACK16, R12X4G12X4B12X4A12X4_UNORM_4PACK16,
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16, B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16, G16B16G16R16_422_UNORM,
    B16G16R16G16_422_UNORM, G16_B16_R16_3PLANE_420_UNORM, G16_B16R16_2PLANE_420_UNORM,
    G16_B16_R16_3PLANE_422_UNORM, G16_B16R16_2PLANE_422_UNORM,
    G16_B16_R16_3PLANE_444_UNORM
}
#[doc(alias = "VkStructureType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStructureType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StructureType(pub i32);
impl StructureType {
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
    /// vk11
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
    pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
    pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(
        1000117003,
    );
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
    pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self = Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
    /// vk12
    pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
    pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
    pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
    pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
    pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
    pub const SUBPASS_END_INFO: Self = Self(1000109006);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(
        1000161003,
    );
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(
        1000161004,
    );
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(
        1000253000,
    );
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(
        1000175000,
    );
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(
        1000241000,
    );
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
    pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
    /// vk13
    pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(
        1000215000,
    );
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(
        1000276000,
    );
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(
        1000297000,
    );
    pub const MEMORY_BARRIER_2: Self = Self(1000314000);
    pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
    pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
    pub const DEPENDENCY_INFO: Self = Self(1000314003);
    pub const SUBMIT_INFO_2: Self = Self(1000314004);
    pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
    pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(
        1000325000,
    );
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
    pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
    pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
    pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
    pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
    pub const BUFFER_COPY_2: Self = Self(1000337006);
    pub const IMAGE_COPY_2: Self = Self(1000337007);
    pub const IMAGE_BLIT_2: Self = Self(1000337008);
    pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
    pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(
        1000225001,
    );
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(
        1000066000,
    );
    pub const RENDERING_INFO: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(
        1000280000,
    );
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(
        1000280001,
    );
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
    pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
    /// khr_swapchain
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
    pub const PRESENT_INFO_KHR: Self = Self(1000001001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
    /// khr_display
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
    /// khr_display_swapchain
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
    /// khr_xlib_surface
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
    /// khr_xcb_surface
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
    /// khr_wayland_surface
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
    /// khr_android_surface
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
    /// khr_win32_surface
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
    /// ext_debug_report
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
    /// amd_rasterization_order
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(
        1000018000,
    );
    /// ext_debug_marker
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
    /// khr_video_queue
    pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
    pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
    pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
    pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
    pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
    pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
    pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
    /// khr_video_decode_queue
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
    /// nv_dedicated_allocation
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
    /// ext_transform_feedback
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(
        1000028002,
    );
    /// nvx_binary_import
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
    /// nvx_image_view_handle
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
    /// ext_video_encode_h264
    pub const VIDEO_ENCODE_H264_CAPABILITIES_EXT: Self = Self(1000038000);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(
        1000038001,
    );
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000038002);
    pub const VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT: Self = Self(1000038003);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT: Self = Self(1000038004);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT: Self = Self(1000038005);
    pub const VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_INFO_EXT: Self = Self(
        1000038006,
    );
    pub const VIDEO_ENCODE_H264_PROFILE_INFO_EXT: Self = Self(1000038007);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT: Self = Self(1000038008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT: Self = Self(1000038009);
    pub const VIDEO_ENCODE_H264_REFERENCE_LISTS_INFO_EXT: Self = Self(1000038010);
    /// ext_video_encode_h265
    pub const VIDEO_ENCODE_H265_CAPABILITIES_EXT: Self = Self(1000039000);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(
        1000039001,
    );
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000039002);
    pub const VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT: Self = Self(1000039003);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT: Self = Self(1000039004);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_EXT: Self = Self(1000039005);
    pub const VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_INFO_EXT: Self = Self(
        1000039006,
    );
    pub const VIDEO_ENCODE_H265_PROFILE_INFO_EXT: Self = Self(1000039007);
    pub const VIDEO_ENCODE_H265_REFERENCE_LISTS_INFO_EXT: Self = Self(1000039008);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT: Self = Self(1000039009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT: Self = Self(1000039010);
    /// ext_video_decode_h264
    pub const VIDEO_DECODE_H264_CAPABILITIES_EXT: Self = Self(1000040000);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_EXT: Self = Self(1000040001);
    pub const VIDEO_DECODE_H264_PROFILE_INFO_EXT: Self = Self(1000040003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(
        1000040004,
    );
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000040005);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT: Self = Self(1000040006);
    /// amd_texture_gather_bias_lod
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
    /// khr_dynamic_rendering
    pub const RENDERING_INFO_KHR: Self = Self::RENDERING_INFO;
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self::RENDERING_ATTACHMENT_INFO;
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self::PIPELINE_RENDERING_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self = Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(
        1000044006,
    );
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(
        1000044007,
    );
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
    /// ggp_stream_descriptor_surface
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
    /// nv_corner_sampled_image
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
    /// khr_multiview
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
    /// nv_external_memory
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
    /// nv_external_memory_win32
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
    /// nv_win32_keyed_mutex
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
    /// khr_get_physical_device_properties2
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self::FORMAT_PROPERTIES_2;
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self = Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self = Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
    /// khr_device_group
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self = Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self = Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
    /// ext_validation_flags
    pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
    /// nn_vi_surface
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
    /// ext_texture_compression_astc_hdr
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
    /// ext_astc_decode_mode
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
    /// ext_pipeline_robustness
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self(1000068000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self = Self(
        1000068002,
    );
    /// khr_device_group_creation
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
    /// khr_external_memory_capabilities
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self = Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self = Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
    /// khr_external_memory
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
    /// khr_external_memory_win32
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
    /// khr_external_memory_fd
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
    /// khr_win32_keyed_mutex
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
    /// khr_external_semaphore_capabilities
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self = Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
    /// khr_external_semaphore
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
    /// khr_external_semaphore_win32
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
    /// khr_external_semaphore_fd
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
    /// khr_push_descriptor
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1000080000);
    /// ext_conditional_rendering
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(
        1000081000,
    );
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(
        1000081001,
    );
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
    /// khr_shader_float16_int8
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    /// khr_16bit_storage
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
    /// khr_incremental_present
    pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
    /// khr_descriptor_update_template
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
    /// nv_clip_space_w_scaling
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
    /// ext_display_surface_counter
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
    /// ext_display_control
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
    /// google_display_timing
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
    /// nvx_multiview_per_view_attributes
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self = Self(
        1000097000,
    );
    /// nv_viewport_swizzle
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
    /// ext_discard_rectangles
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
    /// ext_conservative_rasterization
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(
        1000101000,
    );
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(
        1000101001,
    );
    /// ext_depth_clip_enable
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(
        1000102001,
    );
    /// ext_hdr_metadata
    pub const HDR_METADATA_EXT: Self = Self(1000105000);
    /// khr_imageless_framebuffer
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
    /// khr_create_renderpass2
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self::ATTACHMENT_DESCRIPTION_2;
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self::ATTACHMENT_REFERENCE_2;
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self::SUBPASS_DESCRIPTION_2;
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self::SUBPASS_DEPENDENCY_2;
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self::RENDER_PASS_CREATE_INFO_2;
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self::SUBPASS_BEGIN_INFO;
    pub const SUBPASS_END_INFO_KHR: Self = Self::SUBPASS_END_INFO;
    /// khr_shared_presentable_image
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
    /// khr_external_fence_capabilities
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self = Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
    /// khr_external_fence
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
    /// khr_external_fence_win32
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
    /// khr_external_fence_fd
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
    /// khr_performance_query
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
    /// khr_maintenance2
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self = Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self = Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
    /// khr_get_surface_capabilities2
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
    /// khr_variable_pointers
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
    /// khr_get_display_properties2
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
    /// mvk_ios_surface
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
    /// mvk_macos_surface
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
    /// khr_dedicated_allocation
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
    /// ext_debug_utils
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
    /// android_external_memory_android_hardware_buffer
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(
        1000129006,
    );
    /// ext_sampler_filter_minmax
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self = Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
    /// ext_inline_uniform_block
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self = Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self = Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
    /// ext_sample_locations
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
    /// khr_get_memory_requirements2
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self::MEMORY_REQUIREMENTS_2;
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self = Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
    /// khr_image_format_list
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
    /// ext_blend_operation_advanced
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(
        1000148000,
    );
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(
        1000148001,
    );
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(
        1000148002,
    );
    /// nv_fragment_coverage_to_color
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
    /// khr_acceleration_structure
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(
        1000150004,
    );
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(
        1000150005,
    );
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(
        1000150013,
    );
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(
        1000150014,
    );
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
    /// khr_ray_tracing_pipeline
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(
        1000347001,
    );
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
    /// khr_ray_query
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
    /// nv_framebuffer_mixed_samples
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
    /// nv_shader_sm_builtins
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
    /// khr_sampler_ycbcr_conversion
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self = Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
    /// khr_bind_memory2
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
    /// ext_image_drm_format_modifier
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(
        1000158002,
    );
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(
        1000158004,
    );
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
    /// ext_validation_cache
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
    /// ext_descriptor_indexing
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self = Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self = Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self = Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
    /// khr_portability_subset
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
    /// nv_shading_rate_image
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(
        1000164000,
    );
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(
        1000164005,
    );
    /// nv_ray_tracing
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
    pub const GEOMETRY_NV: Self = Self(1000165003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
    pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(
        1000165008,
    );
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
    /// nv_representative_fragment_test
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(
        1000166000,
    );
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self = Self(
        1000166001,
    );
    /// khr_maintenance3
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
    /// ext_filter_cubic
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(
        1000170001,
    );
    /// ext_global_priority
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self = Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR;
    /// khr_shader_subgroup_extended_types
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
    /// khr_8bit_storage
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
    /// ext_external_memory_host
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(
        1000178002,
    );
    /// khr_shader_atomic_int64
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
    /// khr_shader_clock
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
    /// amd_pipeline_compiler_control
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
    /// ext_calibrated_timestamps
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000184000);
    /// amd_shader_core_properties
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
    /// ext_video_decode_h265
    pub const VIDEO_DECODE_H265_CAPABILITIES_EXT: Self = Self(1000187000);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(
        1000187001,
    );
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000187002);
    pub const VIDEO_DECODE_H265_PROFILE_INFO_EXT: Self = Self(1000187003);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_EXT: Self = Self(1000187004);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT: Self = Self(1000187005);
    /// khr_global_priority
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self = Self(
        1000388000,
    );
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self = Self(1000388001);
    /// amd_memory_overallocation_behavior
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
    /// ext_vertex_attribute_divisor
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(
        1000190000,
    );
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self = Self(
        1000190001,
    );
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self = Self(
        1000190002,
    );
    /// ggp_frame_token
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
    /// ext_pipeline_creation_feedback
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
    /// khr_driver_properties
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
    /// khr_shader_float_controls
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
    /// khr_depth_stencil_resolve
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self = Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
    /// nv_compute_shader_derivatives
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(
        1000201000,
    );
    /// nv_mesh_shader
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
    /// nv_fragment_shader_barycentric
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self = Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
    /// nv_shader_image_footprint
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(
        1000204000,
    );
    /// nv_scissor_exclusive
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(
        1000205000,
    );
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
    /// nv_device_diagnostic_checkpoints
    pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
    /// khr_timeline_semaphore
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self::SEMAPHORE_WAIT_INFO;
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self::SEMAPHORE_SIGNAL_INFO;
    /// intel_shader_integer_functions2
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(
        1000209000,
    );
    /// intel_performance_query
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
    /// khr_vulkan_memory_model
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
    /// ext_pci_bus_info
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
    /// amd_display_native_hdr
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
    /// fuchsia_imagepipe_surface
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
    /// khr_shader_terminate_invocation
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
    /// ext_metal_surface
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
    /// ext_fragment_density_map
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(
        1000218001,
    );
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
    /// ext_scalar_block_layout
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
    /// ext_subgroup_size_control
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
    /// khr_fragment_shading_rate
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(
        1000226001,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(
        1000226002,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(
        1000226003,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
    /// amd_shader_core_properties2
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
    /// amd_device_coherent_memory
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
    /// ext_shader_image_atomic_int64
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(
        1000234000,
    );
    /// ext_memory_budget
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
    /// ext_memory_priority
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
    /// khr_surface_protected_capabilities
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
    /// nv_dedicated_allocation_image_aliasing
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self = Self(
        1000240000,
    );
    /// khr_separate_depth_stencil_layouts
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self = Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self = Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
    /// ext_buffer_device_address
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(
        1000244000,
    );
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
    /// ext_tooling_info
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TOOL_PROPERTIES;
    /// ext_separate_stencil_usage
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
    /// ext_validation_features
    pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
    /// khr_present_wait
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
    /// nv_cooperative_matrix
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
    /// nv_coverage_reduction_mode
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(
        1000250000,
    );
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
    /// ext_fragment_shader_interlock
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(
        1000251000,
    );
    /// ext_ycbcr_image_arrays
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
    /// khr_uniform_buffer_standard_layout
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
    /// ext_provoking_vertex
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self = Self(
        1000254001,
    );
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
    /// ext_full_screen_exclusive
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
    /// ext_headless_surface
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
    /// khr_buffer_device_address
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self = Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self = Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self = Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
    /// ext_line_rasterization
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000259002);
    /// ext_shader_atomic_float
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
    /// ext_host_query_reset
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
    /// ext_index_type_uint8
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self = Self(1000265000);
    /// ext_extended_dynamic_state
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(
        1000267000,
    );
    /// khr_pipeline_executable_properties
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self = Self(
        1000269000,
    );
    pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
    /// ext_shader_atomic_float2
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(
        1000273000,
    );
    /// ext_shader_demote_to_helper_invocation
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
    /// nv_device_generated_commands
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(
        1000277000,
    );
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(
        1000277007,
    );
    /// nv_inherited_viewport_scissor
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(
        1000278000,
    );
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(
        1000278001,
    );
    /// khr_shader_integer_dot_product
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
    /// ext_texel_buffer_alignment
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(
        1000281000,
    );
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
    /// qcom_render_pass_transform
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self = Self(
        1000282000,
    );
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
    /// ext_device_memory_report
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
    /// ext_robustness2
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1000286001);
    /// ext_custom_border_color
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(
        1000287001,
    );
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
    /// khr_pipeline_library
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
    /// nv_present_barrier
    pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
    pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
    pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
    /// khr_present_id
    pub const PRESENT_ID_KHR: Self = Self(1000294000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
    /// ext_private_data
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self::DEVICE_PRIVATE_DATA_CREATE_INFO;
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self::PRIVATE_DATA_SLOT_CREATE_INFO;
    /// ext_pipeline_creation_cache_control
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
    /// khr_video_encode_queue
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
    pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
    /// nv_device_diagnostics_config
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
    /// ext_metal_objects
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
    /// khr_synchronization2
    pub const MEMORY_BARRIER_2_KHR: Self = Self::MEMORY_BARRIER_2;
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self::BUFFER_MEMORY_BARRIER_2;
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self::IMAGE_MEMORY_BARRIER_2;
    pub const DEPENDENCY_INFO_KHR: Self = Self::DEPENDENCY_INFO;
    pub const SUBMIT_INFO_2_KHR: Self = Self::SUBMIT_INFO_2;
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::SEMAPHORE_SUBMIT_INFO;
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self::COMMAND_BUFFER_SUBMIT_INFO;
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
    /// ext_graphics_pipeline_library
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(
        1000320000,
    );
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(
        1000320001,
    );
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
    /// amd_shader_early_and_late_fragment_tests
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self = Self(
        1000321000,
    );
    /// khr_fragment_shader_barycentric
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(
        1000203000,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self = Self(
        1000322000,
    );
    /// khr_shader_subgroup_uniform_control_flow
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self = Self(
        1000323000,
    );
    /// khr_zero_initialize_workgroup_memory
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
    /// nv_fragment_shading_rate_enums
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(
        1000326000,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(
        1000326001,
    );
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(
        1000326002,
    );
    /// nv_ray_tracing_motion_blur
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(
        1000327000,
    );
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(
        1000327001,
    );
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
    /// ext_mesh_shader
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
    /// ext_ycbcr_2plane_444_formats
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(
        1000330000,
    );
    /// ext_fragment_density_map2
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(
        1000332000,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(
        1000332001,
    );
    /// qcom_rotated_copy_commands
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
    /// ext_image_robustness
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
    /// khr_workgroup_memory_explicit_layout
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self = Self(
        1000336000,
    );
    /// khr_copy_commands2
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self::COPY_BUFFER_INFO_2;
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self::COPY_IMAGE_INFO_2;
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self::COPY_BUFFER_TO_IMAGE_INFO_2;
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self::COPY_IMAGE_TO_BUFFER_INFO_2;
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self::BLIT_IMAGE_INFO_2;
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self::RESOLVE_IMAGE_INFO_2;
    pub const BUFFER_COPY_2_KHR: Self = Self::BUFFER_COPY_2;
    pub const IMAGE_COPY_2_KHR: Self = Self::IMAGE_COPY_2;
    pub const IMAGE_BLIT_2_KHR: Self = Self::IMAGE_BLIT_2;
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self::BUFFER_IMAGE_COPY_2;
    pub const IMAGE_RESOLVE_2_KHR: Self = Self::IMAGE_RESOLVE_2;
    /// ext_image_compression_control
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(
        1000338000,
    );
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self(1000338003);
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
    /// ext_attachment_feedback_loop_layout
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self = Self(
        1000339000,
    );
    /// ext_4444_formats
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
    /// ext_device_fault
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
    pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
    pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
    /// arm_rasterization_order_attachment_access
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self = Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
    /// ext_rgba10x6_formats
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
    /// ext_directfb_surface
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
    /// valve_mutable_descriptor_type
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self = Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self = Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
    /// ext_vertex_input_dynamic_state
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(
        1000352000,
    );
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
    /// ext_physical_device_drm
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
    /// ext_device_address_binding_report
    pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(
        1000354000,
    );
    pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
    /// ext_depth_clip_control
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(
        1000355001,
    );
    /// ext_primitive_topology_list_restart
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(
        1000356000,
    );
    /// khr_format_feature_flags2
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self::FORMAT_PROPERTIES_3;
    /// fuchsia_external_memory
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
    /// fuchsia_external_semaphore
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
    /// fuchsia_buffer_collection
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
    /// huawei_subpass_shading
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
    /// huawei_invocation_mask
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
    /// nv_external_memory_rdma
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
    /// ext_pipeline_properties
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
    pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
    /// ext_multisampled_render_to_single_sampled
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self = Self(
        1000376000,
    );
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
    /// ext_extended_dynamic_state2
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(
        1000377000,
    );
    /// qnx_screen_surface
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
    /// ext_color_write_enable
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
    /// ext_primitives_generated_query
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(
        1000382000,
    );
    /// khr_ray_tracing_maintenance1
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(
        1000386000,
    );
    /// ext_global_priority_query
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self = Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;
    /// ext_image_view_min_lod
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
    /// ext_multi_draw
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
    /// ext_image_2d_view_of_3d
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
    /// ext_opacity_micromap
    pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
    pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
    pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
    pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
    pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
    pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
    pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(
        1000396009,
    );
    /// ext_border_color_swizzle
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(
        1000411001,
    );
    /// ext_pageable_device_local_memory
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(
        1000412000,
    );
    /// khr_maintenance4
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
    /// valve_descriptor_set_host_mapping
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self = Self(
        1000420000,
    );
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
    /// ext_depth_clamp_zero_one
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self = Self(1000421000);
    /// ext_non_seamless_cube_map
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(
        1000422000,
    );
    /// qcom_fragment_density_map_offset
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(
        1000425000,
    );
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self = Self(
        1000425001,
    );
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1000425002);
    /// nv_linear_color_attachment
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(
        1000430000,
    );
    /// ext_image_compression_control_swapchain
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self = Self(
        1000437000,
    );
    /// qcom_image_processing
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
    pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
    /// ext_extended_dynamic_state3
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(
        1000455000,
    );
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(
        1000455001,
    );
    /// ext_subpass_merge_feedback
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(
        1000458000,
    );
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
    /// ext_shader_module_identifier
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(
        1000462000,
    );
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(
        1000462001,
    );
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(
        1000462002,
    );
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
    /// ext_rasterization_order_attachment_access
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self = Self(
        1000342000,
    );
    /// nv_optical_flow
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
    pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
    pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
    pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
    /// ext_legacy_dithering
    pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
    /// ext_pipeline_protected_access
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self = Self(
        1000466000,
    );
    /// qcom_tile_properties
    pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
    pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
    /// sec_amigo_profiling
    pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
    pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
    /// ext_mutable_descriptor_type
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(
        1000351000,
    );
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
}
crate::enum_impl! {
    StructureType : i32, APPLICATION_INFO, INSTANCE_CREATE_INFO,
    DEVICE_QUEUE_CREATE_INFO, DEVICE_CREATE_INFO, SUBMIT_INFO, MEMORY_ALLOCATE_INFO,
    MAPPED_MEMORY_RANGE, BIND_SPARSE_INFO, FENCE_CREATE_INFO, SEMAPHORE_CREATE_INFO,
    EVENT_CREATE_INFO, QUERY_POOL_CREATE_INFO, BUFFER_CREATE_INFO,
    BUFFER_VIEW_CREATE_INFO, IMAGE_CREATE_INFO, IMAGE_VIEW_CREATE_INFO,
    SHADER_MODULE_CREATE_INFO, PIPELINE_CACHE_CREATE_INFO,
    PIPELINE_SHADER_STAGE_CREATE_INFO, PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO, PIPELINE_TESSELLATION_STATE_CREATE_INFO,
    PIPELINE_VIEWPORT_STATE_CREATE_INFO, PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    PIPELINE_MULTISAMPLE_STATE_CREATE_INFO, PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
    PIPELINE_COLOR_BLEND_STATE_CREATE_INFO, PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    GRAPHICS_PIPELINE_CREATE_INFO, COMPUTE_PIPELINE_CREATE_INFO,
    PIPELINE_LAYOUT_CREATE_INFO, SAMPLER_CREATE_INFO, DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
    DESCRIPTOR_POOL_CREATE_INFO, DESCRIPTOR_SET_ALLOCATE_INFO, WRITE_DESCRIPTOR_SET,
    COPY_DESCRIPTOR_SET, FRAMEBUFFER_CREATE_INFO, RENDER_PASS_CREATE_INFO,
    COMMAND_POOL_CREATE_INFO, COMMAND_BUFFER_ALLOCATE_INFO,
    COMMAND_BUFFER_INHERITANCE_INFO, COMMAND_BUFFER_BEGIN_INFO, RENDER_PASS_BEGIN_INFO,
    BUFFER_MEMORY_BARRIER, IMAGE_MEMORY_BARRIER, MEMORY_BARRIER,
    LOADER_INSTANCE_CREATE_INFO, LOADER_DEVICE_CREATE_INFO,
    PHYSICAL_DEVICE_SUBGROUP_PROPERTIES, BIND_BUFFER_MEMORY_INFO, BIND_IMAGE_MEMORY_INFO,
    PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES, MEMORY_DEDICATED_REQUIREMENTS,
    MEMORY_DEDICATED_ALLOCATE_INFO, MEMORY_ALLOCATE_FLAGS_INFO,
    DEVICE_GROUP_RENDER_PASS_BEGIN_INFO, DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
    DEVICE_GROUP_SUBMIT_INFO, DEVICE_GROUP_BIND_SPARSE_INFO,
    BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO, BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
    PHYSICAL_DEVICE_GROUP_PROPERTIES, DEVICE_GROUP_DEVICE_CREATE_INFO,
    BUFFER_MEMORY_REQUIREMENTS_INFO_2, IMAGE_MEMORY_REQUIREMENTS_INFO_2,
    IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2, MEMORY_REQUIREMENTS_2,
    SPARSE_IMAGE_MEMORY_REQUIREMENTS_2, PHYSICAL_DEVICE_FEATURES_2,
    PHYSICAL_DEVICE_PROPERTIES_2, FORMAT_PROPERTIES_2, IMAGE_FORMAT_PROPERTIES_2,
    PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2, QUEUE_FAMILY_PROPERTIES_2,
    PHYSICAL_DEVICE_MEMORY_PROPERTIES_2, SPARSE_IMAGE_FORMAT_PROPERTIES_2,
    PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
    PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
    RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO, IMAGE_VIEW_USAGE_CREATE_INFO,
    PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
    RENDER_PASS_MULTIVIEW_CREATE_INFO, PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
    PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES, PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
    PROTECTED_SUBMIT_INFO, PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
    PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES, DEVICE_QUEUE_INFO_2,
    SAMPLER_YCBCR_CONVERSION_CREATE_INFO, SAMPLER_YCBCR_CONVERSION_INFO,
    BIND_IMAGE_PLANE_MEMORY_INFO, IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
    PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
    SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
    DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO, PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
    EXTERNAL_IMAGE_FORMAT_PROPERTIES, PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
    EXTERNAL_BUFFER_PROPERTIES, PHYSICAL_DEVICE_ID_PROPERTIES,
    EXTERNAL_MEMORY_BUFFER_CREATE_INFO, EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
    EXPORT_MEMORY_ALLOCATE_INFO, PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
    EXTERNAL_FENCE_PROPERTIES, EXPORT_FENCE_CREATE_INFO, EXPORT_SEMAPHORE_CREATE_INFO,
    PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO, EXTERNAL_SEMAPHORE_PROPERTIES,
    PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES, DESCRIPTOR_SET_LAYOUT_SUPPORT,
    PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES, PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
    PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES, PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
    PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES, IMAGE_FORMAT_LIST_CREATE_INFO,
    ATTACHMENT_DESCRIPTION_2, ATTACHMENT_REFERENCE_2, SUBPASS_DESCRIPTION_2,
    SUBPASS_DEPENDENCY_2, RENDER_PASS_CREATE_INFO_2, SUBPASS_BEGIN_INFO,
    SUBPASS_END_INFO, PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
    PHYSICAL_DEVICE_DRIVER_PROPERTIES, PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES,
    PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
    PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES,
    DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
    PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES,
    PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES,
    DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
    DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT,
    PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
    SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
    PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES, IMAGE_STENCIL_USAGE_CREATE_INFO,
    PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES, SAMPLER_REDUCTION_MODE_CREATE_INFO,
    PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES,
    PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES, FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
    FRAMEBUFFER_ATTACHMENT_IMAGE_INFO, RENDER_PASS_ATTACHMENT_BEGIN_INFO,
    PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES,
    PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES,
    PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES,
    ATTACHMENT_REFERENCE_STENCIL_LAYOUT, ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT,
    PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES,
    PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
    PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES, SEMAPHORE_TYPE_CREATE_INFO,
    TIMELINE_SEMAPHORE_SUBMIT_INFO, SEMAPHORE_WAIT_INFO, SEMAPHORE_SIGNAL_INFO,
    PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES, BUFFER_DEVICE_ADDRESS_INFO,
    BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO,
    MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO,
    DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO, PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
    PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES, PIPELINE_CREATION_FEEDBACK_CREATE_INFO,
    PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES,
    PHYSICAL_DEVICE_TOOL_PROPERTIES,
    PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES,
    PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES, DEVICE_PRIVATE_DATA_CREATE_INFO,
    PRIVATE_DATA_SLOT_CREATE_INFO,
    PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES, MEMORY_BARRIER_2,
    BUFFER_MEMORY_BARRIER_2, IMAGE_MEMORY_BARRIER_2, DEPENDENCY_INFO, SUBMIT_INFO_2,
    SEMAPHORE_SUBMIT_INFO, COMMAND_BUFFER_SUBMIT_INFO,
    PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
    PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES,
    PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES, COPY_BUFFER_INFO_2, COPY_IMAGE_INFO_2,
    COPY_BUFFER_TO_IMAGE_INFO_2, COPY_IMAGE_TO_BUFFER_INFO_2, BLIT_IMAGE_INFO_2,
    RESOLVE_IMAGE_INFO_2, BUFFER_COPY_2, IMAGE_COPY_2, IMAGE_BLIT_2, BUFFER_IMAGE_COPY_2,
    IMAGE_RESOLVE_2, PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES,
    PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO,
    PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES,
    PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES,
    PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES,
    WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
    DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
    PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES, RENDERING_INFO,
    RENDERING_ATTACHMENT_INFO, PIPELINE_RENDERING_CREATE_INFO,
    PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
    COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
    PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
    PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES,
    PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES, FORMAT_PROPERTIES_3,
    PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES, PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES,
    DEVICE_BUFFER_MEMORY_REQUIREMENTS, DEVICE_IMAGE_MEMORY_REQUIREMENTS,
    SWAPCHAIN_CREATE_INFO_KHR, PRESENT_INFO_KHR, DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
    IMAGE_SWAPCHAIN_CREATE_INFO_KHR, BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
    ACQUIRE_NEXT_IMAGE_INFO_KHR, DEVICE_GROUP_PRESENT_INFO_KHR,
    DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR, DISPLAY_MODE_CREATE_INFO_KHR,
    DISPLAY_SURFACE_CREATE_INFO_KHR, DISPLAY_PRESENT_INFO_KHR,
    XLIB_SURFACE_CREATE_INFO_KHR, XCB_SURFACE_CREATE_INFO_KHR,
    WAYLAND_SURFACE_CREATE_INFO_KHR, ANDROID_SURFACE_CREATE_INFO_KHR,
    WIN32_SURFACE_CREATE_INFO_KHR, DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
    PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
    DEBUG_MARKER_OBJECT_NAME_INFO_EXT, DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
    DEBUG_MARKER_MARKER_INFO_EXT, VIDEO_PROFILE_INFO_KHR, VIDEO_CAPABILITIES_KHR,
    VIDEO_PICTURE_RESOURCE_INFO_KHR, VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR,
    BIND_VIDEO_SESSION_MEMORY_INFO_KHR, VIDEO_SESSION_CREATE_INFO_KHR,
    VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR, VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR,
    VIDEO_BEGIN_CODING_INFO_KHR, VIDEO_END_CODING_INFO_KHR,
    VIDEO_CODING_CONTROL_INFO_KHR, VIDEO_REFERENCE_SLOT_INFO_KHR,
    QUEUE_FAMILY_VIDEO_PROPERTIES_KHR, VIDEO_PROFILE_LIST_INFO_KHR,
    PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR, VIDEO_FORMAT_PROPERTIES_KHR,
    QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR, VIDEO_DECODE_INFO_KHR,
    VIDEO_DECODE_CAPABILITIES_KHR, VIDEO_DECODE_USAGE_INFO_KHR,
    DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
    DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
    DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
    PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
    PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
    PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT, CU_MODULE_CREATE_INFO_NVX,
    CU_FUNCTION_CREATE_INFO_NVX, CU_LAUNCH_INFO_NVX, IMAGE_VIEW_HANDLE_INFO_NVX,
    IMAGE_VIEW_ADDRESS_PROPERTIES_NVX, VIDEO_ENCODE_H264_CAPABILITIES_EXT,
    VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT,
    VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT,
    VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT, VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT,
    VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT,
    VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_INFO_EXT,
    VIDEO_ENCODE_H264_PROFILE_INFO_EXT, VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT,
    VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT,
    VIDEO_ENCODE_H264_REFERENCE_LISTS_INFO_EXT, VIDEO_ENCODE_H265_CAPABILITIES_EXT,
    VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT,
    VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT,
    VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT, VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT,
    VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_EXT,
    VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_INFO_EXT,
    VIDEO_ENCODE_H265_PROFILE_INFO_EXT, VIDEO_ENCODE_H265_REFERENCE_LISTS_INFO_EXT,
    VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT,
    VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT, VIDEO_DECODE_H264_CAPABILITIES_EXT,
    VIDEO_DECODE_H264_PICTURE_INFO_EXT, VIDEO_DECODE_H264_PROFILE_INFO_EXT,
    VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT,
    VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT,
    VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT, TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
    RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
    RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT, ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
    MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX, STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
    PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
    EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV, EXPORT_MEMORY_ALLOCATE_INFO_NV,
    IMPORT_MEMORY_WIN32_HANDLE_INFO_NV, EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
    WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV, VALIDATION_FLAGS_EXT,
    VI_SURFACE_CREATE_INFO_NN, IMAGE_VIEW_ASTC_DECODE_MODE_EXT,
    PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT, PIPELINE_ROBUSTNESS_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT,
    PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT,
    IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR, EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
    MEMORY_WIN32_HANDLE_PROPERTIES_KHR, MEMORY_GET_WIN32_HANDLE_INFO_KHR,
    IMPORT_MEMORY_FD_INFO_KHR, MEMORY_FD_PROPERTIES_KHR, MEMORY_GET_FD_INFO_KHR,
    WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR, IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
    EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR, D3D12_FENCE_SUBMIT_INFO_KHR,
    SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR, IMPORT_SEMAPHORE_FD_INFO_KHR,
    SEMAPHORE_GET_FD_INFO_KHR, PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
    COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT,
    PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
    CONDITIONAL_RENDERING_BEGIN_INFO_EXT, PRESENT_REGIONS_KHR,
    PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV, SURFACE_CAPABILITIES_2_EXT,
    DISPLAY_POWER_INFO_EXT, DEVICE_EVENT_INFO_EXT, DISPLAY_EVENT_INFO_EXT,
    SWAPCHAIN_COUNTER_CREATE_INFO_EXT, PRESENT_TIMES_INFO_GOOGLE,
    PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
    PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
    PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
    PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
    PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
    PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT, HDR_METADATA_EXT,
    SHARED_PRESENT_SURFACE_CAPABILITIES_KHR, IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
    EXPORT_FENCE_WIN32_HANDLE_INFO_KHR, FENCE_GET_WIN32_HANDLE_INFO_KHR,
    IMPORT_FENCE_FD_INFO_KHR, FENCE_GET_FD_INFO_KHR,
    PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR,
    PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR,
    QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR, PERFORMANCE_QUERY_SUBMIT_INFO_KHR,
    ACQUIRE_PROFILING_LOCK_INFO_KHR, PERFORMANCE_COUNTER_KHR,
    PERFORMANCE_COUNTER_DESCRIPTION_KHR, PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
    SURFACE_CAPABILITIES_2_KHR, SURFACE_FORMAT_2_KHR, DISPLAY_PROPERTIES_2_KHR,
    DISPLAY_PLANE_PROPERTIES_2_KHR, DISPLAY_MODE_PROPERTIES_2_KHR,
    DISPLAY_PLANE_INFO_2_KHR, DISPLAY_PLANE_CAPABILITIES_2_KHR,
    IOS_SURFACE_CREATE_INFO_MVK, MACOS_SURFACE_CREATE_INFO_MVK,
    DEBUG_UTILS_OBJECT_NAME_INFO_EXT, DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
    DEBUG_UTILS_LABEL_EXT, DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
    DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT, ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
    ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
    ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
    IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
    MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID, EXTERNAL_FORMAT_ANDROID,
    ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID, SAMPLE_LOCATIONS_INFO_EXT,
    RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
    PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT, MULTISAMPLE_PROPERTIES_EXT,
    PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
    PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
    PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
    PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
    WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
    ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
    ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
    ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR,
    ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
    ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
    ACCELERATION_STRUCTURE_GEOMETRY_KHR, ACCELERATION_STRUCTURE_VERSION_INFO_KHR,
    COPY_ACCELERATION_STRUCTURE_INFO_KHR, COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR,
    COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR,
    PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
    PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR,
    ACCELERATION_STRUCTURE_CREATE_INFO_KHR, ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR,
    PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
    PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
    RAY_TRACING_PIPELINE_CREATE_INFO_KHR, RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
    RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
    PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
    PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
    PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
    PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
    DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT,
    PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT,
    IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT,
    IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT,
    IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT, DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT,
    VALIDATION_CACHE_CREATE_INFO_EXT, SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR,
    PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR,
    PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV,
    PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
    PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
    PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV,
    RAY_TRACING_PIPELINE_CREATE_INFO_NV, ACCELERATION_STRUCTURE_CREATE_INFO_NV,
    GEOMETRY_NV, GEOMETRY_TRIANGLES_NV, GEOMETRY_AABB_NV,
    BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
    WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
    ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
    PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV, RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
    ACCELERATION_STRUCTURE_INFO_NV,
    PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
    PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
    PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
    FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
    IMPORT_MEMORY_HOST_POINTER_INFO_EXT, MEMORY_HOST_POINTER_PROPERTIES_EXT,
    PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
    PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR, PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
    CALIBRATED_TIMESTAMP_INFO_EXT, PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
    VIDEO_DECODE_H265_CAPABILITIES_EXT,
    VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT,
    VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT,
    VIDEO_DECODE_H265_PROFILE_INFO_EXT, VIDEO_DECODE_H265_PICTURE_INFO_EXT,
    VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT, DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR,
    PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR,
    QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR,
    DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
    PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
    PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT, PRESENT_FRAME_TOKEN_GGP,
    PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
    PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV, PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
    PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
    PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV,
    PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV, CHECKPOINT_DATA_NV,
    QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
    PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL,
    QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
    INITIALIZE_PERFORMANCE_API_INFO_INTEL, PERFORMANCE_MARKER_INFO_INTEL,
    PERFORMANCE_STREAM_MARKER_INFO_INTEL, PERFORMANCE_OVERRIDE_INFO_INTEL,
    PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
    PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
    DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
    SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD, IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
    METAL_SURFACE_CREATE_INFO_EXT, PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
    PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT,
    RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT,
    FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
    PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR,
    PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR,
    PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR,
    PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR,
    PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
    PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
    PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT,
    PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
    PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT, MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
    SURFACE_PROTECTED_CAPABILITIES_KHR,
    PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV,
    PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
    BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT, VALIDATION_FEATURES_EXT,
    PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR,
    PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV, COOPERATIVE_MATRIX_PROPERTIES_NV,
    PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
    PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
    PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
    FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
    PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
    PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
    PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT,
    PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT,
    SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
    SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
    SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT, HEADLESS_SURFACE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT,
    PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT,
    PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT,
    PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
    PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
    PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR, PIPELINE_INFO_KHR,
    PIPELINE_EXECUTABLE_PROPERTIES_KHR, PIPELINE_EXECUTABLE_INFO_KHR,
    PIPELINE_EXECUTABLE_STATISTIC_KHR, PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
    PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT,
    PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
    GRAPHICS_SHADER_GROUP_CREATE_INFO_NV, GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
    INDIRECT_COMMANDS_LAYOUT_TOKEN_NV, INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
    GENERATED_COMMANDS_INFO_NV, GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
    PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
    PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV,
    COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV,
    PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
    COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
    RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
    PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT,
    DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT, DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT,
    PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT,
    PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT,
    SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
    PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT, PIPELINE_LIBRARY_CREATE_INFO_KHR,
    PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV, SURFACE_CAPABILITIES_PRESENT_BARRIER_NV,
    SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV, PRESENT_ID_KHR,
    PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR, VIDEO_ENCODE_INFO_KHR,
    VIDEO_ENCODE_RATE_CONTROL_INFO_KHR, VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR,
    VIDEO_ENCODE_CAPABILITIES_KHR, VIDEO_ENCODE_USAGE_INFO_KHR,
    PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
    DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV, EXPORT_METAL_OBJECT_CREATE_INFO_EXT,
    EXPORT_METAL_OBJECTS_INFO_EXT, EXPORT_METAL_DEVICE_INFO_EXT,
    EXPORT_METAL_COMMAND_QUEUE_INFO_EXT, EXPORT_METAL_BUFFER_INFO_EXT,
    IMPORT_METAL_BUFFER_INFO_EXT, EXPORT_METAL_TEXTURE_INFO_EXT,
    IMPORT_METAL_TEXTURE_INFO_EXT, EXPORT_METAL_IO_SURFACE_INFO_EXT,
    IMPORT_METAL_IO_SURFACE_INFO_EXT, EXPORT_METAL_SHARED_EVENT_INFO_EXT,
    IMPORT_METAL_SHARED_EVENT_INFO_EXT, QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
    CHECKPOINT_DATA_2_NV, PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT,
    PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT,
    GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD,
    PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR,
    PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR,
    PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
    PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
    PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
    PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
    ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV,
    PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
    ACCELERATION_STRUCTURE_MOTION_INFO_NV, PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT,
    PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT,
    PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT,
    PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
    PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT,
    COPY_COMMAND_TRANSFORM_INFO_QCOM,
    PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR,
    PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT,
    IMAGE_COMPRESSION_CONTROL_EXT, SUBRESOURCE_LAYOUT_2_EXT, IMAGE_SUBRESOURCE_2_EXT,
    IMAGE_COMPRESSION_PROPERTIES_EXT,
    PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT,
    PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT, PHYSICAL_DEVICE_FAULT_FEATURES_EXT,
    DEVICE_FAULT_COUNTS_EXT, DEVICE_FAULT_INFO_EXT,
    PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT, DIRECTFB_SURFACE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
    VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT, VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
    PHYSICAL_DEVICE_DRM_PROPERTIES_EXT,
    PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT,
    DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT,
    PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT,
    PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT,
    IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA, MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
    MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA, IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
    SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA, BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
    IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA, BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
    BUFFER_COLLECTION_PROPERTIES_FUCHSIA, BUFFER_CONSTRAINTS_INFO_FUCHSIA,
    BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA, IMAGE_CONSTRAINTS_INFO_FUCHSIA,
    IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA, SYSMEM_COLOR_SPACE_FUCHSIA,
    BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
    SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
    PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
    PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
    PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI, MEMORY_GET_REMOTE_ADDRESS_INFO_NV,
    PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV, PIPELINE_PROPERTIES_IDENTIFIER_EXT,
    PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT,
    PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT,
    SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT,
    MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT,
    PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT,
    SCREEN_SURFACE_CREATE_INFO_QNX, PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT,
    PIPELINE_COLOR_WRITE_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT,
    PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR,
    PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT, IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT, PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT,
    PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT, MICROMAP_BUILD_INFO_EXT,
    MICROMAP_VERSION_INFO_EXT, COPY_MICROMAP_INFO_EXT, COPY_MICROMAP_TO_MEMORY_INFO_EXT,
    COPY_MEMORY_TO_MICROMAP_INFO_EXT, PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT,
    PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT, MICROMAP_CREATE_INFO_EXT,
    MICROMAP_BUILD_SIZES_INFO_EXT, ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT,
    PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT,
    SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
    PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE,
    DESCRIPTOR_SET_BINDING_REFERENCE_VALVE,
    DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
    PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT,
    PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT,
    PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM,
    PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM,
    SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM,
    PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
    PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT,
    PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM,
    PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM,
    IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM,
    PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT,
    PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT,
    PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT,
    RENDER_PASS_CREATION_CONTROL_EXT, RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT,
    RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT,
    PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT,
    PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT,
    PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT,
    SHADER_MODULE_IDENTIFIER_EXT,
    PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT,
    PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV, PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV,
    OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV, OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV,
    OPTICAL_FLOW_SESSION_CREATE_INFO_NV, OPTICAL_FLOW_EXECUTE_INFO_NV,
    OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV,
    PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT,
    PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT,
    PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM, TILE_PROPERTIES_QCOM,
    PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC, AMIGO_PROFILING_SUBMIT_INFO_SEC,
    PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT,
    MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT
}
#[doc(alias = "VkSubpassContents")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassContents.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubpassContents(pub i32);
impl SubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
}
crate::enum_impl! {
    SubpassContents : i32, INLINE, SECONDARY_COMMAND_BUFFERS
}
#[doc(alias = "VkResult")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResult.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Result(pub i32);
impl Result {
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const ERROR_UNKNOWN: Self = Self(-13);
    /// vk11
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
    /// khr_surface
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
    /// khr_swapchain
    pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
    /// ext_debug_report
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Self(-1000011001);
}
crate::enum_impl! {
    Result : i32, SUCCESS, NOT_READY, TIMEOUT, EVENT_SET, EVENT_RESET, INCOMPLETE,
    ERROR_OUT_OF_HOST_MEMORY, ERROR_OUT_OF_DEVICE_MEMORY, ERROR_INITIALIZATION_FAILED,
    ERROR_DEVICE_LOST, ERROR_MEMORY_MAP_FAILED, ERROR_LAYER_NOT_PRESENT,
    ERROR_EXTENSION_NOT_PRESENT, ERROR_FEATURE_NOT_PRESENT, ERROR_INCOMPATIBLE_DRIVER,
    ERROR_TOO_MANY_OBJECTS, ERROR_FORMAT_NOT_SUPPORTED, ERROR_FRAGMENTED_POOL,
    ERROR_UNKNOWN, ERROR_OUT_OF_POOL_MEMORY, ERROR_INVALID_EXTERNAL_HANDLE,
    ERROR_SURFACE_LOST_KHR, ERROR_NATIVE_WINDOW_IN_USE_KHR, SUBOPTIMAL_KHR,
    ERROR_OUT_OF_DATE_KHR, ERROR_VALIDATION_FAILED_EXT
}
#[doc(alias = "VkDynamicState")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DynamicState(pub i32);
impl DynamicState {
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
}
crate::enum_impl! {
    DynamicState : i32, VIEWPORT, SCISSOR, LINE_WIDTH, DEPTH_BIAS, BLEND_CONSTANTS,
    DEPTH_BOUNDS, STENCIL_COMPARE_MASK, STENCIL_WRITE_MASK, STENCIL_REFERENCE
}
#[doc(alias = "VkObjectType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkObjectType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ObjectType(pub i32);
impl ObjectType {
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
    /// vk11
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    /// khr_surface
    pub const SURFACE_KHR: Self = Self(1000000000);
    /// khr_swapchain
    pub const SWAPCHAIN_KHR: Self = Self(1000001000);
    /// ext_debug_report
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
    /// ext_debug_utils
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
}
crate::enum_impl! {
    ObjectType : i32, UNKNOWN, INSTANCE, PHYSICAL_DEVICE, DEVICE, QUEUE, SEMAPHORE,
    COMMAND_BUFFER, FENCE, DEVICE_MEMORY, BUFFER, IMAGE, EVENT, QUERY_POOL, BUFFER_VIEW,
    IMAGE_VIEW, SHADER_MODULE, PIPELINE_CACHE, PIPELINE_LAYOUT, RENDER_PASS, PIPELINE,
    DESCRIPTOR_SET_LAYOUT, SAMPLER, DESCRIPTOR_POOL, DESCRIPTOR_SET, FRAMEBUFFER,
    COMMAND_POOL, SAMPLER_YCBCR_CONVERSION, DESCRIPTOR_UPDATE_TEMPLATE, SURFACE_KHR,
    SWAPCHAIN_KHR, DEBUG_REPORT_CALLBACK_EXT, DEBUG_UTILS_MESSENGER_EXT
}
#[doc(alias = "VkQueueFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueueFlags(pub u32);
impl QueueFlags {
    pub const GRAPHICS: Self = Self(1 << 0);
    pub const COMPUTE: Self = Self(1 << 1);
    pub const TRANSFER: Self = Self(1 << 2);
    pub const SPARSE_BINDING: Self = Self(1 << 3);
    /// vk11
    pub const PROTECTED: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    QueueFlags : u32, 0x1f, GRAPHICS, COMPUTE, TRANSFER, SPARSE_BINDING, PROTECTED
}
#[doc(alias = "VkCullModeFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CullModeFlags(pub u32);
impl CullModeFlags {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(1 << 0);
    pub const BACK: Self = Self(1 << 1);
    pub const FRONT_AND_BACK: Self = Self(3);
}
crate::bitflags_impl! {
    CullModeFlags : u32, 0x3, NONE, FRONT, BACK, FRONT_AND_BACK
}
#[doc(alias = "VkRenderPassCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RenderPassCreateFlags(pub u32);
crate::bitflags_impl! {
    RenderPassCreateFlags : u32, 0x0,
}
#[doc(alias = "VkDeviceQueueCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceQueueCreateFlags(pub u32);
impl DeviceQueueCreateFlags {
    /// vk11
    pub const PROTECTED: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    DeviceQueueCreateFlags : u32, 0x1, PROTECTED
}
#[doc(alias = "VkMemoryPropertyFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryPropertyFlags(pub u32);
impl MemoryPropertyFlags {
    pub const DEVICE_LOCAL: Self = Self(1 << 0);
    pub const HOST_VISIBLE: Self = Self(1 << 1);
    pub const HOST_COHERENT: Self = Self(1 << 2);
    pub const HOST_CACHED: Self = Self(1 << 3);
    pub const LAZILY_ALLOCATED: Self = Self(1 << 4);
    /// vk11
    pub const PROTECTED: Self = Self(1 << 5);
}
crate::bitflags_impl! {
    MemoryPropertyFlags : u32, 0x3f, DEVICE_LOCAL, HOST_VISIBLE, HOST_COHERENT,
    HOST_CACHED, LAZILY_ALLOCATED, PROTECTED
}
#[doc(alias = "VkMemoryHeapFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryHeapFlags(pub u32);
impl MemoryHeapFlags {
    pub const DEVICE_LOCAL: Self = Self(1 << 0);
    /// vk11
    pub const MULTI_INSTANCE: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    MemoryHeapFlags : u32, 0x3, DEVICE_LOCAL, MULTI_INSTANCE
}
#[doc(alias = "VkAccessFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccessFlags(pub u32);
impl AccessFlags {
    pub const INDIRECT_COMMAND_READ: Self = Self(1 << 0);
    pub const INDEX_READ: Self = Self(1 << 1);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(1 << 2);
    pub const UNIFORM_READ: Self = Self(1 << 3);
    pub const INPUT_ATTACHMENT_READ: Self = Self(1 << 4);
    pub const SHADER_READ: Self = Self(1 << 5);
    pub const SHADER_WRITE: Self = Self(1 << 6);
    pub const COLOR_ATTACHMENT_READ: Self = Self(1 << 7);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(1 << 8);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(1 << 9);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1 << 10);
    pub const TRANSFER_READ: Self = Self(1 << 11);
    pub const TRANSFER_WRITE: Self = Self(1 << 12);
    pub const HOST_READ: Self = Self(1 << 13);
    pub const HOST_WRITE: Self = Self(1 << 14);
    pub const MEMORY_READ: Self = Self(1 << 15);
    pub const MEMORY_WRITE: Self = Self(1 << 16);
    /// khr_synchronization2
    pub const NONE_KHR: Self = Self(0);
}
crate::bitflags_impl! {
    AccessFlags : u32, 0x1ffff, INDIRECT_COMMAND_READ, INDEX_READ, VERTEX_ATTRIBUTE_READ,
    UNIFORM_READ, INPUT_ATTACHMENT_READ, SHADER_READ, SHADER_WRITE,
    COLOR_ATTACHMENT_READ, COLOR_ATTACHMENT_WRITE, DEPTH_STENCIL_ATTACHMENT_READ,
    DEPTH_STENCIL_ATTACHMENT_WRITE, TRANSFER_READ, TRANSFER_WRITE, HOST_READ, HOST_WRITE,
    MEMORY_READ, MEMORY_WRITE, NONE_KHR
}
#[doc(alias = "VkBufferUsageFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BufferUsageFlags(pub u32);
impl BufferUsageFlags {
    pub const TRANSFER_SRC: Self = Self(1 << 0);
    pub const TRANSFER_DST: Self = Self(1 << 1);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 2);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 3);
    pub const UNIFORM_BUFFER: Self = Self(1 << 4);
    pub const STORAGE_BUFFER: Self = Self(1 << 5);
    pub const INDEX_BUFFER: Self = Self(1 << 6);
    pub const VERTEX_BUFFER: Self = Self(1 << 7);
    pub const INDIRECT_BUFFER: Self = Self(1 << 8);
}
crate::bitflags_impl! {
    BufferUsageFlags : u32, 0x1ff, TRANSFER_SRC, TRANSFER_DST, UNIFORM_TEXEL_BUFFER,
    STORAGE_TEXEL_BUFFER, UNIFORM_BUFFER, STORAGE_BUFFER, INDEX_BUFFER, VERTEX_BUFFER,
    INDIRECT_BUFFER
}
#[doc(alias = "VkBufferCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BufferCreateFlags(pub u32);
impl BufferCreateFlags {
    pub const SPARSE_BINDING: Self = Self(1 << 0);
    pub const SPARSE_RESIDENCY: Self = Self(1 << 1);
    pub const SPARSE_ALIASED: Self = Self(1 << 2);
    /// vk11
    pub const PROTECTED: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    BufferCreateFlags : u32, 0xf, SPARSE_BINDING, SPARSE_RESIDENCY, SPARSE_ALIASED,
    PROTECTED
}
#[doc(alias = "VkShaderStageFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderStageFlags(pub u32);
impl ShaderStageFlags {
    pub const VERTEX: Self = Self(1 << 0);
    pub const TESSELLATION_CONTROL: Self = Self(1 << 1);
    pub const TESSELLATION_EVALUATION: Self = Self(1 << 2);
    pub const GEOMETRY: Self = Self(1 << 3);
    pub const FRAGMENT: Self = Self(1 << 4);
    pub const COMPUTE: Self = Self(1 << 5);
    pub const ALL_GRAPHICS: Self = Self(31);
    pub const ALL: Self = Self(2147483647);
}
crate::bitflags_impl! {
    ShaderStageFlags : u32, 0x7fffffff, VERTEX, TESSELLATION_CONTROL,
    TESSELLATION_EVALUATION, GEOMETRY, FRAGMENT, COMPUTE, ALL_GRAPHICS, ALL
}
#[doc(alias = "VkImageUsageFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageUsageFlags(pub u32);
impl ImageUsageFlags {
    pub const TRANSFER_SRC: Self = Self(1 << 0);
    pub const TRANSFER_DST: Self = Self(1 << 1);
    pub const SAMPLED: Self = Self(1 << 2);
    pub const STORAGE: Self = Self(1 << 3);
    pub const COLOR_ATTACHMENT: Self = Self(1 << 4);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 5);
    pub const TRANSIENT_ATTACHMENT: Self = Self(1 << 6);
    pub const INPUT_ATTACHMENT: Self = Self(1 << 7);
}
crate::bitflags_impl! {
    ImageUsageFlags : u32, 0xff, TRANSFER_SRC, TRANSFER_DST, SAMPLED, STORAGE,
    COLOR_ATTACHMENT, DEPTH_STENCIL_ATTACHMENT, TRANSIENT_ATTACHMENT, INPUT_ATTACHMENT
}
#[doc(alias = "VkImageCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageCreateFlags(pub u32);
impl ImageCreateFlags {
    pub const SPARSE_BINDING: Self = Self(1 << 0);
    pub const SPARSE_RESIDENCY: Self = Self(1 << 1);
    pub const SPARSE_ALIASED: Self = Self(1 << 2);
    pub const MUTABLE_FORMAT: Self = Self(1 << 3);
    pub const CUBE_COMPATIBLE: Self = Self(1 << 4);
    /// vk11
    pub const ALIAS: Self = Self(1 << 10);
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1 << 6);
    pub const C2D_ARRAY_COMPATIBLE: Self = Self(1 << 5);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(1 << 7);
    pub const EXTENDED_USAGE: Self = Self(1 << 8);
    pub const PROTECTED: Self = Self(1 << 11);
    pub const DISJOINT: Self = Self(1 << 9);
}
crate::bitflags_impl! {
    ImageCreateFlags : u32, 0xfff, SPARSE_BINDING, SPARSE_RESIDENCY, SPARSE_ALIASED,
    MUTABLE_FORMAT, CUBE_COMPATIBLE, ALIAS, SPLIT_INSTANCE_BIND_REGIONS,
    C2D_ARRAY_COMPATIBLE, BLOCK_TEXEL_VIEW_COMPATIBLE, EXTENDED_USAGE, PROTECTED,
    DISJOINT
}
#[doc(alias = "VkImageViewCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageViewCreateFlags(pub u32);
crate::bitflags_impl! {
    ImageViewCreateFlags : u32, 0x0,
}
#[doc(alias = "VkSamplerCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerCreateFlags(pub u32);
crate::bitflags_impl! {
    SamplerCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCreateFlags(pub u32);
impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION: Self = Self(1 << 0);
    pub const ALLOW_DERIVATIVES: Self = Self(1 << 1);
    pub const DERIVATIVE: Self = Self(1 << 2);
    /// vk11
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(1 << 3);
    pub const DISPATCH_BASE: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    PipelineCreateFlags : u32, 0x1f, DISABLE_OPTIMIZATION, ALLOW_DERIVATIVES, DERIVATIVE,
    VIEW_INDEX_FROM_DEVICE_INDEX, DISPATCH_BASE
}
#[doc(alias = "VkPipelineShaderStageCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineShaderStageCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineShaderStageCreateFlags : u32, 0x0,
}
#[doc(alias = "VkColorComponentFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ColorComponentFlags(pub u32);
impl ColorComponentFlags {
    pub const R: Self = Self(1 << 0);
    pub const G: Self = Self(1 << 1);
    pub const B: Self = Self(1 << 2);
    pub const A: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    ColorComponentFlags : u32, 0xf, R, G, B, A
}
#[doc(alias = "VkFenceCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FenceCreateFlags(pub u32);
impl FenceCreateFlags {
    pub const SIGNALED: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    FenceCreateFlags : u32, 0x1, SIGNALED
}
#[doc(alias = "VkFormatFeatureFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FormatFeatureFlags(pub u32);
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE: Self = Self(1 << 0);
    pub const STORAGE_IMAGE: Self = Self(1 << 1);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(1 << 2);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 3);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 4);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(1 << 5);
    pub const VERTEX_BUFFER: Self = Self(1 << 6);
    pub const COLOR_ATTACHMENT: Self = Self(1 << 7);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(1 << 8);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 9);
    pub const BLIT_SRC: Self = Self(1 << 10);
    pub const BLIT_DST: Self = Self(1 << 11);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(1 << 12);
    /// vk11
    pub const TRANSFER_SRC: Self = Self(1 << 14);
    pub const TRANSFER_DST: Self = Self(1 << 15);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(1 << 17);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(1 << 18);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(
        1 << 19,
    );
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(
        1 << 20,
    );
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self(
        1 << 21,
    );
    pub const DISJOINT: Self = Self(1 << 22);
    pub const COSITED_CHROMA_SAMPLES: Self = Self(1 << 23);
}
crate::bitflags_impl! {
    FormatFeatureFlags : u32, 0xfedfff, SAMPLED_IMAGE, STORAGE_IMAGE,
    STORAGE_IMAGE_ATOMIC, UNIFORM_TEXEL_BUFFER, STORAGE_TEXEL_BUFFER,
    STORAGE_TEXEL_BUFFER_ATOMIC, VERTEX_BUFFER, COLOR_ATTACHMENT, COLOR_ATTACHMENT_BLEND,
    DEPTH_STENCIL_ATTACHMENT, BLIT_SRC, BLIT_DST, SAMPLED_IMAGE_FILTER_LINEAR,
    TRANSFER_SRC, TRANSFER_DST, MIDPOINT_CHROMA_SAMPLES,
    SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER,
    SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER,
    SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT,
    SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE, DISJOINT,
    COSITED_CHROMA_SAMPLES
}
#[doc(alias = "VkQueryControlFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryControlFlags(pub u32);
impl QueryControlFlags {
    pub const PRECISE: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    QueryControlFlags : u32, 0x1, PRECISE
}
#[doc(alias = "VkQueryResultFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryResultFlags(pub u32);
impl QueryResultFlags {
    pub const R64: Self = Self(1 << 0);
    pub const WAIT: Self = Self(1 << 1);
    pub const WITH_AVAILABILITY: Self = Self(1 << 2);
    pub const PARTIAL: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    QueryResultFlags : u32, 0xf, R64, WAIT, WITH_AVAILABILITY, PARTIAL
}
#[doc(alias = "VkCommandBufferUsageFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferUsageFlags(pub u32);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = Self(1 << 0);
    pub const RENDER_PASS_CONTINUE: Self = Self(1 << 1);
    pub const SIMULTANEOUS_USE: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    CommandBufferUsageFlags : u32, 0x7, ONE_TIME_SUBMIT, RENDER_PASS_CONTINUE,
    SIMULTANEOUS_USE
}
#[doc(alias = "VkQueryPipelineStatisticFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryPipelineStatisticFlags(pub u32);
impl QueryPipelineStatisticFlags {
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1 << 0);
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(1 << 1);
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(1 << 2);
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(1 << 3);
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(1 << 4);
    pub const CLIPPING_INVOCATIONS: Self = Self(1 << 5);
    pub const CLIPPING_PRIMITIVES: Self = Self(1 << 6);
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(1 << 7);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(1 << 8);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(1 << 9);
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1 << 10);
}
crate::bitflags_impl! {
    QueryPipelineStatisticFlags : u32, 0x7ff, INPUT_ASSEMBLY_VERTICES,
    INPUT_ASSEMBLY_PRIMITIVES, VERTEX_SHADER_INVOCATIONS, GEOMETRY_SHADER_INVOCATIONS,
    GEOMETRY_SHADER_PRIMITIVES, CLIPPING_INVOCATIONS, CLIPPING_PRIMITIVES,
    FRAGMENT_SHADER_INVOCATIONS, TESSELLATION_CONTROL_SHADER_PATCHES,
    TESSELLATION_EVALUATION_SHADER_INVOCATIONS, COMPUTE_SHADER_INVOCATIONS
}
#[doc(alias = "VkImageAspectFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageAspectFlags(pub u32);
impl ImageAspectFlags {
    pub const COLOR: Self = Self(1 << 0);
    pub const DEPTH: Self = Self(1 << 1);
    pub const STENCIL: Self = Self(1 << 2);
    pub const METADATA: Self = Self(1 << 3);
    /// vk11
    pub const PLANE_0: Self = Self(1 << 4);
    pub const PLANE_1: Self = Self(1 << 5);
    pub const PLANE_2: Self = Self(1 << 6);
}
crate::bitflags_impl! {
    ImageAspectFlags : u32, 0x7f, COLOR, DEPTH, STENCIL, METADATA, PLANE_0, PLANE_1,
    PLANE_2
}
#[doc(alias = "VkSparseImageFormatFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SparseImageFormatFlags(pub u32);
impl SparseImageFormatFlags {
    pub const SINGLE_MIPTAIL: Self = Self(1 << 0);
    pub const ALIGNED_MIP_SIZE: Self = Self(1 << 1);
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    SparseImageFormatFlags : u32, 0x7, SINGLE_MIPTAIL, ALIGNED_MIP_SIZE,
    NONSTANDARD_BLOCK_SIZE
}
#[doc(alias = "VkSparseMemoryBindFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SparseMemoryBindFlags(pub u32);
impl SparseMemoryBindFlags {
    pub const METADATA: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    SparseMemoryBindFlags : u32, 0x1, METADATA
}
#[doc(alias = "VkPipelineStageFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineStageFlags(pub u32);
impl PipelineStageFlags {
    pub const TOP_OF_PIPE: Self = Self(1 << 0);
    pub const DRAW_INDIRECT: Self = Self(1 << 1);
    pub const VERTEX_INPUT: Self = Self(1 << 2);
    pub const VERTEX_SHADER: Self = Self(1 << 3);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(1 << 4);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(1 << 5);
    pub const GEOMETRY_SHADER: Self = Self(1 << 6);
    pub const FRAGMENT_SHADER: Self = Self(1 << 7);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(1 << 8);
    pub const LATE_FRAGMENT_TESTS: Self = Self(1 << 9);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1 << 10);
    pub const COMPUTE_SHADER: Self = Self(1 << 11);
    pub const TRANSFER: Self = Self(1 << 12);
    pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
    pub const HOST: Self = Self(1 << 14);
    pub const ALL_GRAPHICS: Self = Self(1 << 15);
    pub const ALL_COMMANDS: Self = Self(1 << 16);
    /// khr_synchronization2
    pub const NONE_KHR: Self = Self(0);
}
crate::bitflags_impl! {
    PipelineStageFlags : u32, 0x1ffff, TOP_OF_PIPE, DRAW_INDIRECT, VERTEX_INPUT,
    VERTEX_SHADER, TESSELLATION_CONTROL_SHADER, TESSELLATION_EVALUATION_SHADER,
    GEOMETRY_SHADER, FRAGMENT_SHADER, EARLY_FRAGMENT_TESTS, LATE_FRAGMENT_TESTS,
    COLOR_ATTACHMENT_OUTPUT, COMPUTE_SHADER, TRANSFER, BOTTOM_OF_PIPE, HOST,
    ALL_GRAPHICS, ALL_COMMANDS, NONE_KHR
}
#[doc(alias = "VkCommandPoolCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandPoolCreateFlags(pub u32);
impl CommandPoolCreateFlags {
    pub const TRANSIENT: Self = Self(1 << 0);
    pub const RESET_COMMAND_BUFFER: Self = Self(1 << 1);
    /// vk11
    pub const PROTECTED: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    CommandPoolCreateFlags : u32, 0x7, TRANSIENT, RESET_COMMAND_BUFFER, PROTECTED
}
#[doc(alias = "VkCommandPoolResetFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandPoolResetFlags(pub u32);
impl CommandPoolResetFlags {
    pub const RELEASE_RESOURCES: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    CommandPoolResetFlags : u32, 0x1, RELEASE_RESOURCES
}
#[doc(alias = "VkCommandBufferResetFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferResetFlags(pub u32);
impl CommandBufferResetFlags {
    pub const RELEASE_RESOURCES: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    CommandBufferResetFlags : u32, 0x1, RELEASE_RESOURCES
}
#[doc(alias = "VkSampleCountFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SampleCountFlags(pub u32);
impl SampleCountFlags {
    pub const C1: Self = Self(1 << 0);
    pub const C2: Self = Self(1 << 1);
    pub const C4: Self = Self(1 << 2);
    pub const C8: Self = Self(1 << 3);
    pub const C16: Self = Self(1 << 4);
    pub const C32: Self = Self(1 << 5);
    pub const C64: Self = Self(1 << 6);
}
crate::bitflags_impl! {
    SampleCountFlags : u32, 0x7f, C1, C2, C4, C8, C16, C32, C64
}
#[doc(alias = "VkAttachmentDescriptionFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentDescriptionFlags(pub u32);
impl AttachmentDescriptionFlags {
    pub const MAY_ALIAS: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    AttachmentDescriptionFlags : u32, 0x1, MAY_ALIAS
}
#[doc(alias = "VkStencilFaceFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StencilFaceFlags(pub u32);
impl StencilFaceFlags {
    pub const FRONT: Self = Self(1 << 0);
    pub const BACK: Self = Self(1 << 1);
    pub const FRONT_AND_BACK: Self = Self(3);
}
crate::bitflags_impl! {
    StencilFaceFlags : u32, 0x3, FRONT, BACK, FRONT_AND_BACK
}
#[doc(alias = "VkDescriptorPoolCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorPoolCreateFlags(pub u32);
impl DescriptorPoolCreateFlags {
    pub const FREE_DESCRIPTOR_SET: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    DescriptorPoolCreateFlags : u32, 0x1, FREE_DESCRIPTOR_SET
}
#[doc(alias = "VkDependencyFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DependencyFlags(pub u32);
impl DependencyFlags {
    pub const BY_REGION: Self = Self(1 << 0);
    /// vk11
    pub const DEVICE_GROUP: Self = Self(1 << 2);
    pub const VIEW_LOCAL: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    DependencyFlags : u32, 0x7, BY_REGION, DEVICE_GROUP, VIEW_LOCAL
}
#[doc(alias = "VkDescriptorSetLayoutCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorSetLayoutCreateFlags(pub u32);
crate::bitflags_impl! {
    DescriptorSetLayoutCreateFlags : u32, 0x0,
}
#[doc(alias = "VkSubpassDescriptionFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubpassDescriptionFlags(pub u32);
crate::bitflags_impl! {
    SubpassDescriptionFlags : u32, 0x0,
}
#[doc(alias = "VkVendorId")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVendorId.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VendorId(pub i32);
impl VendorId {
    pub const VIV: Self = Self(65537);
    pub const VSI: Self = Self(65538);
    pub const KAZAN: Self = Self(65539);
    pub const CODEPLAY: Self = Self(65540);
    pub const MESA: Self = Self(65541);
    pub const POCL: Self = Self(65542);
}
crate::enum_impl! {
    VendorId : i32, VIV, VSI, KAZAN, CODEPLAY, MESA, POCL
}
#[doc(alias = "VkFramebufferCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FramebufferCreateFlags(pub u32);
crate::bitflags_impl! {
    FramebufferCreateFlags : u32, 0x0,
}
#[doc(alias = "VkEventCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct EventCreateFlags(pub u32);
impl EventCreateFlags {
    /// khr_synchronization2
    pub const DEVICE_ONLY_KHR: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    EventCreateFlags : u32, 0x1, DEVICE_ONLY_KHR
}
#[doc(alias = "VkPipelineLayoutCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineLayoutCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineLayoutCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineColorBlendStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineColorBlendStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineColorBlendStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkPipelineDepthStencilStateCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineDepthStencilStateCreateFlags(pub u32);
crate::bitflags_impl! {
    PipelineDepthStencilStateCreateFlags : u32, 0x0,
}
#[doc(alias = "VkInstanceCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct InstanceCreateFlags(pub u32);
crate::bitflags_impl! {
    InstanceCreateFlags : u32, 0x0,
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateInstance")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
pub unsafe fn create_instance(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut Instance,
) -> Result {
    (crate::loader::tables::GLOBAL_ENTRY_TABLE
        .create_instance
        .unwrap())(p_create_info, p_allocator, p_instance)
}
#[cfg(feature = "wrappers")]
impl crate::EntryWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateInstance")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
    pub unsafe fn create_instance(
        &self,
        create_info: &InstanceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Instance> {
        let create_instance = (*self.table).create_instance.unwrap();
        let mut instance = Default::default();
        let result = create_instance(
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut instance,
        );
        crate::new_result(instance, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyInstance")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
pub unsafe fn destroy_instance(
    instance: Instance,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .destroy_instance
        .unwrap())(instance, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyInstance")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
    pub unsafe fn destroy_instance(&self, allocator: Option<&AllocationCallbacks>) {
        let destroy_instance = (*self.table).destroy_instance.unwrap();
        destroy_instance(
            self.handle,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumeratePhysicalDevices")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
pub unsafe fn enumerate_physical_devices(
    instance: Instance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut PhysicalDevice,
) -> Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .enumerate_physical_devices
        .unwrap())(instance, p_physical_device_count, p_physical_devices)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
    pub unsafe fn enumerate_physical_devices(
        &self,
        physical_device_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<PhysicalDevice>, Result)> {
        let enumerate_physical_devices = (*self.table)
            .enumerate_physical_devices
            .unwrap();
        let mut physical_device_count = match physical_device_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_physical_devices(self.handle, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut physical_devices = vec![
            Default::default(); physical_device_count as usize
        ];
        let result = enumerate_physical_devices(
            self.handle,
            &mut physical_device_count,
            physical_devices.as_mut_ptr(),
        );
        crate::new_result((physical_devices, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceProcAddr")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
pub unsafe fn get_device_proc_addr(
    device: Device,
    p_name: *const std::os::raw::c_char,
) -> PfnVoidFunction {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_proc_addr
        .unwrap())(device, p_name)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceProcAddr")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
    pub unsafe fn get_device_proc_addr(&self, name: Option<&std::ffi::CStr>) {
        let get_device_proc_addr = (*self.table).get_device_proc_addr.unwrap();
        get_device_proc_addr(
            self.handle,
            match name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetInstanceProcAddr")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
pub unsafe fn get_instance_proc_addr(
    instance: Instance,
    p_name: *const std::os::raw::c_char,
) -> PfnVoidFunction {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_instance_proc_addr
        .unwrap())(instance, p_name)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetInstanceProcAddr")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
    pub unsafe fn get_instance_proc_addr(&self, name: Option<&std::ffi::CStr>) {
        let get_instance_proc_addr = (*self.table).get_instance_proc_addr.unwrap();
        get_instance_proc_addr(
            self.handle,
            match name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
pub unsafe fn get_physical_device_properties(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_properties
        .unwrap())(physical_device, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties {
        let get_physical_device_properties = (*self.table)
            .get_physical_device_properties
            .unwrap();
        let mut properties = Default::default();
        get_physical_device_properties(physical_device, &mut properties);
        properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
pub unsafe fn get_physical_device_queue_family_properties(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_queue_family_properties
        .unwrap())(
        physical_device,
        p_queue_family_property_count,
        p_queue_family_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        queue_family_property_count: Option<u32>,
    ) -> Vec<QueueFamilyProperties> {
        let get_physical_device_queue_family_properties = (*self.table)
            .get_physical_device_queue_family_properties
            .unwrap();
        let mut queue_family_property_count = match queue_family_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_queue_family_properties(
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
        get_physical_device_queue_family_properties(
            physical_device,
            &mut queue_family_property_count,
            queue_family_properties.as_mut_ptr(),
        );
        queue_family_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
pub unsafe fn get_physical_device_memory_properties(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_memory_properties
        .unwrap())(physical_device, p_memory_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties {
        let get_physical_device_memory_properties = (*self.table)
            .get_physical_device_memory_properties
            .unwrap();
        let mut memory_properties = Default::default();
        get_physical_device_memory_properties(physical_device, &mut memory_properties);
        memory_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
pub unsafe fn get_physical_device_features(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_features
        .unwrap())(physical_device, p_features)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        let get_physical_device_features = (*self.table)
            .get_physical_device_features
            .unwrap();
        let mut features = Default::default();
        get_physical_device_features(physical_device, &mut features);
        features
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
pub unsafe fn get_physical_device_format_properties(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_format_properties
        .unwrap())(physical_device, format, p_format_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
    ) -> FormatProperties {
        let get_physical_device_format_properties = (*self.table)
            .get_physical_device_format_properties
            .unwrap();
        let mut format_properties = Default::default();
        get_physical_device_format_properties(
            physical_device,
            format as _,
            &mut format_properties,
        );
        format_properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
pub unsafe fn get_physical_device_image_format_properties(
    physical_device: PhysicalDevice,
    format: Format,
    kind: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    p_image_format_properties: *mut ImageFormatProperties,
) -> Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_image_format_properties
        .unwrap())(
        physical_device,
        format,
        kind,
        tiling,
        usage,
        flags,
        p_image_format_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        kind: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: Option<ImageCreateFlags>,
    ) -> crate::VulkanResult<ImageFormatProperties> {
        let get_physical_device_image_format_properties = (*self.table)
            .get_physical_device_image_format_properties
            .unwrap();
        let mut image_format_properties = Default::default();
        let result = get_physical_device_image_format_properties(
            physical_device,
            format as _,
            kind as _,
            tiling as _,
            usage as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            &mut image_format_properties,
        );
        crate::new_result(image_format_properties, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDevice")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
pub unsafe fn create_device(
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_device: *mut Device,
) -> Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_device
        .unwrap())(physical_device, p_create_info, p_allocator, p_device)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDevice")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        create_info: &DeviceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Device> {
        let create_device = (*self.table).create_device.unwrap();
        let mut device = Default::default();
        let result = create_device(
            physical_device,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut device,
        );
        crate::new_result(device, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDevice")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
pub unsafe fn destroy_device(device: Device, p_allocator: *const AllocationCallbacks) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_device
        .unwrap())(device, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDevice")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
    pub unsafe fn destroy_device(&self, allocator: Option<&AllocationCallbacks>) {
        let destroy_device = (*self.table).destroy_device.unwrap();
        destroy_device(
            self.handle,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
pub unsafe fn enumerate_instance_layer_properties(
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result {
    (crate::loader::tables::GLOBAL_ENTRY_TABLE
        .enumerate_instance_layer_properties
        .unwrap())(p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::EntryWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumerateInstanceLayerProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<LayerProperties>, Result)> {
        let enumerate_instance_layer_properties = (*self.table)
            .enumerate_instance_layer_properties
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_instance_layer_properties(&mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = enumerate_instance_layer_properties(
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
pub unsafe fn enumerate_instance_extension_properties(
    p_layer_name: *const std::os::raw::c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result {
    (crate::loader::tables::GLOBAL_ENTRY_TABLE
        .enumerate_instance_extension_properties
        .unwrap())(p_layer_name, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::EntryWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumerateInstanceExtensionProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&std::ffi::CStr>,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<ExtensionProperties>, Result)> {
        let enumerate_instance_extension_properties = (*self.table)
            .enumerate_instance_extension_properties
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_instance_extension_properties(
                    match layer_name {
                        Some(v) => v.as_ptr(),
                        None => std::ptr::null(),
                    },
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = enumerate_instance_extension_properties(
            match layer_name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumerateDeviceLayerProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
pub unsafe fn enumerate_device_layer_properties(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .enumerate_device_layer_properties
        .unwrap())(physical_device, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<LayerProperties>, Result)> {
        let enumerate_device_layer_properties = (*self.table)
            .enumerate_device_layer_properties
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_device_layer_properties(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = enumerate_device_layer_properties(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumerateDeviceExtensionProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
pub unsafe fn enumerate_device_extension_properties(
    physical_device: PhysicalDevice,
    p_layer_name: *const std::os::raw::c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .enumerate_device_extension_properties
        .unwrap())(physical_device, p_layer_name, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&std::ffi::CStr>,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<ExtensionProperties>, Result)> {
        let enumerate_device_extension_properties = (*self.table)
            .enumerate_device_extension_properties
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_device_extension_properties(
                    physical_device,
                    match layer_name {
                        Some(v) => v.as_ptr(),
                        None => std::ptr::null(),
                    },
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = enumerate_device_extension_properties(
            physical_device,
            match layer_name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceQueue")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
pub unsafe fn get_device_queue(
    device: Device,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Queue,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_queue
        .unwrap())(device, queue_family_index, queue_index, p_queue)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceQueue")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
    pub unsafe fn get_device_queue(
        &self,
        queue_family_index: u32,
        queue_index: u32,
    ) -> Queue {
        let get_device_queue = (*self.table).get_device_queue.unwrap();
        let mut queue = Default::default();
        get_device_queue(
            self.handle,
            queue_family_index as _,
            queue_index as _,
            &mut queue,
        );
        queue
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueSubmit")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
pub unsafe fn queue_submit(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo,
    fence: Fence,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_submit
        .unwrap())(queue, submit_count, p_submits, fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueSubmit")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submits: &[SubmitInfo],
        fence: Fence,
    ) -> crate::VulkanResult<()> {
        let queue_submit = (*self.table).queue_submit.unwrap();
        let submit_count = submits.len();
        let result = queue_submit(queue, submit_count as _, submits.as_ptr(), fence);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueWaitIdle")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
pub unsafe fn queue_wait_idle(queue: Queue) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.queue_wait_idle.unwrap())(queue)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueWaitIdle")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> crate::VulkanResult<()> {
        let queue_wait_idle = (*self.table).queue_wait_idle.unwrap();
        let result = queue_wait_idle(queue);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDeviceWaitIdle")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
pub unsafe fn device_wait_idle(device: Device) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.device_wait_idle.unwrap())(device)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkDeviceWaitIdle")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
    pub unsafe fn device_wait_idle(&self) -> crate::VulkanResult<()> {
        let device_wait_idle = (*self.table).device_wait_idle.unwrap();
        let result = device_wait_idle(self.handle);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAllocateMemory")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
pub unsafe fn allocate_memory(
    device: Device,
    p_allocate_info: *const MemoryAllocateInfo,
    p_allocator: *const AllocationCallbacks,
    p_memory: *mut DeviceMemory,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .allocate_memory
        .unwrap())(device, p_allocate_info, p_allocator, p_memory)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAllocateMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
    pub unsafe fn allocate_memory(
        &self,
        allocate_info: &MemoryAllocateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<DeviceMemory> {
        let allocate_memory = (*self.table).allocate_memory.unwrap();
        let mut memory = Default::default();
        let result = allocate_memory(
            self.handle,
            allocate_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut memory,
        );
        crate::new_result(memory, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkFreeMemory")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
pub unsafe fn free_memory(
    device: Device,
    memory: DeviceMemory,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .free_memory
        .unwrap())(device, memory, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkFreeMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
    pub unsafe fn free_memory(
        &self,
        memory: DeviceMemory,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let free_memory = (*self.table).free_memory.unwrap();
        free_memory(
            self.handle,
            memory,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkMapMemory")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
pub unsafe fn map_memory(
    device: Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut std::os::raw::c_void,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .map_memory
        .unwrap())(device, memory, offset, size, flags, pp_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkMapMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
    pub unsafe fn map_memory(
        &self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: Option<MemoryMapFlags>,
        data: &mut *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<()> {
        let map_memory = (*self.table).map_memory.unwrap();
        let result = map_memory(
            self.handle,
            memory,
            offset as _,
            size as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            data as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkUnmapMemory")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
pub unsafe fn unmap_memory(device: Device, memory: DeviceMemory) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.unmap_memory.unwrap())(device, memory)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkUnmapMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
    pub unsafe fn unmap_memory(&self, memory: DeviceMemory) {
        let unmap_memory = (*self.table).unmap_memory.unwrap();
        unmap_memory(self.handle, memory);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkFlushMappedMemoryRanges")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
pub unsafe fn flush_mapped_memory_ranges(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .flush_mapped_memory_ranges
        .unwrap())(device, memory_range_count, p_memory_ranges)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::VulkanResult<()> {
        let flush_mapped_memory_ranges = (*self.table)
            .flush_mapped_memory_ranges
            .unwrap();
        let memory_range_count = memory_ranges.len();
        let result = flush_mapped_memory_ranges(
            self.handle,
            memory_range_count as _,
            memory_ranges.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkInvalidateMappedMemoryRanges")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
pub unsafe fn invalidate_mapped_memory_ranges(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .invalidate_mapped_memory_ranges
        .unwrap())(device, memory_range_count, p_memory_ranges)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::VulkanResult<()> {
        let invalidate_mapped_memory_ranges = (*self.table)
            .invalidate_mapped_memory_ranges
            .unwrap();
        let memory_range_count = memory_ranges.len();
        let result = invalidate_mapped_memory_ranges(
            self.handle,
            memory_range_count as _,
            memory_ranges.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceMemoryCommitment")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
pub unsafe fn get_device_memory_commitment(
    device: Device,
    memory: DeviceMemory,
    p_committed_memory_in_bytes: *mut DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_memory_commitment
        .unwrap())(device, memory, p_committed_memory_in_bytes)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
    pub unsafe fn get_device_memory_commitment(
        &self,
        memory: DeviceMemory,
    ) -> DeviceSize {
        let get_device_memory_commitment = (*self.table)
            .get_device_memory_commitment
            .unwrap();
        let mut committed_memory_in_bytes = Default::default();
        get_device_memory_commitment(
            self.handle,
            memory,
            &mut committed_memory_in_bytes,
        );
        committed_memory_in_bytes
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
pub unsafe fn get_buffer_memory_requirements(
    device: Device,
    buffer: Buffer,
    p_memory_requirements: *mut MemoryRequirements,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_memory_requirements
        .unwrap())(device, buffer, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        buffer: Buffer,
    ) -> MemoryRequirements {
        let get_buffer_memory_requirements = (*self.table)
            .get_buffer_memory_requirements
            .unwrap();
        let mut memory_requirements = Default::default();
        get_buffer_memory_requirements(self.handle, buffer, &mut memory_requirements);
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindBufferMemory")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
pub unsafe fn bind_buffer_memory(
    device: Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_buffer_memory
        .unwrap())(device, buffer, memory, memory_offset)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindBufferMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::VulkanResult<()> {
        let bind_buffer_memory = (*self.table).bind_buffer_memory.unwrap();
        let result = bind_buffer_memory(self.handle, buffer, memory, memory_offset as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
pub unsafe fn get_image_memory_requirements(
    device: Device,
    image: Image,
    p_memory_requirements: *mut MemoryRequirements,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_memory_requirements
        .unwrap())(device, image, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
    pub unsafe fn get_image_memory_requirements(
        &self,
        image: Image,
    ) -> MemoryRequirements {
        let get_image_memory_requirements = (*self.table)
            .get_image_memory_requirements
            .unwrap();
        let mut memory_requirements = Default::default();
        get_image_memory_requirements(self.handle, image, &mut memory_requirements);
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindImageMemory")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
pub unsafe fn bind_image_memory(
    device: Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_image_memory
        .unwrap())(device, image, memory, memory_offset)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindImageMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
    pub unsafe fn bind_image_memory(
        &self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::VulkanResult<()> {
        let bind_image_memory = (*self.table).bind_image_memory.unwrap();
        let result = bind_image_memory(self.handle, image, memory, memory_offset as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageSparseMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
pub unsafe fn get_image_sparse_memory_requirements(
    device: Device,
    image: Image,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_sparse_memory_requirements
        .unwrap())(
        device,
        image,
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        image: Image,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<SparseImageMemoryRequirements> {
        let get_image_sparse_memory_requirements = (*self.table)
            .get_image_sparse_memory_requirements
            .unwrap();
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_image_sparse_memory_requirements(
                    self.handle,
                    image,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut sparse_memory_requirements = vec![
            Default::default(); sparse_memory_requirement_count as usize
        ];
        get_image_sparse_memory_requirements(
            self.handle,
            image,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
pub unsafe fn get_physical_device_sparse_image_format_properties(
    physical_device: PhysicalDevice,
    format: Format,
    kind: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_sparse_image_format_properties
        .unwrap())(
        physical_device,
        format,
        kind,
        samples,
        usage,
        tiling,
        p_property_count,
        p_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        kind: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        property_count: Option<u32>,
    ) -> Vec<SparseImageFormatProperties> {
        let get_physical_device_sparse_image_format_properties = (*self.table)
            .get_physical_device_sparse_image_format_properties
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_sparse_image_format_properties(
                    physical_device,
                    format as _,
                    kind as _,
                    samples as _,
                    usage as _,
                    tiling as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        get_physical_device_sparse_image_format_properties(
            physical_device,
            format as _,
            kind as _,
            samples as _,
            usage as _,
            tiling as _,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        properties
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueBindSparse")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
pub unsafe fn queue_bind_sparse(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo,
    fence: Fence,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_bind_sparse
        .unwrap())(queue, bind_info_count, p_bind_info, fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueBindSparse")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info: &[BindSparseInfo],
        fence: Fence,
    ) -> crate::VulkanResult<()> {
        let queue_bind_sparse = (*self.table).queue_bind_sparse.unwrap();
        let bind_info_count = bind_info.len();
        let result = queue_bind_sparse(
            queue,
            bind_info_count as _,
            bind_info.as_ptr(),
            fence,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateFence")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
pub unsafe fn create_fence(
    device: Device,
    p_create_info: *const FenceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_fence
        .unwrap())(device, p_create_info, p_allocator, p_fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateFence")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
    pub unsafe fn create_fence(
        &self,
        create_info: &FenceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Fence> {
        let create_fence = (*self.table).create_fence.unwrap();
        let mut fence = Default::default();
        let result = create_fence(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut fence,
        );
        crate::new_result(fence, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyFence")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
pub unsafe fn destroy_fence(
    device: Device,
    fence: Fence,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_fence
        .unwrap())(device, fence, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyFence")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
    pub unsafe fn destroy_fence(
        &self,
        fence: Fence,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_fence = (*self.table).destroy_fence.unwrap();
        destroy_fence(
            self.handle,
            fence,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetFences")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
pub unsafe fn reset_fences(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .reset_fences
        .unwrap())(device, fence_count, p_fences)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkResetFences")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
    pub unsafe fn reset_fences(&self, fences: &[Fence]) -> crate::VulkanResult<()> {
        let reset_fences = (*self.table).reset_fences.unwrap();
        let fence_count = fences.len();
        let result = reset_fences(self.handle, fence_count as _, fences.as_ptr());
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetFenceStatus")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
pub unsafe fn get_fence_status(device: Device, fence: Fence) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.get_fence_status.unwrap())(device, fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetFenceStatus")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
    pub unsafe fn get_fence_status(&self, fence: Fence) -> crate::VulkanResult<Result> {
        let get_fence_status = (*self.table).get_fence_status.unwrap();
        let result = get_fence_status(self.handle, fence);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWaitForFences")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
pub unsafe fn wait_for_fences(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .wait_for_fences
        .unwrap())(device, fence_count, p_fences, wait_all, timeout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkWaitForFences")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
    pub unsafe fn wait_for_fences(
        &self,
        fences: &[Fence],
        wait_all: bool,
        timeout: u64,
    ) -> crate::VulkanResult<Result> {
        let wait_for_fences = (*self.table).wait_for_fences.unwrap();
        let fence_count = fences.len();
        let result = wait_for_fences(
            self.handle,
            fence_count as _,
            fences.as_ptr(),
            wait_all as _,
            timeout as _,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateSemaphore")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
pub unsafe fn create_semaphore(
    device: Device,
    p_create_info: *const SemaphoreCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_semaphore: *mut Semaphore,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_semaphore
        .unwrap())(device, p_create_info, p_allocator, p_semaphore)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateSemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
    pub unsafe fn create_semaphore(
        &self,
        create_info: &SemaphoreCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Semaphore> {
        let create_semaphore = (*self.table).create_semaphore.unwrap();
        let mut semaphore = Default::default();
        let result = create_semaphore(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut semaphore,
        );
        crate::new_result(semaphore, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroySemaphore")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
pub unsafe fn destroy_semaphore(
    device: Device,
    semaphore: Semaphore,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_semaphore
        .unwrap())(device, semaphore, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroySemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
    pub unsafe fn destroy_semaphore(
        &self,
        semaphore: Semaphore,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_semaphore = (*self.table).destroy_semaphore.unwrap();
        destroy_semaphore(
            self.handle,
            semaphore,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateEvent")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
pub unsafe fn create_event(
    device: Device,
    p_create_info: *const EventCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_event: *mut Event,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_event
        .unwrap())(device, p_create_info, p_allocator, p_event)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
    pub unsafe fn create_event(
        &self,
        create_info: &EventCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Event> {
        let create_event = (*self.table).create_event.unwrap();
        let mut event = Default::default();
        let result = create_event(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut event,
        );
        crate::new_result(event, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyEvent")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
pub unsafe fn destroy_event(
    device: Device,
    event: Event,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_event
        .unwrap())(device, event, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
    pub unsafe fn destroy_event(
        &self,
        event: Event,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_event = (*self.table).destroy_event.unwrap();
        destroy_event(
            self.handle,
            event,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetEventStatus")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
pub unsafe fn get_event_status(device: Device, event: Event) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.get_event_status.unwrap())(device, event)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetEventStatus")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
    pub unsafe fn get_event_status(&self, event: Event) -> crate::VulkanResult<Result> {
        let get_event_status = (*self.table).get_event_status.unwrap();
        let result = get_event_status(self.handle, event);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetEvent")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
pub unsafe fn set_event(device: Device, event: Event) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.set_event.unwrap())(device, event)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
    pub unsafe fn set_event(&self, event: Event) -> crate::VulkanResult<()> {
        let set_event = (*self.table).set_event.unwrap();
        let result = set_event(self.handle, event);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetEvent")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
pub unsafe fn reset_event(device: Device, event: Event) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE.reset_event.unwrap())(device, event)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkResetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
    pub unsafe fn reset_event(&self, event: Event) -> crate::VulkanResult<()> {
        let reset_event = (*self.table).reset_event.unwrap();
        let result = reset_event(self.handle, event);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateQueryPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
pub unsafe fn create_query_pool(
    device: Device,
    p_create_info: *const QueryPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_query_pool: *mut QueryPool,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_query_pool
        .unwrap())(device, p_create_info, p_allocator, p_query_pool)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
    pub unsafe fn create_query_pool(
        &self,
        create_info: &QueryPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<QueryPool> {
        let create_query_pool = (*self.table).create_query_pool.unwrap();
        let mut query_pool = Default::default();
        let result = create_query_pool(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut query_pool,
        );
        crate::new_result(query_pool, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyQueryPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
pub unsafe fn destroy_query_pool(
    device: Device,
    query_pool: QueryPool,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_query_pool
        .unwrap())(device, query_pool, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
    pub unsafe fn destroy_query_pool(
        &self,
        query_pool: QueryPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_query_pool = (*self.table).destroy_query_pool.unwrap();
        destroy_query_pool(
            self.handle,
            query_pool,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetQueryPoolResults")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
pub unsafe fn get_query_pool_results(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut std::os::raw::c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_query_pool_results
        .unwrap())(
        device,
        query_pool,
        first_query,
        query_count,
        data_size,
        p_data,
        stride,
        flags,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetQueryPoolResults")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
    pub unsafe fn get_query_pool_results(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        data: *mut std::os::raw::c_void,
        stride: DeviceSize,
        flags: Option<QueryResultFlags>,
    ) -> crate::VulkanResult<Result> {
        let get_query_pool_results = (*self.table).get_query_pool_results.unwrap();
        let result = get_query_pool_results(
            self.handle,
            query_pool,
            first_query as _,
            query_count as _,
            data_size,
            data,
            stride as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
pub unsafe fn create_buffer(
    device: Device,
    p_create_info: *const BufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_buffer: *mut Buffer,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_buffer
        .unwrap())(device, p_create_info, p_allocator, p_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
    pub unsafe fn create_buffer(
        &self,
        create_info: &BufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Buffer> {
        let create_buffer = (*self.table).create_buffer.unwrap();
        let mut buffer = Default::default();
        let result = create_buffer(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut buffer,
        );
        crate::new_result(buffer, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
pub unsafe fn destroy_buffer(
    device: Device,
    buffer: Buffer,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_buffer
        .unwrap())(device, buffer, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
    pub unsafe fn destroy_buffer(
        &self,
        buffer: Buffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_buffer = (*self.table).destroy_buffer.unwrap();
        destroy_buffer(
            self.handle,
            buffer,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateBufferView")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
pub unsafe fn create_buffer_view(
    device: Device,
    p_create_info: *const BufferViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut BufferView,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_buffer_view
        .unwrap())(device, p_create_info, p_allocator, p_view)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateBufferView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
    pub unsafe fn create_buffer_view(
        &self,
        create_info: &BufferViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<BufferView> {
        let create_buffer_view = (*self.table).create_buffer_view.unwrap();
        let mut view = Default::default();
        let result = create_buffer_view(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut view,
        );
        crate::new_result(view, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyBufferView")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
pub unsafe fn destroy_buffer_view(
    device: Device,
    buffer_view: BufferView,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_buffer_view
        .unwrap())(device, buffer_view, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyBufferView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
    pub unsafe fn destroy_buffer_view(
        &self,
        buffer_view: BufferView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_buffer_view = (*self.table).destroy_buffer_view.unwrap();
        destroy_buffer_view(
            self.handle,
            buffer_view,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
pub unsafe fn create_image(
    device: Device,
    p_create_info: *const ImageCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_image: *mut Image,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_image
        .unwrap())(device, p_create_info, p_allocator, p_image)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
    pub unsafe fn create_image(
        &self,
        create_info: &ImageCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Image> {
        let create_image = (*self.table).create_image.unwrap();
        let mut image = Default::default();
        let result = create_image(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut image,
        );
        crate::new_result(image, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
pub unsafe fn destroy_image(
    device: Device,
    image: Image,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_image
        .unwrap())(device, image, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
    pub unsafe fn destroy_image(
        &self,
        image: Image,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_image = (*self.table).destroy_image.unwrap();
        destroy_image(
            self.handle,
            image,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageSubresourceLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
pub unsafe fn get_image_subresource_layout(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_subresource_layout
        .unwrap())(device, image, p_subresource, p_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageSubresourceLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        let get_image_subresource_layout = (*self.table)
            .get_image_subresource_layout
            .unwrap();
        let mut layout = Default::default();
        get_image_subresource_layout(self.handle, image, subresource as _, &mut layout);
        layout
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateImageView")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
pub unsafe fn create_image_view(
    device: Device,
    p_create_info: *const ImageViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut ImageView,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_image_view
        .unwrap())(device, p_create_info, p_allocator, p_view)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateImageView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
    pub unsafe fn create_image_view(
        &self,
        create_info: &ImageViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<ImageView> {
        let create_image_view = (*self.table).create_image_view.unwrap();
        let mut view = Default::default();
        let result = create_image_view(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut view,
        );
        crate::new_result(view, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyImageView")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
pub unsafe fn destroy_image_view(
    device: Device,
    image_view: ImageView,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_image_view
        .unwrap())(device, image_view, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyImageView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
    pub unsafe fn destroy_image_view(
        &self,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_image_view = (*self.table).destroy_image_view.unwrap();
        destroy_image_view(
            self.handle,
            image_view,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateShaderModule")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
pub unsafe fn create_shader_module(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_shader_module: *mut ShaderModule,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_shader_module
        .unwrap())(device, p_create_info, p_allocator, p_shader_module)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateShaderModule")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
    pub unsafe fn create_shader_module(
        &self,
        create_info: &ShaderModuleCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<ShaderModule> {
        let create_shader_module = (*self.table).create_shader_module.unwrap();
        let mut shader_module = Default::default();
        let result = create_shader_module(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut shader_module,
        );
        crate::new_result(shader_module, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyShaderModule")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
pub unsafe fn destroy_shader_module(
    device: Device,
    shader_module: ShaderModule,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_shader_module
        .unwrap())(device, shader_module, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyShaderModule")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
    pub unsafe fn destroy_shader_module(
        &self,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_shader_module = (*self.table).destroy_shader_module.unwrap();
        destroy_shader_module(
            self.handle,
            shader_module,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreatePipelineCache")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
pub unsafe fn create_pipeline_cache(
    device: Device,
    p_create_info: *const PipelineCacheCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_cache: *mut PipelineCache,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_pipeline_cache
        .unwrap())(device, p_create_info, p_allocator, p_pipeline_cache)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreatePipelineCache")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
    pub unsafe fn create_pipeline_cache(
        &self,
        create_info: &PipelineCacheCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<PipelineCache> {
        let create_pipeline_cache = (*self.table).create_pipeline_cache.unwrap();
        let mut pipeline_cache = Default::default();
        let result = create_pipeline_cache(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut pipeline_cache,
        );
        crate::new_result(pipeline_cache, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyPipelineCache")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
pub unsafe fn destroy_pipeline_cache(
    device: Device,
    pipeline_cache: PipelineCache,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_pipeline_cache
        .unwrap())(device, pipeline_cache, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyPipelineCache")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
    pub unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: PipelineCache,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_pipeline_cache = (*self.table).destroy_pipeline_cache.unwrap();
        destroy_pipeline_cache(
            self.handle,
            pipeline_cache,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPipelineCacheData")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
pub unsafe fn get_pipeline_cache_data(
    device: Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut std::os::raw::c_void,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_pipeline_cache_data
        .unwrap())(device, pipeline_cache, p_data_size, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPipelineCacheData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
    pub unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: PipelineCache,
        data_size: *mut usize,
        data: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<Result> {
        let get_pipeline_cache_data = (*self.table).get_pipeline_cache_data.unwrap();
        let result = get_pipeline_cache_data(
            self.handle,
            pipeline_cache,
            data_size,
            data,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkMergePipelineCaches")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
pub unsafe fn merge_pipeline_caches(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .merge_pipeline_caches
        .unwrap())(device, dst_cache, src_cache_count, p_src_caches)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkMergePipelineCaches")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
    pub unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> crate::VulkanResult<()> {
        let merge_pipeline_caches = (*self.table).merge_pipeline_caches.unwrap();
        let src_cache_count = src_caches.len();
        let result = merge_pipeline_caches(
            self.handle,
            dst_cache,
            src_cache_count as _,
            src_caches.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateGraphicsPipelines")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
pub unsafe fn create_graphics_pipelines(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_graphics_pipelines
        .unwrap())(
        device,
        pipeline_cache,
        create_info_count,
        p_create_infos,
        p_allocator,
        p_pipelines,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateGraphicsPipelines")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
    pub unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<(Vec<Pipeline>, Result)> {
        let create_graphics_pipelines = (*self.table).create_graphics_pipelines.unwrap();
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as usize];
        let result = create_graphics_pipelines(
            self.handle,
            pipeline_cache,
            create_info_count as _,
            create_infos.as_ptr(),
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            pipelines.as_mut_ptr(),
        );
        crate::new_result((pipelines, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateComputePipelines")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
pub unsafe fn create_compute_pipelines(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_compute_pipelines
        .unwrap())(
        device,
        pipeline_cache,
        create_info_count,
        p_create_infos,
        p_allocator,
        p_pipelines,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateComputePipelines")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
    pub unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<(Vec<Pipeline>, Result)> {
        let create_compute_pipelines = (*self.table).create_compute_pipelines.unwrap();
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as usize];
        let result = create_compute_pipelines(
            self.handle,
            pipeline_cache,
            create_info_count as _,
            create_infos.as_ptr(),
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            pipelines.as_mut_ptr(),
        );
        crate::new_result((pipelines, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyPipeline")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
pub unsafe fn destroy_pipeline(
    device: Device,
    pipeline: Pipeline,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_pipeline
        .unwrap())(device, pipeline, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyPipeline")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
    pub unsafe fn destroy_pipeline(
        &self,
        pipeline: Pipeline,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_pipeline = (*self.table).destroy_pipeline.unwrap();
        destroy_pipeline(
            self.handle,
            pipeline,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreatePipelineLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
pub unsafe fn create_pipeline_layout(
    device: Device,
    p_create_info: *const PipelineLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_layout: *mut PipelineLayout,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_pipeline_layout
        .unwrap())(device, p_create_info, p_allocator, p_pipeline_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreatePipelineLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
    pub unsafe fn create_pipeline_layout(
        &self,
        create_info: &PipelineLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<PipelineLayout> {
        let create_pipeline_layout = (*self.table).create_pipeline_layout.unwrap();
        let mut pipeline_layout = Default::default();
        let result = create_pipeline_layout(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut pipeline_layout,
        );
        crate::new_result(pipeline_layout, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyPipelineLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
pub unsafe fn destroy_pipeline_layout(
    device: Device,
    pipeline_layout: PipelineLayout,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_pipeline_layout
        .unwrap())(device, pipeline_layout, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyPipelineLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
    pub unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_pipeline_layout = (*self.table).destroy_pipeline_layout.unwrap();
        destroy_pipeline_layout(
            self.handle,
            pipeline_layout,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateSampler")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
pub unsafe fn create_sampler(
    device: Device,
    p_create_info: *const SamplerCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_sampler: *mut Sampler,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_sampler
        .unwrap())(device, p_create_info, p_allocator, p_sampler)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateSampler")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
    pub unsafe fn create_sampler(
        &self,
        create_info: &SamplerCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Sampler> {
        let create_sampler = (*self.table).create_sampler.unwrap();
        let mut sampler = Default::default();
        let result = create_sampler(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut sampler,
        );
        crate::new_result(sampler, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroySampler")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
pub unsafe fn destroy_sampler(
    device: Device,
    sampler: Sampler,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_sampler
        .unwrap())(device, sampler, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroySampler")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
    pub unsafe fn destroy_sampler(
        &self,
        sampler: Sampler,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_sampler = (*self.table).destroy_sampler.unwrap();
        destroy_sampler(
            self.handle,
            sampler,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDescriptorSetLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
pub unsafe fn create_descriptor_set_layout(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_set_layout: *mut DescriptorSetLayout,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_descriptor_set_layout
        .unwrap())(device, p_create_info, p_allocator, p_set_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
    pub unsafe fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<DescriptorSetLayout> {
        let create_descriptor_set_layout = (*self.table)
            .create_descriptor_set_layout
            .unwrap();
        let mut set_layout = Default::default();
        let result = create_descriptor_set_layout(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut set_layout,
        );
        crate::new_result(set_layout, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDescriptorSetLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
pub unsafe fn destroy_descriptor_set_layout(
    device: Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_descriptor_set_layout
        .unwrap())(device, descriptor_set_layout, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_descriptor_set_layout = (*self.table)
            .destroy_descriptor_set_layout
            .unwrap();
        destroy_descriptor_set_layout(
            self.handle,
            descriptor_set_layout,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDescriptorPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
pub unsafe fn create_descriptor_pool(
    device: Device,
    p_create_info: *const DescriptorPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_pool: *mut DescriptorPool,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_descriptor_pool
        .unwrap())(device, p_create_info, p_allocator, p_descriptor_pool)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
    pub unsafe fn create_descriptor_pool(
        &self,
        create_info: &DescriptorPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<DescriptorPool> {
        let create_descriptor_pool = (*self.table).create_descriptor_pool.unwrap();
        let mut descriptor_pool = Default::default();
        let result = create_descriptor_pool(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut descriptor_pool,
        );
        crate::new_result(descriptor_pool, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDescriptorPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
pub unsafe fn destroy_descriptor_pool(
    device: Device,
    descriptor_pool: DescriptorPool,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_descriptor_pool
        .unwrap())(device, descriptor_pool, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
    pub unsafe fn destroy_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_descriptor_pool = (*self.table).destroy_descriptor_pool.unwrap();
        destroy_descriptor_pool(
            self.handle,
            descriptor_pool,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetDescriptorPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
pub unsafe fn reset_descriptor_pool(
    device: Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .reset_descriptor_pool
        .unwrap())(device, descriptor_pool, flags)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkResetDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
    pub unsafe fn reset_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        flags: Option<DescriptorPoolResetFlags>,
    ) -> crate::VulkanResult<()> {
        let reset_descriptor_pool = (*self.table).reset_descriptor_pool.unwrap();
        let result = reset_descriptor_pool(
            self.handle,
            descriptor_pool,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAllocateDescriptorSets")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
pub unsafe fn allocate_descriptor_sets(
    device: Device,
    p_allocate_info: *const DescriptorSetAllocateInfo,
    p_descriptor_sets: *mut DescriptorSet,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .allocate_descriptor_sets
        .unwrap())(device, p_allocate_info, p_descriptor_sets)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAllocateDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
    pub unsafe fn allocate_descriptor_sets(
        &self,
        allocate_info: &DescriptorSetAllocateInfo,
    ) -> crate::VulkanResult<Vec<DescriptorSet>> {
        let allocate_descriptor_sets = (*self.table).allocate_descriptor_sets.unwrap();
        let mut descriptor_sets = vec![
            Default::default(); allocate_info.descriptor_set_count as usize
        ];
        let result = allocate_descriptor_sets(
            self.handle,
            allocate_info as _,
            descriptor_sets.as_mut_ptr(),
        );
        crate::new_result(descriptor_sets, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkFreeDescriptorSets")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
pub unsafe fn free_descriptor_sets(
    device: Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .free_descriptor_sets
        .unwrap())(device, descriptor_pool, descriptor_set_count, p_descriptor_sets)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkFreeDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
    pub unsafe fn free_descriptor_sets(
        &self,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> crate::VulkanResult<()> {
        let free_descriptor_sets = (*self.table).free_descriptor_sets.unwrap();
        let descriptor_set_count = descriptor_sets.len();
        let result = free_descriptor_sets(
            self.handle,
            descriptor_pool,
            descriptor_set_count as _,
            descriptor_sets.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkUpdateDescriptorSets")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
pub unsafe fn update_descriptor_sets(
    device: Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .update_descriptor_sets
        .unwrap())(
        device,
        descriptor_write_count,
        p_descriptor_writes,
        descriptor_copy_count,
        p_descriptor_copies,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
    pub unsafe fn update_descriptor_sets(
        &self,
        descriptor_writes: &[WriteDescriptorSet],
        descriptor_copies: &[CopyDescriptorSet],
    ) {
        let update_descriptor_sets = (*self.table).update_descriptor_sets.unwrap();
        let descriptor_write_count = descriptor_writes.len();
        let descriptor_copy_count = descriptor_copies.len();
        update_descriptor_sets(
            self.handle,
            descriptor_write_count as _,
            descriptor_writes.as_ptr(),
            descriptor_copy_count as _,
            descriptor_copies.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateFramebuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
pub unsafe fn create_framebuffer(
    device: Device,
    p_create_info: *const FramebufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_framebuffer: *mut Framebuffer,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_framebuffer
        .unwrap())(device, p_create_info, p_allocator, p_framebuffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateFramebuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
    pub unsafe fn create_framebuffer(
        &self,
        create_info: &FramebufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<Framebuffer> {
        let create_framebuffer = (*self.table).create_framebuffer.unwrap();
        let mut framebuffer = Default::default();
        let result = create_framebuffer(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut framebuffer,
        );
        crate::new_result(framebuffer, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyFramebuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
pub unsafe fn destroy_framebuffer(
    device: Device,
    framebuffer: Framebuffer,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_framebuffer
        .unwrap())(device, framebuffer, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyFramebuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
    pub unsafe fn destroy_framebuffer(
        &self,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_framebuffer = (*self.table).destroy_framebuffer.unwrap();
        destroy_framebuffer(
            self.handle,
            framebuffer,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateRenderPass")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
pub unsafe fn create_render_pass(
    device: Device,
    p_create_info: *const RenderPassCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_render_pass
        .unwrap())(device, p_create_info, p_allocator, p_render_pass)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
    pub unsafe fn create_render_pass(
        &self,
        create_info: &RenderPassCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<RenderPass> {
        let create_render_pass = (*self.table).create_render_pass.unwrap();
        let mut render_pass = Default::default();
        let result = create_render_pass(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut render_pass,
        );
        crate::new_result(render_pass, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyRenderPass")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
pub unsafe fn destroy_render_pass(
    device: Device,
    render_pass: RenderPass,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_render_pass
        .unwrap())(device, render_pass, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
    pub unsafe fn destroy_render_pass(
        &self,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_render_pass = (*self.table).destroy_render_pass.unwrap();
        destroy_render_pass(
            self.handle,
            render_pass,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetRenderAreaGranularity")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
pub unsafe fn get_render_area_granularity(
    device: Device,
    render_pass: RenderPass,
    p_granularity: *mut Extent2D,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_render_area_granularity
        .unwrap())(device, render_pass, p_granularity)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetRenderAreaGranularity")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
    pub unsafe fn get_render_area_granularity(
        &self,
        render_pass: RenderPass,
    ) -> Extent2D {
        let get_render_area_granularity = (*self.table)
            .get_render_area_granularity
            .unwrap();
        let mut granularity = Default::default();
        get_render_area_granularity(self.handle, render_pass, &mut granularity);
        granularity
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateCommandPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
pub unsafe fn create_command_pool(
    device: Device,
    p_create_info: *const CommandPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_command_pool: *mut CommandPool,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_command_pool
        .unwrap())(device, p_create_info, p_allocator, p_command_pool)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
    pub unsafe fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VulkanResult<CommandPool> {
        let create_command_pool = (*self.table).create_command_pool.unwrap();
        let mut command_pool = Default::default();
        let result = create_command_pool(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut command_pool,
        );
        crate::new_result(command_pool, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyCommandPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
pub unsafe fn destroy_command_pool(
    device: Device,
    command_pool: CommandPool,
    p_allocator: *const AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_command_pool
        .unwrap())(device, command_pool, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
    pub unsafe fn destroy_command_pool(
        &self,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let destroy_command_pool = (*self.table).destroy_command_pool.unwrap();
        destroy_command_pool(
            self.handle,
            command_pool,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetCommandPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
pub unsafe fn reset_command_pool(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .reset_command_pool
        .unwrap())(device, command_pool, flags)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkResetCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: CommandPool,
        flags: Option<CommandPoolResetFlags>,
    ) -> crate::VulkanResult<()> {
        let reset_command_pool = (*self.table).reset_command_pool.unwrap();
        let result = reset_command_pool(
            self.handle,
            command_pool,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAllocateCommandBuffers")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
pub unsafe fn allocate_command_buffers(
    device: Device,
    p_allocate_info: *const CommandBufferAllocateInfo,
    p_command_buffers: *mut CommandBuffer,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .allocate_command_buffers
        .unwrap())(device, p_allocate_info, p_command_buffers)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAllocateCommandBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
    pub unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
    ) -> crate::VulkanResult<Vec<CommandBuffer>> {
        let allocate_command_buffers = (*self.table).allocate_command_buffers.unwrap();
        let mut command_buffers = vec![
            Default::default(); allocate_info.command_buffer_count as usize
        ];
        let result = allocate_command_buffers(
            self.handle,
            allocate_info as _,
            command_buffers.as_mut_ptr(),
        );
        crate::new_result(command_buffers, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkFreeCommandBuffers")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
pub unsafe fn free_command_buffers(
    device: Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .free_command_buffers
        .unwrap())(device, command_pool, command_buffer_count, p_command_buffers)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkFreeCommandBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
    pub unsafe fn free_command_buffers(
        &self,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer],
    ) {
        let free_command_buffers = (*self.table).free_command_buffers.unwrap();
        let command_buffer_count = command_buffers.len();
        free_command_buffers(
            self.handle,
            command_pool,
            command_buffer_count as _,
            command_buffers.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBeginCommandBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
pub unsafe fn begin_command_buffer(
    command_buffer: CommandBuffer,
    p_begin_info: *const CommandBufferBeginInfo,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .begin_command_buffer
        .unwrap())(command_buffer, p_begin_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBeginCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &CommandBufferBeginInfo,
    ) -> crate::VulkanResult<()> {
        let begin_command_buffer = (*self.table).begin_command_buffer.unwrap();
        let result = begin_command_buffer(command_buffer, begin_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEndCommandBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
pub unsafe fn end_command_buffer(command_buffer: CommandBuffer) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .end_command_buffer
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEndCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
    pub unsafe fn end_command_buffer(
        &self,
        command_buffer: CommandBuffer,
    ) -> crate::VulkanResult<()> {
        let end_command_buffer = (*self.table).end_command_buffer.unwrap();
        let result = end_command_buffer(command_buffer);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetCommandBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
pub unsafe fn reset_command_buffer(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .reset_command_buffer
        .unwrap())(command_buffer, flags)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkResetCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: Option<CommandBufferResetFlags>,
    ) -> crate::VulkanResult<()> {
        let reset_command_buffer = (*self.table).reset_command_buffer.unwrap();
        let result = reset_command_buffer(
            command_buffer,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindPipeline")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
pub unsafe fn cmd_bind_pipeline(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_pipeline
        .unwrap())(command_buffer, pipeline_bind_point, pipeline)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindPipeline")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        let cmd_bind_pipeline = (*self.table).cmd_bind_pipeline.unwrap();
        cmd_bind_pipeline(command_buffer, pipeline_bind_point as _, pipeline);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewport")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
pub unsafe fn cmd_set_viewport(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport
        .unwrap())(command_buffer, first_viewport, viewport_count, p_viewports)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewport")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewports: &[Viewport],
    ) {
        let cmd_set_viewport = (*self.table).cmd_set_viewport.unwrap();
        let viewport_count = viewports.len();
        cmd_set_viewport(
            command_buffer,
            first_viewport as _,
            viewport_count as _,
            viewports.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetScissor")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
pub unsafe fn cmd_set_scissor(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_scissor
        .unwrap())(command_buffer, first_scissor, scissor_count, p_scissors)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetScissor")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissors: &[Rect2D],
    ) {
        let cmd_set_scissor = (*self.table).cmd_set_scissor.unwrap();
        let scissor_count = scissors.len();
        cmd_set_scissor(
            command_buffer,
            first_scissor as _,
            scissor_count as _,
            scissors.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetLineWidth")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
pub unsafe fn cmd_set_line_width(
    command_buffer: CommandBuffer,
    line_width: std::os::raw::c_float,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_line_width
        .unwrap())(command_buffer, line_width)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetLineWidth")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
    pub unsafe fn cmd_set_line_width(
        &self,
        command_buffer: CommandBuffer,
        line_width: std::os::raw::c_float,
    ) {
        let cmd_set_line_width = (*self.table).cmd_set_line_width.unwrap();
        cmd_set_line_width(command_buffer, line_width as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthBias")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
pub unsafe fn cmd_set_depth_bias(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: std::os::raw::c_float,
    depth_bias_clamp: std::os::raw::c_float,
    depth_bias_slope_factor: std::os::raw::c_float,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_bias
        .unwrap())(
        command_buffer,
        depth_bias_constant_factor,
        depth_bias_clamp,
        depth_bias_slope_factor,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBias")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: std::os::raw::c_float,
        depth_bias_clamp: std::os::raw::c_float,
        depth_bias_slope_factor: std::os::raw::c_float,
    ) {
        let cmd_set_depth_bias = (*self.table).cmd_set_depth_bias.unwrap();
        cmd_set_depth_bias(
            command_buffer,
            depth_bias_constant_factor as _,
            depth_bias_clamp as _,
            depth_bias_slope_factor as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetBlendConstants")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
pub unsafe fn cmd_set_blend_constants(
    command_buffer: CommandBuffer,
    blend_constants: *const [std::os::raw::c_float; 4],
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_blend_constants
        .unwrap())(command_buffer, blend_constants)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetBlendConstants")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: &[std::os::raw::c_float; 4],
    ) {
        let cmd_set_blend_constants = (*self.table).cmd_set_blend_constants.unwrap();
        cmd_set_blend_constants(command_buffer, blend_constants as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthBounds")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
pub unsafe fn cmd_set_depth_bounds(
    command_buffer: CommandBuffer,
    min_depth_bounds: std::os::raw::c_float,
    max_depth_bounds: std::os::raw::c_float,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_bounds
        .unwrap())(command_buffer, min_depth_bounds, max_depth_bounds)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBounds")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: std::os::raw::c_float,
        max_depth_bounds: std::os::raw::c_float,
    ) {
        let cmd_set_depth_bounds = (*self.table).cmd_set_depth_bounds.unwrap();
        cmd_set_depth_bounds(
            command_buffer,
            min_depth_bounds as _,
            max_depth_bounds as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilCompareMask")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
pub unsafe fn cmd_set_stencil_compare_mask(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_stencil_compare_mask
        .unwrap())(command_buffer, face_mask, compare_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        let cmd_set_stencil_compare_mask = (*self.table)
            .cmd_set_stencil_compare_mask
            .unwrap();
        cmd_set_stencil_compare_mask(command_buffer, face_mask as _, compare_mask as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilWriteMask")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
pub unsafe fn cmd_set_stencil_write_mask(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_stencil_write_mask
        .unwrap())(command_buffer, face_mask, write_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        let cmd_set_stencil_write_mask = (*self.table)
            .cmd_set_stencil_write_mask
            .unwrap();
        cmd_set_stencil_write_mask(command_buffer, face_mask as _, write_mask as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilReference")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
pub unsafe fn cmd_set_stencil_reference(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_stencil_reference
        .unwrap())(command_buffer, face_mask, reference)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilReference")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        let cmd_set_stencil_reference = (*self.table).cmd_set_stencil_reference.unwrap();
        cmd_set_stencil_reference(command_buffer, face_mask as _, reference as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindDescriptorSets")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
pub unsafe fn cmd_bind_descriptor_sets(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_descriptor_sets
        .unwrap())(
        command_buffer,
        pipeline_bind_point,
        layout,
        first_set,
        descriptor_set_count,
        p_descriptor_sets,
        dynamic_offset_count,
        p_dynamic_offsets,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        let cmd_bind_descriptor_sets = (*self.table).cmd_bind_descriptor_sets.unwrap();
        let descriptor_set_count = descriptor_sets.len();
        let dynamic_offset_count = dynamic_offsets.len();
        cmd_bind_descriptor_sets(
            command_buffer,
            pipeline_bind_point as _,
            layout,
            first_set as _,
            descriptor_set_count as _,
            descriptor_sets.as_ptr(),
            dynamic_offset_count as _,
            dynamic_offsets.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindIndexBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
pub unsafe fn cmd_bind_index_buffer(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_index_buffer
        .unwrap())(command_buffer, buffer, offset, index_type)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindIndexBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        let cmd_bind_index_buffer = (*self.table).cmd_bind_index_buffer.unwrap();
        cmd_bind_index_buffer(command_buffer, buffer, offset as _, index_type as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindVertexBuffers")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
pub unsafe fn cmd_bind_vertex_buffers(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_vertex_buffers
        .unwrap())(command_buffer, first_binding, binding_count, p_buffers, p_offsets)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
    ) {
        let cmd_bind_vertex_buffers = (*self.table).cmd_bind_vertex_buffers.unwrap();
        let binding_count = buffers.len().min(offsets.len());
        cmd_bind_vertex_buffers(
            command_buffer,
            first_binding as _,
            binding_count as _,
            buffers.as_ptr(),
            offsets.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDraw")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
pub unsafe fn cmd_draw(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw
        .unwrap())(
        command_buffer,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDraw")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let cmd_draw = (*self.table).cmd_draw.unwrap();
        cmd_draw(
            command_buffer,
            vertex_count as _,
            instance_count as _,
            first_vertex as _,
            first_instance as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndexed")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
pub unsafe fn cmd_draw_indexed(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_indexed
        .unwrap())(
        command_buffer,
        index_count,
        instance_count,
        first_index,
        vertex_offset,
        first_instance,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexed")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        let cmd_draw_indexed = (*self.table).cmd_draw_indexed.unwrap();
        cmd_draw_indexed(
            command_buffer,
            index_count as _,
            instance_count as _,
            first_index as _,
            vertex_offset as _,
            first_instance as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndirect")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
pub unsafe fn cmd_draw_indirect(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_indirect
        .unwrap())(command_buffer, buffer, offset, draw_count, stride)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_indirect = (*self.table).cmd_draw_indirect.unwrap();
        cmd_draw_indirect(
            command_buffer,
            buffer,
            offset as _,
            draw_count as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndexedIndirect")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
pub unsafe fn cmd_draw_indexed_indirect(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_indexed_indirect
        .unwrap())(command_buffer, buffer, offset, draw_count, stride)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_indexed_indirect = (*self.table).cmd_draw_indexed_indirect.unwrap();
        cmd_draw_indexed_indirect(
            command_buffer,
            buffer,
            offset as _,
            draw_count as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDispatch")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
pub unsafe fn cmd_dispatch(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_dispatch
        .unwrap())(command_buffer, group_count_x, group_count_y, group_count_z)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDispatch")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let cmd_dispatch = (*self.table).cmd_dispatch.unwrap();
        cmd_dispatch(
            command_buffer,
            group_count_x as _,
            group_count_y as _,
            group_count_z as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDispatchIndirect")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
pub unsafe fn cmd_dispatch_indirect(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_dispatch_indirect
        .unwrap())(command_buffer, buffer, offset)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDispatchIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        let cmd_dispatch_indirect = (*self.table).cmd_dispatch_indirect.unwrap();
        cmd_dispatch_indirect(command_buffer, buffer, offset as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
pub unsafe fn cmd_copy_buffer(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferCopy,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_buffer
        .unwrap())(command_buffer, src_buffer, dst_buffer, region_count, p_regions)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        regions: &[BufferCopy],
    ) {
        let cmd_copy_buffer = (*self.table).cmd_copy_buffer.unwrap();
        let region_count = regions.len();
        cmd_copy_buffer(
            command_buffer,
            src_buffer,
            dst_buffer,
            region_count as _,
            regions.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
pub unsafe fn cmd_copy_image(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageCopy,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_image
        .unwrap())(
        command_buffer,
        src_image,
        src_image_layout,
        dst_image,
        dst_image_layout,
        region_count,
        p_regions,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) {
        let cmd_copy_image = (*self.table).cmd_copy_image.unwrap();
        let region_count = regions.len();
        cmd_copy_image(
            command_buffer,
            src_image,
            src_image_layout as _,
            dst_image,
            dst_image_layout as _,
            region_count as _,
            regions.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBlitImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
pub unsafe fn cmd_blit_image(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_blit_image
        .unwrap())(
        command_buffer,
        src_image,
        src_image_layout,
        dst_image,
        dst_image_layout,
        region_count,
        p_regions,
        filter,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) {
        let cmd_blit_image = (*self.table).cmd_blit_image.unwrap();
        let region_count = regions.len();
        cmd_blit_image(
            command_buffer,
            src_image,
            src_image_layout as _,
            dst_image,
            dst_image_layout as _,
            region_count as _,
            regions.as_ptr(),
            filter as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyBufferToImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
pub unsafe fn cmd_copy_buffer_to_image(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const BufferImageCopy,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_buffer_to_image
        .unwrap())(
        command_buffer,
        src_buffer,
        dst_image,
        dst_image_layout,
        region_count,
        p_regions,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) {
        let cmd_copy_buffer_to_image = (*self.table).cmd_copy_buffer_to_image.unwrap();
        let region_count = regions.len();
        cmd_copy_buffer_to_image(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout as _,
            region_count as _,
            regions.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyImageToBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
pub unsafe fn cmd_copy_image_to_buffer(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferImageCopy,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_image_to_buffer
        .unwrap())(
        command_buffer,
        src_image,
        src_image_layout,
        dst_buffer,
        region_count,
        p_regions,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy],
    ) {
        let cmd_copy_image_to_buffer = (*self.table).cmd_copy_image_to_buffer.unwrap();
        let region_count = regions.len();
        cmd_copy_image_to_buffer(
            command_buffer,
            src_image,
            src_image_layout as _,
            dst_buffer,
            region_count as _,
            regions.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdUpdateBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
pub unsafe fn cmd_update_buffer(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: *const std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_update_buffer
        .unwrap())(command_buffer, dst_buffer, dst_offset, data_size, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdUpdateBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        data: *const std::os::raw::c_void,
    ) {
        let cmd_update_buffer = (*self.table).cmd_update_buffer.unwrap();
        cmd_update_buffer(command_buffer, dst_buffer, dst_offset as _, data_size, data);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdFillBuffer")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
pub unsafe fn cmd_fill_buffer(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_fill_buffer
        .unwrap())(command_buffer, dst_buffer, dst_offset, size, data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdFillBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        let cmd_fill_buffer = (*self.table).cmd_fill_buffer.unwrap();
        cmd_fill_buffer(
            command_buffer,
            dst_buffer,
            dst_offset as _,
            size as _,
            data as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdClearColorImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
pub unsafe fn cmd_clear_color_image(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_clear_color_image
        .unwrap())(command_buffer, image, image_layout, p_color, range_count, p_ranges)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdClearColorImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) {
        let cmd_clear_color_image = (*self.table).cmd_clear_color_image.unwrap();
        let range_count = ranges.len();
        cmd_clear_color_image(
            command_buffer,
            image,
            image_layout as _,
            color as _,
            range_count as _,
            ranges.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdClearDepthStencilImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
pub unsafe fn cmd_clear_depth_stencil_image(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_clear_depth_stencil_image
        .unwrap())(
        command_buffer,
        image,
        image_layout,
        p_depth_stencil,
        range_count,
        p_ranges,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) {
        let cmd_clear_depth_stencil_image = (*self.table)
            .cmd_clear_depth_stencil_image
            .unwrap();
        let range_count = ranges.len();
        cmd_clear_depth_stencil_image(
            command_buffer,
            image,
            image_layout as _,
            depth_stencil as _,
            range_count as _,
            ranges.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdClearAttachments")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
pub unsafe fn cmd_clear_attachments(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_clear_attachments
        .unwrap())(command_buffer, attachment_count, p_attachments, rect_count, p_rects)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdClearAttachments")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachments: &[ClearAttachment],
        rects: &[ClearRect],
    ) {
        let cmd_clear_attachments = (*self.table).cmd_clear_attachments.unwrap();
        let attachment_count = attachments.len();
        let rect_count = rects.len();
        cmd_clear_attachments(
            command_buffer,
            attachment_count as _,
            attachments.as_ptr(),
            rect_count as _,
            rects.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResolveImage")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
pub unsafe fn cmd_resolve_image(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_resolve_image
        .unwrap())(
        command_buffer,
        src_image,
        src_image_layout,
        dst_image,
        dst_image_layout,
        region_count,
        p_regions,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) {
        let cmd_resolve_image = (*self.table).cmd_resolve_image.unwrap();
        let region_count = regions.len();
        cmd_resolve_image(
            command_buffer,
            src_image,
            src_image_layout as _,
            dst_image,
            dst_image_layout as _,
            region_count as _,
            regions.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetEvent")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
pub unsafe fn cmd_set_event(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_event
        .unwrap())(command_buffer, event, stage_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: Option<PipelineStageFlags>,
    ) {
        let cmd_set_event = (*self.table).cmd_set_event.unwrap();
        cmd_set_event(
            command_buffer,
            event,
            match stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResetEvent")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
pub unsafe fn cmd_reset_event(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_reset_event
        .unwrap())(command_buffer, event, stage_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: Option<PipelineStageFlags>,
    ) {
        let cmd_reset_event = (*self.table).cmd_reset_event.unwrap();
        cmd_reset_event(
            command_buffer,
            event,
            match stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWaitEvents")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
pub unsafe fn cmd_wait_events(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_wait_events
        .unwrap())(
        command_buffer,
        event_count,
        p_events,
        src_stage_mask,
        dst_stage_mask,
        memory_barrier_count,
        p_memory_barriers,
        buffer_memory_barrier_count,
        p_buffer_memory_barriers,
        image_memory_barrier_count,
        p_image_memory_barriers,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        src_stage_mask: Option<PipelineStageFlags>,
        dst_stage_mask: Option<PipelineStageFlags>,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        let cmd_wait_events = (*self.table).cmd_wait_events.unwrap();
        let event_count = events.len();
        let memory_barrier_count = memory_barriers.len();
        let buffer_memory_barrier_count = buffer_memory_barriers.len();
        let image_memory_barrier_count = image_memory_barriers.len();
        cmd_wait_events(
            command_buffer,
            event_count as _,
            events.as_ptr(),
            match src_stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
            match dst_stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
            memory_barrier_count as _,
            memory_barriers.as_ptr(),
            buffer_memory_barrier_count as _,
            buffer_memory_barriers.as_ptr(),
            image_memory_barrier_count as _,
            image_memory_barriers.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPipelineBarrier")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
pub unsafe fn cmd_pipeline_barrier(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_pipeline_barrier
        .unwrap())(
        command_buffer,
        src_stage_mask,
        dst_stage_mask,
        dependency_flags,
        memory_barrier_count,
        p_memory_barriers,
        buffer_memory_barrier_count,
        p_buffer_memory_barriers,
        image_memory_barrier_count,
        p_image_memory_barriers,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: Option<PipelineStageFlags>,
        dst_stage_mask: Option<PipelineStageFlags>,
        dependency_flags: Option<DependencyFlags>,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        let cmd_pipeline_barrier = (*self.table).cmd_pipeline_barrier.unwrap();
        let memory_barrier_count = memory_barriers.len();
        let buffer_memory_barrier_count = buffer_memory_barriers.len();
        let image_memory_barrier_count = image_memory_barriers.len();
        cmd_pipeline_barrier(
            command_buffer,
            match src_stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
            match dst_stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
            match dependency_flags {
                Some(v) => v,
                None => Default::default(),
            },
            memory_barrier_count as _,
            memory_barriers.as_ptr(),
            buffer_memory_barrier_count as _,
            buffer_memory_barriers.as_ptr(),
            image_memory_barrier_count as _,
            image_memory_barriers.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginQuery")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
pub unsafe fn cmd_begin_query(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_query
        .unwrap())(command_buffer, query_pool, query, flags)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginQuery")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: Option<QueryControlFlags>,
    ) {
        let cmd_begin_query = (*self.table).cmd_begin_query.unwrap();
        cmd_begin_query(
            command_buffer,
            query_pool,
            query as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndQuery")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
pub unsafe fn cmd_end_query(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_query
        .unwrap())(command_buffer, query_pool, query)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndQuery")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        let cmd_end_query = (*self.table).cmd_end_query.unwrap();
        cmd_end_query(command_buffer, query_pool, query as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResetQueryPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
pub unsafe fn cmd_reset_query_pool(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_reset_query_pool
        .unwrap())(command_buffer, query_pool, first_query, query_count)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResetQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        let cmd_reset_query_pool = (*self.table).cmd_reset_query_pool.unwrap();
        cmd_reset_query_pool(
            command_buffer,
            query_pool,
            first_query as _,
            query_count as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteTimestamp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
pub unsafe fn cmd_write_timestamp(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_timestamp
        .unwrap())(command_buffer, pipeline_stage, query_pool, query)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        let cmd_write_timestamp = (*self.table).cmd_write_timestamp.unwrap();
        cmd_write_timestamp(command_buffer, pipeline_stage as _, query_pool, query as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyQueryPoolResults")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
pub unsafe fn cmd_copy_query_pool_results(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_query_pool_results
        .unwrap())(
        command_buffer,
        query_pool,
        first_query,
        query_count,
        dst_buffer,
        dst_offset,
        stride,
        flags,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: Option<QueryResultFlags>,
    ) {
        let cmd_copy_query_pool_results = (*self.table)
            .cmd_copy_query_pool_results
            .unwrap();
        cmd_copy_query_pool_results(
            command_buffer,
            query_pool,
            first_query as _,
            query_count as _,
            dst_buffer,
            dst_offset as _,
            stride as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPushConstants")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
pub unsafe fn cmd_push_constants(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_push_constants
        .unwrap())(command_buffer, layout, stage_flags, offset, size, p_values)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPushConstants")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        values: *const std::os::raw::c_void,
    ) {
        let cmd_push_constants = (*self.table).cmd_push_constants.unwrap();
        cmd_push_constants(
            command_buffer,
            layout,
            stage_flags as _,
            offset as _,
            size,
            values,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRenderPass")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
pub unsafe fn cmd_begin_render_pass(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_render_pass
        .unwrap())(command_buffer, p_render_pass_begin, contents)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        let cmd_begin_render_pass = (*self.table).cmd_begin_render_pass.unwrap();
        cmd_begin_render_pass(command_buffer, render_pass_begin as _, contents as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdNextSubpass")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
pub unsafe fn cmd_next_subpass(
    command_buffer: CommandBuffer,
    contents: SubpassContents,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_next_subpass
        .unwrap())(command_buffer, contents)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        let cmd_next_subpass = (*self.table).cmd_next_subpass.unwrap();
        cmd_next_subpass(command_buffer, contents as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRenderPass")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
pub unsafe fn cmd_end_render_pass(command_buffer: CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_render_pass
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) {
        let cmd_end_render_pass = (*self.table).cmd_end_render_pass.unwrap();
        cmd_end_render_pass(command_buffer);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdExecuteCommands")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
pub unsafe fn cmd_execute_commands(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_execute_commands
        .unwrap())(command_buffer, command_buffer_count, p_command_buffers)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdExecuteCommands")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffers: &[CommandBuffer],
    ) {
        let cmd_execute_commands = (*self.table).cmd_execute_commands.unwrap();
        let command_buffer_count = command_buffers.len();
        cmd_execute_commands(
            command_buffer,
            command_buffer_count as _,
            command_buffers.as_ptr(),
        );
    }
}
