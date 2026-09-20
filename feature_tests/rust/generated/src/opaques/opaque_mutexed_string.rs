use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OpaqueMutexedString {
    pub(crate) inner: NonNull<ffi::OpaqueMutexedString>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueMutexedStringRef<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueMutexedString>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueMutexedStringRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueMutexedString>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OpaqueMutexedStringSharedArg: crate::private::OpaqueMutexedStringSharedSealed {}

#[doc(hidden)]
pub trait OpaqueMutexedStringMutArg: crate::private::OpaqueMutexedStringMutSealed {}

impl crate::private::OpaqueMutexedStringSharedSealed for OpaqueMutexedString {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueMutexedString {
        self.inner.as_ptr()
    }
}
impl crate::private::OpaqueMutexedStringMutSealed for OpaqueMutexedString {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueMutexedString {
        self.inner.as_ptr()
    }
}
impl OpaqueMutexedStringSharedArg for OpaqueMutexedString {}
impl OpaqueMutexedStringMutArg for OpaqueMutexedString {}
impl<'view> crate::private::OpaqueMutexedStringSharedSealed for OpaqueMutexedStringRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueMutexedString {
        self.inner.as_ptr()
    }
}
impl<'view> OpaqueMutexedStringSharedArg for OpaqueMutexedStringRef<'view> {}
impl<'view> crate::private::OpaqueMutexedStringSharedSealed for OpaqueMutexedStringRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueMutexedString {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::OpaqueMutexedStringMutSealed for OpaqueMutexedStringRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueMutexedString {
        self.inner.as_ptr()
    }
}
impl<'view> OpaqueMutexedStringSharedArg for OpaqueMutexedStringRefMut<'view> {}
impl<'view> OpaqueMutexedStringMutArg for OpaqueMutexedStringRefMut<'view> {}

impl Drop for OpaqueMutexedString {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OpaqueMutexedString_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OpaqueMutexedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueMutexedString")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueMutexedStringRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueMutexedStringRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueMutexedStringRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueMutexedStringRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OpaqueMutexedString {
    pub fn from_usize(number: usize) -> super::OpaqueMutexedString {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_from_usize(number) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedString {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn change(&self, number: usize) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueMutexedString_change(self.inner.as_ptr() as *const _, number) };
    }
    pub fn borrow<'a>(&'a self) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_borrow(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn borrow_other<'a>(
        other: &'a impl super::OpaqueMutexedStringSharedArg,
    ) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OpaqueMutexedString_borrow_other(
                crate::private::OpaqueMutexedStringSharedSealed::__as_const_ptr(other),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn borrow_self_or_other<'a>(
        &'a self,
        other: &'a impl super::OpaqueMutexedStringSharedArg,
    ) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OpaqueMutexedString_borrow_self_or_other(
                self.inner.as_ptr() as *const _,
                crate::private::OpaqueMutexedStringSharedSealed::__as_const_ptr(other),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn get_len_and_add(&self, other: usize) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueMutexedString_get_len_and_add(self.inner.as_ptr() as *const _, other) }
    }
    pub fn dummy_str<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_dummy_str(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn wrapper(&self) -> super::Utf16Wrap {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_wrapper(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Utf16Wrap");
            super::Utf16Wrap {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::OpaqueMutexedString_to_unsigned_from_unsigned(
                self.inner.as_ptr() as *const _,
                input,
            )
        }
    }
}

impl<'view> OpaqueMutexedStringRef<'view> {
    pub fn change(&self, number: usize) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueMutexedString_change(self.inner.as_ptr() as *const _, number) };
    }
    pub fn borrow<'a>(&'a self) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_borrow(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn borrow_self_or_other<'a>(
        &'a self,
        other: &'a impl super::OpaqueMutexedStringSharedArg,
    ) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OpaqueMutexedString_borrow_self_or_other(
                self.inner.as_ptr() as *const _,
                crate::private::OpaqueMutexedStringSharedSealed::__as_const_ptr(other),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn get_len_and_add(&self, other: usize) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueMutexedString_get_len_and_add(self.inner.as_ptr() as *const _, other) }
    }
    pub fn dummy_str<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_dummy_str(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn wrapper(&self) -> super::Utf16Wrap {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_wrapper(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Utf16Wrap");
            super::Utf16Wrap {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::OpaqueMutexedString_to_unsigned_from_unsigned(
                self.inner.as_ptr() as *const _,
                input,
            )
        }
    }
}

impl<'view> OpaqueMutexedStringRefMut<'view> {
    pub fn change(&self, number: usize) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueMutexedString_change(self.inner.as_ptr() as *const _, number) };
    }
    pub fn borrow<'a>(&'a self) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_borrow(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn borrow_self_or_other<'a>(
        &'a self,
        other: &'a impl super::OpaqueMutexedStringSharedArg,
    ) -> super::OpaqueMutexedStringRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OpaqueMutexedString_borrow_self_or_other(
                self.inner.as_ptr() as *const _,
                crate::private::OpaqueMutexedStringSharedSealed::__as_const_ptr(other),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueMutexedString");
            super::OpaqueMutexedStringRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn get_len_and_add(&self, other: usize) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueMutexedString_get_len_and_add(self.inner.as_ptr() as *const _, other) }
    }
    pub fn dummy_str<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_dummy_str(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn wrapper(&self) -> super::Utf16Wrap {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueMutexedString_wrapper(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Utf16Wrap");
            super::Utf16Wrap {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::OpaqueMutexedString_to_unsigned_from_unsigned(
                self.inner.as_ptr() as *const _,
                input,
            )
        }
    }
}
