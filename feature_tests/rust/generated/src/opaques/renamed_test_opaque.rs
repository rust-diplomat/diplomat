use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedTestOpaque {
    pub(crate) inner: NonNull<ffi::RenamedTestOpaque>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedTestOpaqueRef<'view> {
    pub(crate) inner: NonNull<ffi::RenamedTestOpaque>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedTestOpaqueRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedTestOpaque>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedTestOpaqueSharedArg: crate::private::RenamedTestOpaqueSharedSealed {}

#[doc(hidden)]
pub trait RenamedTestOpaqueMutArg: crate::private::RenamedTestOpaqueMutSealed {}

impl crate::private::RenamedTestOpaqueSharedSealed for RenamedTestOpaque {
    fn __as_const_ptr(&self) -> *const ffi::RenamedTestOpaque {
        self.inner.as_ptr()
    }
}
impl crate::private::RenamedTestOpaqueMutSealed for RenamedTestOpaque {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedTestOpaque {
        self.inner.as_ptr()
    }
}
impl RenamedTestOpaqueSharedArg for RenamedTestOpaque {}
impl RenamedTestOpaqueMutArg for RenamedTestOpaque {}
impl<'view> crate::private::RenamedTestOpaqueSharedSealed for RenamedTestOpaqueRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedTestOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedTestOpaqueSharedArg for RenamedTestOpaqueRef<'view> {}
impl<'view> crate::private::RenamedTestOpaqueSharedSealed for RenamedTestOpaqueRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedTestOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::RenamedTestOpaqueMutSealed for RenamedTestOpaqueRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedTestOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedTestOpaqueSharedArg for RenamedTestOpaqueRefMut<'view> {}
impl<'view> RenamedTestOpaqueMutArg for RenamedTestOpaqueRefMut<'view> {}

impl Drop for RenamedTestOpaque {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_TestOpaque_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedTestOpaque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedTestOpaque")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedTestOpaqueRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedTestOpaqueRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedTestOpaqueRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedTestOpaqueRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedTestOpaque {}

impl<'view> RenamedTestOpaqueRef<'view> {}

impl<'view> RenamedTestOpaqueRefMut<'view> {}
