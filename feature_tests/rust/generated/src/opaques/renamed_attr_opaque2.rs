use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedAttrOpaque2 {
    pub(crate) inner: NonNull<ffi::RenamedAttrOpaque2>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedAttrOpaque2Ref<'view> {
    pub(crate) inner: NonNull<ffi::RenamedAttrOpaque2>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedAttrOpaque2RefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedAttrOpaque2>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedAttrOpaque2SharedArg: crate::private::RenamedAttrOpaque2SharedSealed {}

#[doc(hidden)]
pub trait RenamedAttrOpaque2MutArg: crate::private::RenamedAttrOpaque2MutSealed {}

impl crate::private::RenamedAttrOpaque2SharedSealed for RenamedAttrOpaque2 {
    fn __as_const_ptr(&self) -> *const ffi::RenamedAttrOpaque2 {
        self.inner.as_ptr()
    }
}
impl crate::private::RenamedAttrOpaque2MutSealed for RenamedAttrOpaque2 {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedAttrOpaque2 {
        self.inner.as_ptr()
    }
}
impl RenamedAttrOpaque2SharedArg for RenamedAttrOpaque2 {}
impl RenamedAttrOpaque2MutArg for RenamedAttrOpaque2 {}
impl<'view> crate::private::RenamedAttrOpaque2SharedSealed for RenamedAttrOpaque2Ref<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedAttrOpaque2 {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedAttrOpaque2SharedArg for RenamedAttrOpaque2Ref<'view> {}
impl<'view> crate::private::RenamedAttrOpaque2SharedSealed for RenamedAttrOpaque2RefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedAttrOpaque2 {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::RenamedAttrOpaque2MutSealed for RenamedAttrOpaque2RefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedAttrOpaque2 {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedAttrOpaque2SharedArg for RenamedAttrOpaque2RefMut<'view> {}
impl<'view> RenamedAttrOpaque2MutArg for RenamedAttrOpaque2RefMut<'view> {}

impl Drop for RenamedAttrOpaque2 {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_AttrOpaque2_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedAttrOpaque2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedAttrOpaque2")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedAttrOpaque2Ref<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedAttrOpaque2Ref")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedAttrOpaque2RefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedAttrOpaque2RefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedAttrOpaque2 {}

impl<'view> RenamedAttrOpaque2Ref<'view> {}

impl<'view> RenamedAttrOpaque2RefMut<'view> {}
