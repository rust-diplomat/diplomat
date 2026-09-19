use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
use crate::types::*;

/// Owned opaque whose type-level lifetime is tied to the caller's buffer.
pub struct SliceView<'a> {
    pub(crate) inner: NonNull<ffi::SliceView>,
    pub(crate) _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct SliceViewRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::SliceView>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct SliceViewRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::SliceView>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait SliceViewSharedArg: crate::private::SliceViewSharedSealed {}

#[doc(hidden)]
pub trait SliceViewMutArg: crate::private::SliceViewMutSealed {}

impl<'a> crate::private::SliceViewSharedSealed for SliceView<'a> {
    fn __as_const_ptr(&self) -> *const ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'a> crate::private::SliceViewMutSealed for SliceView<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'a> SliceViewSharedArg for SliceView<'a> {}
impl<'a> SliceViewMutArg for SliceView<'a> {}
impl<'view, 'a> crate::private::SliceViewSharedSealed for SliceViewRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> SliceViewSharedArg for SliceViewRef<'view, 'a> {}
impl<'view, 'a> crate::private::SliceViewSharedSealed for SliceViewRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> crate::private::SliceViewMutSealed for SliceViewRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> SliceViewSharedArg for SliceViewRefMut<'view, 'a> {}
impl<'view, 'a> SliceViewMutArg for SliceViewRefMut<'view, 'a> {}

impl<'a> Drop for SliceView<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::SliceView_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> SliceView<'a> {
    pub fn wrap(data: &'a [u8]) -> super::SliceView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_wrap(ffi::DiplomatSlice::from(data)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null SliceView");
            super::SliceView {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_len(self.inner.as_ptr() as *const _) }
    }
    pub fn is_empty(&self) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_is_empty(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn data<'b>(&'b self) -> &'b [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_data(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn fields<'anon_0>(&'anon_0 self) -> FieldView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_fields(self.inner.as_ptr() as *const _) };
        FieldView {
            bytes: result.bytes.into(),
            count: result.count,
            _lifetimes: PhantomData,
        }
    }
}

impl<'view, 'a> SliceViewRef<'view, 'a> {
    pub fn len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_len(self.inner.as_ptr() as *const _) }
    }
    pub fn is_empty(&self) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_is_empty(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn data<'b>(&'b self) -> &'b [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_data(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn fields<'anon_0>(&'anon_0 self) -> FieldView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_fields(self.inner.as_ptr() as *const _) };
        FieldView {
            bytes: result.bytes.into(),
            count: result.count,
            _lifetimes: PhantomData,
        }
    }
}

impl<'view, 'a> SliceViewRefMut<'view, 'a> {
    pub fn len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_len(self.inner.as_ptr() as *const _) }
    }
    pub fn is_empty(&self) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_is_empty(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn data<'b>(&'b self) -> &'b [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_data(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn fields<'anon_0>(&'anon_0 self) -> FieldView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_fields(self.inner.as_ptr() as *const _) };
        FieldView {
            bytes: result.bytes.into(),
            count: result.count,
            _lifetimes: PhantomData,
        }
    }
}
