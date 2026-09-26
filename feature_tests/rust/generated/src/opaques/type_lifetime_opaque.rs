use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct TypeLifetimeOpaque<'a> {
    pub(crate) inner: NonNull<ffi::TypeLifetimeOpaque>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct TypeLifetimeOpaqueRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::TypeLifetimeOpaque>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct TypeLifetimeOpaqueRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::TypeLifetimeOpaque>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait TypeLifetimeOpaqueSharedArg<'a>:
    crate::private::TypeLifetimeOpaqueSharedSealed<'a>
{
}

#[doc(hidden)]
pub trait TypeLifetimeOpaqueMutArg<'a>: crate::private::TypeLifetimeOpaqueMutSealed<'a> {}

impl<'a> crate::private::TypeLifetimeOpaqueSharedSealed<'a> for TypeLifetimeOpaque<'a> {
    fn __as_const_ptr(&self) -> *const ffi::TypeLifetimeOpaque {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> crate::private::TypeLifetimeOpaqueMutSealed<'a> for TypeLifetimeOpaque<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::TypeLifetimeOpaque {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> TypeLifetimeOpaqueSharedArg<'a> for TypeLifetimeOpaque<'a> {}
impl<'a> TypeLifetimeOpaqueMutArg<'a> for TypeLifetimeOpaque<'a> {}

impl<'view, 'a> crate::private::TypeLifetimeOpaqueSharedSealed<'a>
    for TypeLifetimeOpaqueRef<'view, 'a>
{
    fn __as_const_ptr(&self) -> *const ffi::TypeLifetimeOpaque {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> TypeLifetimeOpaqueSharedArg<'a> for TypeLifetimeOpaqueRef<'view, 'a> {}

impl<'view, 'a> crate::private::TypeLifetimeOpaqueSharedSealed<'a>
    for TypeLifetimeOpaqueRefMut<'view, 'a>
{
    fn __as_const_ptr(&self) -> *const ffi::TypeLifetimeOpaque {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> crate::private::TypeLifetimeOpaqueMutSealed<'a>
    for TypeLifetimeOpaqueRefMut<'view, 'a>
{
    fn __as_mut_ptr(&mut self) -> *mut ffi::TypeLifetimeOpaque {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> TypeLifetimeOpaqueSharedArg<'a> for TypeLifetimeOpaqueRefMut<'view, 'a> {}
impl<'view, 'a> TypeLifetimeOpaqueMutArg<'a> for TypeLifetimeOpaqueRefMut<'view, 'a> {}

impl<'a> Drop for TypeLifetimeOpaque<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::TypeLifetimeOpaque_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> fmt::Debug for TypeLifetimeOpaque<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeLifetimeOpaque")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for TypeLifetimeOpaqueRef<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeLifetimeOpaqueRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for TypeLifetimeOpaqueRefMut<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeLifetimeOpaqueRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'a> TypeLifetimeOpaque<'a> {
    pub fn new(value: &'a [u8]) -> crate::TypeLifetimeOpaque<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::TypeLifetimeOpaque_new(ffi::DiplomatSlice::from(value)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null TypeLifetimeOpaque");
            crate::TypeLifetimeOpaque {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn accept_same_lifetime(&mut self, other: &impl crate::TypeLifetimeOpaqueSharedArg<'a>) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::TypeLifetimeOpaque_accept_same_lifetime(
                self.inner.as_ptr(),
                crate::private::TypeLifetimeOpaqueSharedSealed::__as_const_ptr(other),
            )
        };
    }
    pub fn get<'anon_0>(&'anon_0 self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::TypeLifetimeOpaque_get(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view, 'a> TypeLifetimeOpaqueRef<'view, 'a> {
    pub fn get<'anon_0>(&'anon_0 self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::TypeLifetimeOpaque_get(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view, 'a> TypeLifetimeOpaqueRefMut<'view, 'a> {
    pub fn accept_same_lifetime(&mut self, other: &impl crate::TypeLifetimeOpaqueSharedArg<'a>) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::TypeLifetimeOpaque_accept_same_lifetime(
                self.inner.as_ptr(),
                crate::private::TypeLifetimeOpaqueSharedSealed::__as_const_ptr(other),
            )
        };
    }
    pub fn get<'anon_0>(&'anon_0 self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::TypeLifetimeOpaque_get(self.inner.as_ptr() as *const _) };
        result.into()
    }
}
