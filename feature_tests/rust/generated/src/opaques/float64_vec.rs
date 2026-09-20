use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// Float scalars and a float slice. The constructor is the exact shape the shared
/// corpus gates on `memory_sharing` (`feature_tests/src/slices.rs`:
/// `pub fn new(v: &[f64])`), so it holds that shape against this backend's ABI.
pub struct Float64Vec {
    pub(crate) inner: NonNull<ffi::Float64Vec>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct Float64VecRef<'view> {
    pub(crate) inner: NonNull<ffi::Float64Vec>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct Float64VecRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Float64Vec>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait Float64VecSharedArg: crate::private::Float64VecSharedSealed {}

#[doc(hidden)]
pub trait Float64VecMutArg: crate::private::Float64VecMutSealed {}

impl crate::private::Float64VecSharedSealed for Float64Vec {
    fn __as_const_ptr(&self) -> *const ffi::Float64Vec {
        self.inner.as_ptr()
    }
}
impl crate::private::Float64VecMutSealed for Float64Vec {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Float64Vec {
        self.inner.as_ptr()
    }
}
impl Float64VecSharedArg for Float64Vec {}
impl Float64VecMutArg for Float64Vec {}
impl<'view> crate::private::Float64VecSharedSealed for Float64VecRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Float64Vec {
        self.inner.as_ptr()
    }
}
impl<'view> Float64VecSharedArg for Float64VecRef<'view> {}
impl<'view> crate::private::Float64VecSharedSealed for Float64VecRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Float64Vec {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::Float64VecMutSealed for Float64VecRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Float64Vec {
        self.inner.as_ptr()
    }
}
impl<'view> Float64VecSharedArg for Float64VecRefMut<'view> {}
impl<'view> Float64VecMutArg for Float64VecRefMut<'view> {}

impl Drop for Float64Vec {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Float64Vec_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Float64Vec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Float64Vec")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for Float64VecRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Float64VecRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for Float64VecRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Float64VecRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Float64Vec {
    pub fn new(values: &[f64]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new(ffi::DiplomatSlice::from(values)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn sum(&self) -> f64 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> f64 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn scale_in_place(&mut self, factor: f64) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_scale_in_place(self.inner.as_ptr(), factor) };
    }
}

impl<'view> Float64VecRef<'view> {
    pub fn sum(&self) -> f64 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> f64 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_get(self.inner.as_ptr() as *const _, index) }
    }
}

impl<'view> Float64VecRefMut<'view> {
    pub fn sum(&self) -> f64 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> f64 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn scale_in_place(&mut self, factor: f64) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Float64Vec_scale_in_place(self.inner.as_ptr(), factor) };
    }
}
