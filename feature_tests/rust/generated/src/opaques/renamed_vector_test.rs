use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RenamedVectorTest {
    pub(crate) inner: NonNull<ffi::RenamedVectorTest>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedVectorTestRef<'view> {
    pub(crate) inner: NonNull<ffi::RenamedVectorTest>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RenamedVectorTestRefMut<'view> {
    pub(crate) inner: NonNull<ffi::RenamedVectorTest>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RenamedVectorTestSharedArg: crate::private::RenamedVectorTestSharedSealed {}

#[doc(hidden)]
pub trait RenamedVectorTestMutArg: crate::private::RenamedVectorTestMutSealed {}

impl crate::private::RenamedVectorTestSharedSealed for RenamedVectorTest {
    fn __as_const_ptr(&self) -> *const ffi::RenamedVectorTest {
        self.inner.as_ptr()
    }
}

impl crate::private::RenamedVectorTestMutSealed for RenamedVectorTest {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedVectorTest {
        self.inner.as_ptr()
    }
}

impl RenamedVectorTestSharedArg for RenamedVectorTest {}
impl RenamedVectorTestMutArg for RenamedVectorTest {}

impl<'view> crate::private::RenamedVectorTestSharedSealed for RenamedVectorTestRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedVectorTest {
        self.inner.as_ptr()
    }
}

impl<'view> RenamedVectorTestSharedArg for RenamedVectorTestRef<'view> {}

impl<'view> crate::private::RenamedVectorTestSharedSealed for RenamedVectorTestRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::RenamedVectorTest {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::RenamedVectorTestMutSealed for RenamedVectorTestRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RenamedVectorTest {
        self.inner.as_ptr()
    }
}

impl<'view> RenamedVectorTestSharedArg for RenamedVectorTestRefMut<'view> {}
impl<'view> RenamedVectorTestMutArg for RenamedVectorTestRefMut<'view> {}

impl Drop for RenamedVectorTest {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_VectorTest_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for RenamedVectorTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedVectorTest")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedVectorTestRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedVectorTestRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for RenamedVectorTestRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenamedVectorTestRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl RenamedVectorTest {
    pub fn new() -> crate::RenamedVectorTest {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_VectorTest_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null RenamedVectorTest");
            crate::RenamedVectorTest {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_VectorTest_len(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, idx: usize) -> Option<f64> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_VectorTest_get(self.inner.as_ptr() as *const _, idx) };
        result.into()
    }
    pub fn push(&mut self, value: f64) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_VectorTest_push(self.inner.as_ptr(), value) };
    }
}

impl<'view> RenamedVectorTestRef<'view> {
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_VectorTest_len(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, idx: usize) -> Option<f64> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_VectorTest_get(self.inner.as_ptr() as *const _, idx) };
        result.into()
    }
}

impl<'view> RenamedVectorTestRefMut<'view> {
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_VectorTest_len(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, idx: usize) -> Option<f64> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_VectorTest_get(self.inner.as_ptr() as *const _, idx) };
        result.into()
    }
    pub fn push(&mut self, value: f64) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_VectorTest_push(self.inner.as_ptr(), value) };
    }
}
