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
pub trait TwoSharedArg<'a, 'b>: crate::private::TwoSharedSealed<'a, 'b> {}

#[doc(hidden)]
pub trait TwoMutArg<'a, 'b>: crate::private::TwoMutSealed<'a, 'b> {}

impl<'a, 'b> crate::private::TwoSharedSealed<'a, 'b> for Two<'a, 'b> {
    fn __as_const_ptr(&self) -> *const ffi::Two {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<(*mut &'a (), *mut &'b ())> {
        core::marker::PhantomData
    }
}

impl<'a, 'b> crate::private::TwoMutSealed<'a, 'b> for Two<'a, 'b> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Two {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<(*mut &'a (), *mut &'b ())> {
        core::marker::PhantomData
    }
}

impl<'a, 'b> TwoSharedArg<'a, 'b> for Two<'a, 'b> {}
impl<'a, 'b> TwoMutArg<'a, 'b> for Two<'a, 'b> {}

impl<'view, 'a, 'b> crate::private::TwoSharedSealed<'a, 'b> for TwoRef<'view, 'a, 'b> {
    fn __as_const_ptr(&self) -> *const ffi::Two {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<(*mut &'a (), *mut &'b ())> {
        core::marker::PhantomData
    }
}

impl<'view, 'a, 'b> TwoSharedArg<'a, 'b> for TwoRef<'view, 'a, 'b> {}

impl<'view, 'a, 'b> crate::private::TwoSharedSealed<'a, 'b> for TwoRefMut<'view, 'a, 'b> {
    fn __as_const_ptr(&self) -> *const ffi::Two {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<(*mut &'a (), *mut &'b ())> {
        core::marker::PhantomData
    }
}

impl<'view, 'a, 'b> crate::private::TwoMutSealed<'a, 'b> for TwoRefMut<'view, 'a, 'b> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Two {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<(*mut &'a (), *mut &'b ())> {
        core::marker::PhantomData
    }
}

impl<'view, 'a, 'b> TwoSharedArg<'a, 'b> for TwoRefMut<'view, 'a, 'b> {}
impl<'view, 'a, 'b> TwoMutArg<'a, 'b> for TwoRefMut<'view, 'a, 'b> {}

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
