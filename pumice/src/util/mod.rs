mod access;
mod config;
mod impl_macros;
mod pnext;
mod stage;

pub use {access::*, config::*, impl_macros::*, pnext::*, stage::*};

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
    };
}

#[macro_export]
macro_rules! vkcall {
    ($ok:ident, $call:expr) => {{
        let mut value = Default::default();
        let $ok = &mut value as *mut _;
        let raw = $call;
        $crate::new_result(value, raw)
    }};
    ($call:expr) => {{
        let raw = $call;
        $crate::new_result((), raw)
    }};
}
