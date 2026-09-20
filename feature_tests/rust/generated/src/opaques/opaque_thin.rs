use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OpaqueThin {
    pub(crate) inner: NonNull<ffi::OpaqueThin>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueThinRef<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueThin>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueThinRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueThin>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OpaqueThinSharedArg: crate::private::OpaqueThinSharedSealed {}

#[doc(hidden)]
pub trait OpaqueThinMutArg: crate::private::OpaqueThinMutSealed {}

impl crate::private::OpaqueThinSharedSealed for OpaqueThin {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThin {
        self.inner.as_ptr()
    }
}
impl crate::private::OpaqueThinMutSealed for OpaqueThin {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThin {
        self.inner.as_ptr()
    }
}
impl OpaqueThinSharedArg for OpaqueThin {}
impl OpaqueThinMutArg for OpaqueThin {}
impl<'view> crate::private::OpaqueThinSharedSealed for OpaqueThinRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThin {
        self.inner.as_ptr()
    }
}
impl<'view> OpaqueThinSharedArg for OpaqueThinRef<'view> {}
impl<'view> crate::private::OpaqueThinSharedSealed for OpaqueThinRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThin {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::OpaqueThinMutSealed for OpaqueThinRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThin {
        self.inner.as_ptr()
    }
}
impl<'view> OpaqueThinSharedArg for OpaqueThinRefMut<'view> {}
impl<'view> OpaqueThinMutArg for OpaqueThinRefMut<'view> {}

impl Drop for OpaqueThin {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OpaqueThin_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OpaqueThin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThin")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueThinRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueThinRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OpaqueThin {
    pub fn a(&self) -> i32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThin_a(self.inner.as_ptr() as *const _) }
    }
    pub fn b(&self) -> f32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThin_b(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> OpaqueThinRef<'view> {
    pub fn a(&self) -> i32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThin_a(self.inner.as_ptr() as *const _) }
    }
    pub fn b(&self) -> f32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThin_b(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> OpaqueThinRefMut<'view> {
    pub fn a(&self) -> i32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThin_a(self.inner.as_ptr() as *const _) }
    }
    pub fn b(&self) -> f32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThin_b(self.inner.as_ptr() as *const _) }
    }
}
