use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// An error that owns a native allocation.
///
/// A `Result`'s error payload crosses the ABI as a raw pointer, so the generated
/// wrapper is its only owner: a `?` that discards the error still has to run this
/// destructor.
pub struct AllocationFailure {
    pub(crate) inner: NonNull<ffi::AllocationFailure>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct AllocationFailureRef<'view> {
    pub(crate) inner: NonNull<ffi::AllocationFailure>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct AllocationFailureRefMut<'view> {
    pub(crate) inner: NonNull<ffi::AllocationFailure>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait AllocationFailureSharedArg: crate::private::AllocationFailureSharedSealed {}

#[doc(hidden)]
pub trait AllocationFailureMutArg: crate::private::AllocationFailureMutSealed {}

impl crate::private::AllocationFailureSharedSealed for AllocationFailure {
    fn __as_const_ptr(&self) -> *const ffi::AllocationFailure {
        self.inner.as_ptr()
    }
}
impl crate::private::AllocationFailureMutSealed for AllocationFailure {
    fn __as_mut_ptr(&mut self) -> *mut ffi::AllocationFailure {
        self.inner.as_ptr()
    }
}
impl AllocationFailureSharedArg for AllocationFailure {}
impl AllocationFailureMutArg for AllocationFailure {}
impl<'view> crate::private::AllocationFailureSharedSealed for AllocationFailureRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::AllocationFailure {
        self.inner.as_ptr()
    }
}
impl<'view> AllocationFailureSharedArg for AllocationFailureRef<'view> {}
impl<'view> crate::private::AllocationFailureSharedSealed for AllocationFailureRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::AllocationFailure {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::AllocationFailureMutSealed for AllocationFailureRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::AllocationFailure {
        self.inner.as_ptr()
    }
}
impl<'view> AllocationFailureSharedArg for AllocationFailureRefMut<'view> {}
impl<'view> AllocationFailureMutArg for AllocationFailureRefMut<'view> {}

impl Drop for AllocationFailure {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::AllocationFailure_destroy(self.inner.as_ptr()) };
    }
}

impl AllocationFailure {
    pub fn code(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::AllocationFailure_code(self.inner.as_ptr() as *const _) }
    }
    pub fn reset_drop_count() {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::AllocationFailure_reset_drop_count() };
    }
    pub fn drop_count() -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::AllocationFailure_drop_count() }
    }
}

impl<'view> AllocationFailureRef<'view> {
    pub fn code(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::AllocationFailure_code(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> AllocationFailureRefMut<'view> {
    pub fn code(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::AllocationFailure_code(self.inner.as_ptr() as *const _) }
    }
}
