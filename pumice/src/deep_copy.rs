#![allow(unused)]

use std::{
    alloc::Layout,
    borrow::Borrow,
    ffi::CStr,
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_void},
    ptr::{self, NonNull},
};

pub struct DeepCopyBox<T> {
    copy: NonNull<T>,
    measure: CopyMeasure,
}

impl<T> Deref for DeepCopyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.copy.as_ref() }
    }
}

impl<T> DerefMut for DeepCopyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.copy.as_mut() }
    }
}

impl<T: DeepCopy> Clone for DeepCopyBox<T> {
    fn clone(&self) -> Self {
        unsafe {
            let mut writer = CopyWriter::new(&self.measure);
            writer.write_ptr(self.copy.as_ptr());

            Self {
                copy: NonNull::new(writer.finish().cast()).unwrap(),
                measure: self.measure.clone(),
            }
        }
    }
}

impl<T> Drop for DeepCopyBox<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.copy.as_ptr().cast(), self.measure.layout());
        }
    }
}

pub unsafe trait DeepCopy: Sized {
    fn measure(&self, measure: &mut CopyMeasure);
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter);
    unsafe fn deep_copy(&self) -> DeepCopyBox<Self> {
        let mut measure = CopyMeasure::new();
        measure.alloc::<Self>();
        self.measure(&mut measure);

        let mut writer = CopyWriter::new(&measure);
        writer.write_ptr(self);

        DeepCopyBox {
            copy: NonNull::new(writer.finish().cast()).unwrap(),
            measure,
        }
    }
}

#[derive(Clone)]
pub struct CopyMeasure {
    layout: Layout,
}

impl CopyMeasure {
    fn new() -> Self {
        Self {
            layout: Layout::new::<()>(),
        }
    }
    pub fn layout(&self) -> Layout {
        self.layout
    }
    pub unsafe fn measure_ptr<T: DeepCopy>(&mut self, what: *const T) {
        if let Some(what) = what.as_ref() {
            self.alloc::<T>();
            what.measure(self);
        }
    }
    pub unsafe fn measure_arr_ptr<T: DeepCopy>(&mut self, ptr: *const T, len: usize) {
        if !ptr.is_null() && len > 0 {
            self.alloc_arr::<T>(len);

            let slice = std::slice::from_raw_parts(ptr, len);
            for element in slice {
                element.measure(self);
            }
        }
    }
    pub unsafe fn measure_cstr(&mut self, ptr: *const c_char) {
        if !ptr.is_null() {
            let len = CStr::from_ptr(ptr).to_bytes_with_nul().len();
            self.alloc_arr::<c_char>(len);
        }
    }
    #[track_caller]
    pub unsafe fn measure_pnext(&mut self, mut p_next: *const c_void) {
        while !p_next.is_null() {
            #[rustfmt::skip]
            crate::pnext_visit!(
                p_next, stype, object,
                // hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
                // I am very sorry
                self.measure_ptr((&&*object).get_or_die())
            );
        }
    }
    pub unsafe fn alloc<T>(&mut self) {
        layout_extend::<T>(&mut self.layout);
    }
    pub unsafe fn alloc_arr<T>(&mut self, len: usize) {
        layout_extend_arr::<T>(&mut self.layout, len);
    }
}

pub struct CopyWriter {
    buf: *mut u8,
    layout: Layout,
}

impl CopyWriter {
    pub unsafe fn new(measure: &CopyMeasure) -> Self {
        Self {
            buf: std::alloc::alloc(measure.layout),
            layout: Layout::from_size_align(0, measure.layout.align()).unwrap(),
        }
    }
    pub unsafe fn write_ptr<T: DeepCopy>(&mut self, what: *const T) -> *mut T {
        if let Some(what_ref) = what.as_ref() {
            let ptr = self.alloc::<T>();

            ptr::copy_nonoverlapping(what, ptr, 1);
            what_ref.copy(ptr, self);
            ptr
        } else {
            ptr::null_mut()
        }
    }
    pub unsafe fn write_arr_ptr<T: DeepCopy>(&mut self, ptr: *const T, len: usize) -> *mut T {
        if !ptr.is_null() && len > 0 {
            let slice = std::slice::from_raw_parts(ptr, len);
            let ptr = self.alloc_arr::<T>(len);

            ptr::copy_nonoverlapping(slice.as_ptr(), ptr, len);
            for (i, element) in slice.iter().enumerate() {
                element.copy(ptr.add(i), self);
            }

            ptr
        } else {
            ptr::null_mut()
        }
    }
    pub unsafe fn write_cstr(&mut self, what: *const c_char) -> *const c_char {
        if !what.is_null() {
            let len = CStr::from_ptr(what).to_bytes_with_nul().len();
            let ptr = self.alloc_arr::<c_char>(len);

            ptr::copy_nonoverlapping(what, ptr, len);
            ptr
        } else {
            ptr::null()
        }
    }
    #[track_caller]
    pub unsafe fn write_pnext(&mut self, mut p_next: *const c_void) -> *mut c_void {
        let mut first = None;
        let mut prev: *mut *const c_void = ptr::null_mut();
        while !p_next.is_null() {
            #[rustfmt::skip]
            crate::pnext_visit!(
                p_next, stype, object,
                // hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
                // I am very sorry
                {
                    let ptr = self.write_ptr((&&*object).get_or_die());
                    if first.is_none() {
                        first = Some(ptr.cast::<c_void>());
                    } else {
                        *prev = ptr as *const _;
                    }
                    prev = std::mem::transmute(std::ptr::addr_of!((*ptr).p_next));
                }
            );
        }
        first.unwrap_or(ptr::null_mut())
    }
    pub unsafe fn alloc<T>(&mut self) -> *mut T {
        let offset = layout_extend::<T>(&mut self.layout);
        let ptr = self.buf.add(offset).cast();
        ptr
    }
    pub unsafe fn alloc_arr<T>(&mut self, len: usize) -> *mut T {
        let offset = layout_extend_arr::<T>(&mut self.layout, len);
        let ptr = self.buf.add(offset).cast();
        ptr
    }
    pub unsafe fn finish(self) -> *mut u8 {
        self.buf
    }
}

// hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

struct NeverInstantiated {
    p_next: *const c_void,
}

unsafe impl DeepCopy for NeverInstantiated {
    fn measure(&self, _: &mut CopyMeasure) {
        unreachable!()
    }
    unsafe fn copy(&self, _: *mut Self, _: &mut CopyWriter) {
        unreachable!()
    }
}

trait ViaPanic {
    fn get_or_die(&self) -> &NeverInstantiated {
        panic!("{:?} does not implement DeepCopy, this is likely because it contains a void pointer without any size information which cannot be manipulated and thus no implementation could be generated.", std::any::type_name::<Self>());
    }
}
impl<T> ViaPanic for &&T {}

trait ViaActual<T>: Borrow<T> {
    fn get_or_die(&self) -> &T {
        self.borrow()
    }
}
impl<T: DeepCopy> ViaActual<T> for &T {}

#[inline]
#[track_caller]
fn layout_extend<T>(layout: &mut Layout) -> usize {
    let (new_layout, offset) = layout.extend(Layout::new::<T>()).unwrap();
    *layout = new_layout;
    offset
}

#[inline]
#[track_caller]
fn layout_extend_arr<T>(layout: &mut Layout, len: usize) -> usize {
    let (new_layout, offset) = layout.extend(Layout::array::<T>(len).unwrap()).unwrap();
    *layout = new_layout;
    offset
}

/// All memory directly contained in a struct is always already accounted for by the encasing
/// type's write() call, further calls are required only when indirectly referenced memory needs to be copied
macro_rules! value_type_copy_impl {
    ($($name:path),+) => {
        $(
            unsafe impl DeepCopy for $name {
                fn measure(&self, _measure: &mut CopyMeasure) {
                    {}
                }
                unsafe fn copy(&self, _copy: *mut Self, _writer: &mut CopyWriter) {
                    {}
                }
            }
        )+
    };
}

