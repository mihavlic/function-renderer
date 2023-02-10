#![allow(unused_unsafe)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod deep_copy;
pub mod dumb_hash;
pub mod loader;
#[cfg(feature = "surface")]
pub mod surface;
pub mod util;

pub mod extensions;

pub type VulkanResult<T> = Result<T, vk::Result>;

#[inline]
pub fn new_result<T>(value: T, result: vk::Result) -> VulkanResult<T> {
    if result.0.is_negative() {
        VulkanResult::Err(result)
    } else {
        VulkanResult::Ok(value)
    }
}

impl From<vk::Result> for VulkanResult<()> {
    fn from(value: vk::Result) -> Self {
        if value.0.is_negative() {
            VulkanResult::Err(value)
        } else {
            VulkanResult::Ok(())
        }
    }
}

impl std::error::Error for vk::Result {}
impl std::fmt::Display for vk::Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub mod vk;
pub mod vk10;
pub mod vk11;
#[derive(Clone)]
pub struct EntryWrapper {
    pub(crate) table: *const crate::loader::tables::EntryTable,
}
impl EntryWrapper {
    pub unsafe fn new(table: *const crate::loader::tables::EntryTable) -> Self {
        Self { table }
    }
    #[inline]
    pub fn table(&self) -> *const crate::loader::tables::EntryTable {
        self.table
    }
}
/// When creating a EntryWrapper, you are promising to keep the pointed to table alive for the lifetime of the wrapper
unsafe impl Send for EntryWrapper {}
unsafe impl Sync for EntryWrapper {}
#[derive(Clone)]
pub struct InstanceWrapper {
    pub(crate) handle: crate::vk10::Instance,
    pub(crate) table: *const crate::loader::tables::InstanceTable,
}
impl InstanceWrapper {
    pub unsafe fn new(
        handle: crate::vk10::Instance,
        table: *const crate::loader::tables::InstanceTable,
    ) -> Self {
        Self { handle, table }
    }
    #[inline]
    pub fn handle(&self) -> crate::vk10::Instance {
        self.handle
    }
    #[inline]
    pub fn table(&self) -> *const crate::loader::tables::InstanceTable {
        self.table
    }
}
/// When creating a InstanceWrapper, you are promising to keep the pointed to table alive for the lifetime of the wrapper
unsafe impl Send for InstanceWrapper {}
unsafe impl Sync for InstanceWrapper {}
#[derive(Clone)]
pub struct DeviceWrapper {
    pub(crate) handle: crate::vk10::Device,
    pub(crate) table: *const crate::loader::tables::DeviceTable,
}
impl DeviceWrapper {
    pub unsafe fn new(
        handle: crate::vk10::Device,
        table: *const crate::loader::tables::DeviceTable,
    ) -> Self {
        Self { handle, table }
    }
    #[inline]
    pub fn handle(&self) -> crate::vk10::Device {
        self.handle
    }
    #[inline]
    pub fn table(&self) -> *const crate::loader::tables::DeviceTable {
        self.table
    }
}
/// When creating a DeviceWrapper, you are promising to keep the pointed to table alive for the lifetime of the wrapper
unsafe impl Send for DeviceWrapper {}
unsafe impl Sync for DeviceWrapper {}
