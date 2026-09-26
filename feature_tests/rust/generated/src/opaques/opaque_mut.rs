use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OpaqueMut {
    pub(crate) inner: NonNull<ffi::OpaqueMut>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueMutRef<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueMut>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueMutRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueMut>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OpaqueMutSharedArg: crate::private::OpaqueMutSharedSealed {}

#[doc(hidden)]
pub trait OpaqueMutMutArg: crate::private::OpaqueMutMutSealed {}

impl crate::private::OpaqueMutSharedSealed for OpaqueMut {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueMut {
        self.inner.as_ptr()
    }
}

impl crate::private::OpaqueMutMutSealed for OpaqueMut {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueMut {
        self.inner.as_ptr()
    }
}

impl OpaqueMutSharedArg for OpaqueMut {}
impl OpaqueMutMutArg for OpaqueMut {}

impl<'view> crate::private::OpaqueMutSharedSealed for OpaqueMutRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueMut {
        self.inner.as_ptr()
    }
}

impl<'view> OpaqueMutSharedArg for OpaqueMutRef<'view> {}

impl<'view> crate::private::OpaqueMutSharedSealed for OpaqueMutRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueMut {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::OpaqueMutMutSealed for OpaqueMutRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueMut {
        self.inner.as_ptr()
    }
}

impl<'view> OpaqueMutSharedArg for OpaqueMutRefMut<'view> {}
impl<'view> OpaqueMutMutArg for OpaqueMutRefMut<'view> {}

impl Drop for OpaqueMut {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OpaqueMut_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OpaqueMut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueMutRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueMutRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueMutRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueMutRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OpaqueMut {
    pub fn new() -> crate::OpaqueMut {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMut_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMut");
            crate::OpaqueMut {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
}
