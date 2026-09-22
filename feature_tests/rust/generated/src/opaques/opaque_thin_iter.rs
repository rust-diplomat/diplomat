use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OpaqueThinIter<'a> {
    pub(crate) inner: NonNull<ffi::OpaqueThinIter>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueThinIterRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::OpaqueThinIter>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueThinIterRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::OpaqueThinIter>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OpaqueThinIterSharedArg: crate::private::OpaqueThinIterSharedSealed {}

#[doc(hidden)]
pub trait OpaqueThinIterMutArg: crate::private::OpaqueThinIterMutSealed {}

impl<'a> crate::private::OpaqueThinIterSharedSealed for OpaqueThinIter<'a> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
}

impl<'a> crate::private::OpaqueThinIterMutSealed for OpaqueThinIter<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
}

impl<'a> OpaqueThinIterSharedArg for OpaqueThinIter<'a> {}
impl<'a> OpaqueThinIterMutArg for OpaqueThinIter<'a> {}

impl<'view, 'a> crate::private::OpaqueThinIterSharedSealed for OpaqueThinIterRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
}

impl<'view, 'a> OpaqueThinIterSharedArg for OpaqueThinIterRef<'view, 'a> {}

impl<'view, 'a> crate::private::OpaqueThinIterSharedSealed for OpaqueThinIterRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
}

impl<'view, 'a> crate::private::OpaqueThinIterMutSealed for OpaqueThinIterRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
}

impl<'view, 'a> OpaqueThinIterSharedArg for OpaqueThinIterRefMut<'view, 'a> {}
impl<'view, 'a> OpaqueThinIterMutArg for OpaqueThinIterRefMut<'view, 'a> {}

impl<'a> Drop for OpaqueThinIter<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OpaqueThinIter_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> fmt::Debug for OpaqueThinIter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinIter")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for OpaqueThinIterRef<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinIterRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for OpaqueThinIterRefMut<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinIterRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'a> OpaqueThinIter<'a> {
    pub fn next(&'a mut self) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinIter_next(self.inner.as_ptr()) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}

impl<'view, 'a> OpaqueThinIterRefMut<'view, 'a> {
    pub fn next(&'a mut self) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinIter_next(self.inner.as_ptr()) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}
