//! Sealed capability traits and unsafe ABI reconstruction helpers.
//!
//! This module is private, so downstream crates cannot name the sealed traits and
//! therefore cannot implement the public capability traits themselves. The helpers
//! may be unused for a given provider, hence the `dead_code` allow.
#![allow(dead_code)]

pub trait BytesSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Bytes;
}
pub trait BytesMutSealed: BytesSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Bytes;
}
pub trait ChildSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Child;
}
pub trait ChildMutSealed: ChildSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Child;
}
pub trait CounterSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Counter;
}
pub trait CounterMutSealed: CounterSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Counter;
}
pub trait Float64VecSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Float64Vec;
}
pub trait Float64VecMutSealed: Float64VecSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Float64Vec;
}
pub trait MessageSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Message;
}
pub trait MessageMutSealed: MessageSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Message;
}
pub trait NumbersSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Numbers;
}
pub trait NumbersMutSealed: NumbersSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Numbers;
}
pub trait SliceViewSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::SliceView;
}
pub trait SliceViewMutSealed: SliceViewSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::SliceView;
}
pub trait WideMessageSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::WideMessage;
}
pub trait WideMessageMutSealed: WideMessageSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::WideMessage;
}

/// Reconstruct a shared slice from a provider-returned pointer/length pair.
///
/// # Safety
///
/// The caller must uphold the provider's validity, alignment, aliasing, and
/// lifetime contract for `ptr`/`len` for the returned lifetime.
pub(crate) unsafe fn slice_from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if ptr.is_null() {
        debug_assert_eq!(len, 0, "provider returned a null slice with nonzero length");
        return &[];
    }
    core::slice::from_raw_parts(ptr, len)
}

/// Reconstruct an exclusive slice from a provider-returned pointer/length pair.
///
/// # Safety
///
/// The caller must uphold the provider's validity, alignment, uniqueness, and
/// lifetime contract for `ptr`/`len` for the returned lifetime.
pub(crate) unsafe fn slice_from_raw_parts_mut<'a, T>(ptr: *mut T, len: usize) -> &'a mut [T] {
    if ptr.is_null() {
        debug_assert_eq!(len, 0, "provider returned a null slice with nonzero length");
        return &mut [];
    }
    core::slice::from_raw_parts_mut(ptr, len)
}

/// Reconstruct a validated `&str` from a provider-returned pointer/length pair.
///
/// # Safety
///
/// The caller must uphold the provider's validity, alignment, aliasing, UTF-8,
/// and lifetime contract for `ptr`/`len` for the returned lifetime.
pub(crate) unsafe fn str_from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a str {
    core::str::from_utf8_unchecked(slice_from_raw_parts(ptr, len))
}

/// Take ownership of a provider-allocated `Box<[T]>` returned across the ABI.
///
/// # Safety
///
/// `ptr`/`len` must describe a `Box<[T]>` allocated by the provider, and the
/// provider and consumer must share an allocator (Diplomat's owned-slice
/// contract). Ownership transfers to the returned `Box`.
pub(crate) unsafe fn owned_slice_into_box<T>(ptr: *mut T, len: usize) -> Box<[T]> {
    if ptr.is_null() {
        debug_assert_eq!(len, 0, "provider returned a null slice with nonzero length");
        return Box::new([]);
    }
    Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len))
}
