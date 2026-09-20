use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RefListParameter {
    pub(crate) inner: NonNull<ffi::RefListParameter>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RefListParameterRef<'view> {
    pub(crate) inner: NonNull<ffi::RefListParameter>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RefListParameterRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RefListParameter>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RefListParameterSharedArg: crate::private::RefListParameterSharedSealed {}

#[doc(hidden)]
pub trait RefListParameterMutArg: crate::private::RefListParameterMutSealed {}

impl crate::private::RefListParameterSharedSealed for RefListParameter {
    fn __as_const_ptr(&self) -> *const ffi::RefListParameter {
        self.inner.as_ptr()
    }
}
impl crate::private::RefListParameterMutSealed for RefListParameter {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RefListParameter {
        self.inner.as_ptr()
    }
}
impl RefListParameterSharedArg for RefListParameter {}
impl RefListParameterMutArg for RefListParameter {}
impl<'view> crate::private::RefListParameterSharedSealed for RefListParameterRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RefListParameter {
        self.inner.as_ptr()
    }
}
impl<'view> RefListParameterSharedArg for RefListParameterRef<'view> {}
impl<'view> crate::private::RefListParameterSharedSealed for RefListParameterRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RefListParameter {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::RefListParameterMutSealed for RefListParameterRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RefListParameter {
        self.inner.as_ptr()
    }
}
impl<'view> RefListParameterSharedArg for RefListParameterRefMut<'view> {}
impl<'view> RefListParameterMutArg for RefListParameterRefMut<'view> {}

impl Drop for RefListParameter {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::RefListParameter_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RefListParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefListParameter")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RefListParameterRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefListParameterRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RefListParameterRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefListParameterRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RefListParameter {}

impl<'view> RefListParameterRef<'view> {}

impl<'view> RefListParameterRefMut<'view> {}
