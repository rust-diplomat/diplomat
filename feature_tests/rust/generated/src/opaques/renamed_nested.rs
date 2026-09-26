use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedNested {
    pub(crate) inner: NonNull<ffi::RenamedNested>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedNestedRef<'view> {
    pub(crate) inner: NonNull<ffi::RenamedNested>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedNestedRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedNested>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedNestedSharedArg: crate::private::RenamedNestedSharedSealed {}

#[doc(hidden)]
pub trait RenamedNestedMutArg: crate::private::RenamedNestedMutSealed {}

impl crate::private::RenamedNestedSharedSealed for RenamedNested {
    fn __as_const_ptr(&self) -> *const ffi::RenamedNested {
        self.inner.as_ptr()
    }
}

impl crate::private::RenamedNestedMutSealed for RenamedNested {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedNested {
        self.inner.as_ptr()
    }
}

impl RenamedNestedSharedArg for RenamedNested {}
impl RenamedNestedMutArg for RenamedNested {}

impl<'view> crate::private::RenamedNestedSharedSealed for RenamedNestedRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedNested {
        self.inner.as_ptr()
    }
}

impl<'view> RenamedNestedSharedArg for RenamedNestedRef<'view> {}

impl<'view> crate::private::RenamedNestedSharedSealed for RenamedNestedRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedNested {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::RenamedNestedMutSealed for RenamedNestedRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedNested {
        self.inner.as_ptr()
    }
}

impl<'view> RenamedNestedSharedArg for RenamedNestedRefMut<'view> {}
impl<'view> RenamedNestedMutArg for RenamedNestedRefMut<'view> {}

impl Drop for RenamedNested {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_Nested_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedNested {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedNested")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedNestedRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedNestedRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedNestedRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedNestedRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}
