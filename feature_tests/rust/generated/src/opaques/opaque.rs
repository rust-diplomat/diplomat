use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
use crate::types::*;

pub struct Opaque {
    pub(crate) inner: NonNull<ffi::Opaque>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueRef<'view> {
    pub(crate) inner: NonNull<ffi::Opaque>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Opaque>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OpaqueSharedArg: crate::private::OpaqueSharedSealed {}

#[doc(hidden)]
pub trait OpaqueMutArg: crate::private::OpaqueMutSealed {}

impl crate::private::OpaqueSharedSealed for Opaque {
    fn __as_const_ptr(&self) -> *const ffi::Opaque {
        self.inner.as_ptr()
    }
}
impl crate::private::OpaqueMutSealed for Opaque {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Opaque {
        self.inner.as_ptr()
    }
}
impl OpaqueSharedArg for Opaque {}
impl OpaqueMutArg for Opaque {}
impl<'view> crate::private::OpaqueSharedSealed for OpaqueRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Opaque {
        self.inner.as_ptr()
    }
}
impl<'view> OpaqueSharedArg for OpaqueRef<'view> {}
impl<'view> crate::private::OpaqueSharedSealed for OpaqueRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Opaque {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::OpaqueMutSealed for OpaqueRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Opaque {
        self.inner.as_ptr()
    }
}
impl<'view> OpaqueSharedArg for OpaqueRefMut<'view> {}
impl<'view> OpaqueMutArg for OpaqueRefMut<'view> {}

impl Drop for Opaque {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Opaque_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Opaque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Opaque").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view> fmt::Debug for OpaqueRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Opaque {
    pub fn new() -> super::Opaque {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Opaque_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Opaque");
            super::Opaque {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn try_from_utf8(input: &[u8]) -> Option<super::Opaque> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Opaque_try_from_utf8(ffi::DiplomatSlice::from(input)) };
        NonNull::new(result as *mut _).map(|inner| super::Opaque {
            inner,
            _not_send_sync: PhantomData,
        })
    }
    pub fn from_str(input: &str) -> super::Opaque {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Opaque_from_str(ffi::DiplomatSlice::from(input.as_bytes())) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Opaque");
            super::Opaque {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn returns_usize() -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Opaque_returns_usize() }
    }
    pub fn returns_imported() -> ImportedStruct {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Opaque_returns_imported() }
    }
}

impl<'view> OpaqueRef<'view> {}

impl<'view> OpaqueRefMut<'view> {}
