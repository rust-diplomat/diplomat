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
pub trait OpaqueThinIterSharedArg<'a>: crate::private::OpaqueThinIterSharedSealed<'a> {}

#[doc(hidden)]
pub trait OpaqueThinIterMutArg<'a>: crate::private::OpaqueThinIterMutSealed<'a> {}

impl<'a> crate::private::OpaqueThinIterSharedSealed<'a> for OpaqueThinIter<'a> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> crate::private::OpaqueThinIterMutSealed<'a> for OpaqueThinIter<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> OpaqueThinIterSharedArg<'a> for OpaqueThinIter<'a> {}
impl<'a> OpaqueThinIterMutArg<'a> for OpaqueThinIter<'a> {}

impl<'view, 'a> crate::private::OpaqueThinIterSharedSealed<'a> for OpaqueThinIterRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> OpaqueThinIterSharedArg<'a> for OpaqueThinIterRef<'view, 'a> {}

impl<'view, 'a> crate::private::OpaqueThinIterSharedSealed<'a> for OpaqueThinIterRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> crate::private::OpaqueThinIterMutSealed<'a> for OpaqueThinIterRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThinIter {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> OpaqueThinIterSharedArg<'a> for OpaqueThinIterRefMut<'view, 'a> {}
impl<'view, 'a> OpaqueThinIterMutArg<'a> for OpaqueThinIterRefMut<'view, 'a> {}

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
