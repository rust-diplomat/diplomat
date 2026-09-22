use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
use crate::types::*;

pub struct ResultOpaque {
    pub(crate) inner: NonNull<ffi::ResultOpaque>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct ResultOpaqueRef<'view> {
    pub(crate) inner: NonNull<ffi::ResultOpaque>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct ResultOpaqueRefMut<'view> {
    pub(crate) inner: NonNull<ffi::ResultOpaque>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait ResultOpaqueSharedArg: crate::private::ResultOpaqueSharedSealed {}

#[doc(hidden)]
pub trait ResultOpaqueMutArg: crate::private::ResultOpaqueMutSealed {}

impl crate::private::ResultOpaqueSharedSealed for ResultOpaque {
    fn __as_const_ptr(&self) -> *const ffi::ResultOpaque {
        self.inner.as_ptr()
    }
}
impl crate::private::ResultOpaqueMutSealed for ResultOpaque {
    fn __as_mut_ptr(&mut self) -> *mut ffi::ResultOpaque {
        self.inner.as_ptr()
    }
}
impl ResultOpaqueSharedArg for ResultOpaque {}
impl ResultOpaqueMutArg for ResultOpaque {}
impl<'view> crate::private::ResultOpaqueSharedSealed for ResultOpaqueRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::ResultOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> ResultOpaqueSharedArg for ResultOpaqueRef<'view> {}
impl<'view> crate::private::ResultOpaqueSharedSealed for ResultOpaqueRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::ResultOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::ResultOpaqueMutSealed for ResultOpaqueRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::ResultOpaque {
        self.inner.as_ptr()
    }
}
impl<'view> ResultOpaqueSharedArg for ResultOpaqueRefMut<'view> {}
impl<'view> ResultOpaqueMutArg for ResultOpaqueRefMut<'view> {}

impl Drop for ResultOpaque {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::ResultOpaque_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for ResultOpaque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ResultOpaque")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for ResultOpaqueRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ResultOpaqueRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for ResultOpaqueRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ResultOpaqueRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl ResultOpaque {
    pub fn new(i: i32) -> Result<super::ResultOpaque, ErrorEnum> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new(i) };
        match Result::from(result) {
            Ok(result) => Ok({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null ResultOpaque");
                super::ResultOpaque {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
            Err(result) => Err(result),
        }
    }
    pub fn new_failing_foo() -> Result<super::ResultOpaque, ErrorEnum> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_failing_foo() };
        match Result::from(result) {
            Ok(result) => Ok({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null ResultOpaque");
                super::ResultOpaque {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
            Err(result) => Err(result),
        }
    }
    pub fn new_failing_bar() -> Result<super::ResultOpaque, ErrorEnum> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_failing_bar() };
        match Result::from(result) {
            Ok(result) => Ok({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null ResultOpaque");
                super::ResultOpaque {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
            Err(result) => Err(result),
        }
    }
    pub fn new_failing_unit() -> Result<super::ResultOpaque, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_failing_unit() };
        match Result::from(result) {
            Ok(result) => Ok({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null ResultOpaque");
                super::ResultOpaque {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
            Err(result) => Err(result),
        }
    }
    pub fn new_in_err(i: i32) -> Result<(), super::ResultOpaque> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_in_err(i) };
        match Result::from(result) {
            Ok(result) => Ok(result),
            Err(result) => Err({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null ResultOpaque");
                super::ResultOpaque {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
        }
    }
    pub fn new_int(i: i32) -> Result<i32, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_int(i) };
        result.into()
    }
    pub fn new_failing_int(i: i32) -> Result<(), i32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_failing_int(i) };
        result.into()
    }
    pub fn new_failing_char(c: char) -> Result<(), ErrorWithChar> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_failing_char(c as u32) };
        match Result::from(result) {
            Ok(result) => Ok(result),
            Err(result) => Err(ErrorWithChar {
                c: crate::private::char_from_u32(result.c),
            }),
        }
    }
    pub fn new_failing_char_scalar(c: char) -> Result<(), char> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_failing_char_scalar(c as u32) };
        match Result::from(result) {
            Ok(result) => Ok(result),
            Err(result) => Err(crate::private::char_from_u32(result)),
        }
    }
    pub fn new_in_enum_err(i: i32) -> Result<ErrorEnum, super::ResultOpaque> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ResultOpaque_new_in_enum_err(i) };
        match Result::from(result) {
            Ok(result) => Ok(result),
            Err(result) => Err({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null ResultOpaque");
                super::ResultOpaque {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
        }
    }
    /// When we take &str, the return type becomes a Result
    /// Test that this interacts gracefully with returning a reference type
    pub fn takes_str<'a, 'anon_0>(&'a mut self, _v: &'anon_0 str) -> super::ResultOpaqueRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::ResultOpaque_takes_str(
                self.inner.as_ptr(),
                ffi::DiplomatSlice::from(_v.as_bytes()),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null ResultOpaque");
            super::ResultOpaqueRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn assert_integer(&self, i: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::ResultOpaque_assert_integer(self.inner.as_ptr() as *const _, i) };
    }
}

impl<'view> ResultOpaqueRef<'view> {
    pub fn assert_integer(&self, i: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::ResultOpaque_assert_integer(self.inner.as_ptr() as *const _, i) };
    }
}

impl<'view> ResultOpaqueRefMut<'view> {
    /// When we take &str, the return type becomes a Result
    /// Test that this interacts gracefully with returning a reference type
    pub fn takes_str<'a, 'anon_0>(&'a mut self, _v: &'anon_0 str) -> super::ResultOpaqueRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::ResultOpaque_takes_str(
                self.inner.as_ptr(),
                ffi::DiplomatSlice::from(_v.as_bytes()),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null ResultOpaque");
            super::ResultOpaqueRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn assert_integer(&self, i: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::ResultOpaque_assert_integer(self.inner.as_ptr() as *const _, i) };
    }
}
