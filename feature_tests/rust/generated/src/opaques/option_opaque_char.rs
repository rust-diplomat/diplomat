use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OptionOpaqueChar {
    pub(crate) inner: NonNull<ffi::OptionOpaqueChar>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OptionOpaqueCharRef<'view> {
    pub(crate) inner: NonNull<ffi::OptionOpaqueChar>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OptionOpaqueCharRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OptionOpaqueChar>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OptionOpaqueCharSharedArg: crate::private::OptionOpaqueCharSharedSealed {}

#[doc(hidden)]
pub trait OptionOpaqueCharMutArg: crate::private::OptionOpaqueCharMutSealed {}

impl crate::private::OptionOpaqueCharSharedSealed for OptionOpaqueChar {
    fn __as_const_ptr(&self) -> *const ffi::OptionOpaqueChar {
        self.inner.as_ptr()
    }
}
impl crate::private::OptionOpaqueCharMutSealed for OptionOpaqueChar {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OptionOpaqueChar {
        self.inner.as_ptr()
    }
}
impl OptionOpaqueCharSharedArg for OptionOpaqueChar {}
impl OptionOpaqueCharMutArg for OptionOpaqueChar {}
impl<'view> crate::private::OptionOpaqueCharSharedSealed for OptionOpaqueCharRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OptionOpaqueChar {
        self.inner.as_ptr()
    }
}
impl<'view> OptionOpaqueCharSharedArg for OptionOpaqueCharRef<'view> {}
impl<'view> crate::private::OptionOpaqueCharSharedSealed for OptionOpaqueCharRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OptionOpaqueChar {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::OptionOpaqueCharMutSealed for OptionOpaqueCharRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OptionOpaqueChar {
        self.inner.as_ptr()
    }
}
impl<'view> OptionOpaqueCharSharedArg for OptionOpaqueCharRefMut<'view> {}
impl<'view> OptionOpaqueCharMutArg for OptionOpaqueCharRefMut<'view> {}

impl Drop for OptionOpaqueChar {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OptionOpaqueChar_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OptionOpaqueChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionOpaqueChar")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OptionOpaqueCharRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionOpaqueCharRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OptionOpaqueCharRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionOpaqueCharRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OptionOpaqueChar {}

impl<'view> OptionOpaqueCharRef<'view> {}

impl<'view> OptionOpaqueCharRefMut<'view> {}
