#[doc(alias = "VkFlags64")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFlags64.html)
pub type Flags64 = u64;
#[doc(alias = "VkAccessFlags2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlags2KHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccessFlags2KHR(pub u64);
impl AccessFlags2KHR {
    pub const NONE: Self = Self(0);
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
    pub const SHADER_SAMPLED_READ: Self = Self(1 << 32);
    pub const SHADER_STORAGE_READ: Self = Self(1 << 33);
    pub const SHADER_STORAGE_WRITE: Self = Self(1 << 34);
}
crate::bitflags_impl! {
    AccessFlags2KHR : u64, 0x70001ffff, NONE, INDIRECT_COMMAND_READ, INDEX_READ,
    VERTEX_ATTRIBUTE_READ, UNIFORM_READ, INPUT_ATTACHMENT_READ, SHADER_READ,
    SHADER_WRITE, COLOR_ATTACHMENT_READ, COLOR_ATTACHMENT_WRITE,
    DEPTH_STENCIL_ATTACHMENT_READ, DEPTH_STENCIL_ATTACHMENT_WRITE, TRANSFER_READ,
    TRANSFER_WRITE, HOST_READ, HOST_WRITE, MEMORY_READ, MEMORY_WRITE,
    SHADER_SAMPLED_READ, SHADER_STORAGE_READ, SHADER_STORAGE_WRITE
}
#[doc(alias = "VkPipelineStageFlags2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlags2KHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineStageFlags2KHR(pub u64);
impl PipelineStageFlags2KHR {
    pub const NONE: Self = Self(0);
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
    pub const ALL_TRANSFER: Self = Self(1 << 12);
    pub const TRANSFER: Self = Self::ALL_TRANSFER;
    pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
    pub const HOST: Self = Self(1 << 14);
    pub const ALL_GRAPHICS: Self = Self(1 << 15);
    pub const ALL_COMMANDS: Self = Self(1 << 16);
    pub const COPY: Self = Self(1 << 32);
    pub const RESOLVE: Self = Self(1 << 33);
    pub const BLIT: Self = Self(1 << 34);
    pub const CLEAR: Self = Self(1 << 35);
    pub const INDEX_INPUT: Self = Self(1 << 36);
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(1 << 37);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 38);
}
crate::bitflags_impl! {
    PipelineStageFlags2KHR : u64, 0x7f0001ffff, NONE, TOP_OF_PIPE, DRAW_INDIRECT,
    VERTEX_INPUT, VERTEX_SHADER, TESSELLATION_CONTROL_SHADER,
    TESSELLATION_EVALUATION_SHADER, GEOMETRY_SHADER, FRAGMENT_SHADER,
    EARLY_FRAGMENT_TESTS, LATE_FRAGMENT_TESTS, COLOR_ATTACHMENT_OUTPUT, COMPUTE_SHADER,
    ALL_TRANSFER, BOTTOM_OF_PIPE, HOST, ALL_GRAPHICS, ALL_COMMANDS, COPY, RESOLVE, BLIT,
    CLEAR, INDEX_INPUT, VERTEX_ATTRIBUTE_INPUT, PRE_RASTERIZATION_SHADERS
}
#[doc(alias = "VkSubmitFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubmitFlagsKHR(pub u32);
impl SubmitFlagsKHR {
    pub const PROTECTED: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    SubmitFlagsKHR : u32, 0x1, PROTECTED
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2KHR.html)
pub struct MemoryBarrier2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_stage_mask: PipelineStageFlags2KHR,
    pub src_access_mask: AccessFlags2KHR,
    pub dst_stage_mask: PipelineStageFlags2KHR,
    pub dst_access_mask: AccessFlags2KHR,
}
impl Default for MemoryBarrier2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageMemoryBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2KHR.html)
pub struct ImageMemoryBarrier2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_stage_mask: PipelineStageFlags2KHR,
    pub src_access_mask: AccessFlags2KHR,
    pub dst_stage_mask: PipelineStageFlags2KHR,
    pub dst_access_mask: AccessFlags2KHR,
    pub old_layout: crate::vk10::ImageLayout,
    pub new_layout: crate::vk10::ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: crate::vk10::Image,
    pub subresource_range: crate::vk10::ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
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
#[doc(alias = "VkBufferMemoryBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2KHR.html)
pub struct BufferMemoryBarrier2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_stage_mask: PipelineStageFlags2KHR,
    pub src_access_mask: AccessFlags2KHR,
    pub dst_stage_mask: PipelineStageFlags2KHR,
    pub dst_access_mask: AccessFlags2KHR,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: crate::vk10::Buffer,
    pub offset: crate::vk10::DeviceSize,
    pub size: crate::vk10::DeviceSize,
}
impl Default for BufferMemoryBarrier2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
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
#[doc(alias = "VkDependencyInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyInfoKHR.html)
pub struct DependencyInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub dependency_flags: crate::vk10::DependencyFlags,
    pub memory_barrier_count: u32,
    pub p_memory_barriers: *const MemoryBarrier2KHR,
    pub buffer_memory_barrier_count: u32,
    pub p_buffer_memory_barriers: *const BufferMemoryBarrier2KHR,
    pub image_memory_barrier_count: u32,
    pub p_image_memory_barriers: *const ImageMemoryBarrier2KHR,
}
impl Default for DependencyInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEPENDENCY_INFO,
            p_next: std::ptr::null(),
            dependency_flags: Default::default(),
            memory_barrier_count: Default::default(),
            p_memory_barriers: std::ptr::null(),
            buffer_memory_barrier_count: Default::default(),
            p_buffer_memory_barriers: std::ptr::null(),
            image_memory_barrier_count: Default::default(),
            p_image_memory_barriers: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfoKHR.html)
