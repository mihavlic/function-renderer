use crate::{
    cstr,
    loader::{DeviceLoad, EntryLoad, InstanceLoad},
    util::ApiLoadConfig,
};

/// Oh, yes. Little Bobby Tables we call him.

macro_rules! load_fns {
    ($table:ident, $loader:ident, $(($name:ident, $str:literal))+) => {
        $(
            $table.$name = ::std::mem::transmute($loader.load($crate::cstr!($str).as_ptr()));
        )+
    };
}

/// https://github.com/maxbla/const-zero#how-does-it-work
union ConstZeroedHack<T, const S: usize> {
    bytes: [u8; S],
    inner: std::mem::ManuallyDrop<T>,
}

impl<T, const S: usize> ConstZeroedHack<T, S> {
    const unsafe fn zero() -> T {
        std::mem::ManuallyDrop::into_inner(Self { bytes: [0; S] }.inner)
    }
}
#[cfg(feature = "global")]
pub static mut GLOBAL_ENTRY_TABLE: EntryTable = EntryTable::new_empty();
#[cfg(feature = "global")]
pub static mut GLOBAL_INSTANCE_TABLE: InstanceTable = InstanceTable::new_empty();
#[cfg(feature = "global")]
pub static mut GLOBAL_DEVICE_TABLE: DeviceTable = DeviceTable::new_empty();
pub struct EntryTable {
    pub create_instance: Option<
        unsafe extern "system" fn(
            p_create_info: *const crate::vk10::InstanceCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_instance: *mut crate::vk10::Instance,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_instance_layer_properties: Option<
        unsafe extern "system" fn(
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::LayerProperties,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_instance_extension_properties: Option<
        unsafe extern "system" fn(
            p_layer_name: *const std::os::raw::c_char,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::ExtensionProperties,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_instance_version: Option<
        unsafe extern "system" fn(p_api_version: *mut u32) -> crate::vk10::Result,
    >,
}
impl EntryTable {
    pub const fn new_empty() -> Self {
        unsafe {
            const SIZE: usize = std::mem::size_of::<EntryTable>();
            ConstZeroedHack::<EntryTable, SIZE>::zero()
        }
    }
    pub fn load(&mut self, loader: &impl EntryLoad) {
        unsafe {
            load_fns! {
                self, loader, (create_instance, "vkCreateInstance")
                (enumerate_instance_layer_properties,
                "vkEnumerateInstanceLayerProperties")
                (enumerate_instance_extension_properties,
                "vkEnumerateInstanceExtensionProperties") (enumerate_instance_version,
                "vkEnumerateInstanceVersion")
            }
        }
    }
}
#[cfg(feature = "raw")]
impl EntryTable {
    #[track_caller]
    #[doc(alias = "vkCreateInstance")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
    pub unsafe fn create_instance(
        &self,
        p_create_info: *const crate::vk10::InstanceCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_instance: *mut crate::vk10::Instance,
    ) -> crate::vk10::Result {
        (self.create_instance.unwrap())(p_create_info, p_allocator, p_instance)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceLayerProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::LayerProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_instance_layer_properties
            .unwrap())(p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceExtensionProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        p_layer_name: *const std::os::raw::c_char,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::ExtensionProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_instance_extension_properties
            .unwrap())(p_layer_name, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
    pub unsafe fn enumerate_instance_version(
        &self,
        p_api_version: *mut u32,
    ) -> crate::vk10::Result {
        (self.enumerate_instance_version.unwrap())(p_api_version)
    }
}
impl Default for EntryTable {
    fn default() -> Self {
        Self::new_empty()
    }
}
pub struct InstanceTable {
    pub destroy_instance: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub enumerate_physical_devices: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_physical_device_count: *mut u32,
            p_physical_devices: *mut crate::vk10::PhysicalDevice,
        ) -> crate::vk10::Result,
    >,
    pub get_instance_proc_addr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_name: *const std::os::raw::c_char,
        ) -> crate::vk10::PfnVoidFunction,
    >,
    pub get_physical_device_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_properties: *mut crate::vk10::PhysicalDeviceProperties,
        ),
    >,
    pub get_physical_device_queue_family_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut crate::vk10::QueueFamilyProperties,
        ),
    >,
    pub get_physical_device_memory_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_memory_properties: *mut crate::vk10::PhysicalDeviceMemoryProperties,
        ),
    >,
    pub get_physical_device_features: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_features: *mut crate::vk10::PhysicalDeviceFeatures,
        ),
    >,
    pub get_physical_device_format_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            p_format_properties: *mut crate::vk10::FormatProperties,
        ),
    >,
    pub get_physical_device_image_format_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            kind: crate::vk10::ImageType,
            tiling: crate::vk10::ImageTiling,
            usage: crate::vk10::ImageUsageFlags,
            flags: crate::vk10::ImageCreateFlags,
            p_image_format_properties: *mut crate::vk10::ImageFormatProperties,
        ) -> crate::vk10::Result,
    >,
    pub create_device: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_create_info: *const crate::vk10::DeviceCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_device: *mut crate::vk10::Device,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_device_layer_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::LayerProperties,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_device_extension_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_layer_name: *const std::os::raw::c_char,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::ExtensionProperties,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_sparse_image_format_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            kind: crate::vk10::ImageType,
            samples: crate::vk10::SampleCountFlags,
            usage: crate::vk10::ImageUsageFlags,
            tiling: crate::vk10::ImageTiling,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::SparseImageFormatProperties,
        ),
    >,
    pub get_physical_device_features_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
        ),
    >,
    pub get_physical_device_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
        ),
    >,
    pub get_physical_device_format_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            p_format_properties: *mut crate::vk11::FormatProperties2,
        ),
    >,
    pub get_physical_device_image_format_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
            p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_queue_family_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
        ),
    >,
    pub get_physical_device_memory_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
        ),
    >,
    pub get_physical_device_sparse_image_format_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk11::SparseImageFormatProperties2,
        ),
    >,
    pub get_physical_device_external_buffer_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
            p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
        ),
    >,
    pub get_physical_device_external_semaphore_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
            p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
        ),
    >,
    pub get_physical_device_external_fence_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
            p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
        ),
    >,
    pub enumerate_physical_device_groups: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_physical_device_group_count: *mut u32,
            p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
        ) -> crate::vk10::Result,
    >,
    pub destroy_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_physical_device_surface_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_supported: *mut crate::vk10::Bool32,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_capabilities_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_surface_capabilities: *mut crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_formats_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_surface_format_count: *mut u32,
            p_surface_formats: *mut crate::extensions::khr_surface::SurfaceFormatKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_present_modes_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_present_mode_count: *mut u32,
            p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_present_rectangles_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_rect_count: *mut u32,
            p_rects: *mut crate::vk10::Rect2D,
        ) -> crate::vk10::Result,
    >,
    pub create_xlib_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_xlib_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            dpy: *mut crate::extensions::khr_xcb_surface::Display,
            visual_id: crate::extensions::khr_xcb_surface::VisualID,
        ) -> crate::vk10::Bool32,
    >,
    pub create_xcb_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_xcb_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            connection: *mut crate::extensions::khr_xcb_surface::xcb_connection_t,
            visual_id: crate::extensions::khr_xcb_surface::xcb_visualid_t,
        ) -> crate::vk10::Bool32,
    >,
    pub create_wayland_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_wayland_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            display: *mut crate::extensions::khr_wayland_surface::wl_display,
        ) -> crate::vk10::Bool32,
    >,
    pub create_android_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_win_32_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_win_32_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
        ) -> crate::vk10::Bool32,
    >,
    pub create_stream_descriptor_surface_ggp: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_features_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
        ),
    >,
    pub get_physical_device_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
        ),
    >,
    pub get_physical_device_format_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            p_format_properties: *mut crate::vk11::FormatProperties2,
        ),
    >,
    pub get_physical_device_image_format_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
            p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_queue_family_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
        ),
    >,
    pub get_physical_device_memory_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
        ),
    >,
    pub get_physical_device_sparse_image_format_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk11::SparseImageFormatProperties2,
        ),
    >,
    pub create_vi_surface_nn: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_iossurface_mvk: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_mac_ossurface_mvk: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_debug_utils_messenger_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_messenger: *mut crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
        ) -> crate::vk10::Result,
    >,
    pub destroy_debug_utils_messenger_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub submit_debug_utils_message_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
            message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
        ),
    >,
    pub create_image_pipe_surface_fuchsia: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_metal_surface_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_headless_surface_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_direct_fbsurface_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_direct_fbpresentation_support_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            dfb: *mut crate::extensions::ext_directfb_surface::IDirectFB,
        ) -> crate::vk10::Bool32,
    >,
    pub create_screen_surface_qnx: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_screen_presentation_support_qnx: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            window: *mut crate::extensions::qnx_screen_surface::_screen_window,
        ) -> crate::vk10::Bool32,
    >,
}
impl InstanceTable {
    pub const fn new_empty() -> Self {
        unsafe {
            const SIZE: usize = std::mem::size_of::<InstanceTable>();
            ConstZeroedHack::<InstanceTable, SIZE>::zero()
        }
    }
    pub fn load(&mut self, loader: &impl InstanceLoad, conf: &ApiLoadConfig) {
        unsafe {
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 0, 0)) {
                load_fns! {
                    self, loader, (destroy_instance, "vkDestroyInstance")
                    (enumerate_physical_devices, "vkEnumeratePhysicalDevices")
                    (get_instance_proc_addr, "vkGetInstanceProcAddr")
                    (get_physical_device_properties, "vkGetPhysicalDeviceProperties")
                    (get_physical_device_queue_family_properties,
                    "vkGetPhysicalDeviceQueueFamilyProperties")
                    (get_physical_device_memory_properties,
                    "vkGetPhysicalDeviceMemoryProperties") (get_physical_device_features,
                    "vkGetPhysicalDeviceFeatures")
                    (get_physical_device_format_properties,
                    "vkGetPhysicalDeviceFormatProperties")
                    (get_physical_device_image_format_properties,
                    "vkGetPhysicalDeviceImageFormatProperties") (create_device,
                    "vkCreateDevice") (enumerate_device_layer_properties,
                    "vkEnumerateDeviceLayerProperties")
                    (enumerate_device_extension_properties,
                    "vkEnumerateDeviceExtensionProperties")
                    (get_physical_device_sparse_image_format_properties,
                    "vkGetPhysicalDeviceSparseImageFormatProperties")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 1, 0)) {
                load_fns! {
                    self, loader, (get_physical_device_features_2,
                    "vkGetPhysicalDeviceFeatures2") (get_physical_device_properties_2,
                    "vkGetPhysicalDeviceProperties2")
                    (get_physical_device_format_properties_2,
                    "vkGetPhysicalDeviceFormatProperties2")
                    (get_physical_device_image_format_properties_2,
                    "vkGetPhysicalDeviceImageFormatProperties2")
                    (get_physical_device_queue_family_properties_2,
                    "vkGetPhysicalDeviceQueueFamilyProperties2")
                    (get_physical_device_memory_properties_2,
                    "vkGetPhysicalDeviceMemoryProperties2")
                    (get_physical_device_sparse_image_format_properties_2,
                    "vkGetPhysicalDeviceSparseImageFormatProperties2")
                    (get_physical_device_external_buffer_properties,
                    "vkGetPhysicalDeviceExternalBufferProperties")
                    (get_physical_device_external_semaphore_properties,
                    "vkGetPhysicalDeviceExternalSemaphoreProperties")
                    (get_physical_device_external_fence_properties,
                    "vkGetPhysicalDeviceExternalFenceProperties")
                    (enumerate_physical_device_groups, "vkEnumeratePhysicalDeviceGroups")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_surface")) {
                load_fns! {
                    self, loader, (destroy_surface_khr, "vkDestroySurfaceKHR")
                    (get_physical_device_surface_support_khr,
                    "vkGetPhysicalDeviceSurfaceSupportKHR")
                    (get_physical_device_surface_capabilities_khr,
                    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
                    (get_physical_device_surface_formats_khr,
                    "vkGetPhysicalDeviceSurfaceFormatsKHR")
                    (get_physical_device_surface_present_modes_khr,
                    "vkGetPhysicalDeviceSurfacePresentModesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_swapchain")) {
                load_fns! {
                    self, loader, (get_physical_device_present_rectangles_khr,
                    "vkGetPhysicalDevicePresentRectanglesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_xlib_surface")) {
                load_fns! {
                    self, loader, (create_xlib_surface_khr, "vkCreateXlibSurfaceKHR")
                    (get_physical_device_xlib_presentation_support_khr,
                    "vkGetPhysicalDeviceXlibPresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_xcb_surface")) {
                load_fns! {
                    self, loader, (create_xcb_surface_khr, "vkCreateXcbSurfaceKHR")
                    (get_physical_device_xcb_presentation_support_khr,
                    "vkGetPhysicalDeviceXcbPresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_wayland_surface")) {
                load_fns! {
                    self, loader, (create_wayland_surface_khr,
                    "vkCreateWaylandSurfaceKHR")
                    (get_physical_device_wayland_presentation_support_khr,
                    "vkGetPhysicalDeviceWaylandPresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_android_surface")) {
                load_fns! {
                    self, loader, (create_android_surface_khr,
                    "vkCreateAndroidSurfaceKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_win32_surface")) {
                load_fns! {
                    self, loader, (create_win_32_surface_khr, "vkCreateWin32SurfaceKHR")
                    (get_physical_device_win_32_presentation_support_khr,
                    "vkGetPhysicalDeviceWin32PresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_GGP_stream_descriptor_surface")) {
                load_fns! {
                    self, loader, (create_stream_descriptor_surface_ggp,
                    "vkCreateStreamDescriptorSurfaceGGP")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_get_physical_device_properties2")) {
                load_fns! {
                    self, loader, (get_physical_device_features_2_khr,
                    "vkGetPhysicalDeviceFeatures2KHR")
                    (get_physical_device_properties_2_khr,
                    "vkGetPhysicalDeviceProperties2KHR")
                    (get_physical_device_format_properties_2_khr,
                    "vkGetPhysicalDeviceFormatProperties2KHR")
                    (get_physical_device_image_format_properties_2_khr,
                    "vkGetPhysicalDeviceImageFormatProperties2KHR")
                    (get_physical_device_queue_family_properties_2_khr,
                    "vkGetPhysicalDeviceQueueFamilyProperties2KHR")
                    (get_physical_device_memory_properties_2_khr,
                    "vkGetPhysicalDeviceMemoryProperties2KHR")
                    (get_physical_device_sparse_image_format_properties_2_khr,
                    "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_NN_vi_surface")) {
                load_fns! {
                    self, loader, (create_vi_surface_nn, "vkCreateViSurfaceNN")
                }
            }
            if conf.extension_enabled(cstr!("VK_MVK_ios_surface")) {
                load_fns! {
                    self, loader, (create_iossurface_mvk, "vkCreateIOSSurfaceMVK")
                }
            }
            if conf.extension_enabled(cstr!("VK_MVK_macos_surface")) {
                load_fns! {
                    self, loader, (create_mac_ossurface_mvk, "vkCreateMacOSSurfaceMVK")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_debug_utils")) {
                load_fns! {
                    self, loader, (create_debug_utils_messenger_ext,
                    "vkCreateDebugUtilsMessengerEXT") (destroy_debug_utils_messenger_ext,
                    "vkDestroyDebugUtilsMessengerEXT") (submit_debug_utils_message_ext,
                    "vkSubmitDebugUtilsMessageEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_FUCHSIA_imagepipe_surface")) {
                load_fns! {
                    self, loader, (create_image_pipe_surface_fuchsia,
                    "vkCreateImagePipeSurfaceFUCHSIA")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_metal_surface")) {
                load_fns! {
                    self, loader, (create_metal_surface_ext, "vkCreateMetalSurfaceEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_headless_surface")) {
                load_fns! {
                    self, loader, (create_headless_surface_ext,
                    "vkCreateHeadlessSurfaceEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_directfb_surface")) {
                load_fns! {
                    self, loader, (create_direct_fbsurface_ext,
                    "vkCreateDirectFBSurfaceEXT")
                    (get_physical_device_direct_fbpresentation_support_ext,
                    "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_QNX_screen_surface")) {
                load_fns! {
                    self, loader, (create_screen_surface_qnx, "vkCreateScreenSurfaceQNX")
                    (get_physical_device_screen_presentation_support_qnx,
                    "vkGetPhysicalDeviceScreenPresentationSupportQNX")
                }
            }
        }
    }
}
#[cfg(feature = "raw")]
impl InstanceTable {
    #[track_caller]
    #[doc(alias = "vkDestroyInstance")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
    pub unsafe fn destroy_instance(
        &self,
        instance: crate::vk10::Instance,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_instance.unwrap())(instance, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: crate::vk10::Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut crate::vk10::PhysicalDevice,
    ) -> crate::vk10::Result {
        (self
            .enumerate_physical_devices
            .unwrap())(instance, p_physical_device_count, p_physical_devices)
    }
    #[track_caller]
    #[doc(alias = "vkGetInstanceProcAddr")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: crate::vk10::Instance,
        p_name: *const std::os::raw::c_char,
    ) -> crate::vk10::PfnVoidFunction {
        (self.get_instance_proc_addr.unwrap())(instance, p_name)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_properties: *mut crate::vk10::PhysicalDeviceProperties,
    ) {
        (self.get_physical_device_properties.unwrap())(physical_device, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk10::QueueFamilyProperties,
    ) {
        (self
            .get_physical_device_queue_family_properties
            .unwrap())(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_memory_properties: *mut crate::vk10::PhysicalDeviceMemoryProperties,
    ) {
        (self
            .get_physical_device_memory_properties
            .unwrap())(physical_device, p_memory_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_features: *mut crate::vk10::PhysicalDeviceFeatures,
    ) {
        (self.get_physical_device_features.unwrap())(physical_device, p_features)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        p_format_properties: *mut crate::vk10::FormatProperties,
    ) {
        (self
            .get_physical_device_format_properties
            .unwrap())(physical_device, format, p_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        kind: crate::vk10::ImageType,
        tiling: crate::vk10::ImageTiling,
        usage: crate::vk10::ImageUsageFlags,
        flags: crate::vk10::ImageCreateFlags,
        p_image_format_properties: *mut crate::vk10::ImageFormatProperties,
    ) -> crate::vk10::Result {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCreateDevice")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
    pub unsafe fn create_device(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_create_info: *const crate::vk10::DeviceCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_device: *mut crate::vk10::Device,
    ) -> crate::vk10::Result {
        (self
            .create_device
            .unwrap())(physical_device, p_create_info, p_allocator, p_device)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::LayerProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_device_layer_properties
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_layer_name: *const std::os::raw::c_char,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::ExtensionProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_device_extension_properties
            .unwrap())(physical_device, p_layer_name, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        kind: crate::vk10::ImageType,
        samples: crate::vk10::SampleCountFlags,
        usage: crate::vk10::ImageUsageFlags,
        tiling: crate::vk10::ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::SparseImageFormatProperties,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
    pub unsafe fn get_physical_device_features_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
    ) {
        (self.get_physical_device_features_2.unwrap())(physical_device, p_features)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
    pub unsafe fn get_physical_device_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
    ) {
        (self.get_physical_device_properties_2.unwrap())(physical_device, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
    pub unsafe fn get_physical_device_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        p_format_properties: *mut crate::vk11::FormatProperties2,
    ) {
        (self
            .get_physical_device_format_properties_2
            .unwrap())(physical_device, format, p_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
    pub unsafe fn get_physical_device_image_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_image_format_properties_2
            .unwrap())(physical_device, p_image_format_info, p_image_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
    pub unsafe fn get_physical_device_queue_family_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
    ) {
        (self
            .get_physical_device_queue_family_properties_2
            .unwrap())(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
    pub unsafe fn get_physical_device_memory_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
    ) {
        (self
            .get_physical_device_memory_properties_2
            .unwrap())(physical_device, p_memory_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk11::SparseImageFormatProperties2,
    ) {
        (self
            .get_physical_device_sparse_image_format_properties_2
            .unwrap())(physical_device, p_format_info, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
    ) {
        (self
            .get_physical_device_external_buffer_properties
            .unwrap())(
            physical_device,
            p_external_buffer_info,
            p_external_buffer_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
    ) {
        (self
            .get_physical_device_external_semaphore_properties
            .unwrap())(
            physical_device,
            p_external_semaphore_info,
            p_external_semaphore_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
    ) {
        (self
            .get_physical_device_external_fence_properties
            .unwrap())(
            physical_device,
            p_external_fence_info,
            p_external_fence_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: crate::vk10::Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_physical_device_groups
            .unwrap())(
            instance,
            p_physical_device_group_count,
            p_physical_device_group_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkDestroySurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
    pub unsafe fn destroy_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_surface_khr.unwrap())(instance, surface, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_supported: *mut crate::vk10::Bool32,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_support_khr
            .unwrap())(physical_device, queue_family_index, surface, p_supported)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_capabilities: *mut crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_capabilities_khr
            .unwrap())(physical_device, surface, p_surface_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut crate::extensions::khr_surface::SurfaceFormatKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_formats_khr
            .unwrap())(
            physical_device,
            surface,
            p_surface_format_count,
            p_surface_formats,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_present_modes_khr
            .unwrap())(physical_device, surface, p_present_mode_count, p_present_modes)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut crate::vk10::Rect2D,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_present_rectangles_khr
            .unwrap())(physical_device, surface, p_rect_count, p_rects)
    }
    #[track_caller]
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html)
    pub unsafe fn create_xlib_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_xlib_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        visual_id: crate::extensions::khr_xcb_surface::VisualID,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_xlib_presentation_support_khr
            .unwrap())(physical_device, queue_family_index, dpy, visual_id)
    }
    #[track_caller]
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html)
    pub unsafe fn create_xcb_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_xcb_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut crate::extensions::khr_xcb_surface::xcb_connection_t,
        visual_id: crate::extensions::khr_xcb_surface::xcb_visualid_t,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_xcb_presentation_support_khr
            .unwrap())(physical_device, queue_family_index, connection, visual_id)
    }
    #[track_caller]
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
    pub unsafe fn create_wayland_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_wayland_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        display: *mut crate::extensions::khr_wayland_surface::wl_display,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_wayland_presentation_support_khr
            .unwrap())(physical_device, queue_family_index, display)
    }
    #[track_caller]
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html)
    pub unsafe fn create_android_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_android_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)
    pub unsafe fn create_win_32_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_win_32_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
    pub unsafe fn get_physical_device_win_32_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_win_32_presentation_support_khr
            .unwrap())(physical_device, queue_family_index)
    }
    #[track_caller]
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html)
    pub unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_stream_descriptor_surface_ggp
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html)
    pub unsafe fn get_physical_device_features_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
    ) {
        (self.get_physical_device_features_2_khr.unwrap())(physical_device, p_features)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html)
    pub unsafe fn get_physical_device_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
    ) {
        (self
            .get_physical_device_properties_2_khr
            .unwrap())(physical_device, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        p_format_properties: *mut crate::vk11::FormatProperties2,
    ) {
        (self
            .get_physical_device_format_properties_2_khr
            .unwrap())(physical_device, format, p_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_image_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_image_format_properties_2_khr
            .unwrap())(physical_device, p_image_format_info, p_image_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
    pub unsafe fn get_physical_device_queue_family_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
    ) {
        (self
            .get_physical_device_queue_family_properties_2_khr
            .unwrap())(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html)
    pub unsafe fn get_physical_device_memory_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
    ) {
        (self
            .get_physical_device_memory_properties_2_khr
            .unwrap())(physical_device, p_memory_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk11::SparseImageFormatProperties2,
    ) {
        (self
            .get_physical_device_sparse_image_format_properties_2_khr
            .unwrap())(physical_device, p_format_info, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkCreateViSurfaceNN")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html)
    pub unsafe fn create_vi_surface_nn(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_vi_surface_nn
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html)
    pub unsafe fn create_iossurface_mvk(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_iossurface_mvk
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
    pub unsafe fn create_mac_ossurface_mvk(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_mac_ossurface_mvk
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_messenger: *mut crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
    ) -> crate::vk10::Result {
        (self
            .create_debug_utils_messenger_ext
            .unwrap())(instance, p_create_info, p_allocator, p_messenger)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        instance: crate::vk10::Instance,
        messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_debug_utils_messenger_ext
            .unwrap())(instance, messenger, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: crate::vk10::Instance,
        message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
        message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    ) {
        (self
            .submit_debug_utils_message_ext
            .unwrap())(instance, message_severity, message_types, p_callback_data)
    }
    #[track_caller]
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_image_pipe_surface_fuchsia
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html)
    pub unsafe fn create_metal_surface_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_metal_surface_ext
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
    pub unsafe fn create_headless_surface_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_headless_surface_ext
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
    pub unsafe fn create_direct_fbsurface_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_direct_fbsurface_ext
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
    pub unsafe fn get_physical_device_direct_fbpresentation_support_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut crate::extensions::ext_directfb_surface::IDirectFB,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_direct_fbpresentation_support_ext
            .unwrap())(physical_device, queue_family_index, dfb)
    }
    #[track_caller]
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html)
    pub unsafe fn create_screen_surface_qnx(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_screen_surface_qnx
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        window: *mut crate::extensions::qnx_screen_surface::_screen_window,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_screen_presentation_support_qnx
            .unwrap())(physical_device, queue_family_index, window)
    }
}
impl Default for InstanceTable {
    fn default() -> Self {
        Self::new_empty()
    }
}
pub struct DeviceTable {
    pub get_device_proc_addr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_name: *const std::os::raw::c_char,
        ) -> crate::vk10::PfnVoidFunction,
    >,
    pub destroy_device: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_device_queue: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            queue_family_index: u32,
            queue_index: u32,
            p_queue: *mut crate::vk10::Queue,
        ),
    >,
    pub queue_submit: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            submit_count: u32,
            p_submits: *const crate::vk10::SubmitInfo,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub queue_wait_idle: Option<
        unsafe extern "system" fn(queue: crate::vk10::Queue) -> crate::vk10::Result,
    >,
    pub device_wait_idle: Option<
        unsafe extern "system" fn(device: crate::vk10::Device) -> crate::vk10::Result,
    >,
    pub allocate_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocate_info: *const crate::vk10::MemoryAllocateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_memory: *mut crate::vk10::DeviceMemory,
        ) -> crate::vk10::Result,
    >,
    pub free_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub map_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            offset: crate::vk10::DeviceSize,
            size: crate::vk10::DeviceSize,
            flags: crate::vk10::MemoryMapFlags,
            pp_data: *mut *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub unmap_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
        ),
    >,
    pub flush_mapped_memory_ranges: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory_range_count: u32,
            p_memory_ranges: *const crate::vk10::MappedMemoryRange,
        ) -> crate::vk10::Result,
    >,
    pub invalidate_mapped_memory_ranges: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory_range_count: u32,
            p_memory_ranges: *const crate::vk10::MappedMemoryRange,
        ) -> crate::vk10::Result,
    >,
    pub get_device_memory_commitment: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            p_committed_memory_in_bytes: *mut crate::vk10::DeviceSize,
        ),
    >,
    pub get_buffer_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: crate::vk10::Buffer,
            p_memory_requirements: *mut crate::vk10::MemoryRequirements,
        ),
    >,
    pub bind_buffer_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: crate::vk10::Buffer,
            memory: crate::vk10::DeviceMemory,
            memory_offset: crate::vk10::DeviceSize,
        ) -> crate::vk10::Result,
    >,
    pub get_image_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_memory_requirements: *mut crate::vk10::MemoryRequirements,
        ),
    >,
    pub bind_image_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            memory: crate::vk10::DeviceMemory,
            memory_offset: crate::vk10::DeviceSize,
        ) -> crate::vk10::Result,
    >,
    pub get_image_sparse_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk10::SparseImageMemoryRequirements,
        ),
    >,
    pub queue_bind_sparse: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            bind_info_count: u32,
            p_bind_info: *const crate::vk10::BindSparseInfo,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub create_fence: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::FenceCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_fence: *mut crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub destroy_fence: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence: crate::vk10::Fence,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub reset_fences: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence_count: u32,
            p_fences: *const crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub get_fence_status: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub wait_for_fences: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence_count: u32,
            p_fences: *const crate::vk10::Fence,
            wait_all: crate::vk10::Bool32,
            timeout: u64,
        ) -> crate::vk10::Result,
    >,
    pub create_semaphore: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::SemaphoreCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_semaphore: *mut crate::vk10::Semaphore,
        ) -> crate::vk10::Result,
    >,
    pub destroy_semaphore: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            semaphore: crate::vk10::Semaphore,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::EventCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_event: *mut crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub destroy_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_event_status: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub set_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub reset_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub create_query_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::QueryPoolCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_query_pool: *mut crate::vk10::QueryPool,
        ) -> crate::vk10::Result,
    >,
    pub destroy_query_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            query_pool: crate::vk10::QueryPool,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_query_pool_results: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
            stride: crate::vk10::DeviceSize,
            flags: crate::vk10::QueryResultFlags,
        ) -> crate::vk10::Result,
    >,
    pub create_buffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::BufferCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_buffer: *mut crate::vk10::Buffer,
        ) -> crate::vk10::Result,
    >,
    pub destroy_buffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: crate::vk10::Buffer,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_buffer_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::BufferViewCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_view: *mut crate::vk10::BufferView,
        ) -> crate::vk10::Result,
    >,
    pub destroy_buffer_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer_view: crate::vk10::BufferView,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_image: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ImageCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_image: *mut crate::vk10::Image,
        ) -> crate::vk10::Result,
    >,
    pub destroy_image: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_image_subresource_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_subresource: *const crate::vk10::ImageSubresource,
            p_layout: *mut crate::vk10::SubresourceLayout,
        ),
    >,
    pub create_image_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ImageViewCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_view: *mut crate::vk10::ImageView,
        ) -> crate::vk10::Result,
    >,
    pub destroy_image_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image_view: crate::vk10::ImageView,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_shader_module: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_shader_module: *mut crate::vk10::ShaderModule,
        ) -> crate::vk10::Result,
    >,
    pub destroy_shader_module: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            shader_module: crate::vk10::ShaderModule,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_pipeline_cache: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::PipelineCacheCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipeline_cache: *mut crate::vk10::PipelineCache,
        ) -> crate::vk10::Result,
    >,
    pub destroy_pipeline_cache: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_pipeline_cache_data: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            p_data_size: *mut usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub merge_pipeline_caches: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            dst_cache: crate::vk10::PipelineCache,
            src_cache_count: u32,
            p_src_caches: *const crate::vk10::PipelineCache,
        ) -> crate::vk10::Result,
    >,
    pub create_graphics_pipelines: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            create_info_count: u32,
            p_create_infos: *const crate::vk10::GraphicsPipelineCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipelines: *mut crate::vk10::Pipeline,
        ) -> crate::vk10::Result,
    >,
    pub create_compute_pipelines: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            create_info_count: u32,
            p_create_infos: *const crate::vk10::ComputePipelineCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipelines: *mut crate::vk10::Pipeline,
        ) -> crate::vk10::Result,
    >,
    pub destroy_pipeline: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_pipeline_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::PipelineLayoutCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipeline_layout: *mut crate::vk10::PipelineLayout,
        ) -> crate::vk10::Result,
    >,
    pub destroy_pipeline_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_layout: crate::vk10::PipelineLayout,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_sampler: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::SamplerCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_sampler: *mut crate::vk10::Sampler,
        ) -> crate::vk10::Result,
    >,
    pub destroy_sampler: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            sampler: crate::vk10::Sampler,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_descriptor_set_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_set_layout: *mut crate::vk10::DescriptorSetLayout,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_set_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_set_layout: crate::vk10::DescriptorSetLayout,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_descriptor_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorPoolCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_descriptor_pool: *mut crate::vk10::DescriptorPool,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_pool: crate::vk10::DescriptorPool,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub reset_descriptor_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_pool: crate::vk10::DescriptorPool,
            flags: crate::vk10::DescriptorPoolResetFlags,
        ) -> crate::vk10::Result,
    >,
    pub allocate_descriptor_sets: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocate_info: *const crate::vk10::DescriptorSetAllocateInfo,
            p_descriptor_sets: *mut crate::vk10::DescriptorSet,
        ) -> crate::vk10::Result,
    >,
    pub free_descriptor_sets: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_pool: crate::vk10::DescriptorPool,
            descriptor_set_count: u32,
            p_descriptor_sets: *const crate::vk10::DescriptorSet,
        ) -> crate::vk10::Result,
    >,
    pub update_descriptor_sets: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_write_count: u32,
            p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
            descriptor_copy_count: u32,
            p_descriptor_copies: *const crate::vk10::CopyDescriptorSet,
        ),
    >,
    pub create_framebuffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::FramebufferCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_framebuffer: *mut crate::vk10::Framebuffer,
        ) -> crate::vk10::Result,
    >,
    pub destroy_framebuffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            framebuffer: crate::vk10::Framebuffer,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_render_pass: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::RenderPassCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_render_pass: *mut crate::vk10::RenderPass,
        ) -> crate::vk10::Result,
    >,
    pub destroy_render_pass: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            render_pass: crate::vk10::RenderPass,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_render_area_granularity: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            render_pass: crate::vk10::RenderPass,
            p_granularity: *mut crate::vk10::Extent2D,
        ),
    >,
    pub create_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::CommandPoolCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_command_pool: *mut crate::vk10::CommandPool,
        ) -> crate::vk10::Result,
    >,
    pub destroy_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub reset_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            flags: crate::vk10::CommandPoolResetFlags,
        ) -> crate::vk10::Result,
    >,
    pub allocate_command_buffers: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocate_info: *const crate::vk10::CommandBufferAllocateInfo,
            p_command_buffers: *mut crate::vk10::CommandBuffer,
        ) -> crate::vk10::Result,
    >,
    pub free_command_buffers: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            command_buffer_count: u32,
            p_command_buffers: *const crate::vk10::CommandBuffer,
        ),
    >,
    pub begin_command_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_begin_info: *const crate::vk10::CommandBufferBeginInfo,
        ) -> crate::vk10::Result,
    >,
    pub end_command_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
        ) -> crate::vk10::Result,
    >,
    pub reset_command_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            flags: crate::vk10::CommandBufferResetFlags,
        ) -> crate::vk10::Result,
    >,
    pub cmd_bind_pipeline: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_bind_point: crate::vk10::PipelineBindPoint,
            pipeline: crate::vk10::Pipeline,
        ),
    >,
    pub cmd_set_viewport: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_viewport: u32,
            viewport_count: u32,
            p_viewports: *const crate::vk10::Viewport,
        ),
    >,
    pub cmd_set_scissor: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_scissor: u32,
            scissor_count: u32,
            p_scissors: *const crate::vk10::Rect2D,
        ),
    >,
    pub cmd_set_line_width: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            line_width: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_depth_bias: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_bias_constant_factor: std::os::raw::c_float,
            depth_bias_clamp: std::os::raw::c_float,
            depth_bias_slope_factor: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_blend_constants: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            blend_constants: *const [std::os::raw::c_float; 4],
        ),
    >,
    pub cmd_set_depth_bounds: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            min_depth_bounds: std::os::raw::c_float,
            max_depth_bounds: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_stencil_compare_mask: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            compare_mask: u32,
        ),
    >,
    pub cmd_set_stencil_write_mask: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            write_mask: u32,
        ),
    >,
    pub cmd_set_stencil_reference: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            reference: u32,
        ),
    >,
    pub cmd_bind_descriptor_sets: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_bind_point: crate::vk10::PipelineBindPoint,
            layout: crate::vk10::PipelineLayout,
            first_set: u32,
            descriptor_set_count: u32,
            p_descriptor_sets: *const crate::vk10::DescriptorSet,
            dynamic_offset_count: u32,
            p_dynamic_offsets: *const u32,
        ),
    >,
    pub cmd_bind_index_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            index_type: crate::vk10::IndexType,
        ),
    >,
    pub cmd_bind_vertex_buffers: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_binding: u32,
            binding_count: u32,
            p_buffers: *const crate::vk10::Buffer,
            p_offsets: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_draw: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            vertex_count: u32,
            instance_count: u32,
            first_vertex: u32,
            first_instance: u32,
        ),
    >,
    pub cmd_draw_indexed: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            index_count: u32,
            instance_count: u32,
            first_index: u32,
            vertex_offset: i32,
            first_instance: u32,
        ),
    >,
    pub cmd_draw_indirect: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_indexed_indirect: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_dispatch: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        ),
    >,
    pub cmd_dispatch_indirect: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_copy_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_buffer: crate::vk10::Buffer,
            dst_buffer: crate::vk10::Buffer,
            region_count: u32,
            p_regions: *const crate::vk10::BufferCopy,
        ),
    >,
    pub cmd_copy_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::ImageCopy,
        ),
    >,
    pub cmd_blit_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::ImageBlit,
            filter: crate::vk10::Filter,
        ),
    >,
    pub cmd_copy_buffer_to_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_buffer: crate::vk10::Buffer,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::BufferImageCopy,
        ),
    >,
    pub cmd_copy_image_to_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_buffer: crate::vk10::Buffer,
            region_count: u32,
            p_regions: *const crate::vk10::BufferImageCopy,
        ),
    >,
    pub cmd_update_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            data_size: crate::vk10::DeviceSize,
            p_data: *const std::os::raw::c_void,
        ),
    >,
    pub cmd_fill_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            size: crate::vk10::DeviceSize,
            data: u32,
        ),
    >,
    pub cmd_clear_color_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            image: crate::vk10::Image,
            image_layout: crate::vk10::ImageLayout,
            p_color: *const crate::vk10::ClearColorValue,
            range_count: u32,
            p_ranges: *const crate::vk10::ImageSubresourceRange,
        ),
    >,
    pub cmd_clear_depth_stencil_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            image: crate::vk10::Image,
            image_layout: crate::vk10::ImageLayout,
            p_depth_stencil: *const crate::vk10::ClearDepthStencilValue,
            range_count: u32,
            p_ranges: *const crate::vk10::ImageSubresourceRange,
        ),
    >,
    pub cmd_clear_attachments: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            attachment_count: u32,
            p_attachments: *const crate::vk10::ClearAttachment,
            rect_count: u32,
            p_rects: *const crate::vk10::ClearRect,
        ),
    >,
    pub cmd_resolve_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::ImageResolve,
        ),
    >,
    pub cmd_set_event: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::vk10::PipelineStageFlags,
        ),
    >,
    pub cmd_reset_event: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::vk10::PipelineStageFlags,
        ),
    >,
    pub cmd_wait_events: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event_count: u32,
            p_events: *const crate::vk10::Event,
            src_stage_mask: crate::vk10::PipelineStageFlags,
            dst_stage_mask: crate::vk10::PipelineStageFlags,
            memory_barrier_count: u32,
            p_memory_barriers: *const crate::vk10::MemoryBarrier,
            buffer_memory_barrier_count: u32,
            p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
            image_memory_barrier_count: u32,
            p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
        ),
    >,
    pub cmd_pipeline_barrier: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_stage_mask: crate::vk10::PipelineStageFlags,
            dst_stage_mask: crate::vk10::PipelineStageFlags,
            dependency_flags: crate::vk10::DependencyFlags,
            memory_barrier_count: u32,
            p_memory_barriers: *const crate::vk10::MemoryBarrier,
            buffer_memory_barrier_count: u32,
            p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
            image_memory_barrier_count: u32,
            p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
        ),
    >,
    pub cmd_begin_query: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            query: u32,
            flags: crate::vk10::QueryControlFlags,
        ),
    >,
    pub cmd_end_query: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
    pub cmd_reset_query_pool: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
        ),
    >,
    pub cmd_write_timestamp: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_stage: crate::vk10::PipelineStageFlags,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
    pub cmd_copy_query_pool_results: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            stride: crate::vk10::DeviceSize,
            flags: crate::vk10::QueryResultFlags,
        ),
    >,
    pub cmd_push_constants: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            layout: crate::vk10::PipelineLayout,
            stage_flags: crate::vk10::ShaderStageFlags,
            offset: u32,
            size: u32,
            p_values: *const std::os::raw::c_void,
        ),
    >,
    pub cmd_begin_render_pass: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
            contents: crate::vk10::SubpassContents,
        ),
    >,
    pub cmd_next_subpass: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            contents: crate::vk10::SubpassContents,
        ),
    >,
    pub cmd_end_render_pass: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub cmd_execute_commands: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            command_buffer_count: u32,
            p_command_buffers: *const crate::vk10::CommandBuffer,
        ),
    >,
    pub trim_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            flags: crate::vk11::CommandPoolTrimFlags,
        ),
    >,
    pub get_device_group_peer_memory_features: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            heap_index: u32,
            local_device_index: u32,
            remote_device_index: u32,
            p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
        ),
    >,
    pub bind_buffer_memory_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
        ) -> crate::vk10::Result,
    >,
    pub bind_image_memory_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_device_mask: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            device_mask: u32,
        ),
    >,
    pub cmd_dispatch_base: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            base_group_x: u32,
            base_group_y: u32,
            base_group_z: u32,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        ),
    >,
    pub create_descriptor_update_template: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_update_template: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub update_descriptor_set_with_template: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_set: crate::vk10::DescriptorSet,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            p_data: *const std::os::raw::c_void,
        ),
    >,
    pub get_buffer_memory_requirements_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_image_memory_requirements_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_image_sparse_memory_requirements_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
        ),
    >,
    pub create_sampler_ycbcr_conversion: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
        ) -> crate::vk10::Result,
    >,
    pub destroy_sampler_ycbcr_conversion: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_device_queue_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_queue_info: *const crate::vk11::DeviceQueueInfo2,
            p_queue: *mut crate::vk10::Queue,
        ),
    >,
    pub get_descriptor_set_layout_support: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
            p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
        ),
    >,
    pub create_swapchain_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_swapchain: *mut crate::extensions::khr_swapchain::SwapchainKHR,
        ) -> crate::vk10::Result,
    >,
    pub destroy_swapchain_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_swapchain_images_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            p_swapchain_image_count: *mut u32,
            p_swapchain_images: *mut crate::vk10::Image,
        ) -> crate::vk10::Result,
    >,
    pub acquire_next_image_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            timeout: u64,
            semaphore: crate::vk10::Semaphore,
            fence: crate::vk10::Fence,
            p_image_index: *mut u32,
        ) -> crate::vk10::Result,
    >,
    pub queue_present_khr: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_present_info: *const crate::extensions::khr_swapchain::PresentInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_device_group_present_capabilities_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_device_group_present_capabilities: *mut crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_device_group_surface_present_modes_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_next_image_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_acquire_info: *const crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
            p_image_index: *mut u32,
        ) -> crate::vk10::Result,
    >,
    pub cmd_begin_rendering_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_rendering_info: *const crate::extensions::khr_dynamic_rendering::RenderingInfoKHR,
        ),
    >,
    pub cmd_end_rendering_khr: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub create_render_pass_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::khr_create_renderpass2::RenderPassCreateInfo2KHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_render_pass: *mut crate::vk10::RenderPass,
        ) -> crate::vk10::Result,
    >,
    pub cmd_begin_render_pass_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
            p_subpass_begin_info: *const crate::extensions::khr_create_renderpass2::SubpassBeginInfoKHR,
        ),
    >,
    pub cmd_next_subpass_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_subpass_begin_info: *const crate::extensions::khr_create_renderpass2::SubpassBeginInfoKHR,
            p_subpass_end_info: *const crate::extensions::khr_create_renderpass2::SubpassEndInfoKHR,
        ),
    >,
    pub cmd_end_render_pass_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_subpass_end_info: *const crate::extensions::khr_create_renderpass2::SubpassEndInfoKHR,
        ),
    >,
    pub set_debug_utils_object_name_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_name_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub set_debug_utils_object_tag_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_tag_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub queue_begin_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub queue_end_debug_utils_label_ext: Option<
        unsafe extern "system" fn(queue: crate::vk10::Queue),
    >,
    pub queue_insert_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub cmd_begin_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub cmd_end_debug_utils_label_ext: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub cmd_insert_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub get_semaphore_counter_value_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            semaphore: crate::vk10::Semaphore,
            p_value: *mut u64,
        ) -> crate::vk10::Result,
    >,
    pub wait_semaphores_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_wait_info: *const crate::extensions::khr_timeline_semaphore::SemaphoreWaitInfoKHR,
            timeout: u64,
        ) -> crate::vk10::Result,
    >,
    pub signal_semaphore_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_signal_info: *const crate::extensions::khr_timeline_semaphore::SemaphoreSignalInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_event_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            p_dependency_info: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
        ),
    >,
    pub cmd_reset_event_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
        ),
    >,
    pub cmd_wait_events_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event_count: u32,
            p_events: *const crate::vk10::Event,
            p_dependency_infos: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
        ),
    >,
    pub cmd_pipeline_barrier_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_dependency_info: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
        ),
    >,
    pub queue_submit_2_khr: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            submit_count: u32,
            p_submits: *const crate::extensions::khr_synchronization2::SubmitInfo2KHR,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub cmd_write_timestamp_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
}
impl DeviceTable {
    pub const fn new_empty() -> Self {
        unsafe {
            const SIZE: usize = std::mem::size_of::<DeviceTable>();
            ConstZeroedHack::<DeviceTable, SIZE>::zero()
        }
    }
    pub fn load(&mut self, loader: &impl DeviceLoad, conf: &ApiLoadConfig) {
        unsafe {
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 0, 0)) {
                load_fns! {
                    self, loader, (get_device_proc_addr, "vkGetDeviceProcAddr")
                    (destroy_device, "vkDestroyDevice") (get_device_queue,
                    "vkGetDeviceQueue") (queue_submit, "vkQueueSubmit") (queue_wait_idle,
                    "vkQueueWaitIdle") (device_wait_idle, "vkDeviceWaitIdle")
                    (allocate_memory, "vkAllocateMemory") (free_memory, "vkFreeMemory")
                    (map_memory, "vkMapMemory") (unmap_memory, "vkUnmapMemory")
                    (flush_mapped_memory_ranges, "vkFlushMappedMemoryRanges")
                    (invalidate_mapped_memory_ranges, "vkInvalidateMappedMemoryRanges")
                    (get_device_memory_commitment, "vkGetDeviceMemoryCommitment")
                    (get_buffer_memory_requirements, "vkGetBufferMemoryRequirements")
                    (bind_buffer_memory, "vkBindBufferMemory")
                    (get_image_memory_requirements, "vkGetImageMemoryRequirements")
                    (bind_image_memory, "vkBindImageMemory")
                    (get_image_sparse_memory_requirements,
                    "vkGetImageSparseMemoryRequirements") (queue_bind_sparse,
                    "vkQueueBindSparse") (create_fence, "vkCreateFence") (destroy_fence,
                    "vkDestroyFence") (reset_fences, "vkResetFences") (get_fence_status,
                    "vkGetFenceStatus") (wait_for_fences, "vkWaitForFences")
                    (create_semaphore, "vkCreateSemaphore") (destroy_semaphore,
                    "vkDestroySemaphore") (create_event, "vkCreateEvent") (destroy_event,
                    "vkDestroyEvent") (get_event_status, "vkGetEventStatus") (set_event,
                    "vkSetEvent") (reset_event, "vkResetEvent") (create_query_pool,
                    "vkCreateQueryPool") (destroy_query_pool, "vkDestroyQueryPool")
                    (get_query_pool_results, "vkGetQueryPoolResults") (create_buffer,
                    "vkCreateBuffer") (destroy_buffer, "vkDestroyBuffer")
                    (create_buffer_view, "vkCreateBufferView") (destroy_buffer_view,
                    "vkDestroyBufferView") (create_image, "vkCreateImage")
                    (destroy_image, "vkDestroyImage") (get_image_subresource_layout,
                    "vkGetImageSubresourceLayout") (create_image_view,
                    "vkCreateImageView") (destroy_image_view, "vkDestroyImageView")
                    (create_shader_module, "vkCreateShaderModule")
                    (destroy_shader_module, "vkDestroyShaderModule")
                    (create_pipeline_cache, "vkCreatePipelineCache")
                    (destroy_pipeline_cache, "vkDestroyPipelineCache")
                    (get_pipeline_cache_data, "vkGetPipelineCacheData")
                    (merge_pipeline_caches, "vkMergePipelineCaches")
                    (create_graphics_pipelines, "vkCreateGraphicsPipelines")
                    (create_compute_pipelines, "vkCreateComputePipelines")
                    (destroy_pipeline, "vkDestroyPipeline") (create_pipeline_layout,
                    "vkCreatePipelineLayout") (destroy_pipeline_layout,
                    "vkDestroyPipelineLayout") (create_sampler, "vkCreateSampler")
                    (destroy_sampler, "vkDestroySampler") (create_descriptor_set_layout,
                    "vkCreateDescriptorSetLayout") (destroy_descriptor_set_layout,
                    "vkDestroyDescriptorSetLayout") (create_descriptor_pool,
                    "vkCreateDescriptorPool") (destroy_descriptor_pool,
                    "vkDestroyDescriptorPool") (reset_descriptor_pool,
                    "vkResetDescriptorPool") (allocate_descriptor_sets,
                    "vkAllocateDescriptorSets") (free_descriptor_sets,
                    "vkFreeDescriptorSets") (update_descriptor_sets,
                    "vkUpdateDescriptorSets") (create_framebuffer, "vkCreateFramebuffer")
                    (destroy_framebuffer, "vkDestroyFramebuffer") (create_render_pass,
                    "vkCreateRenderPass") (destroy_render_pass, "vkDestroyRenderPass")
                    (get_render_area_granularity, "vkGetRenderAreaGranularity")
                    (create_command_pool, "vkCreateCommandPool") (destroy_command_pool,
                    "vkDestroyCommandPool") (reset_command_pool, "vkResetCommandPool")
                    (allocate_command_buffers, "vkAllocateCommandBuffers")
                    (free_command_buffers, "vkFreeCommandBuffers") (begin_command_buffer,
                    "vkBeginCommandBuffer") (end_command_buffer, "vkEndCommandBuffer")
                    (reset_command_buffer, "vkResetCommandBuffer") (cmd_bind_pipeline,
                    "vkCmdBindPipeline") (cmd_set_viewport, "vkCmdSetViewport")
                    (cmd_set_scissor, "vkCmdSetScissor") (cmd_set_line_width,
                    "vkCmdSetLineWidth") (cmd_set_depth_bias, "vkCmdSetDepthBias")
                    (cmd_set_blend_constants, "vkCmdSetBlendConstants")
                    (cmd_set_depth_bounds, "vkCmdSetDepthBounds")
                    (cmd_set_stencil_compare_mask, "vkCmdSetStencilCompareMask")
                    (cmd_set_stencil_write_mask, "vkCmdSetStencilWriteMask")
                    (cmd_set_stencil_reference, "vkCmdSetStencilReference")
                    (cmd_bind_descriptor_sets, "vkCmdBindDescriptorSets")
                    (cmd_bind_index_buffer, "vkCmdBindIndexBuffer")
                    (cmd_bind_vertex_buffers, "vkCmdBindVertexBuffers") (cmd_draw,
                    "vkCmdDraw") (cmd_draw_indexed, "vkCmdDrawIndexed")
                    (cmd_draw_indirect, "vkCmdDrawIndirect") (cmd_draw_indexed_indirect,
                    "vkCmdDrawIndexedIndirect") (cmd_dispatch, "vkCmdDispatch")
                    (cmd_dispatch_indirect, "vkCmdDispatchIndirect") (cmd_copy_buffer,
                    "vkCmdCopyBuffer") (cmd_copy_image, "vkCmdCopyImage")
                    (cmd_blit_image, "vkCmdBlitImage") (cmd_copy_buffer_to_image,
                    "vkCmdCopyBufferToImage") (cmd_copy_image_to_buffer,
                    "vkCmdCopyImageToBuffer") (cmd_update_buffer, "vkCmdUpdateBuffer")
                    (cmd_fill_buffer, "vkCmdFillBuffer") (cmd_clear_color_image,
                    "vkCmdClearColorImage") (cmd_clear_depth_stencil_image,
                    "vkCmdClearDepthStencilImage") (cmd_clear_attachments,
                    "vkCmdClearAttachments") (cmd_resolve_image, "vkCmdResolveImage")
                    (cmd_set_event, "vkCmdSetEvent") (cmd_reset_event, "vkCmdResetEvent")
                    (cmd_wait_events, "vkCmdWaitEvents") (cmd_pipeline_barrier,
                    "vkCmdPipelineBarrier") (cmd_begin_query, "vkCmdBeginQuery")
                    (cmd_end_query, "vkCmdEndQuery") (cmd_reset_query_pool,
                    "vkCmdResetQueryPool") (cmd_write_timestamp, "vkCmdWriteTimestamp")
                    (cmd_copy_query_pool_results, "vkCmdCopyQueryPoolResults")
                    (cmd_push_constants, "vkCmdPushConstants") (cmd_begin_render_pass,
                    "vkCmdBeginRenderPass") (cmd_next_subpass, "vkCmdNextSubpass")
                    (cmd_end_render_pass, "vkCmdEndRenderPass") (cmd_execute_commands,
                    "vkCmdExecuteCommands")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 1, 0)) {
                load_fns! {
                    self, loader, (trim_command_pool, "vkTrimCommandPool")
                    (get_device_group_peer_memory_features,
                    "vkGetDeviceGroupPeerMemoryFeatures") (bind_buffer_memory_2,
                    "vkBindBufferMemory2") (bind_image_memory_2, "vkBindImageMemory2")
                    (cmd_set_device_mask, "vkCmdSetDeviceMask") (cmd_dispatch_base,
                    "vkCmdDispatchBase") (create_descriptor_update_template,
                    "vkCreateDescriptorUpdateTemplate")
                    (destroy_descriptor_update_template,
                    "vkDestroyDescriptorUpdateTemplate")
                    (update_descriptor_set_with_template,
                    "vkUpdateDescriptorSetWithTemplate")
                    (get_buffer_memory_requirements_2, "vkGetBufferMemoryRequirements2")
                    (get_image_memory_requirements_2, "vkGetImageMemoryRequirements2")
                    (get_image_sparse_memory_requirements_2,
                    "vkGetImageSparseMemoryRequirements2")
                    (create_sampler_ycbcr_conversion, "vkCreateSamplerYcbcrConversion")
                    (destroy_sampler_ycbcr_conversion, "vkDestroySamplerYcbcrConversion")
                    (get_device_queue_2, "vkGetDeviceQueue2")
                    (get_descriptor_set_layout_support,
                    "vkGetDescriptorSetLayoutSupport")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_swapchain")) {
                load_fns! {
                    self, loader, (create_swapchain_khr, "vkCreateSwapchainKHR")
                    (destroy_swapchain_khr, "vkDestroySwapchainKHR")
                    (get_swapchain_images_khr, "vkGetSwapchainImagesKHR")
                    (acquire_next_image_khr, "vkAcquireNextImageKHR") (queue_present_khr,
                    "vkQueuePresentKHR") (get_device_group_present_capabilities_khr,
                    "vkGetDeviceGroupPresentCapabilitiesKHR")
                    (get_device_group_surface_present_modes_khr,
                    "vkGetDeviceGroupSurfacePresentModesKHR") (acquire_next_image_2_khr,
                    "vkAcquireNextImage2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_dynamic_rendering")) {
                load_fns! {
                    self, loader, (cmd_begin_rendering_khr, "vkCmdBeginRenderingKHR")
                    (cmd_end_rendering_khr, "vkCmdEndRenderingKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_create_renderpass2")) {
                load_fns! {
                    self, loader, (create_render_pass_2_khr, "vkCreateRenderPass2KHR")
                    (cmd_begin_render_pass_2_khr, "vkCmdBeginRenderPass2KHR")
                    (cmd_next_subpass_2_khr, "vkCmdNextSubpass2KHR")
                    (cmd_end_render_pass_2_khr, "vkCmdEndRenderPass2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_debug_utils")) {
                load_fns! {
                    self, loader, (set_debug_utils_object_name_ext,
                    "vkSetDebugUtilsObjectNameEXT") (set_debug_utils_object_tag_ext,
                    "vkSetDebugUtilsObjectTagEXT") (queue_begin_debug_utils_label_ext,
                    "vkQueueBeginDebugUtilsLabelEXT") (queue_end_debug_utils_label_ext,
                    "vkQueueEndDebugUtilsLabelEXT") (queue_insert_debug_utils_label_ext,
                    "vkQueueInsertDebugUtilsLabelEXT") (cmd_begin_debug_utils_label_ext,
                    "vkCmdBeginDebugUtilsLabelEXT") (cmd_end_debug_utils_label_ext,
                    "vkCmdEndDebugUtilsLabelEXT") (cmd_insert_debug_utils_label_ext,
                    "vkCmdInsertDebugUtilsLabelEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_timeline_semaphore")) {
                load_fns! {
                    self, loader, (get_semaphore_counter_value_khr,
                    "vkGetSemaphoreCounterValueKHR") (wait_semaphores_khr,
                    "vkWaitSemaphoresKHR") (signal_semaphore_khr, "vkSignalSemaphoreKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_synchronization2")) {
                load_fns! {
                    self, loader, (cmd_set_event_2_khr, "vkCmdSetEvent2KHR")
                    (cmd_reset_event_2_khr, "vkCmdResetEvent2KHR")
                    (cmd_wait_events_2_khr, "vkCmdWaitEvents2KHR")
                    (cmd_pipeline_barrier_2_khr, "vkCmdPipelineBarrier2KHR")
                    (queue_submit_2_khr, "vkQueueSubmit2KHR") (cmd_write_timestamp_2_khr,
                    "vkCmdWriteTimestamp2KHR")
                }
            }
        }
    }
}
#[cfg(feature = "raw")]
impl DeviceTable {
    #[track_caller]
    #[doc(alias = "vkGetDeviceProcAddr")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
    pub unsafe fn get_device_proc_addr(
        &self,
        device: crate::vk10::Device,
        p_name: *const std::os::raw::c_char,
    ) -> crate::vk10::PfnVoidFunction {
        (self.get_device_proc_addr.unwrap())(device, p_name)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDevice")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
    pub unsafe fn destroy_device(
        &self,
        device: crate::vk10::Device,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_device.unwrap())(device, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceQueue")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
    pub unsafe fn get_device_queue(
        &self,
        device: crate::vk10::Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut crate::vk10::Queue,
    ) {
        (self
            .get_device_queue
            .unwrap())(device, queue_family_index, queue_index, p_queue)
    }
    #[track_caller]
    #[doc(alias = "vkQueueSubmit")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
    pub unsafe fn queue_submit(
        &self,
        queue: crate::vk10::Queue,
        submit_count: u32,
        p_submits: *const crate::vk10::SubmitInfo,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_submit.unwrap())(queue, submit_count, p_submits, fence)
    }
    #[track_caller]
    #[doc(alias = "vkQueueWaitIdle")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
    pub unsafe fn queue_wait_idle(
        &self,
        queue: crate::vk10::Queue,
    ) -> crate::vk10::Result {
        (self.queue_wait_idle.unwrap())(queue)
    }
    #[track_caller]
    #[doc(alias = "vkDeviceWaitIdle")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
    pub unsafe fn device_wait_idle(
        &self,
        device: crate::vk10::Device,
    ) -> crate::vk10::Result {
        (self.device_wait_idle.unwrap())(device)
    }
    #[track_caller]
    #[doc(alias = "vkAllocateMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
    pub unsafe fn allocate_memory(
        &self,
        device: crate::vk10::Device,
        p_allocate_info: *const crate::vk10::MemoryAllocateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_memory: *mut crate::vk10::DeviceMemory,
    ) -> crate::vk10::Result {
        (self.allocate_memory.unwrap())(device, p_allocate_info, p_allocator, p_memory)
    }
    #[track_caller]
    #[doc(alias = "vkFreeMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
    pub unsafe fn free_memory(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.free_memory.unwrap())(device, memory, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkMapMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
    pub unsafe fn map_memory(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        offset: crate::vk10::DeviceSize,
        size: crate::vk10::DeviceSize,
        flags: crate::vk10::MemoryMapFlags,
        pp_data: *mut *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self.map_memory.unwrap())(device, memory, offset, size, flags, pp_data)
    }
    #[track_caller]
    #[doc(alias = "vkUnmapMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
    pub unsafe fn unmap_memory(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
    ) {
        (self.unmap_memory.unwrap())(device, memory)
    }
    #[track_caller]
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: crate::vk10::Device,
        memory_range_count: u32,
        p_memory_ranges: *const crate::vk10::MappedMemoryRange,
    ) -> crate::vk10::Result {
        (self
            .flush_mapped_memory_ranges
            .unwrap())(device, memory_range_count, p_memory_ranges)
    }
    #[track_caller]
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: crate::vk10::Device,
        memory_range_count: u32,
        p_memory_ranges: *const crate::vk10::MappedMemoryRange,
    ) -> crate::vk10::Result {
        (self
            .invalidate_mapped_memory_ranges
            .unwrap())(device, memory_range_count, p_memory_ranges)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        p_committed_memory_in_bytes: *mut crate::vk10::DeviceSize,
    ) {
        (self
            .get_device_memory_commitment
            .unwrap())(device, memory, p_committed_memory_in_bytes)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: crate::vk10::Device,
        buffer: crate::vk10::Buffer,
        p_memory_requirements: *mut crate::vk10::MemoryRequirements,
    ) {
        (self
            .get_buffer_memory_requirements
            .unwrap())(device, buffer, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkBindBufferMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
    pub unsafe fn bind_buffer_memory(
        &self,
        device: crate::vk10::Device,
        buffer: crate::vk10::Buffer,
        memory: crate::vk10::DeviceMemory,
        memory_offset: crate::vk10::DeviceSize,
    ) -> crate::vk10::Result {
        (self.bind_buffer_memory.unwrap())(device, buffer, memory, memory_offset)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_memory_requirements: *mut crate::vk10::MemoryRequirements,
    ) {
        (self
            .get_image_memory_requirements
            .unwrap())(device, image, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkBindImageMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
    pub unsafe fn bind_image_memory(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        memory: crate::vk10::DeviceMemory,
        memory_offset: crate::vk10::DeviceSize,
    ) -> crate::vk10::Result {
        (self.bind_image_memory.unwrap())(device, image, memory, memory_offset)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk10::SparseImageMemoryRequirements,
    ) {
        (self
            .get_image_sparse_memory_requirements
            .unwrap())(
            device,
            image,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkQueueBindSparse")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: crate::vk10::Queue,
        bind_info_count: u32,
        p_bind_info: *const crate::vk10::BindSparseInfo,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_bind_sparse.unwrap())(queue, bind_info_count, p_bind_info, fence)
    }
    #[track_caller]
    #[doc(alias = "vkCreateFence")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
    pub unsafe fn create_fence(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::FenceCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_fence: *mut crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.create_fence.unwrap())(device, p_create_info, p_allocator, p_fence)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyFence")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
    pub unsafe fn destroy_fence(
        &self,
        device: crate::vk10::Device,
        fence: crate::vk10::Fence,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_fence.unwrap())(device, fence, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkResetFences")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
    pub unsafe fn reset_fences(
        &self,
        device: crate::vk10::Device,
        fence_count: u32,
        p_fences: *const crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.reset_fences.unwrap())(device, fence_count, p_fences)
    }
    #[track_caller]
    #[doc(alias = "vkGetFenceStatus")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
    pub unsafe fn get_fence_status(
        &self,
        device: crate::vk10::Device,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.get_fence_status.unwrap())(device, fence)
    }
    #[track_caller]
    #[doc(alias = "vkWaitForFences")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
    pub unsafe fn wait_for_fences(
        &self,
        device: crate::vk10::Device,
        fence_count: u32,
        p_fences: *const crate::vk10::Fence,
        wait_all: crate::vk10::Bool32,
        timeout: u64,
    ) -> crate::vk10::Result {
        (self.wait_for_fences.unwrap())(device, fence_count, p_fences, wait_all, timeout)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
    pub unsafe fn create_semaphore(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::SemaphoreCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_semaphore: *mut crate::vk10::Semaphore,
    ) -> crate::vk10::Result {
        (self.create_semaphore.unwrap())(device, p_create_info, p_allocator, p_semaphore)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
    pub unsafe fn destroy_semaphore(
        &self,
        device: crate::vk10::Device,
        semaphore: crate::vk10::Semaphore,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_semaphore.unwrap())(device, semaphore, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
    pub unsafe fn create_event(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::EventCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_event: *mut crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.create_event.unwrap())(device, p_create_info, p_allocator, p_event)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
    pub unsafe fn destroy_event(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_event.unwrap())(device, event, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetEventStatus")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
    pub unsafe fn get_event_status(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.get_event_status.unwrap())(device, event)
    }
    #[track_caller]
    #[doc(alias = "vkSetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
    pub unsafe fn set_event(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.set_event.unwrap())(device, event)
    }
    #[track_caller]
    #[doc(alias = "vkResetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
    pub unsafe fn reset_event(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.reset_event.unwrap())(device, event)
    }
    #[track_caller]
    #[doc(alias = "vkCreateQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
    pub unsafe fn create_query_pool(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::QueryPoolCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_query_pool: *mut crate::vk10::QueryPool,
    ) -> crate::vk10::Result {
        (self
            .create_query_pool
            .unwrap())(device, p_create_info, p_allocator, p_query_pool)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
    pub unsafe fn destroy_query_pool(
        &self,
        device: crate::vk10::Device,
        query_pool: crate::vk10::QueryPool,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_query_pool.unwrap())(device, query_pool, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetQueryPoolResults")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
    pub unsafe fn get_query_pool_results(
        &self,
        device: crate::vk10::Device,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
        stride: crate::vk10::DeviceSize,
        flags: crate::vk10::QueryResultFlags,
    ) -> crate::vk10::Result {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCreateBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
    pub unsafe fn create_buffer(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::BufferCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_buffer: *mut crate::vk10::Buffer,
    ) -> crate::vk10::Result {
        (self.create_buffer.unwrap())(device, p_create_info, p_allocator, p_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
    pub unsafe fn destroy_buffer(
        &self,
        device: crate::vk10::Device,
        buffer: crate::vk10::Buffer,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_buffer.unwrap())(device, buffer, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateBufferView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
    pub unsafe fn create_buffer_view(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::BufferViewCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_view: *mut crate::vk10::BufferView,
    ) -> crate::vk10::Result {
        (self.create_buffer_view.unwrap())(device, p_create_info, p_allocator, p_view)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyBufferView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
    pub unsafe fn destroy_buffer_view(
        &self,
        device: crate::vk10::Device,
        buffer_view: crate::vk10::BufferView,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_buffer_view.unwrap())(device, buffer_view, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
    pub unsafe fn create_image(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ImageCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_image: *mut crate::vk10::Image,
    ) -> crate::vk10::Result {
        (self.create_image.unwrap())(device, p_create_info, p_allocator, p_image)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
    pub unsafe fn destroy_image(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_image.unwrap())(device, image, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSubresourceLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_subresource: *const crate::vk10::ImageSubresource,
        p_layout: *mut crate::vk10::SubresourceLayout,
    ) {
        (self
            .get_image_subresource_layout
            .unwrap())(device, image, p_subresource, p_layout)
    }
    #[track_caller]
    #[doc(alias = "vkCreateImageView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
    pub unsafe fn create_image_view(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ImageViewCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_view: *mut crate::vk10::ImageView,
    ) -> crate::vk10::Result {
        (self.create_image_view.unwrap())(device, p_create_info, p_allocator, p_view)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyImageView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
    pub unsafe fn destroy_image_view(
        &self,
        device: crate::vk10::Device,
        image_view: crate::vk10::ImageView,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_image_view.unwrap())(device, image_view, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateShaderModule")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
    pub unsafe fn create_shader_module(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_shader_module: *mut crate::vk10::ShaderModule,
    ) -> crate::vk10::Result {
        (self
            .create_shader_module
            .unwrap())(device, p_create_info, p_allocator, p_shader_module)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyShaderModule")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
    pub unsafe fn destroy_shader_module(
        &self,
        device: crate::vk10::Device,
        shader_module: crate::vk10::ShaderModule,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_shader_module.unwrap())(device, shader_module, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreatePipelineCache")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
    pub unsafe fn create_pipeline_cache(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::PipelineCacheCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipeline_cache: *mut crate::vk10::PipelineCache,
    ) -> crate::vk10::Result {
        (self
            .create_pipeline_cache
            .unwrap())(device, p_create_info, p_allocator, p_pipeline_cache)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPipelineCache")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_pipeline_cache.unwrap())(device, pipeline_cache, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetPipelineCacheData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_pipeline_cache_data
            .unwrap())(device, pipeline_cache, p_data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkMergePipelineCaches")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: crate::vk10::Device,
        dst_cache: crate::vk10::PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const crate::vk10::PipelineCache,
    ) -> crate::vk10::Result {
        (self
            .merge_pipeline_caches
            .unwrap())(device, dst_cache, src_cache_count, p_src_caches)
    }
    #[track_caller]
    #[doc(alias = "vkCreateGraphicsPipelines")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const crate::vk10::GraphicsPipelineCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipelines: *mut crate::vk10::Pipeline,
    ) -> crate::vk10::Result {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCreateComputePipelines")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
    pub unsafe fn create_compute_pipelines(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const crate::vk10::ComputePipelineCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipelines: *mut crate::vk10::Pipeline,
    ) -> crate::vk10::Result {
        (self
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
    #[track_caller]
    #[doc(alias = "vkDestroyPipeline")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
    pub unsafe fn destroy_pipeline(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_pipeline.unwrap())(device, pipeline, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreatePipelineLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
    pub unsafe fn create_pipeline_layout(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::PipelineLayoutCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipeline_layout: *mut crate::vk10::PipelineLayout,
    ) -> crate::vk10::Result {
        (self
            .create_pipeline_layout
            .unwrap())(device, p_create_info, p_allocator, p_pipeline_layout)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPipelineLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: crate::vk10::Device,
        pipeline_layout: crate::vk10::PipelineLayout,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_pipeline_layout.unwrap())(device, pipeline_layout, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSampler")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
    pub unsafe fn create_sampler(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::SamplerCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_sampler: *mut crate::vk10::Sampler,
    ) -> crate::vk10::Result {
        (self.create_sampler.unwrap())(device, p_create_info, p_allocator, p_sampler)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySampler")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
    pub unsafe fn destroy_sampler(
        &self,
        device: crate::vk10::Device,
        sampler: crate::vk10::Sampler,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_sampler.unwrap())(device, sampler, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_set_layout: *mut crate::vk10::DescriptorSetLayout,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_set_layout
            .unwrap())(device, p_create_info, p_allocator, p_set_layout)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: crate::vk10::Device,
        descriptor_set_layout: crate::vk10::DescriptorSetLayout,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_descriptor_set_layout
            .unwrap())(device, descriptor_set_layout, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
    pub unsafe fn create_descriptor_pool(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorPoolCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_descriptor_pool: *mut crate::vk10::DescriptorPool,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_pool
            .unwrap())(device, p_create_info, p_allocator, p_descriptor_pool)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: crate::vk10::Device,
        descriptor_pool: crate::vk10::DescriptorPool,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_descriptor_pool.unwrap())(device, descriptor_pool, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkResetDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: crate::vk10::Device,
        descriptor_pool: crate::vk10::DescriptorPool,
        flags: crate::vk10::DescriptorPoolResetFlags,
    ) -> crate::vk10::Result {
        (self.reset_descriptor_pool.unwrap())(device, descriptor_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkAllocateDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: crate::vk10::Device,
        p_allocate_info: *const crate::vk10::DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut crate::vk10::DescriptorSet,
    ) -> crate::vk10::Result {
        (self
            .allocate_descriptor_sets
            .unwrap())(device, p_allocate_info, p_descriptor_sets)
    }
    #[track_caller]
    #[doc(alias = "vkFreeDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
    pub unsafe fn free_descriptor_sets(
        &self,
        device: crate::vk10::Device,
        descriptor_pool: crate::vk10::DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const crate::vk10::DescriptorSet,
    ) -> crate::vk10::Result {
        (self
            .free_descriptor_sets
            .unwrap())(device, descriptor_pool, descriptor_set_count, p_descriptor_sets)
    }
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
    pub unsafe fn update_descriptor_sets(
        &self,
        device: crate::vk10::Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const crate::vk10::CopyDescriptorSet,
    ) {
        (self
            .update_descriptor_sets
            .unwrap())(
            device,
            descriptor_write_count,
            p_descriptor_writes,
            descriptor_copy_count,
            p_descriptor_copies,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateFramebuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
    pub unsafe fn create_framebuffer(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::FramebufferCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_framebuffer: *mut crate::vk10::Framebuffer,
    ) -> crate::vk10::Result {
        (self
            .create_framebuffer
            .unwrap())(device, p_create_info, p_allocator, p_framebuffer)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyFramebuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
    pub unsafe fn destroy_framebuffer(
        &self,
        device: crate::vk10::Device,
        framebuffer: crate::vk10::Framebuffer,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_framebuffer.unwrap())(device, framebuffer, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
    pub unsafe fn create_render_pass(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::RenderPassCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_render_pass: *mut crate::vk10::RenderPass,
    ) -> crate::vk10::Result {
        (self
            .create_render_pass
            .unwrap())(device, p_create_info, p_allocator, p_render_pass)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
    pub unsafe fn destroy_render_pass(
        &self,
        device: crate::vk10::Device,
        render_pass: crate::vk10::RenderPass,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_render_pass.unwrap())(device, render_pass, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetRenderAreaGranularity")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
    pub unsafe fn get_render_area_granularity(
        &self,
        device: crate::vk10::Device,
        render_pass: crate::vk10::RenderPass,
        p_granularity: *mut crate::vk10::Extent2D,
    ) {
        (self.get_render_area_granularity.unwrap())(device, render_pass, p_granularity)
    }
    #[track_caller]
    #[doc(alias = "vkCreateCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
    pub unsafe fn create_command_pool(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::CommandPoolCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_command_pool: *mut crate::vk10::CommandPool,
    ) -> crate::vk10::Result {
        (self
            .create_command_pool
            .unwrap())(device, p_create_info, p_allocator, p_command_pool)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
    pub unsafe fn destroy_command_pool(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_command_pool.unwrap())(device, command_pool, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkResetCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
    pub unsafe fn reset_command_pool(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        flags: crate::vk10::CommandPoolResetFlags,
    ) -> crate::vk10::Result {
        (self.reset_command_pool.unwrap())(device, command_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkAllocateCommandBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
    pub unsafe fn allocate_command_buffers(
        &self,
        device: crate::vk10::Device,
        p_allocate_info: *const crate::vk10::CommandBufferAllocateInfo,
        p_command_buffers: *mut crate::vk10::CommandBuffer,
    ) -> crate::vk10::Result {
        (self
            .allocate_command_buffers
            .unwrap())(device, p_allocate_info, p_command_buffers)
    }
    #[track_caller]
    #[doc(alias = "vkFreeCommandBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
    pub unsafe fn free_command_buffers(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const crate::vk10::CommandBuffer,
    ) {
        (self
            .free_command_buffers
            .unwrap())(device, command_pool, command_buffer_count, p_command_buffers)
    }
    #[track_caller]
    #[doc(alias = "vkBeginCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_begin_info: *const crate::vk10::CommandBufferBeginInfo,
    ) -> crate::vk10::Result {
        (self.begin_command_buffer.unwrap())(command_buffer, p_begin_info)
    }
    #[track_caller]
    #[doc(alias = "vkEndCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
    pub unsafe fn end_command_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) -> crate::vk10::Result {
        (self.end_command_buffer.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkResetCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        flags: crate::vk10::CommandBufferResetFlags,
    ) -> crate::vk10::Result {
        (self.reset_command_buffer.unwrap())(command_buffer, flags)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindPipeline")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        pipeline: crate::vk10::Pipeline,
    ) {
        (self.cmd_bind_pipeline.unwrap())(command_buffer, pipeline_bind_point, pipeline)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewport")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const crate::vk10::Viewport,
    ) {
        (self
            .cmd_set_viewport
            .unwrap())(command_buffer, first_viewport, viewport_count, p_viewports)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetScissor")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const crate::vk10::Rect2D,
    ) {
        (self
            .cmd_set_scissor
            .unwrap())(command_buffer, first_scissor, scissor_count, p_scissors)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLineWidth")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
    pub unsafe fn cmd_set_line_width(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        line_width: std::os::raw::c_float,
    ) {
        (self.cmd_set_line_width.unwrap())(command_buffer, line_width)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBias")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bias_constant_factor: std::os::raw::c_float,
        depth_bias_clamp: std::os::raw::c_float,
        depth_bias_slope_factor: std::os::raw::c_float,
    ) {
        (self
            .cmd_set_depth_bias
            .unwrap())(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetBlendConstants")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        blend_constants: *const [std::os::raw::c_float; 4],
    ) {
        (self.cmd_set_blend_constants.unwrap())(command_buffer, blend_constants)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBounds")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        min_depth_bounds: std::os::raw::c_float,
        max_depth_bounds: std::os::raw::c_float,
    ) {
        (self
            .cmd_set_depth_bounds
            .unwrap())(command_buffer, min_depth_bounds, max_depth_bounds)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        compare_mask: u32,
    ) {
        (self
            .cmd_set_stencil_compare_mask
            .unwrap())(command_buffer, face_mask, compare_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        write_mask: u32,
    ) {
        (self.cmd_set_stencil_write_mask.unwrap())(command_buffer, face_mask, write_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilReference")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        reference: u32,
    ) {
        (self.cmd_set_stencil_reference.unwrap())(command_buffer, face_mask, reference)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        layout: crate::vk10::PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const crate::vk10::DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdBindIndexBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        index_type: crate::vk10::IndexType,
    ) {
        (self.cmd_bind_index_buffer.unwrap())(command_buffer, buffer, offset, index_type)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const crate::vk10::Buffer,
        p_offsets: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_bind_vertex_buffers
            .unwrap())(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDraw")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        (self
            .cmd_draw
            .unwrap())(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexed")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indirect
            .unwrap())(command_buffer, buffer, offset, draw_count, stride)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indexed_indirect
            .unwrap())(command_buffer, buffer, offset, draw_count, stride)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDispatch")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .cmd_dispatch
            .unwrap())(command_buffer, group_count_x, group_count_y, group_count_z)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDispatchIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
    ) {
        (self.cmd_dispatch_indirect.unwrap())(command_buffer, buffer, offset)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_buffer: crate::vk10::Buffer,
        dst_buffer: crate::vk10::Buffer,
        region_count: u32,
        p_regions: *const crate::vk10::BufferCopy,
    ) {
        (self
            .cmd_copy_buffer
            .unwrap())(command_buffer, src_buffer, dst_buffer, region_count, p_regions)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::ImageCopy,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::ImageBlit,
        filter: crate::vk10::Filter,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_buffer: crate::vk10::Buffer,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::BufferImageCopy,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_buffer: crate::vk10::Buffer,
        region_count: u32,
        p_regions: *const crate::vk10::BufferImageCopy,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdUpdateBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        data_size: crate::vk10::DeviceSize,
        p_data: *const std::os::raw::c_void,
    ) {
        (self
            .cmd_update_buffer
            .unwrap())(command_buffer, dst_buffer, dst_offset, data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdFillBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        size: crate::vk10::DeviceSize,
        data: u32,
    ) {
        (self
            .cmd_fill_buffer
            .unwrap())(command_buffer, dst_buffer, dst_offset, size, data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdClearColorImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image: crate::vk10::Image,
        image_layout: crate::vk10::ImageLayout,
        p_color: *const crate::vk10::ClearColorValue,
        range_count: u32,
        p_ranges: *const crate::vk10::ImageSubresourceRange,
    ) {
        (self
            .cmd_clear_color_image
            .unwrap())(
            command_buffer,
            image,
            image_layout,
            p_color,
            range_count,
            p_ranges,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image: crate::vk10::Image,
        image_layout: crate::vk10::ImageLayout,
        p_depth_stencil: *const crate::vk10::ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const crate::vk10::ImageSubresourceRange,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdClearAttachments")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        attachment_count: u32,
        p_attachments: *const crate::vk10::ClearAttachment,
        rect_count: u32,
        p_rects: *const crate::vk10::ClearRect,
    ) {
        (self
            .cmd_clear_attachments
            .unwrap())(
            command_buffer,
            attachment_count,
            p_attachments,
            rect_count,
            p_rects,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::ImageResolve,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::vk10::PipelineStageFlags,
    ) {
        (self.cmd_set_event.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::vk10::PipelineStageFlags,
    ) {
        (self.cmd_reset_event.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event_count: u32,
        p_events: *const crate::vk10::Event,
        src_stage_mask: crate::vk10::PipelineStageFlags,
        dst_stage_mask: crate::vk10::PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const crate::vk10::MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_stage_mask: crate::vk10::PipelineStageFlags,
        dst_stage_mask: crate::vk10::PipelineStageFlags,
        dependency_flags: crate::vk10::DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const crate::vk10::MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdBeginQuery")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
        flags: crate::vk10::QueryControlFlags,
    ) {
        (self.cmd_begin_query.unwrap())(command_buffer, query_pool, query, flags)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndQuery")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self.cmd_end_query.unwrap())(command_buffer, query_pool, query)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self
            .cmd_reset_query_pool
            .unwrap())(command_buffer, query_pool, first_query, query_count)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_stage: crate::vk10::PipelineStageFlags,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self
            .cmd_write_timestamp
            .unwrap())(command_buffer, pipeline_stage, query_pool, query)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        stride: crate::vk10::DeviceSize,
        flags: crate::vk10::QueryResultFlags,
    ) {
        (self
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
    #[track_caller]
    #[doc(alias = "vkCmdPushConstants")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        layout: crate::vk10::PipelineLayout,
        stage_flags: crate::vk10::ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const std::os::raw::c_void,
    ) {
        (self
            .cmd_push_constants
            .unwrap())(command_buffer, layout, stage_flags, offset, size, p_values)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
        contents: crate::vk10::SubpassContents,
    ) {
        (self
            .cmd_begin_render_pass
            .unwrap())(command_buffer, p_render_pass_begin, contents)
    }
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        contents: crate::vk10::SubpassContents,
    ) {
        (self.cmd_next_subpass.unwrap())(command_buffer, contents)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
    pub unsafe fn cmd_end_render_pass(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_render_pass.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdExecuteCommands")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const crate::vk10::CommandBuffer,
    ) {
        (self
            .cmd_execute_commands
            .unwrap())(command_buffer, command_buffer_count, p_command_buffers)
    }
    #[track_caller]
    #[doc(alias = "vkTrimCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html)
    pub unsafe fn trim_command_pool(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        flags: crate::vk11::CommandPoolTrimFlags,
    ) {
        (self.trim_command_pool.unwrap())(device, command_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: crate::vk10::Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
    ) {
        (self
            .get_device_group_peer_memory_features
            .unwrap())(
            device,
            heap_index,
            local_device_index,
            remote_device_index,
            p_peer_memory_features,
        )
    }
    #[track_caller]
    #[doc(alias = "vkBindBufferMemory2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)
    pub unsafe fn bind_buffer_memory_2(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
    ) -> crate::vk10::Result {
        (self.bind_buffer_memory_2.unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkBindImageMemory2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)
    pub unsafe fn bind_image_memory_2(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
    ) -> crate::vk10::Result {
        (self.bind_image_memory_2.unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDeviceMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html)
    pub unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        device_mask: u32,
    ) {
        (self.cmd_set_device_mask.unwrap())(command_buffer, device_mask)
    }
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
        (self
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
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_update_template
            .unwrap())(device, p_create_info, p_allocator, p_descriptor_update_template)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: crate::vk10::Device,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_descriptor_update_template
            .unwrap())(device, descriptor_update_template, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: crate::vk10::Device,
        descriptor_set: crate::vk10::DescriptorSet,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        p_data: *const std::os::raw::c_void,
    ) {
        (self
            .update_descriptor_set_with_template
            .unwrap())(device, descriptor_set, descriptor_update_template, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html)
    pub unsafe fn get_buffer_memory_requirements_2(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_buffer_memory_requirements_2
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html)
    pub unsafe fn get_image_memory_requirements_2(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_image_memory_requirements_2
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
    pub unsafe fn get_image_sparse_memory_requirements_2(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
    ) {
        (self
            .get_image_sparse_memory_requirements_2
            .unwrap())(
            device,
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
    ) -> crate::vk10::Result {
        (self
            .create_sampler_ycbcr_conversion
            .unwrap())(device, p_create_info, p_allocator, p_ycbcr_conversion)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: crate::vk10::Device,
        ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_sampler_ycbcr_conversion
            .unwrap())(device, ycbcr_conversion, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceQueue2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)
    pub unsafe fn get_device_queue_2(
        &self,
        device: crate::vk10::Device,
        p_queue_info: *const crate::vk11::DeviceQueueInfo2,
        p_queue: *mut crate::vk10::Queue,
    ) {
        (self.get_device_queue_2.unwrap())(device, p_queue_info, p_queue)
    }
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
        p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
    ) {
        (self
            .get_descriptor_set_layout_support
            .unwrap())(device, p_create_info, p_support)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSwapchainKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
    pub unsafe fn create_swapchain_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_swapchain: *mut crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk10::Result {
        (self
            .create_swapchain_khr
            .unwrap())(device, p_create_info, p_allocator, p_swapchain)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySwapchainKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
    pub unsafe fn destroy_swapchain_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_swapchain_khr.unwrap())(device, swapchain, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
    pub unsafe fn get_swapchain_images_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut crate::vk10::Image,
    ) -> crate::vk10::Result {
        (self
            .get_swapchain_images_khr
            .unwrap())(device, swapchain, p_swapchain_image_count, p_swapchain_images)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireNextImageKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
    pub unsafe fn acquire_next_image_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        timeout: u64,
        semaphore: crate::vk10::Semaphore,
        fence: crate::vk10::Fence,
        p_image_index: *mut u32,
    ) -> crate::vk10::Result {
        (self
            .acquire_next_image_khr
            .unwrap())(device, swapchain, timeout, semaphore, fence, p_image_index)
    }
    #[track_caller]
    #[doc(alias = "vkQueuePresentKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
    pub unsafe fn queue_present_khr(
        &self,
        queue: crate::vk10::Queue,
        p_present_info: *const crate::extensions::khr_swapchain::PresentInfoKHR,
    ) -> crate::vk10::Result {
        (self.queue_present_khr.unwrap())(queue, p_present_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: crate::vk10::Device,
        p_device_group_present_capabilities: *mut crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_device_group_present_capabilities_khr
            .unwrap())(device, p_device_group_present_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: crate::vk10::Device,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::vk10::Result {
        (self
            .get_device_group_surface_present_modes_khr
            .unwrap())(device, surface, p_modes)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireNextImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)
    pub unsafe fn acquire_next_image_2_khr(
        &self,
        device: crate::vk10::Device,
        p_acquire_info: *const crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
        p_image_index: *mut u32,
    ) -> crate::vk10::Result {
        (self.acquire_next_image_2_khr.unwrap())(device, p_acquire_info, p_image_index)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html)
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_rendering_info: *const crate::extensions::khr_dynamic_rendering::RenderingInfoKHR,
    ) {
        (self.cmd_begin_rendering_khr.unwrap())(command_buffer, p_rendering_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html)
    pub unsafe fn cmd_end_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_rendering_khr.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html)
    pub unsafe fn create_render_pass_2_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::khr_create_renderpass2::RenderPassCreateInfo2KHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_render_pass: *mut crate::vk10::RenderPass,
    ) -> crate::vk10::Result {
        (self
            .create_render_pass_2_khr
            .unwrap())(device, p_create_info, p_allocator, p_render_pass)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html)
    pub unsafe fn cmd_begin_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
        p_subpass_begin_info: *const crate::extensions::khr_create_renderpass2::SubpassBeginInfoKHR,
    ) {
        (self
            .cmd_begin_render_pass_2_khr
            .unwrap())(command_buffer, p_render_pass_begin, p_subpass_begin_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html)
    pub unsafe fn cmd_next_subpass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_subpass_begin_info: *const crate::extensions::khr_create_renderpass2::SubpassBeginInfoKHR,
        p_subpass_end_info: *const crate::extensions::khr_create_renderpass2::SubpassEndInfoKHR,
    ) {
        (self
            .cmd_next_subpass_2_khr
            .unwrap())(command_buffer, p_subpass_begin_info, p_subpass_end_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html)
    pub unsafe fn cmd_end_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_subpass_end_info: *const crate::extensions::khr_create_renderpass2::SubpassEndInfoKHR,
    ) {
        (self.cmd_end_render_pass_2_khr.unwrap())(command_buffer, p_subpass_end_info)
    }
    #[track_caller]
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: crate::vk10::Device,
        p_name_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
    ) -> crate::vk10::Result {
        (self.set_debug_utils_object_name_ext.unwrap())(device, p_name_info)
    }
    #[track_caller]
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: crate::vk10::Device,
        p_tag_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
    ) -> crate::vk10::Result {
        (self.set_debug_utils_object_tag_ext.unwrap())(device, p_tag_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: crate::vk10::Queue,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.queue_begin_debug_utils_label_ext.unwrap())(queue, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: crate::vk10::Queue) {
        (self.queue_end_debug_utils_label_ext.unwrap())(queue)
    }
    #[track_caller]
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: crate::vk10::Queue,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.queue_insert_debug_utils_label_ext.unwrap())(queue, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.cmd_begin_debug_utils_label_ext.unwrap())(command_buffer, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_end_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_debug_utils_label_ext.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.cmd_insert_debug_utils_label_ext.unwrap())(command_buffer, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html)
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        device: crate::vk10::Device,
        semaphore: crate::vk10::Semaphore,
        p_value: *mut u64,
    ) -> crate::vk10::Result {
        (self.get_semaphore_counter_value_khr.unwrap())(device, semaphore, p_value)
    }
    #[track_caller]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html)
    pub unsafe fn wait_semaphores_khr(
        &self,
        device: crate::vk10::Device,
        p_wait_info: *const crate::extensions::khr_timeline_semaphore::SemaphoreWaitInfoKHR,
        timeout: u64,
    ) -> crate::vk10::Result {
        (self.wait_semaphores_khr.unwrap())(device, p_wait_info, timeout)
    }
    #[track_caller]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html)
    pub unsafe fn signal_semaphore_khr(
        &self,
        device: crate::vk10::Device,
        p_signal_info: *const crate::extensions::khr_timeline_semaphore::SemaphoreSignalInfoKHR,
    ) -> crate::vk10::Result {
        (self.signal_semaphore_khr.unwrap())(device, p_signal_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html)
    pub unsafe fn cmd_set_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        p_dependency_info: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
    ) {
        (self.cmd_set_event_2_khr.unwrap())(command_buffer, event, p_dependency_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html)
    pub unsafe fn cmd_reset_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    ) {
        (self.cmd_reset_event_2_khr.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html)
    pub unsafe fn cmd_wait_events_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event_count: u32,
        p_events: *const crate::vk10::Event,
        p_dependency_infos: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
    ) {
        (self
            .cmd_wait_events_2_khr
            .unwrap())(command_buffer, event_count, p_events, p_dependency_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html)
    pub unsafe fn cmd_pipeline_barrier_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_dependency_info: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
    ) {
        (self.cmd_pipeline_barrier_2_khr.unwrap())(command_buffer, p_dependency_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueSubmit2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html)
    pub unsafe fn queue_submit_2_khr(
        &self,
        queue: crate::vk10::Queue,
        submit_count: u32,
        p_submits: *const crate::extensions::khr_synchronization2::SubmitInfo2KHR,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_submit_2_khr.unwrap())(queue, submit_count, p_submits, fence)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html)
    pub unsafe fn cmd_write_timestamp_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self
            .cmd_write_timestamp_2_khr
            .unwrap())(command_buffer, stage, query_pool, query)
    }
}
impl Default for DeviceTable {
    fn default() -> Self {
        Self::new_empty()
    }
}
