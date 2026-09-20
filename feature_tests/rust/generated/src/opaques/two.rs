use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct Two<'a, 'b> {
    pub(crate) inner: NonNull<ffi::Two>,
    pub(crate) _lifetimes: PhantomData<(*mut &'a (), *mut &'b ())>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct TwoRef<'view, 'a, 'b> {
    pub(crate) inner: NonNull<ffi::Two>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<(*mut &'a (), *mut &'b ())>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct TwoRefMut<'view, 'a, 'b> {
    pub(crate) inner: NonNull<ffi::Two>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<(*mut &'a (), *mut &'b ())>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait TwoSharedArg: crate::private::TwoSharedSealed {}

#[doc(hidden)]
pub trait TwoMutArg: crate::private::TwoMutSealed {}

impl<'a, 'b> crate::private::TwoSharedSealed for Two<'a, 'b> {
    fn __as_const_ptr(&self) -> *const ffi::Two {
        self.inner.as_ptr()
    }
}
impl<'a, 'b> crate::private::TwoMutSealed for Two<'a, 'b> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Two {
        self.inner.as_ptr()
    }
}
impl<'a, 'b> TwoSharedArg for Two<'a, 'b> {}
impl<'a, 'b> TwoMutArg for Two<'a, 'b> {}
impl<'view, 'a, 'b> crate::private::TwoSharedSealed for TwoRef<'view, 'a, 'b> {
    fn __as_const_ptr(&self) -> *const ffi::Two {
        self.inner.as_ptr()
    }
}
impl<'view, 'a, 'b> TwoSharedArg for TwoRef<'view, 'a, 'b> {}
impl<'view, 'a, 'b> crate::private::TwoSharedSealed for TwoRefMut<'view, 'a, 'b> {
    fn __as_const_ptr(&self) -> *const ffi::Two {
        self.inner.as_ptr()
    }
}
impl<'view, 'a, 'b> crate::private::TwoMutSealed for TwoRefMut<'view, 'a, 'b> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Two {
        self.inner.as_ptr()
    }
}
impl<'view, 'a, 'b> TwoSharedArg for TwoRefMut<'view, 'a, 'b> {}
impl<'view, 'a, 'b> TwoMutArg for TwoRefMut<'view, 'a, 'b> {}

impl<'a, 'b> Drop for Two<'a, 'b> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Two_destroy(self.inner.as_ptr()) };
    }
}

impl<'a, 'b> fmt::Debug for Two<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Two").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a, 'b> fmt::Debug for TwoRef<'view, 'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TwoRef").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a, 'b> fmt::Debug for TwoRefMut<'view, 'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TwoRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'a, 'b> Two<'a, 'b> {}

impl<'view, 'a, 'b> TwoRef<'view, 'a, 'b> {}

impl<'view, 'a, 'b> TwoRefMut<'view, 'a, 'b> {}
