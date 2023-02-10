#[macro_export]
macro_rules! enum_impl {
    ($name:ident: $ty:ident, $($variant:ident),*) => {
        impl $name {
            #[inline]
            pub const fn as_raw(self) -> $ty {
                self.0
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(
                    match *self {
                        $(
                            Self::$variant => stringify!($variant),
                        )+
                        _ => return f.write_str("(unknown)")
                    }
                )
            }
        }
    };
}

#[macro_export]
macro_rules! bitflags_impl {
    ($name:ident: $ty:ty, $all:expr, $($variant:ident),*) => {
        impl $name {
            #[inline]
            pub const fn empty() -> Self {
                Self(0)
            }
            #[inline]
            pub const fn all() -> Self {
                Self($all)
            }
            #[inline]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
            #[inline]
            pub const fn is_all(self) -> bool {
                self.0 & Self::all().0 == Self::all().0
            }
            #[inline]
            pub const fn intersects(self, other: Self) -> bool {
                self.0 & other.0 != Self::empty().0
            }
            #[inline]
            pub const fn contains(self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
            #[inline]
            pub fn as_raw(self) -> $ty {
                self.0
            }
        }
        impl ::std::ops::BitOr for $name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }
        impl ::std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = *self | rhs
            }
        }
        impl ::std::ops::BitAnd for $name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }
        impl ::std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = *self & rhs
            }
        }
        impl ::std::ops::BitXor for $name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ::std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = *self ^ rhs
            }
        }
        impl ::std::ops::Sub for $name {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                self & !rhs
            }
        }
        impl ::std::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs
            }
        }
        impl ::std::ops::Not for $name {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                self ^ Self::all()
            }
        }
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut first = true;
                $(
                    if self.contains(Self::$variant) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(::std::stringify!($variant))?;
                    }
                )*
                let extra_bits = self.0 & !Self::all().0;
                if extra_bits != 0 {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("0x")?;
                    ::std::fmt::LowerHex::fmt(&extra_bits, f)?;
                }
                if first {
                    f.write_str("(empty)")?;
                }
                Ok(())
            }
        }
    };
}

// stolen from the bitflags crate for use with the our own bitflags macro
/// This macro allows easily implementing the same bitflags as are generated in this crate.
#[macro_export]
macro_rules! bitflags_impl_external {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $vis struct $BitFlags(pub $T);

        impl $BitFlags {
            $(
                $vis const $Flag: Self = Self($value);
            )*
        }

        $crate::bitflags_impl! {
            $BitFlags: $T, $($value)|*, $($Flag),*
        }
    };
    () => {};
}

pub trait ObjectHandle {
    const TYPE: crate::vk10::ObjectType;
    fn null() -> Self;
    fn as_raw(self) -> u64;
    fn from_raw(raw: u64) -> Self;
}

// adapted from erupt
#[doc(hidden)]
#[macro_export]
macro_rules! non_dispatchable_handle {
    ($name:ident, $ty:ident, $doc_alias:literal, $doc:literal) => {
        #[doc = $doc]
        #[doc(alias = $doc_alias)]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        pub struct $name(pub u64);

        impl $crate::util::ObjectHandle for $name {
            const TYPE: $crate::vk10::ObjectType = $crate::vk10::ObjectType::$ty;

            #[inline]
            fn null() -> Self {
                Self(0)
            }

            #[inline]
            fn as_raw(self) -> u64 {
                self.0
            }

            #[inline]
            fn from_raw(raw: u64) -> Self {
                $name(raw)
            }
        }

        impl std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}

// adapted from erupt
#[doc(hidden)]
#[macro_export]
macro_rules! dispatchable_handle {
    ($name:ident, $ty:ident, $doc_alias:literal, $doc:literal) => {
        #[doc = $doc]
        #[doc(alias = $doc_alias)]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(pub *mut ());

        impl $crate::util::ObjectHandle for $name {
            const TYPE: $crate::vk10::ObjectType = $crate::vk10::ObjectType::$ty;

            #[inline]
            fn null() -> Self {
                Self(std::ptr::null_mut())
            }

            #[inline]
            fn as_raw(self) -> u64 {
                self.0 as u64
            }

            #[inline]
            fn from_raw(raw: u64) -> Self {
                $name(raw as _)
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Default for $name {
            fn default() -> $name {
                $name(std::ptr::null_mut())
            }
        }

        impl std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Pointer::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}
