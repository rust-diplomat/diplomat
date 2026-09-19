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

/// Reconstruct a validated `&str` from a provider-returned UTF-8 slice.
///
/// The runtime can turn a `DiplomatSlice` back into a `&[T]`, but
/// `DiplomatUtf8StrSlice`'s field is private, so this is the one conversion the
/// runtime cannot express and the generated crate keeps locally.
///
/// # Safety
///
/// The caller must uphold the provider's validity, alignment, aliasing, UTF-8,
/// and lifetime contract for the slice for the returned lifetime. The ABI type
/// cannot express UTF-8 validity, so the provider must send valid UTF-8.
pub(crate) unsafe fn utf8_str_from_slice<'a>(
    slice: diplomat_runtime::DiplomatSlice<'a, u8>,
) -> &'a str {
    core::str::from_utf8_unchecked(<&[u8]>::from(slice))
}
