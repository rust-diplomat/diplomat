use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// A distinct owner-side opaque returned only as a borrow from Counter.
pub struct Child {
    pub(crate) inner: NonNull<ffi::Child>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct ChildRef<'view> {
    pub(crate) inner: NonNull<ffi::Child>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct ChildRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Child>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait ChildSharedArg: crate::private::ChildSharedSealed {}

#[doc(hidden)]
pub trait ChildMutArg: crate::private::ChildMutSealed {}

impl crate::private::ChildSharedSealed for Child {
    fn __as_const_ptr(&self) -> *const ffi::Child {
        self.inner.as_ptr()
    }
}
impl crate::private::ChildMutSealed for Child {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Child {
        self.inner.as_ptr()
    }
}
impl ChildSharedArg for Child {}
impl ChildMutArg for Child {}
impl<'view> crate::private::ChildSharedSealed for ChildRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Child {
        self.inner.as_ptr()
    }
}
impl<'view> ChildSharedArg for ChildRef<'view> {}
impl<'view> crate::private::ChildSharedSealed for ChildRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Child {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::ChildMutSealed for ChildRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Child {
        self.inner.as_ptr()
    }
}
impl<'view> ChildSharedArg for ChildRefMut<'view> {}
impl<'view> ChildMutArg for ChildRefMut<'view> {}

impl Drop for Child {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Child_destroy(self.inner.as_ptr()) };
    }
}

impl Child {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_get(self.inner.as_ptr() as *const _) }
    }
    pub fn set(&mut self, value: u32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_set(self.inner.as_ptr(), value) };
    }
    pub fn owner_identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_owner_identity(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> ChildRef<'view> {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_get(self.inner.as_ptr() as *const _) }
    }
    pub fn owner_identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_owner_identity(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> ChildRefMut<'view> {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_get(self.inner.as_ptr() as *const _) }
    }
    pub fn set(&mut self, value: u32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_set(self.inner.as_ptr(), value) };
    }
    pub fn owner_identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_owner_identity(self.inner.as_ptr() as *const _) }
    }
}
