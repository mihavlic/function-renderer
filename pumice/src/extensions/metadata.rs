use std::ffi::CStr;

pub struct ExtensionMetadata {
    pub name: &'static CStr,
    /// true - instance, false - device
    pub instance: bool,
    pub core_version: u32,
    pub requires_extensions: &'static [&'static CStr],
}

pub fn get_metadata(name: &CStr) -> Option<&'static ExtensionMetadata> {
    let index = EXTENSION_METADATA
        .binary_search_by_key(&name, |m| &m.name)
        .ok()?;
    Some(&EXTENSION_METADATA[index])
}
pub const EXTENSION_METADATA: &[ExtensionMetadata] = &[
    ExtensionMetadata {
        name: crate::cstr!("VK_EXT_debug_report"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_EXT_debug_utils"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_EXT_directfb_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_EXT_headless_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_EXT_metal_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_EXT_scalar_block_layout"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_get_physical_device_properties2")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_FUCHSIA_imagepipe_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_GGP_stream_descriptor_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_android_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_create_renderpass2"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[
            crate::cstr!("VK_KHR_multiview"),
            crate::cstr!("VK_KHR_maintenance2"),
        ],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_depth_stencil_resolve"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_create_renderpass2")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_dynamic_rendering"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[
            crate::cstr!("VK_KHR_depth_stencil_resolve"),
            crate::cstr!("VK_KHR_get_physical_device_properties2"),
        ],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_get_physical_device_properties2"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_maintenance2"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_multiview"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_get_physical_device_properties2")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_swapchain"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_synchronization2"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_get_physical_device_properties2")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_timeline_semaphore"),
        instance: false,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_get_physical_device_properties2")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_wayland_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_win32_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_xcb_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_KHR_xlib_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_MVK_ios_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_MVK_macos_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_NN_vi_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
    ExtensionMetadata {
        name: crate::cstr!("VK_QNX_screen_surface"),
        instance: true,
        core_version: crate::vk10::API_VERSION_1_0,
        requires_extensions: &[crate::cstr!("VK_KHR_surface")],
    },
];
