use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
use crate::types::*;

/// Opaque with a primitive slice, exercising borrowed slice inputs/outputs.
pub struct Numbers {
    pub(crate) inner: NonNull<ffi::Numbers>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct NumbersRef<'view> {
    pub(crate) inner: NonNull<ffi::Numbers>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct NumbersRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Numbers>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait NumbersSharedArg: crate::private::NumbersSharedSealed {}

#[doc(hidden)]
pub trait NumbersMutArg: crate::private::NumbersMutSealed {}

impl crate::private::NumbersSharedSealed for Numbers {
    fn __as_const_ptr(&self) -> *const ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl crate::private::NumbersMutSealed for Numbers {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl NumbersSharedArg for Numbers {}
impl NumbersMutArg for Numbers {}
impl<'view> crate::private::NumbersSharedSealed for NumbersRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl<'view> NumbersSharedArg for NumbersRef<'view> {}
impl<'view> crate::private::NumbersSharedSealed for NumbersRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::NumbersMutSealed for NumbersRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl<'view> NumbersSharedArg for NumbersRefMut<'view> {}
impl<'view> NumbersMutArg for NumbersRefMut<'view> {}

impl Drop for Numbers {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Numbers_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Numbers")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for NumbersRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NumbersRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for NumbersRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NumbersRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Numbers {
    pub fn new(values: &[u32]) -> super::Numbers {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_new(ffi::DiplomatSlice::from(values)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Numbers");
            super::Numbers {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn values<'a>(&'a self) -> &'a [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values(self.inner.as_ptr() as *const _) };
        result.into()
    }
    /// `&mut [T]` in the return position, gated on `mutable_slices`.
    pub fn values_mut<'a>(&'a mut self) -> &'a mut [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values_mut(self.inner.as_ptr()) };
        result.into()
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Numbers_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn total<'a>(fields: FieldView<'a>) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_total(ffi::FieldView {
                bytes: ffi::DiplomatSlice::from(fields.bytes),
                count: fields.count,
            })
        }
    }
    /// `&mut [T]` in the parameter position, gated on the same flag.
    pub fn fill(&self, out: &mut [u32]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_fill(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::from(out),
            )
        };
    }
    /// Borrowed slice pointing at caller-owned memory, gated on `memory_sharing`.
    pub fn from_slice(values: &[u32]) -> super::Numbers {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_from_slice(ffi::DiplomatSlice::from(values)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Numbers");
            super::Numbers {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    /// A `'static` slice parameter, gated on `static_slices`: the borrow carries no
    /// caller lifetime, so the generated signature must not introduce one.
    pub fn from_static(values: &'static [u32]) -> super::Numbers {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_from_static(ffi::DiplomatSlice::from(values)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Numbers");
            super::Numbers {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
}

impl<'view> NumbersRef<'view> {
    pub fn values<'a>(&'a self) -> &'a [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Numbers_sum(self.inner.as_ptr() as *const _) }
    }
    /// `&mut [T]` in the parameter position, gated on the same flag.
    pub fn fill(&self, out: &mut [u32]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_fill(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::from(out),
            )
        };
    }
}

impl<'view> NumbersRefMut<'view> {
    pub fn values<'a>(&'a self) -> &'a [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values(self.inner.as_ptr() as *const _) };
        result.into()
    }
    /// `&mut [T]` in the return position, gated on `mutable_slices`.
    pub fn values_mut<'a>(&'a mut self) -> &'a mut [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values_mut(self.inner.as_ptr()) };
        result.into()
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Numbers_sum(self.inner.as_ptr() as *const _) }
    }
    /// `&mut [T]` in the parameter position, gated on the same flag.
    pub fn fill(&self, out: &mut [u32]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_fill(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::from(out),
            )
        };
    }
}