pub struct SemaphoreSubmitInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub value: u64,
    pub stage_mask: PipelineStageFlags2KHR,
    pub device_index: u32,
}
impl Default for SemaphoreSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_SUBMIT_INFO,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: Default::default(),
            stage_mask: Default::default(),
            device_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfoKHR.html)
pub struct CommandBufferSubmitInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub command_buffer: crate::vk10::CommandBuffer,
    pub device_mask: u32,
}
impl Default for CommandBufferSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_SUBMIT_INFO,
            p_next: std::ptr::null(),
            command_buffer: Default::default(),
            device_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubmitInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2KHR.html)
pub struct SubmitInfo2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SubmitFlagsKHR,
    pub wait_semaphore_info_count: u32,
    pub p_wait_semaphore_infos: *const SemaphoreSubmitInfoKHR,
    pub command_buffer_info_count: u32,
    pub p_command_buffer_infos: *const CommandBufferSubmitInfoKHR,
    pub signal_semaphore_info_count: u32,
    pub p_signal_semaphore_infos: *const SemaphoreSubmitInfoKHR,
}
impl Default for SubmitInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBMIT_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            wait_semaphore_info_count: Default::default(),
            p_wait_semaphore_infos: std::ptr::null(),
            command_buffer_info_count: Default::default(),
            p_command_buffer_infos: std::ptr::null(),
            signal_semaphore_info_count: Default::default(),
            p_signal_semaphore_infos: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSynchronization2FeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSynchronization2FeaturesKHR.html)
pub struct PhysicalDeviceSynchronization2FeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub synchronization_2: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSynchronization2FeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
            p_next: std::ptr::null_mut(),
            synchronization_2: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetEvent2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html)
pub unsafe fn cmd_set_event_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    event: crate::vk10::Event,
    p_dependency_info: *const DependencyInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_event_2_khr
        .unwrap())(command_buffer, event, p_dependency_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html)
    pub unsafe fn cmd_set_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        dependency_info: &DependencyInfoKHR,
    ) {
        let cmd_set_event_2_khr = (*self.table).cmd_set_event_2_khr.unwrap();
        cmd_set_event_2_khr(command_buffer, event, dependency_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResetEvent2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html)
pub unsafe fn cmd_reset_event_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    event: crate::vk10::Event,
    stage_mask: PipelineStageFlags2KHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_reset_event_2_khr
        .unwrap())(command_buffer, event, stage_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html)
    pub unsafe fn cmd_reset_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: Option<PipelineStageFlags2KHR>,
    ) {
        let cmd_reset_event_2_khr = (*self.table).cmd_reset_event_2_khr.unwrap();
        cmd_reset_event_2_khr(
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
#[doc(alias = "vkCmdWaitEvents2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html)
pub unsafe fn cmd_wait_events_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    event_count: u32,
    p_events: *const crate::vk10::Event,
    p_dependency_infos: *const DependencyInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_wait_events_2_khr
        .unwrap())(command_buffer, event_count, p_events, p_dependency_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html)
    pub unsafe fn cmd_wait_events_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        events: &[crate::vk10::Event],
        dependency_infos: &[DependencyInfoKHR],
    ) {
        let cmd_wait_events_2_khr = (*self.table).cmd_wait_events_2_khr.unwrap();
        let event_count = events.len().min(dependency_infos.len());
        cmd_wait_events_2_khr(
            command_buffer,
            event_count as _,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html)
pub unsafe fn cmd_pipeline_barrier_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_dependency_info: *const DependencyInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_pipeline_barrier_2_khr
        .unwrap())(command_buffer, p_dependency_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html)
    pub unsafe fn cmd_pipeline_barrier_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dependency_info: &DependencyInfoKHR,
    ) {
        let cmd_pipeline_barrier_2_khr = (*self.table)
            .cmd_pipeline_barrier_2_khr
            .unwrap();
        cmd_pipeline_barrier_2_khr(command_buffer, dependency_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueSubmit2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html)
pub unsafe fn queue_submit_2_khr(
    queue: crate::vk10::Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo2KHR,
    fence: crate::vk10::Fence,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_submit_2_khr
        .unwrap())(queue, submit_count, p_submits, fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueSubmit2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html)
    pub unsafe fn queue_submit_2_khr(
        &self,
        queue: crate::vk10::Queue,
        submits: &[SubmitInfo2KHR],
        fence: crate::vk10::Fence,
    ) -> crate::VulkanResult<()> {
        let queue_submit_2_khr = (*self.table).queue_submit_2_khr.unwrap();
        let submit_count = submits.len();
        let result = queue_submit_2_khr(
            queue,
            submit_count as _,
            submits.as_ptr(),
            fence,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html)
pub unsafe fn cmd_write_timestamp_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    stage: PipelineStageFlags2KHR,
    query_pool: crate::vk10::QueryPool,
    query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_timestamp_2_khr
        .unwrap())(command_buffer, stage, query_pool, query)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html)
    pub unsafe fn cmd_write_timestamp_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: Option<PipelineStageFlags2KHR>,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        let cmd_write_timestamp_2_khr = (*self.table).cmd_write_timestamp_2_khr.unwrap();
        cmd_write_timestamp_2_khr(
            command_buffer,
            match stage {
                Some(v) => v,
                None => Default::default(),
            },
            query_pool,
            query as _,
        );
    }
}
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_synchronization2"
);
