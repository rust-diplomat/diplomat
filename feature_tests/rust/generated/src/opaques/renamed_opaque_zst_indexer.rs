use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedOpaqueZSTIndexer {
    pub(crate) inner: NonNull<ffi::RenamedOpaqueZSTIndexer>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedOpaqueZSTIndexerRef<'view> {
    pub(crate) inner: NonNull<ffi::RenamedOpaqueZSTIndexer>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedOpaqueZSTIndexerRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedOpaqueZSTIndexer>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedOpaqueZSTIndexerSharedArg:
    crate::private::RenamedOpaqueZSTIndexerSharedSealed
{
}

#[doc(hidden)]
pub trait RenamedOpaqueZSTIndexerMutArg: crate::private::RenamedOpaqueZSTIndexerMutSealed {}

impl crate::private::RenamedOpaqueZSTIndexerSharedSealed for RenamedOpaqueZSTIndexer {
    fn __as_const_ptr(&self) -> *const ffi::RenamedOpaqueZSTIndexer {
        self.inner.as_ptr()
    }
}
impl crate::private::RenamedOpaqueZSTIndexerMutSealed for RenamedOpaqueZSTIndexer {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedOpaqueZSTIndexer {
        self.inner.as_ptr()
    }
}
impl RenamedOpaqueZSTIndexerSharedArg for RenamedOpaqueZSTIndexer {}
impl RenamedOpaqueZSTIndexerMutArg for RenamedOpaqueZSTIndexer {}
impl<'view> crate::private::RenamedOpaqueZSTIndexerSharedSealed
    for RenamedOpaqueZSTIndexerRef<'view>
{
    fn __as_const_ptr(&self) -> *const ffi::RenamedOpaqueZSTIndexer {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedOpaqueZSTIndexerSharedArg for RenamedOpaqueZSTIndexerRef<'view> {}
impl<'view> crate::private::RenamedOpaqueZSTIndexerSharedSealed
    for RenamedOpaqueZSTIndexerRefMut<'view>
{
    fn __as_const_ptr(&self) -> *const ffi::RenamedOpaqueZSTIndexer {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::RenamedOpaqueZSTIndexerMutSealed
    for RenamedOpaqueZSTIndexerRefMut<'view>
{
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedOpaqueZSTIndexer {
        self.inner.as_ptr()
    }
}
impl<'view> RenamedOpaqueZSTIndexerSharedArg for RenamedOpaqueZSTIndexerRefMut<'view> {}
impl<'view> RenamedOpaqueZSTIndexerMutArg for RenamedOpaqueZSTIndexerRefMut<'view> {}

impl Drop for RenamedOpaqueZSTIndexer {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_OpaqueZSTIndexer_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedOpaqueZSTIndexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedOpaqueZSTIndexer")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedOpaqueZSTIndexerRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedOpaqueZSTIndexerRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedOpaqueZSTIndexerRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedOpaqueZSTIndexerRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedOpaqueZSTIndexer {
    pub fn new() -> super::RenamedOpaqueZSTIndexer {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_OpaqueZSTIndexer_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null RenamedOpaqueZSTIndexer");
            super::RenamedOpaqueZSTIndexer {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn index(&self, idx: usize) -> Option<super::RenamedOpaqueZSTIndexer> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::namespace_OpaqueZSTIndexer_index(self.inner.as_ptr() as *const _, idx) };
        NonNull::new(result as *mut _).map(|inner| super::RenamedOpaqueZSTIndexer {
            inner,
            _not_send_sync: PhantomData,
        })
    }
}

impl<'view> RenamedOpaqueZSTIndexerRef<'view> {
    pub fn index(&self, idx: usize) -> Option<super::RenamedOpaqueZSTIndexer> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::namespace_OpaqueZSTIndexer_index(self.inner.as_ptr() as *const _, idx) };
        NonNull::new(result as *mut _).map(|inner| super::RenamedOpaqueZSTIndexer {
            inner,
            _not_send_sync: PhantomData,
        })
    }
}

impl<'view> RenamedOpaqueZSTIndexerRefMut<'view> {
    pub fn index(&self, idx: usize) -> Option<super::RenamedOpaqueZSTIndexer> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::namespace_OpaqueZSTIndexer_index(self.inner.as_ptr() as *const _, idx) };
        NonNull::new(result as *mut _).map(|inner| super::RenamedOpaqueZSTIndexer {
            inner,
            _not_send_sync: PhantomData,
        })
    }
}
