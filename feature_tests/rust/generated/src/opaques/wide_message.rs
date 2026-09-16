use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// UTF-16 strings in both directions, gated on `utf16_strings`.
pub struct WideMessage {
    pub(crate) inner: NonNull<ffi::WideMessage>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct WideMessageRef<'view> {
    pub(crate) inner: NonNull<ffi::WideMessage>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct WideMessageRefMut<'view> {
    pub(crate) inner: NonNull<ffi::WideMessage>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait WideMessageSharedArg: crate::private::WideMessageSharedSealed {}

#[doc(hidden)]
pub trait WideMessageMutArg: crate::private::WideMessageMutSealed {}

impl crate::private::WideMessageSharedSealed for WideMessage {
    fn __as_const_ptr(&self) -> *const ffi::WideMessage {
        self.inner.as_ptr()
    }
}
impl crate::private::WideMessageMutSealed for WideMessage {
    fn __as_mut_ptr(&mut self) -> *mut ffi::WideMessage {
        self.inner.as_ptr()
    }
}
impl WideMessageSharedArg for WideMessage {}
impl WideMessageMutArg for WideMessage {}
impl<'view> crate::private::WideMessageSharedSealed for WideMessageRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::WideMessage {
        self.inner.as_ptr()
    }
}
impl<'view> WideMessageSharedArg for WideMessageRef<'view> {}
impl<'view> crate::private::WideMessageSharedSealed for WideMessageRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::WideMessage {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::WideMessageMutSealed for WideMessageRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::WideMessage {
        self.inner.as_ptr()
    }
}
impl<'view> WideMessageSharedArg for WideMessageRefMut<'view> {}
impl<'view> WideMessageMutArg for WideMessageRefMut<'view> {}

impl Drop for WideMessage {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::WideMessage_destroy(self.inner.as_ptr()) };
    }
}

impl WideMessage {
    pub fn new(v: &[u16]) -> super::WideMessage {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::WideMessage_new(ffi::DiplomatSlice::<u16> {
                ptr: v.as_ptr(),
                len: v.len(),
            })
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null WideMessage");
            super::WideMessage {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn units<'a>(&'a self) -> &'a [u16] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::WideMessage_units(self.inner.as_ptr() as *const _) };
        unsafe { crate::private::slice_from_raw_parts(result.ptr, result.len) }
    }
}

impl<'view> WideMessageRef<'view> {
    pub fn units<'a>(&'a self) -> &'a [u16] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::WideMessage_units(self.inner.as_ptr() as *const _) };
        unsafe { crate::private::slice_from_raw_parts(result.ptr, result.len) }
    }
}

impl<'view> WideMessageRefMut<'view> {
    pub fn units<'a>(&'a self) -> &'a [u16] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::WideMessage_units(self.inner.as_ptr() as *const _) };
        unsafe { crate::private::slice_from_raw_parts(result.ptr, result.len) }
    }
}
