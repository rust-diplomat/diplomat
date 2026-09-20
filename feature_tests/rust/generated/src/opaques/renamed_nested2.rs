use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedNested2 {
    pub(crate) inner: NonNull<ffi::RenamedNested2>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedNested2Ref<'view> {
    pub(crate) inner: NonNull<ffi::RenamedNested2>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedNested2RefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedNested2>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedNested2SharedArg: crate::private::RenamedNested2SharedSealed {}

#[doc(hidden)]
pub trait RenamedNested2MutArg: crate::private::RenamedNested2MutSealed {}

impl crate::private::RenamedNested2SharedSealed for RenamedNested2 {
    fn __as_const_ptr(&self) -> *const ffi::RenamedNested2 {
        self.inner.as_ptr()
    }
}
impl crate::private::RenamedNested2MutSealed for RenamedNested2 {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedNested2 {
        self.inner.as_ptr()
    }
}
impl RenamedNested2SharedArg for RenamedNested2 {}
impl RenamedNested2MutArg for RenamedNested2 {}
impl<'view> crate::private::RenamedNested2SharedSealed for RenamedNested2Ref<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedNested2 {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedNested2SharedArg for RenamedNested2Ref<'view> {}
impl<'view> crate::private::RenamedNested2SharedSealed for RenamedNested2RefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedNested2 {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::RenamedNested2MutSealed for RenamedNested2RefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedNested2 {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedNested2SharedArg for RenamedNested2RefMut<'view> {}
impl<'view> RenamedNested2MutArg for RenamedNested2RefMut<'view> {}

impl Drop for RenamedNested2 {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_Nested2_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedNested2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedNested2")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedNested2Ref<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedNested2Ref")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedNested2RefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedNested2RefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedNested2 {}

impl<'view> RenamedNested2Ref<'view> {}

impl<'view> RenamedNested2RefMut<'view> {}
