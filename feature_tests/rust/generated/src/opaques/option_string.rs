use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OptionString {
    pub(crate) inner: NonNull<ffi::OptionString>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OptionStringRef<'view> {
    pub(crate) inner: NonNull<ffi::OptionString>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OptionStringRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OptionString>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OptionStringSharedArg: crate::private::OptionStringSharedSealed {}

#[doc(hidden)]
pub trait OptionStringMutArg: crate::private::OptionStringMutSealed {}

impl crate::private::OptionStringSharedSealed for OptionString {
    fn __as_const_ptr(&self) -> *const ffi::OptionString {
        self.inner.as_ptr()
    }
}

impl crate::private::OptionStringMutSealed for OptionString {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OptionString {
        self.inner.as_ptr()
    }
}

impl OptionStringSharedArg for OptionString {}
impl OptionStringMutArg for OptionString {}

impl<'view> crate::private::OptionStringSharedSealed for OptionStringRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OptionString {
        self.inner.as_ptr()
    }
}

impl<'view> OptionStringSharedArg for OptionStringRef<'view> {}

impl<'view> crate::private::OptionStringSharedSealed for OptionStringRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OptionString {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::OptionStringMutSealed for OptionStringRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OptionString {
        self.inner.as_ptr()
    }
}

impl<'view> OptionStringSharedArg for OptionStringRefMut<'view> {}
impl<'view> OptionStringMutArg for OptionStringRefMut<'view> {}

impl Drop for OptionString {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OptionString_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OptionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionString")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OptionStringRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionStringRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OptionStringRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionStringRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OptionString {
    pub fn new(diplomat_str: &[u8]) -> Option<crate::OptionString> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionString_new(ffi::DiplomatSlice::from(diplomat_str)) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionString {
            inner,
            _not_send_sync: PhantomData,
        })
    }
    pub fn write(&self) -> Result<String, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let (result, text) = crate::private::with_write(|write| unsafe {
            ffi::OptionString_write(self.inner.as_ptr() as *const _, write)
        });
        Result::from(result).map(|()| text)
    }
}

impl<'view> OptionStringRef<'view> {
    pub fn write(&self) -> Result<String, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let (result, text) = crate::private::with_write(|write| unsafe {
            ffi::OptionString_write(self.inner.as_ptr() as *const _, write)
        });
        Result::from(result).map(|()| text)
    }
}

impl<'view> OptionStringRefMut<'view> {
    pub fn write(&self) -> Result<String, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let (result, text) = crate::private::with_write(|write| unsafe {
            ffi::OptionString_write(self.inner.as_ptr() as *const _, write)
        });
        Result::from(result).map(|()| text)
    }
}
