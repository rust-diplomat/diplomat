use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct PrimitiveStructVec {
    pub(crate) inner: NonNull<ffi::PrimitiveStructVec>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct PrimitiveStructVecRef<'view> {
    pub(crate) inner: NonNull<ffi::PrimitiveStructVec>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct PrimitiveStructVecRefMut<'view> {
    pub(crate) inner: NonNull<ffi::PrimitiveStructVec>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait PrimitiveStructVecSharedArg: crate::private::PrimitiveStructVecSharedSealed {}

#[doc(hidden)]
pub trait PrimitiveStructVecMutArg: crate::private::PrimitiveStructVecMutSealed {}

impl crate::private::PrimitiveStructVecSharedSealed for PrimitiveStructVec {
    fn __as_const_ptr(&self) -> *const ffi::PrimitiveStructVec {
        self.inner.as_ptr()
    }
}
impl crate::private::PrimitiveStructVecMutSealed for PrimitiveStructVec {
    fn __as_mut_ptr(&mut self) -> *mut ffi::PrimitiveStructVec {
        self.inner.as_ptr()
    }
}
impl PrimitiveStructVecSharedArg for PrimitiveStructVec {}
impl PrimitiveStructVecMutArg for PrimitiveStructVec {}
impl<'view> crate::private::PrimitiveStructVecSharedSealed for PrimitiveStructVecRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::PrimitiveStructVec {
        self.inner.as_ptr()
    }
}
impl<'view> PrimitiveStructVecSharedArg for PrimitiveStructVecRef<'view> {}
impl<'view> crate::private::PrimitiveStructVecSharedSealed for PrimitiveStructVecRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::PrimitiveStructVec {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::PrimitiveStructVecMutSealed for PrimitiveStructVecRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::PrimitiveStructVec {
        self.inner.as_ptr()
    }
}
impl<'view> PrimitiveStructVecSharedArg for PrimitiveStructVecRefMut<'view> {}
impl<'view> PrimitiveStructVecMutArg for PrimitiveStructVecRefMut<'view> {}

impl Drop for PrimitiveStructVec {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::PrimitiveStructVec_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for PrimitiveStructVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PrimitiveStructVec")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for PrimitiveStructVecRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PrimitiveStructVecRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for PrimitiveStructVecRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PrimitiveStructVecRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl PrimitiveStructVec {
    pub fn new() -> super::PrimitiveStructVec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::PrimitiveStructVec_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null PrimitiveStructVec");
            super::PrimitiveStructVec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::PrimitiveStructVec_len(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> PrimitiveStructVecRef<'view> {
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::PrimitiveStructVec_len(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> PrimitiveStructVecRefMut<'view> {
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::PrimitiveStructVec_len(self.inner.as_ptr() as *const _) }
    }
}
