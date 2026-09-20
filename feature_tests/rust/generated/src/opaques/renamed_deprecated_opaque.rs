use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedDeprecatedOpaque {
    pub(crate) inner: NonNull<ffi::RenamedDeprecatedOpaque>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedDeprecatedOpaqueRef<'view> {
    pub(crate) inner: NonNull<ffi::RenamedDeprecatedOpaque>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedDeprecatedOpaqueRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedDeprecatedOpaque>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedDeprecatedOpaqueSharedArg:
    crate::private::RenamedDeprecatedOpaqueSharedSealed
{
}

#[doc(hidden)]
pub trait RenamedDeprecatedOpaqueMutArg: crate::private::RenamedDeprecatedOpaqueMutSealed {}

impl crate::private::RenamedDeprecatedOpaqueSharedSealed for RenamedDeprecatedOpaque {
    fn __as_const_ptr(&self) -> *const ffi::RenamedDeprecatedOpaque {
        self.inner.as_ptr()
    }
}
impl crate::private::RenamedDeprecatedOpaqueMutSealed for RenamedDeprecatedOpaque {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedDeprecatedOpaque {
        self.inner.as_ptr()
    }
}
impl RenamedDeprecatedOpaqueSharedArg for RenamedDeprecatedOpaque {}
impl RenamedDeprecatedOpaqueMutArg for RenamedDeprecatedOpaque {}
impl<'view> crate::private::RenamedDeprecatedOpaqueSharedSealed
    for RenamedDeprecatedOpaqueRef<'view>
{
    fn __as_const_ptr(&self) -> *const ffi::RenamedDeprecatedOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedDeprecatedOpaqueSharedArg for RenamedDeprecatedOpaqueRef<'view> {}
impl<'view> crate::private::RenamedDeprecatedOpaqueSharedSealed
    for RenamedDeprecatedOpaqueRefMut<'view>
{
    fn __as_const_ptr(&self) -> *const ffi::RenamedDeprecatedOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::RenamedDeprecatedOpaqueMutSealed
    for RenamedDeprecatedOpaqueRefMut<'view>
{
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedDeprecatedOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedDeprecatedOpaqueSharedArg for RenamedDeprecatedOpaqueRefMut<'view> {}
impl<'view> RenamedDeprecatedOpaqueMutArg for RenamedDeprecatedOpaqueRefMut<'view> {}

impl Drop for RenamedDeprecatedOpaque {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_DeprecatedOpaque_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedDeprecatedOpaque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedDeprecatedOpaque")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedDeprecatedOpaqueRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedDeprecatedOpaqueRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedDeprecatedOpaqueRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedDeprecatedOpaqueRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedDeprecatedOpaque {}

impl<'view> RenamedDeprecatedOpaqueRef<'view> {}

impl<'view> RenamedDeprecatedOpaqueRefMut<'view> {}
