#[macro_export]
macro_rules! pnext_visit {
    ($pnext:expr, $stype:ident, $object:ident, $op:expr) => {
        let $stype = * $pnext .cast:: < $crate ::vk10::StructureType > (); match $stype {
        $crate ::vk10::StructureType::APPLICATION_INFO => { let $object = $pnext .cast::
        < crate ::vk10::ApplicationInfo > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_QUEUE_CREATE_INFO => { let $object = $pnext .cast::
        < crate ::vk10::DeviceQueueCreateInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::DEVICE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::DeviceCreateInfo > (); $op; $pnext = (* $object).p_next;
        } $crate ::vk10::StructureType::INSTANCE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::InstanceCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::MEMORY_ALLOCATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::MemoryAllocateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::MAPPED_MEMORY_RANGE => { let $object =
        $pnext .cast:: < crate ::vk10::MappedMemoryRange > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::WRITE_DESCRIPTOR_SET => { let $object =
        $pnext .cast:: < crate ::vk10::WriteDescriptorSet > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COPY_DESCRIPTOR_SET => { let $object =
        $pnext .cast:: < crate ::vk10::CopyDescriptorSet > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::BufferCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_VIEW_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::BufferViewCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_BARRIER => { let $object
        = $pnext .cast:: < crate ::vk10::MemoryBarrier > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_MEMORY_BARRIER => { let $object =
        $pnext .cast:: < crate ::vk10::BufferMemoryBarrier > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_MEMORY_BARRIER => { let
        $object = $pnext .cast:: < crate ::vk10::ImageMemoryBarrier > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::IMAGE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::ImageCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_VIEW_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::ImageViewCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::BIND_SPARSE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::BindSparseInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SHADER_MODULE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::ShaderModuleCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::DescriptorSetLayoutCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::DESCRIPTOR_POOL_CREATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk10::DescriptorPoolCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::DescriptorSetAllocateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::PipelineShaderStageCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMPUTE_PIPELINE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::ComputePipelineCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO
        => { let $object = $pnext .cast:: < crate
        ::vk10::PipelineVertexInputStateCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::PipelineInputAssemblyStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineTessellationStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineViewportStateCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::PipelineRasterizationStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineMultisampleStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineColorBlendStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineDynamicStateCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::PipelineDepthStencilStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GRAPHICS_PIPELINE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::GraphicsPipelineCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PIPELINE_CACHE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::PipelineCacheCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_LAYOUT_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::PipelineLayoutCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::SAMPLER_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::SamplerCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COMMAND_POOL_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::CommandPoolCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_ALLOCATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::CommandBufferAllocateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::CommandBufferInheritanceInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_BEGIN_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::CommandBufferBeginInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::RENDER_PASS_BEGIN_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::RenderPassBeginInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::RENDER_PASS_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::RenderPassCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::EVENT_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::EventCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::FENCE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::FenceCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::SemaphoreCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::QUERY_POOL_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::QueryPoolCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FRAMEBUFFER_CREATE_INFO => { let $object = $pnext .cast::
        < crate ::vk10::FramebufferCreateInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::SUBMIT_INFO => { let $object = $pnext .cast:: <
        crate ::vk10::SubmitInfo > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FEATURES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::PhysicalDeviceFeatures2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_PROPERTIES_2 => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceProperties2 > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::FORMAT_PROPERTIES_2
        => { let $object = $pnext .cast:: < crate ::vk11::FormatProperties2 > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_FORMAT_PROPERTIES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::ImageFormatProperties2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => {
        let $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceImageFormatInfo2 > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_PROPERTIES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::QueueFamilyProperties2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => {
        let $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceMemoryProperties2 >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::SparseImageFormatProperties2 > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceSparseImageFormatInfo2 >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceVariablePointersFeatures >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalImageFormatInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::ExternalImageFormatProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalBufferInfo > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::EXTERNAL_BUFFER_PROPERTIES
        => { let $object = $pnext .cast:: < crate ::vk11::ExternalBufferProperties > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ID_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::PhysicalDeviceIDProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::ExternalMemoryImageCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::ExternalMemoryBufferCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::EXPORT_MEMORY_ALLOCATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk11::ExportMemoryAllocateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalSemaphoreInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_SEMAPHORE_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::ExternalSemaphoreProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::EXPORT_SEMAPHORE_CREATE_INFO =>
        { let $object = $pnext .cast:: < crate ::vk11::ExportSemaphoreCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalFenceInfo > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::EXTERNAL_FENCE_PROPERTIES
        => { let $object = $pnext .cast:: < crate ::vk11::ExternalFenceProperties > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_FENCE_CREATE_INFO => { let $object = $pnext .cast::
        < crate ::vk11::ExportFenceCreateInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDeviceMultiviewFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceMultiviewProperties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::RenderPassMultiviewCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::PhysicalDeviceGroupProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_ALLOCATE_FLAGS_INFO => {
        let $object = $pnext .cast:: < crate ::vk11::MemoryAllocateFlagsInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_BUFFER_MEMORY_INFO => { let $object = $pnext .cast::
        < crate ::vk11::BindBufferMemoryInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::BindBufferMemoryDeviceGroupInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_IMAGE_MEMORY_INFO => { let $object = $pnext .cast:: <
        crate ::vk11::BindImageMemoryInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::BindImageMemoryDeviceGroupInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DeviceGroupRenderPassBeginInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DeviceGroupCommandBufferBeginInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_SUBMIT_INFO => { let $object = $pnext .cast::
        < crate ::vk11::DeviceGroupSubmitInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::DEVICE_GROUP_BIND_SPARSE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DeviceGroupBindSparseInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk11::DeviceGroupDeviceCreateInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DescriptorUpdateTemplateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => { let
        $object = $pnext .cast:: < crate
        ::vk11::RenderPassInputAttachmentAspectCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES
        => { let $object = $pnext .cast:: < crate
        ::vk11::PhysicalDevice16BitStorageFeatures > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => {
        let $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceSubgroupProperties >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => { let $object =
        $pnext .cast:: < crate ::vk11::BufferMemoryRequirementsInfo2 > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => { let $object = $pnext
        .cast:: < crate ::vk11::ImageMemoryRequirementsInfo2 > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => { let $object =
        $pnext .cast:: < crate ::vk11::ImageSparseMemoryRequirementsInfo2 > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_REQUIREMENTS_2 => { let $object = $pnext .cast:: <
        crate ::vk11::MemoryRequirements2 > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => { let $object
        = $pnext .cast:: < crate ::vk11::SparseImageMemoryRequirements2 > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDevicePointClippingProperties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_DEDICATED_REQUIREMENTS => { let $object = $pnext
        .cast:: < crate ::vk11::MemoryDedicatedRequirements > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk11::MemoryDedicatedAllocateInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_VIEW_USAGE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk11::ImageViewUsageCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate
        ::vk11::PipelineTessellationDomainOriginStateCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_INFO =>
        { let $object = $pnext .cast:: < crate ::vk11::SamplerYcbcrConversionInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::SamplerYcbcrConversionCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_IMAGE_PLANE_MEMORY_INFO => { let $object = $pnext
        .cast:: < crate ::vk11::BindImagePlaneMemoryInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO =>
        { let $object = $pnext .cast:: < crate ::vk11::ImagePlaneMemoryRequirementsInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk11::SamplerYcbcrConversionImageFormatProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PROTECTED_SUBMIT_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::ProtectedSubmitInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDeviceProtectedMemoryFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceProtectedMemoryProperties
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_QUEUE_INFO_2 => { let $object = $pnext .cast:: <
        crate ::vk11::DeviceQueueInfo2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDeviceMaintenance3Properties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT => { let $object = $pnext
        .cast:: < crate ::vk11::DescriptorSetLayoutSupport > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk11::PhysicalDeviceShaderDrawParametersFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SWAPCHAIN_CREATE_INFO_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::SwapchainCreateInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PRESENT_INFO_KHR => { let
        $object = $pnext .cast:: < crate ::extensions::khr_swapchain::PresentInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_swapchain::ImageSwapchainCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::AcquireNextImageInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEVICE_GROUP_PRESENT_INFO_KHR =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::DeviceGroupPresentInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::XLIB_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::XCB_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ANDROID_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WIN32_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RENDERING_CREATE_INFO => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_dynamic_rendering::PipelineRenderingCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::RENDERING_INFO => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::RenderingInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::RENDERING_ATTACHMENT_INFO => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::PhysicalDeviceDynamicRenderingFeaturesKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::CommandBufferInheritanceRenderingInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => { let $object
        = $pnext .cast:: < crate
        ::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VI_SURFACE_CREATE_INFO_NN => { let $object = $pnext
        .cast:: < crate ::extensions::nn_vi_surface::ViSurfaceCreateInfoNN > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ATTACHMENT_DESCRIPTION_2 => { let $object = $pnext .cast::
        < crate ::extensions::khr_create_renderpass2::AttachmentDescription2KHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ATTACHMENT_REFERENCE_2 => { let $object = $pnext .cast:: <
        crate ::extensions::khr_create_renderpass2::AttachmentReference2KHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SUBPASS_DESCRIPTION_2 => { let $object = $pnext .cast:: <
        crate ::extensions::khr_create_renderpass2::SubpassDescription2KHR > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::SUBPASS_DEPENDENCY_2
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_create_renderpass2::SubpassDependency2KHR > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::RENDER_PASS_CREATE_INFO_2 =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_create_renderpass2::RenderPassCreateInfo2KHR > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::SUBPASS_BEGIN_INFO => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_create_renderpass2::SubpassBeginInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SUBPASS_END_INFO => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_create_renderpass2::SubpassEndInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IOS_SURFACE_CREATE_INFO_MVK => {
        let $object = $pnext .cast:: < crate
        ::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MACOS_SURFACE_CREATE_INFO_MVK =>
        { let $object = $pnext .cast:: < crate
        ::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEBUG_UTILS_LABEL_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsLabelEXT > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_depth_stencil_resolve::PhysicalDeviceDepthStencilResolvePropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_depth_stencil_resolve::SubpassDescriptionDepthStencilResolveKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_timeline_semaphore::PhysicalDeviceTimelineSemaphoreFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_timeline_semaphore::PhysicalDeviceTimelineSemaphorePropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SEMAPHORE_TYPE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::extensions::khr_timeline_semaphore::SemaphoreTypeCreateInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_timeline_semaphore::TimelineSemaphoreSubmitInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_WAIT_INFO
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_timeline_semaphore::SemaphoreWaitInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_SIGNAL_INFO => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_timeline_semaphore::SemaphoreSignalInfoKHR > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::METAL_SURFACE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_scalar_block_layout::PhysicalDeviceScalarBlockLayoutFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_BARRIER_2 => { let $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::MemoryBarrier2KHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_MEMORY_BARRIER_2 => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::ImageMemoryBarrier2KHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::BUFFER_MEMORY_BARRIER_2 => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::BufferMemoryBarrier2KHR > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::DEPENDENCY_INFO => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::DependencyInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_SUBMIT_INFO => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::SemaphoreSubmitInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::COMMAND_BUFFER_SUBMIT_INFO => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::CommandBufferSubmitInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::SUBMIT_INFO_2 => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::SubmitInfo2KHR > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::PhysicalDeviceSynchronization2FeaturesKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SCREEN_SURFACE_CREATE_INFO_QNX => { let $object = $pnext
        .cast:: < crate ::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX >
        (); $op; $pnext = (* $object).p_next; } _ =>
        panic!("Unknown StructureType value ({:?})", $stype) }
    };
}
