use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
use crate::types::*;

/// Owner of a `Vec<Point>`. Slice APIs cannot live on the value struct itself
/// (methods on structs are still rejected).
pub struct Points {
    pub(crate) inner: NonNull<ffi::Points>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct PointsRef<'view> {
    pub(crate) inner: NonNull<ffi::Points>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct PointsRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Points>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait PointsSharedArg: crate::private::PointsSharedSealed {}

#[doc(hidden)]
pub trait PointsMutArg: crate::private::PointsMutSealed {}

impl crate::private::PointsSharedSealed for Points {
    fn __as_const_ptr(&self) -> *const ffi::Points {
        self.inner.as_ptr()
    }
}
impl crate::private::PointsMutSealed for Points {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Points {
        self.inner.as_ptr()
    }
}
impl PointsSharedArg for Points {}
impl PointsMutArg for Points {}
impl<'view> crate::private::PointsSharedSealed for PointsRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Points {
        self.inner.as_ptr()
    }
}
impl<'view> PointsSharedArg for PointsRef<'view> {}
impl<'view> crate::private::PointsSharedSealed for PointsRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Points {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::PointsMutSealed for PointsRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Points {
        self.inner.as_ptr()
    }
}
impl<'view> PointsSharedArg for PointsRefMut<'view> {}
impl<'view> PointsMutArg for PointsRefMut<'view> {}

impl Drop for Points {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Points_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Points").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view> fmt::Debug for PointsRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PointsRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for PointsRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PointsRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Points {
    /// Sum of `x + y` over the whole slice in one call. Gated on `abi_compatibles`:
    /// without the flag, `#[diplomat::attr(auto, abi_compatible)]` does not stick
    /// and HIR refuses the slice, so this symbol disappears from the generated API.
    pub fn total(points: &[Point]) -> i32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Points_total(ffi::DiplomatSlice::from(points)) }
    }
    pub fn new(points: &[Point]) -> super::Points {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Points_new(ffi::DiplomatSlice::from(points)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Points");
            super::Points {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    /// Borrow the stored points. The consumer loops over this in Rust.
    pub fn as_slice<'a>(&'a self) -> &'a [Point] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Points_as_slice(self.inner.as_ptr() as *const _) };
        result.into()
    }
    /// Scale each point in place. Gated on both `abi_compatibles` and
    /// `mutable_slices` so dropping either flag removes the symbol.
    pub fn scale(points: &mut [Point], factor: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Points_scale(ffi::DiplomatSliceMut::from(points), factor) };
    }
}

impl<'view> PointsRef<'view> {
    /// Borrow the stored points. The consumer loops over this in Rust.
    pub fn as_slice<'a>(&'a self) -> &'a [Point] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Points_as_slice(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view> PointsRefMut<'view> {
    /// Borrow the stored points. The consumer loops over this in Rust.
    pub fn as_slice<'a>(&'a self) -> &'a [Point] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Points_as_slice(self.inner.as_ptr() as *const _) };
        result.into()
    }
}
