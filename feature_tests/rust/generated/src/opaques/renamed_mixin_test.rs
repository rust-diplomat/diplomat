use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedMixinTest {
    pub(crate) inner: NonNull<ffi::RenamedMixinTest>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedMixinTestRef<'view> {
    pub(crate) inner: NonNull<ffi::RenamedMixinTest>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedMixinTestRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedMixinTest>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedMixinTestSharedArg: crate::private::RenamedMixinTestSharedSealed {}

#[doc(hidden)]
pub trait RenamedMixinTestMutArg: crate::private::RenamedMixinTestMutSealed {}

impl crate::private::RenamedMixinTestSharedSealed for RenamedMixinTest {
    fn __as_const_ptr(&self) -> *const ffi::RenamedMixinTest {
        self.inner.as_ptr()
    }
}

impl crate::private::RenamedMixinTestMutSealed for RenamedMixinTest {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedMixinTest {
        self.inner.as_ptr()
    }
}

impl RenamedMixinTestSharedArg for RenamedMixinTest {}
impl RenamedMixinTestMutArg for RenamedMixinTest {}

impl<'view> crate::private::RenamedMixinTestSharedSealed for RenamedMixinTestRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedMixinTest {
        self.inner.as_ptr()
    }
}

impl<'view> RenamedMixinTestSharedArg for RenamedMixinTestRef<'view> {}

impl<'view> crate::private::RenamedMixinTestSharedSealed for RenamedMixinTestRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedMixinTest {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::RenamedMixinTestMutSealed for RenamedMixinTestRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedMixinTest {
        self.inner.as_ptr()
    }
}

impl<'view> RenamedMixinTestSharedArg for RenamedMixinTestRefMut<'view> {}
impl<'view> RenamedMixinTestMutArg for RenamedMixinTestRefMut<'view> {}

impl Drop for RenamedMixinTest {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_MixinTest_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedMixinTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedMixinTest")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedMixinTestRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedMixinTestRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedMixinTestRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedMixinTestRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedMixinTest {
    pub fn hello() -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::namespace_MixinTest_hello(write) };
        })
        .1
    }
}