#[test]
fn test() {
    use crate::vk::{
        DeviceCreateInfo, GraphicsPipelineCreateInfo, PipelineInputAssemblyStateCreateInfo,
        PrimitiveTopology,
    };

    let a = PipelineInputAssemblyStateCreateInfo {
        topology: PrimitiveTopology::TRIANGLE_FAN,
        ..Default::default()
    };
    let a = GraphicsPipelineCreateInfo {
        p_next: &a as *const _ as *const _,
        ..Default::default()
    };
    unsafe {
        assert_eq!(
            (*a.p_next.cast::<PipelineInputAssemblyStateCreateInfo>()).topology,
            PrimitiveTopology::TRIANGLE_FAN
        );
        let copy = a.deep_copy();
        assert_eq!(
            (*copy.p_next.cast::<PipelineInputAssemblyStateCreateInfo>()).topology,
            PrimitiveTopology::TRIANGLE_FAN
        );

        let names = [
            crate::cstr!("Aaaa").as_ptr(),
            crate::cstr!("Baaa").as_ptr(),
            crate::cstr!("Aaaa").as_ptr(),
            crate::cstr!("Aaaa").as_ptr(),
        ];

        let a = DeviceCreateInfo {
            enabled_layer_count: 4,
            pp_enabled_layer_names: names.as_ptr(),
            ..Default::default()
        };

        let copy = a.deep_copy();

        assert_ne!(copy.pp_enabled_layer_names, a.pp_enabled_layer_names);

        assert_eq!(
            CStr::from_ptr(*copy.pp_enabled_layer_names.add(1)),
            crate::cstr!("Baaa"),
        )
    }
}
unsafe impl DeepCopy for crate::vk10::BaseOutStructure {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_ptr(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk10::BaseInStructure {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_ptr(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk10::ApplicationInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_application_name);
            measure.measure_cstr(self.p_engine_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_application_name = writer.write_cstr(self.p_application_name);
        (*copy).p_engine_name = writer.write_cstr(self.p_engine_name);
    }
}
unsafe impl DeepCopy for crate::vk10::AllocationCallbacks {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {}
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {}
}
unsafe impl DeepCopy for crate::vk10::DeviceQueueCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_queue_priorities, (self.queue_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_priorities = writer
            .write_arr_ptr(self.p_queue_priorities, (self.queue_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DeviceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_create_infos,
                    (self.queue_create_info_count) as usize,
                );
            let len = (self.enabled_layer_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                measure.measure_cstr(ptr);
            }
            let len = (self.enabled_extension_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                measure.measure_cstr(ptr);
            }
            measure.measure_ptr(self.p_enabled_features);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_create_infos = writer
            .write_arr_ptr(
                self.p_queue_create_infos,
                (self.queue_create_info_count) as usize,
            );
        let len = (self.enabled_layer_count) as usize;
        (*copy)
            .pp_enabled_layer_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_layer_names.add(i);
            let copy = (*copy).pp_enabled_layer_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
        let len = (self.enabled_extension_count) as usize;
        (*copy)
            .pp_enabled_extension_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_extension_names.add(i);
            let copy = (*copy).pp_enabled_extension_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
        (*copy).p_enabled_features = writer.write_ptr(self.p_enabled_features);
    }
}
unsafe impl DeepCopy for crate::vk10::InstanceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_application_info);
            let len = (self.enabled_layer_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                measure.measure_cstr(ptr);
            }
            let len = (self.enabled_extension_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                measure.measure_cstr(ptr);
            }
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_application_info = writer.write_ptr(self.p_application_info);
        let len = (self.enabled_layer_count) as usize;
        (*copy)
            .pp_enabled_layer_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_layer_names.add(i);
            let copy = (*copy).pp_enabled_layer_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
        let len = (self.enabled_extension_count) as usize;
        (*copy)
            .pp_enabled_extension_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_extension_names.add(i);
            let copy = (*copy).pp_enabled_extension_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
    }
}
unsafe impl DeepCopy for crate::vk10::MemoryAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::MappedMemoryRange {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::WriteDescriptorSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_image_info, (self.descriptor_count) as usize);
            measure
                .measure_arr_ptr(self.p_buffer_info, (self.descriptor_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_texel_buffer_view,
                    (self.descriptor_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_image_info = writer
            .write_arr_ptr(self.p_image_info, (self.descriptor_count) as usize);
        (*copy)
            .p_buffer_info = writer
            .write_arr_ptr(self.p_buffer_info, (self.descriptor_count) as usize);
        (*copy)
            .p_texel_buffer_view = writer
            .write_arr_ptr(self.p_texel_buffer_view, (self.descriptor_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::CopyDescriptorSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::BufferCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::BufferViewCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::MemoryBarrier {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::BufferMemoryBarrier {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::ImageMemoryBarrier {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::ImageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::ImageViewCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::SparseBufferMemoryBindInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_binds, (self.bind_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_binds = writer.write_arr_ptr(self.p_binds, (self.bind_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SparseImageOpaqueMemoryBindInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_binds, (self.bind_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_binds = writer.write_arr_ptr(self.p_binds, (self.bind_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SparseImageMemoryBindInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_binds, (self.bind_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_binds = writer.write_arr_ptr(self.p_binds, (self.bind_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::BindSparseInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphores,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(self.p_buffer_binds, (self.buffer_bind_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_image_opaque_binds,
                    (self.image_opaque_bind_count) as usize,
                );
            measure
                .measure_arr_ptr(self.p_image_binds, (self.image_bind_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphores,
                    (self.signal_semaphore_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphores = writer
            .write_arr_ptr(self.p_wait_semaphores, (self.wait_semaphore_count) as usize);
        (*copy)
            .p_buffer_binds = writer
            .write_arr_ptr(self.p_buffer_binds, (self.buffer_bind_count) as usize);
        (*copy)
            .p_image_opaque_binds = writer
            .write_arr_ptr(
                self.p_image_opaque_binds,
                (self.image_opaque_bind_count) as usize,
            );
        (*copy)
            .p_image_binds = writer
            .write_arr_ptr(self.p_image_binds, (self.image_bind_count) as usize);
        (*copy)
            .p_signal_semaphores = writer
            .write_arr_ptr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::ShaderModuleCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_code, (self.code_size / 4) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_code = writer.write_arr_ptr(self.p_code, (self.code_size / 4) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorSetLayoutBinding {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_immutable_samplers,
                    (self.descriptor_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_immutable_samplers = writer
            .write_arr_ptr(self.p_immutable_samplers, (self.descriptor_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorSetLayoutCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_bindings, (self.binding_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_bindings = writer
            .write_arr_ptr(self.p_bindings, (self.binding_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorPoolCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_pool_sizes, (self.pool_size_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_pool_sizes = writer
            .write_arr_ptr(self.p_pool_sizes, (self.pool_size_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorSetAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_set_layouts,
                    (self.descriptor_set_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_set_layouts = writer
            .write_arr_ptr(self.p_set_layouts, (self.descriptor_set_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SpecializationInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_map_entries, (self.map_entry_count) as usize);
            measure.measure_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_map_entries = writer
            .write_arr_ptr(self.p_map_entries, (self.map_entry_count) as usize);
        (*copy)
            .p_data = writer
            .write_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineShaderStageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_name);
            measure.measure_ptr(self.p_specialization_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_name = writer.write_cstr(self.p_name);
        (*copy).p_specialization_info = writer.write_ptr(self.p_specialization_info);
    }
}
unsafe impl DeepCopy for crate::vk10::ComputePipelineCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.stage);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.stage.copy(&mut (*copy).stage, writer);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineVertexInputStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_vertex_binding_descriptions,
                    (self.vertex_binding_description_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_vertex_attribute_descriptions,
                    (self.vertex_attribute_description_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_vertex_binding_descriptions = writer
            .write_arr_ptr(
                self.p_vertex_binding_descriptions,
                (self.vertex_binding_description_count) as usize,
            );
        (*copy)
            .p_vertex_attribute_descriptions = writer
            .write_arr_ptr(
                self.p_vertex_attribute_descriptions,
                (self.vertex_attribute_description_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineInputAssemblyStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineTessellationStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineViewportStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_viewports, (self.viewport_count) as usize);
            measure.measure_arr_ptr(self.p_scissors, (self.scissor_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_viewports = writer
            .write_arr_ptr(self.p_viewports, (self.viewport_count) as usize);
        (*copy)
            .p_scissors = writer
            .write_arr_ptr(self.p_scissors, (self.scissor_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineRasterizationStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineMultisampleStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_sample_mask,
                    ((self.rasterization_samples.0 + 31) / 32) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_sample_mask = writer
            .write_arr_ptr(
                self.p_sample_mask,
                ((self.rasterization_samples.0 + 31) / 32) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineColorBlendStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineDynamicStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_dynamic_states,
                    (self.dynamic_state_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_dynamic_states = writer
            .write_arr_ptr(self.p_dynamic_states, (self.dynamic_state_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineDepthStencilStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::GraphicsPipelineCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_stages, (self.stage_count) as usize);
            measure.measure_ptr(self.p_vertex_input_state);
            measure.measure_ptr(self.p_input_assembly_state);
            measure.measure_ptr(self.p_tessellation_state);
            measure.measure_ptr(self.p_viewport_state);
            measure.measure_ptr(self.p_rasterization_state);
            measure.measure_ptr(self.p_multisample_state);
            measure.measure_ptr(self.p_depth_stencil_state);
            measure.measure_ptr(self.p_color_blend_state);
            measure.measure_ptr(self.p_dynamic_state);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_stages = writer.write_arr_ptr(self.p_stages, (self.stage_count) as usize);
        (*copy).p_vertex_input_state = writer.write_ptr(self.p_vertex_input_state);
        (*copy).p_input_assembly_state = writer.write_ptr(self.p_input_assembly_state);
        (*copy).p_tessellation_state = writer.write_ptr(self.p_tessellation_state);
        (*copy).p_viewport_state = writer.write_ptr(self.p_viewport_state);
        (*copy).p_rasterization_state = writer.write_ptr(self.p_rasterization_state);
        (*copy).p_multisample_state = writer.write_ptr(self.p_multisample_state);
        (*copy).p_depth_stencil_state = writer.write_ptr(self.p_depth_stencil_state);
        (*copy).p_color_blend_state = writer.write_ptr(self.p_color_blend_state);
        (*copy).p_dynamic_state = writer.write_ptr(self.p_dynamic_state);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineCacheCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_initial_data.cast::<u8>(),
                    (self.initial_data_size) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_initial_data = writer
            .write_arr_ptr(
                self.p_initial_data.cast::<u8>(),
                (self.initial_data_size) as usize,
            )
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineLayoutCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_set_layouts, (self.set_layout_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_push_constant_ranges,
                    (self.push_constant_range_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_set_layouts = writer
            .write_arr_ptr(self.p_set_layouts, (self.set_layout_count) as usize);
        (*copy)
            .p_push_constant_ranges = writer
            .write_arr_ptr(
                self.p_push_constant_ranges,
                (self.push_constant_range_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::SamplerCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandPoolCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandBufferAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandBufferInheritanceInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandBufferBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_inheritance_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_inheritance_info = writer.write_ptr(self.p_inheritance_info);
    }
}
unsafe impl DeepCopy for crate::vk10::RenderPassBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_clear_values, (self.clear_value_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_clear_values = writer
            .write_arr_ptr(self.p_clear_values, (self.clear_value_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SubpassDescription {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_input_attachments,
                    (self.input_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_color_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_resolve_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure.measure_ptr(self.p_depth_stencil_attachment);
            measure
                .measure_arr_ptr(
                    self.p_preserve_attachments,
                    (self.preserve_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_input_attachments = writer
            .write_arr_ptr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
            );
        (*copy)
            .p_color_attachments = writer
            .write_arr_ptr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_resolve_attachments = writer
            .write_arr_ptr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_depth_stencil_attachment = writer
            .write_ptr(self.p_depth_stencil_attachment);
        (*copy)
            .p_preserve_attachments = writer
            .write_arr_ptr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::RenderPassCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
            measure.measure_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
            measure
                .measure_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        (*copy)
            .p_subpasses = writer
            .write_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
        (*copy)
            .p_dependencies = writer
            .write_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::EventCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::FenceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::SemaphoreCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::QueryPoolCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::FramebufferCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphores,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_wait_dst_stage_mask,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_command_buffers,
                    (self.command_buffer_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphores,
                    (self.signal_semaphore_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphores = writer
            .write_arr_ptr(self.p_wait_semaphores, (self.wait_semaphore_count) as usize);
        (*copy)
            .p_wait_dst_stage_mask = writer
            .write_arr_ptr(
                self.p_wait_dst_stage_mask,
                (self.wait_semaphore_count) as usize,
            );
        (*copy)
            .p_command_buffers = writer
            .write_arr_ptr(self.p_command_buffers, (self.command_buffer_count) as usize);
        (*copy)
            .p_signal_semaphores = writer
            .write_arr_ptr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::PresentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphores,
                    (self.wait_semaphore_count) as usize,
                );
            measure.measure_arr_ptr(self.p_swapchains, (self.swapchain_count) as usize);
            measure
                .measure_arr_ptr(self.p_image_indices, (self.swapchain_count) as usize);
            measure.measure_arr_ptr(self.p_results, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphores = writer
            .write_arr_ptr(self.p_wait_semaphores, (self.wait_semaphore_count) as usize);
        (*copy)
            .p_swapchains = writer
            .write_arr_ptr(self.p_swapchains, (self.swapchain_count) as usize);
        (*copy)
            .p_image_indices = writer
            .write_arr_ptr(self.p_image_indices, (self.swapchain_count) as usize);
        (*copy)
            .p_results = writer
            .write_arr_ptr(self.p_results, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceFeatures2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::FormatProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ImageFormatProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceImageFormatInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::QueueFamilyProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMemoryProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::SparseImageFormatProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceSparseImageFormatInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceVariablePointersFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalImageFormatInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalImageFormatProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalBufferInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalBufferProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceIDProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalMemoryImageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalMemoryBufferCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExportMemoryAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalSemaphoreInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalSemaphoreProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ExportSemaphoreCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalFenceInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalFenceProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ExportFenceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMultiviewFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMultiviewProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::RenderPassMultiviewCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_view_masks, (self.subpass_count) as usize);
            measure
                .measure_arr_ptr(self.p_view_offsets, (self.dependency_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_correlation_masks,
                    (self.correlation_mask_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_view_masks = writer
            .write_arr_ptr(self.p_view_masks, (self.subpass_count) as usize);
        (*copy)
            .p_view_offsets = writer
            .write_arr_ptr(self.p_view_offsets, (self.dependency_count) as usize);
        (*copy)
            .p_correlation_masks = writer
            .write_arr_ptr(
                self.p_correlation_masks,
                (self.correlation_mask_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceGroupProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryAllocateFlagsInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindBufferMemoryInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindBufferMemoryDeviceGroupInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_indices,
                    (self.device_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_indices = writer
            .write_arr_ptr(self.p_device_indices, (self.device_index_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::BindImageMemoryInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindImageMemoryDeviceGroupInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_indices,
                    (self.device_index_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_split_instance_bind_regions,
                    (self.split_instance_bind_region_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_indices = writer
            .write_arr_ptr(self.p_device_indices, (self.device_index_count) as usize);
        (*copy)
            .p_split_instance_bind_regions = writer
            .write_arr_ptr(
                self.p_split_instance_bind_regions,
                (self.split_instance_bind_region_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupRenderPassBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_render_areas,
                    (self.device_render_area_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_render_areas = writer
            .write_arr_ptr(
                self.p_device_render_areas,
                (self.device_render_area_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupCommandBufferBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_device_indices,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_command_buffer_device_masks,
                    (self.command_buffer_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_device_indices,
                    (self.signal_semaphore_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_device_indices = writer
            .write_arr_ptr(
                self.p_wait_semaphore_device_indices,
                (self.wait_semaphore_count) as usize,
            );
        (*copy)
            .p_command_buffer_device_masks = writer
            .write_arr_ptr(
                self.p_command_buffer_device_masks,
                (self.command_buffer_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_device_indices = writer
            .write_arr_ptr(
                self.p_signal_semaphore_device_indices,
                (self.signal_semaphore_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupBindSparseInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::ImageSwapchainCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::AcquireNextImageInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::DeviceGroupPresentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_device_masks, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_masks = writer
            .write_arr_ptr(self.p_device_masks, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupDeviceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_physical_devices,
                    (self.physical_device_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_physical_devices = writer
            .write_arr_ptr(
                self.p_physical_devices,
                (self.physical_device_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::DescriptorUpdateTemplateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_descriptor_update_entries,
                    (self.descriptor_update_entry_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_descriptor_update_entries = writer
            .write_arr_ptr(
                self.p_descriptor_update_entries,
                (self.descriptor_update_entry_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::RenderPassInputAttachmentAspectCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_aspect_references,
                    (self.aspect_reference_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_aspect_references = writer
            .write_arr_ptr(
                self.p_aspect_references,
                (self.aspect_reference_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDevice16BitStorageFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceSubgroupProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::BufferMemoryRequirementsInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImageMemoryRequirementsInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImageSparseMemoryRequirementsInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryRequirements2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::SparseImageMemoryRequirements2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDevicePointClippingProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryDedicatedRequirements {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryDedicatedAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImageViewUsageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PipelineTessellationDomainOriginStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::SamplerYcbcrConversionInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::SamplerYcbcrConversionCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindImagePlaneMemoryInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImagePlaneMemoryRequirementsInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::SamplerYcbcrConversionImageFormatProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ProtectedSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceProtectedMemoryFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceProtectedMemoryProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceQueueInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_image_format_list::ImageFormatListCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_view_formats = writer
            .write_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMaintenance3Properties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::DescriptorSetLayoutSupport {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceShaderDrawParametersFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_object_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_object_name = writer.write_cstr(self.p_object_name);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_tag.cast::<u8>(), (self.tag_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_tag = writer
            .write_arr_ptr(self.p_tag.cast::<u8>(), (self.tag_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::extensions::ext_debug_utils::DebugUtilsLabelEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_label_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_label_name = writer.write_cstr(self.p_label_name);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_message_id_name);
            measure.measure_cstr(self.p_message);
            measure
                .measure_arr_ptr(self.p_queue_labels, (self.queue_label_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_cmd_buf_labels,
                    (self.cmd_buf_label_count) as usize,
                );
            measure.measure_arr_ptr(self.p_objects, (self.object_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_message_id_name = writer.write_cstr(self.p_message_id_name);
        (*copy).p_message = writer.write_cstr(self.p_message);
        (*copy)
            .p_queue_labels = writer
            .write_arr_ptr(self.p_queue_labels, (self.queue_label_count) as usize);
        (*copy)
            .p_cmd_buf_labels = writer
            .write_arr_ptr(self.p_cmd_buf_labels, (self.cmd_buf_label_count) as usize);
        (*copy)
            .p_objects = writer
            .write_arr_ptr(self.p_objects, (self.object_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_create_renderpass2::AttachmentDescription2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_create_renderpass2::AttachmentReference2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_create_renderpass2::SubpassDescription2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_input_attachments,
                    (self.input_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_color_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_resolve_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure.measure_ptr(self.p_depth_stencil_attachment);
            measure
                .measure_arr_ptr(
                    self.p_preserve_attachments,
                    (self.preserve_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_input_attachments = writer
            .write_arr_ptr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
            );
        (*copy)
            .p_color_attachments = writer
            .write_arr_ptr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_resolve_attachments = writer
            .write_arr_ptr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_depth_stencil_attachment = writer
            .write_ptr(self.p_depth_stencil_attachment);
        (*copy)
            .p_preserve_attachments = writer
            .write_arr_ptr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_create_renderpass2::SubpassDependency2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_create_renderpass2::RenderPassCreateInfo2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
            measure.measure_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
            measure
                .measure_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_correlated_view_masks,
                    (self.correlated_view_mask_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        (*copy)
            .p_subpasses = writer
            .write_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
        (*copy)
            .p_dependencies = writer
            .write_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
        (*copy)
            .p_correlated_view_masks = writer
            .write_arr_ptr(
                self.p_correlated_view_masks,
                (self.correlated_view_mask_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::extensions::khr_create_renderpass2::SubpassBeginInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_create_renderpass2::SubpassEndInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_timeline_semaphore::PhysicalDeviceTimelineSemaphoreFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_timeline_semaphore::PhysicalDeviceTimelineSemaphorePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_timeline_semaphore::SemaphoreTypeCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_timeline_semaphore::TimelineSemaphoreSubmitInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_values,
                    (self.wait_semaphore_value_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_values,
                    (self.signal_semaphore_value_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_values = writer
            .write_arr_ptr(
                self.p_wait_semaphore_values,
                (self.wait_semaphore_value_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_values = writer
            .write_arr_ptr(
                self.p_signal_semaphore_values,
                (self.signal_semaphore_value_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_timeline_semaphore::SemaphoreWaitInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_semaphores, (self.semaphore_count) as usize);
            measure.measure_arr_ptr(self.p_values, (self.semaphore_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_semaphores = writer
            .write_arr_ptr(self.p_semaphores, (self.semaphore_count) as usize);
        (*copy)
            .p_values = writer
            .write_arr_ptr(self.p_values, (self.semaphore_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_timeline_semaphore::SemaphoreSignalInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_depth_stencil_resolve::PhysicalDeviceDepthStencilResolvePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_depth_stencil_resolve::SubpassDescriptionDepthStencilResolveKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_depth_stencil_resolve_attachment);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_depth_stencil_resolve_attachment = writer
            .write_ptr(self.p_depth_stencil_resolve_attachment);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_scalar_block_layout::PhysicalDeviceScalarBlockLayoutFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_imageless_framebuffer::PhysicalDeviceImagelessFramebufferFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_imageless_framebuffer::FramebufferAttachmentsCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_attachment_image_infos,
                    (self.attachment_image_info_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachment_image_infos = writer
            .write_arr_ptr(
                self.p_attachment_image_infos,
                (self.attachment_image_info_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_imageless_framebuffer::FramebufferAttachmentImageInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_view_formats = writer
            .write_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_imageless_framebuffer::RenderPassAttachmentBeginInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_synchronization2::MemoryBarrier2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_synchronization2::ImageMemoryBarrier2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_synchronization2::BufferMemoryBarrier2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_synchronization2::DependencyInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_memory_barriers,
                    (self.memory_barrier_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_buffer_memory_barriers,
                    (self.buffer_memory_barrier_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_image_memory_barriers,
                    (self.image_memory_barrier_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_memory_barriers = writer
            .write_arr_ptr(self.p_memory_barriers, (self.memory_barrier_count) as usize);
        (*copy)
            .p_buffer_memory_barriers = writer
            .write_arr_ptr(
                self.p_buffer_memory_barriers,
                (self.buffer_memory_barrier_count) as usize,
            );
        (*copy)
            .p_image_memory_barriers = writer
            .write_arr_ptr(
                self.p_image_memory_barriers,
                (self.image_memory_barrier_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_synchronization2::SemaphoreSubmitInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_synchronization2::CommandBufferSubmitInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_synchronization2::SubmitInfo2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_infos,
                    (self.wait_semaphore_info_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_command_buffer_infos,
                    (self.command_buffer_info_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_infos,
                    (self.signal_semaphore_info_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_infos = writer
            .write_arr_ptr(
                self.p_wait_semaphore_infos,
                (self.wait_semaphore_info_count) as usize,
            );
        (*copy)
            .p_command_buffer_infos = writer
            .write_arr_ptr(
                self.p_command_buffer_infos,
                (self.command_buffer_info_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_infos = writer
            .write_arr_ptr(
                self.p_signal_semaphore_infos,
                (self.signal_semaphore_info_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_synchronization2::PhysicalDeviceSynchronization2FeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::PipelineRenderingCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachment_formats,
                    (self.color_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachment_formats = writer
            .write_arr_ptr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::extensions::khr_dynamic_rendering::RenderingInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure.measure_ptr(self.p_depth_attachment);
            measure.measure_ptr(self.p_stencil_attachment);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachments = writer
            .write_arr_ptr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy).p_depth_attachment = writer.write_ptr(self.p_depth_attachment);
        (*copy).p_stencil_attachment = writer.write_ptr(self.p_stencil_attachment);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::PhysicalDeviceDynamicRenderingFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::CommandBufferInheritanceRenderingInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachment_formats,
                    (self.color_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachment_formats = writer
            .write_arr_ptr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
            );
    }
}
value_type_copy_impl! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize, crate
    ::vk10::QueryPoolCreateFlags, crate ::vk10::PipelineDynamicStateCreateFlags, crate
    ::vk10::PipelineMultisampleStateCreateFlags, crate
    ::vk10::PipelineRasterizationStateCreateFlags, crate
    ::vk10::PipelineViewportStateCreateFlags, crate
    ::vk10::PipelineTessellationStateCreateFlags, crate
    ::vk10::PipelineInputAssemblyStateCreateFlags, crate
    ::vk10::PipelineVertexInputStateCreateFlags, crate ::vk10::BufferViewCreateFlags,
    crate ::vk10::DeviceCreateFlags, crate ::vk10::SemaphoreCreateFlags, crate
    ::vk10::ShaderModuleCreateFlags, crate ::vk10::MemoryMapFlags, crate
    ::vk10::DescriptorPoolResetFlags, crate ::vk11::DescriptorUpdateTemplateCreateFlags,
    crate ::extensions::khr_timeline_semaphore::SemaphoreWaitFlagsKHR, crate
    ::extensions::khr_synchronization2::AccessFlags2KHR, crate
    ::extensions::khr_synchronization2::PipelineStageFlags2KHR, crate
    ::extensions::khr_dynamic_rendering::RenderingFlagsKHR, crate
    ::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR, crate
    ::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN, crate
    ::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR, crate
    ::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR, crate
    ::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR, crate
    ::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR, crate
    ::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT, crate
    ::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK, crate
    ::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK, crate
    ::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT, crate
    ::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA, crate
    ::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    crate ::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT, crate
    ::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX, crate
    ::vk11::CommandPoolTrimFlags, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT, crate
    ::extensions::khr_depth_stencil_resolve::ResolveModeFlagsKHR, crate
    ::extensions::khr_synchronization2::SubmitFlagsKHR, crate ::vk10::Instance, crate
    ::vk10::PhysicalDevice, crate ::vk10::Device, crate ::vk10::Queue, crate
    ::vk10::CommandBuffer, crate ::vk10::DeviceMemory, crate ::vk10::CommandPool, crate
    ::vk10::Buffer, crate ::vk10::BufferView, crate ::vk10::Image, crate
    ::vk10::ImageView, crate ::vk10::ShaderModule, crate ::vk10::Pipeline, crate
    ::vk10::PipelineLayout, crate ::vk10::Sampler, crate ::vk10::DescriptorSet, crate
    ::vk10::DescriptorSetLayout, crate ::vk10::DescriptorPool, crate ::vk10::Fence, crate
    ::vk10::Semaphore, crate ::vk10::Event, crate ::vk10::QueryPool, crate
    ::vk10::Framebuffer, crate ::vk10::RenderPass, crate ::vk10::PipelineCache, crate
    ::vk11::DescriptorUpdateTemplate, crate ::vk11::SamplerYcbcrConversion, crate
    ::extensions::khr_surface::SurfaceKHR, crate
    ::extensions::khr_swapchain::SwapchainKHR, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerEXT, crate
    ::extensions::khr_timeline_semaphore::SemaphoreTypeKHR, crate ::vk10::Offset2D, crate
    ::vk10::Offset3D, crate ::vk10::Extent2D, crate ::vk10::Extent3D, crate
    ::vk10::Viewport, crate ::vk10::Rect2D, crate ::vk10::ClearRect, crate
    ::vk10::ComponentMapping, crate ::vk10::PhysicalDeviceProperties, crate
    ::vk10::ExtensionProperties, crate ::vk10::LayerProperties, crate
    ::vk10::QueueFamilyProperties, crate ::vk10::PhysicalDeviceMemoryProperties, crate
    ::vk10::MemoryRequirements, crate ::vk10::SparseImageFormatProperties, crate
    ::vk10::SparseImageMemoryRequirements, crate ::vk10::MemoryType, crate
    ::vk10::MemoryHeap, crate ::vk10::FormatProperties, crate
    ::vk10::ImageFormatProperties, crate ::vk10::DescriptorBufferInfo, crate
    ::vk10::DescriptorImageInfo, crate ::vk10::ImageSubresource, crate
    ::vk10::ImageSubresourceLayers, crate ::vk10::ImageSubresourceRange, crate
    ::vk10::SubresourceLayout, crate ::vk10::BufferCopy, crate ::vk10::SparseMemoryBind,
    crate ::vk10::SparseImageMemoryBind, crate ::vk10::ImageCopy, crate
    ::vk10::ImageBlit, crate ::vk10::BufferImageCopy, crate ::vk10::ImageResolve, crate
    ::vk10::DescriptorPoolSize, crate ::vk10::SpecializationMapEntry, crate
    ::vk10::VertexInputBindingDescription, crate ::vk10::VertexInputAttributeDescription,
    crate ::vk10::PipelineColorBlendAttachmentState, crate ::vk10::StencilOpState, crate
    ::vk10::PipelineCacheHeaderVersionOne, crate ::vk10::PushConstantRange, crate
    ::vk10::ClearColorValue, crate ::vk10::ClearDepthStencilValue, crate
    ::vk10::ClearValue, crate ::vk10::ClearAttachment, crate
    ::vk10::AttachmentDescription, crate ::vk10::AttachmentReference, crate
    ::vk10::SubpassDependency, crate ::vk10::PhysicalDeviceFeatures, crate
    ::vk10::PhysicalDeviceSparseProperties, crate ::vk10::PhysicalDeviceLimits, crate
    ::vk10::DrawIndirectCommand, crate ::vk10::DrawIndexedIndirectCommand, crate
    ::vk10::DispatchIndirectCommand, crate
    ::extensions::khr_surface::SurfaceCapabilitiesKHR, crate
    ::extensions::khr_surface::SurfaceFormatKHR, crate ::vk11::ExternalMemoryProperties,
    crate ::vk11::DescriptorUpdateTemplateEntry, crate
    ::vk11::InputAttachmentAspectReference, crate ::vk10::ImageLayout, crate
    ::vk10::AttachmentLoadOp, crate ::vk10::AttachmentStoreOp, crate ::vk10::ImageType,
    crate ::vk10::ImageTiling, crate ::vk10::ImageViewType, crate
    ::vk10::CommandBufferLevel, crate ::vk10::ComponentSwizzle, crate
    ::vk10::DescriptorType, crate ::vk10::QueryType, crate ::vk10::BorderColor, crate
    ::vk10::PipelineBindPoint, crate ::vk10::PipelineCacheHeaderVersion, crate
    ::vk10::PipelineCacheCreateFlags, crate ::vk10::PrimitiveTopology, crate
    ::vk10::SharingMode, crate ::vk10::IndexType, crate ::vk10::Filter, crate
    ::vk10::SamplerMipmapMode, crate ::vk10::SamplerAddressMode, crate ::vk10::CompareOp,
    crate ::vk10::PolygonMode, crate ::vk10::FrontFace, crate ::vk10::BlendFactor, crate
    ::vk10::BlendOp, crate ::vk10::StencilOp, crate ::vk10::LogicOp, crate
    ::vk10::InternalAllocationType, crate ::vk10::SystemAllocationScope, crate
    ::vk10::PhysicalDeviceType, crate ::vk10::VertexInputRate, crate ::vk10::Format,
    crate ::vk10::StructureType, crate ::vk10::SubpassContents, crate ::vk10::Result,
    crate ::vk10::DynamicState, crate ::vk11::DescriptorUpdateTemplateType, crate
    ::vk10::ObjectType, crate ::vk10::QueueFlags, crate ::vk10::CullModeFlags, crate
    ::vk10::RenderPassCreateFlags, crate ::vk10::DeviceQueueCreateFlags, crate
    ::vk10::MemoryPropertyFlags, crate ::vk10::MemoryHeapFlags, crate
    ::vk10::AccessFlags, crate ::vk10::BufferUsageFlags, crate ::vk10::BufferCreateFlags,
    crate ::vk10::ShaderStageFlags, crate ::vk10::ImageUsageFlags, crate
    ::vk10::ImageCreateFlags, crate ::vk10::ImageViewCreateFlags, crate
    ::vk10::SamplerCreateFlags, crate ::vk10::PipelineCreateFlags, crate
    ::vk10::PipelineShaderStageCreateFlags, crate ::vk10::ColorComponentFlags, crate
    ::vk10::FenceCreateFlags, crate ::vk10::FormatFeatureFlags, crate
    ::vk10::QueryControlFlags, crate ::vk10::QueryResultFlags, crate
    ::vk10::CommandBufferUsageFlags, crate ::vk10::QueryPipelineStatisticFlags, crate
    ::vk10::ImageAspectFlags, crate ::vk10::SparseImageFormatFlags, crate
    ::vk10::SparseMemoryBindFlags, crate ::vk10::PipelineStageFlags, crate
    ::vk10::CommandPoolCreateFlags, crate ::vk10::CommandPoolResetFlags, crate
    ::vk10::CommandBufferResetFlags, crate ::vk10::SampleCountFlags, crate
    ::vk10::AttachmentDescriptionFlags, crate ::vk10::StencilFaceFlags, crate
    ::vk10::DescriptorPoolCreateFlags, crate ::vk10::DependencyFlags, crate
    ::extensions::khr_surface::PresentModeKHR, crate
    ::extensions::khr_surface::ColorSpaceKHR, crate
    ::extensions::khr_surface::CompositeAlphaFlagsKHR, crate
    ::extensions::khr_surface::SurfaceTransformFlagsKHR, crate
    ::vk11::SubgroupFeatureFlags, crate ::vk10::DescriptorSetLayoutCreateFlags, crate
    ::vk11::ExternalMemoryHandleTypeFlags, crate ::vk11::ExternalMemoryFeatureFlags,
    crate ::vk11::ExternalSemaphoreHandleTypeFlags, crate
    ::vk11::ExternalSemaphoreFeatureFlags, crate ::vk11::SemaphoreImportFlags, crate
    ::vk11::ExternalFenceHandleTypeFlags, crate ::vk11::ExternalFenceFeatureFlags, crate
    ::vk11::FenceImportFlags, crate ::vk11::PeerMemoryFeatureFlags, crate
    ::vk11::MemoryAllocateFlags, crate
    ::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR, crate
    ::extensions::khr_swapchain::SwapchainCreateFlagsKHR, crate
    ::vk10::SubpassDescriptionFlags, crate ::vk11::PointClippingBehavior, crate
    ::vk11::TessellationDomainOrigin, crate ::vk11::SamplerYcbcrModelConversion, crate
    ::vk11::SamplerYcbcrRange, crate ::vk11::ChromaLocation, crate
    ::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT, crate ::vk10::VendorId,
    crate ::vk10::FramebufferCreateFlags, crate ::vk10::EventCreateFlags, crate
    ::vk10::PipelineLayoutCreateFlags, crate ::vk10::PipelineColorBlendStateCreateFlags,
    crate ::vk10::PipelineDepthStencilStateCreateFlags, crate ::vk10::InstanceCreateFlags
}
