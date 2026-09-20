use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

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
    pub fn new(v: &[f64]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn bool(v: &[bool]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new_bool(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn i16(v: &[i16]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new_i16(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn u16(v: &[u16]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new_u16(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn isize(v: &[isize]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new_isize(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn usize(v: &[usize]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new_usize(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn f64_be_bytes(v: &[u8]) -> super::Float64Vec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_new_f64_be_bytes(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Float64Vec");
            super::Float64Vec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn as_slice<'a>(&'a self) -> &'a [f64] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_as_slice(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn fill_slice(&self, v: &mut [f64]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Float64Vec_fill_slice(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::from(v),
            )
        };
    }
    pub fn set_value(&mut self, new_slice: &[f64]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Float64Vec_set_value(self.inner.as_ptr(), ffi::DiplomatSlice::from(new_slice))
        };
    }
    pub fn borrow<'a>(&'a self) -> &'a [f64] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_borrow(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn get(&self, i: usize) -> Option<f64> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_get(self.inner.as_ptr() as *const _, i) };
        result.into()
    }
}

impl<'view> Float64VecRef<'view> {
    pub fn as_slice<'a>(&'a self) -> &'a [f64] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_as_slice(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn fill_slice(&self, v: &mut [f64]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Float64Vec_fill_slice(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::from(v),
            )
        };
    }
    pub fn borrow<'a>(&'a self) -> &'a [f64] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_borrow(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn get(&self, i: usize) -> Option<f64> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_get(self.inner.as_ptr() as *const _, i) };
        result.into()
    }
}

impl<'view> Float64VecRefMut<'view> {
    pub fn as_slice<'a>(&'a self) -> &'a [f64] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_as_slice(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn fill_slice(&self, v: &mut [f64]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Float64Vec_fill_slice(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::from(v),
            )
        };
    }
    pub fn set_value(&mut self, new_slice: &[f64]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Float64Vec_set_value(self.inner.as_ptr(), ffi::DiplomatSlice::from(new_slice))
        };
    }
    pub fn borrow<'a>(&'a self) -> &'a [f64] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_borrow(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn get(&self, i: usize) -> Option<f64> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Float64Vec_get(self.inner.as_ptr() as *const _, i) };
        result.into()
    }
}
