pub mod tables;

use crate::vk;
use std::os::raw::c_char;

#[cfg(feature = "linked")]
extern "system" {
    fn vkGetInstanceProcAddr(instance: vk::Instance, name: *const c_char) -> vk::PfnVoidFunction;
}

pub trait FunctionLoad {
    #[track_caller]
    unsafe fn load(&self, name: *const c_char) -> Option<vk::PfnVoidFunction>;
}

pub trait EntryLoad: FunctionLoad {
    #[allow(non_snake_case)]
    unsafe fn get_vkGetInstanceProcAddr(
        &self,
    ) -> unsafe extern "system" fn(vk::Instance, name: *const c_char) -> Option<vk::PfnVoidFunction>;
}
pub trait InstanceLoad: FunctionLoad {}
pub trait DeviceLoad: FunctionLoad {}

#[allow(non_snake_case)]
pub enum EntryLoader {
    #[cfg(feature = "linked")]
    Linked,
    #[cfg(feature = "loaded")]
    Loaded {
        _lib: libloading::Library,
        vkGetInstanceProcAddr: unsafe extern "system" fn(
            vk::Instance,
            name: *const c_char,
        ) -> Option<vk::PfnVoidFunction>,
    },
}

#[allow(non_snake_case)]
pub struct InstanceLoader {
    vkGetInstanceProcAddr:
        unsafe extern "system" fn(vk::Instance, name: *const c_char) -> Option<vk::PfnVoidFunction>,
    instance: vk::Instance,
}

#[allow(non_snake_case)]
pub struct DeviceLoader {
    vkGetDeviceProcAddr:
        unsafe extern "system" fn(vk::Device, name: *const c_char) -> Option<vk::PfnVoidFunction>,
    device: vk::Device,
}

impl EntryLoad for EntryLoader {
    #[inline]
    unsafe fn get_vkGetInstanceProcAddr(
        &self,
    ) -> unsafe extern "system" fn(vk::Instance, name: *const c_char) -> Option<vk::PfnVoidFunction>
    {
        match self {
            #[cfg(feature = "linked")]
            EntryLoader::Linked => vkGetInstanceProcAddr,
            #[cfg(feature = "loaded")]
            EntryLoader::Loaded {
                vkGetInstanceProcAddr,
                ..
            } => *vkGetInstanceProcAddr,
        }
    }
}

impl FunctionLoad for EntryLoader {
    unsafe fn load(&self, name: *const c_char) -> Option<vk::PfnVoidFunction> {
        // in this case it is correct to pass in null
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
        (self.get_vkGetInstanceProcAddr())(std::mem::transmute(0u64), name)
    }
}

impl InstanceLoad for InstanceLoader {}
impl DeviceLoad for InstanceLoader {}
impl FunctionLoad for InstanceLoader {
    unsafe fn load(&self, name: *const c_char) -> Option<vk::PfnVoidFunction> {
        (self.vkGetInstanceProcAddr)(self.instance, name)
    }
}

impl DeviceLoad for DeviceLoader {}
impl FunctionLoad for DeviceLoader {
    unsafe fn load(&self, name: *const c_char) -> Option<vk::PfnVoidFunction> {
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html
        (self.vkGetDeviceProcAddr)(self.device, name)
    }
}

impl EntryLoader {
    pub unsafe fn new() -> Result<Self, libloading::Error> {
        #[cfg(feature = "linked")]
        let entry = Ok(Self::new_linked());
        #[cfg(all(not(feature = "linked"), feature = "loaded"))]
        let entry = Self::new_loaded();

        entry
    }
    #[cfg(feature = "loaded")]
    pub unsafe fn new_loaded() -> Result<Self, libloading::Error> {
        #[cfg(windows)]
        const LIB_PATH: &str = "vulkan-1.dll";

        #[cfg(all(
            unix,
            not(any(target_os = "macos", target_os = "ios", target_os = "android"))
        ))]
        const LIB_PATH: &str = "libvulkan.so.1";

        #[cfg(target_os = "android")]
        const LIB_PATH: &str = "libvulkan.so";

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        const LIB_PATH: &str = "libvulkan.dylib";

        Self::new_loaded_with_path(LIB_PATH)
    }
    #[cfg(feature = "loaded")]
    pub unsafe fn new_loaded_with_path(path: &str) -> Result<Self, libloading::Error> {
        let lib = libloading::Library::new(path)?;

        static ENTRY_POINT: &[u8] = b"vkGetInstanceProcAddr\0";

        let symbol: libloading::Symbol<
            unsafe extern "system" fn(
                vk::Instance,
                name: *const c_char,
            ) -> Option<vk::PfnVoidFunction>,
        > = lib.get(ENTRY_POINT).unwrap();

        let ptr: unsafe extern "system" fn(
            vk::Instance,
            name: *const c_char,
        ) -> Option<vk::PfnVoidFunction> = *symbol;

        if (ptr as *const ()).is_null() {
            panic!(
                "vkGetInstanceProcAddr dynamically loaded from '{}' is null!",
                path
            );
        }

        Ok(Self::Loaded {
            vkGetInstanceProcAddr: *symbol,
            _lib: lib,
        })
    }
    #[cfg(feature = "linked")]
    pub fn new_linked() -> Self {
        Self::Linked
    }
}

impl InstanceLoader {
    pub unsafe fn new(instance: vk::Instance, entry: &impl EntryLoad) -> Self {
        Self {
            vkGetInstanceProcAddr: entry.get_vkGetInstanceProcAddr(),
            instance,
        }
    }
}

impl DeviceLoader {
    pub fn new(device: vk::Device, entry: &impl DeviceLoad) -> Self {
        let ptr = unsafe { entry.load("vkGetDeviceProcAddr\0".as_ptr().cast()) };
        let fun = unsafe { std::mem::transmute(ptr) };

        Self {
            vkGetDeviceProcAddr: fun,
            device,
        }
    }
}
