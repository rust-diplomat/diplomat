use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
#[allow(unused_imports)]
use crate::types::*;

pub struct OptionOpaque {
    pub(crate) inner: NonNull<ffi::OptionOpaque>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OptionOpaqueRef<'view> {
    pub(crate) inner: NonNull<ffi::OptionOpaque>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OptionOpaqueRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OptionOpaque>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OptionOpaqueSharedArg: crate::private::OptionOpaqueSharedSealed {}

#[doc(hidden)]
pub trait OptionOpaqueMutArg: crate::private::OptionOpaqueMutSealed {}

impl crate::private::OptionOpaqueSharedSealed for OptionOpaque {
    fn __as_const_ptr(&self) -> *const ffi::OptionOpaque {
        self.inner.as_ptr()
    }
}

impl crate::private::OptionOpaqueMutSealed for OptionOpaque {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OptionOpaque {
        self.inner.as_ptr()
    }
}

impl OptionOpaqueSharedArg for OptionOpaque {}
impl OptionOpaqueMutArg for OptionOpaque {}

impl<'view> crate::private::OptionOpaqueSharedSealed for OptionOpaqueRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OptionOpaque {
        self.inner.as_ptr()
    }
}

impl<'view> OptionOpaqueSharedArg for OptionOpaqueRef<'view> {}

impl<'view> crate::private::OptionOpaqueSharedSealed for OptionOpaqueRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OptionOpaque {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::OptionOpaqueMutSealed for OptionOpaqueRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OptionOpaque {
        self.inner.as_ptr()
    }
}

impl<'view> OptionOpaqueSharedArg for OptionOpaqueRefMut<'view> {}
impl<'view> OptionOpaqueMutArg for OptionOpaqueRefMut<'view> {}

impl Drop for OptionOpaque {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OptionOpaque_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OptionOpaque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionOpaque")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OptionOpaqueRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionOpaqueRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OptionOpaqueRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OptionOpaqueRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OptionOpaque {
    pub fn new(i: i32) -> Option<crate::OptionOpaque> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_new(i) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaque {
            inner,
            _not_send_sync: PhantomData,
        })
    }
    pub fn new_none() -> Option<crate::OptionOpaque> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_new_none() };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaque {
            inner,
            _not_send_sync: PhantomData,
        })
    }
    pub fn option_isize(&self) -> Option<isize> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_isize(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_usize(&self) -> Option<usize> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_usize(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_i32(&self) -> Option<i32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_i32(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_u32(&self) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_u32(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn returns_none_self<'a>(&'a self) -> Option<crate::OptionOpaqueRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::OptionOpaque_returns_none_self(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaqueRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn returns_some_self<'a>(&'a self) -> Option<crate::OptionOpaqueRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::OptionOpaque_returns_some_self(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaqueRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn assert_integer(&self, i: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OptionOpaque_assert_integer(self.inner.as_ptr() as *const _, i) };
    }
    pub fn accepts_option_u8(arg: Option<u8>, sentinel: u8) -> Option<u8> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OptionOpaque_accepts_option_u8(ffi::DiplomatOption::from(arg), sentinel)
        };
        result.into()
    }
    pub fn accepts_option_enum(arg: Option<OptionEnum>, sentinel: u8) -> Option<OptionEnum> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OptionOpaque_accepts_option_enum(ffi::DiplomatOption::from(arg), sentinel)
        };
        result.into()
    }
    pub fn accepts_borrowing_option_struct<'anon_0>(arg: BorrowingOptionStruct<'anon_0>) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::OptionOpaque_accepts_borrowing_option_struct(ffi::BorrowingOptionStruct {
                a: ffi::DiplomatOption::from(arg.a.map(ffi::DiplomatSlice::from)),
            })
        };
    }
    pub fn accepts_multiple_option_enum(
        sentinel1: u8,
        arg1: Option<OptionEnum>,
        arg2: Option<OptionEnum>,
        arg3: Option<OptionEnum>,
        sentinel2: u8,
    ) -> Option<OptionEnum> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OptionOpaque_accepts_multiple_option_enum(
                sentinel1,
                ffi::DiplomatOption::from(arg1),
                ffi::DiplomatOption::from(arg2),
                ffi::DiplomatOption::from(arg3),
                sentinel2,
            )
        };
        result.into()
    }
    pub fn accepts_option_input_struct(
        arg: Option<OptionInputStruct>,
        sentinel: u8,
    ) -> Option<OptionInputStruct> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OptionOpaque_accepts_option_input_struct(
                ffi::DiplomatOption::from(arg.map(|__v| ffi::OptionInputStruct {
                    a: ffi::DiplomatOption::from(__v.a),
                    b: ffi::DiplomatOption::from(__v.b.map(|__v| __v as u32)),
                    c: ffi::DiplomatOption::from(__v.c),
                })),
                sentinel,
            )
        };
        result.into_option().map(|__v| OptionInputStruct {
            a: __v.a.into_option(),
            b: __v.b.into_option().map(crate::private::char_from_u32),
            c: __v.c.into_option(),
        })
    }
    pub fn returns_option_input_struct() -> OptionInputStruct {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_returns_option_input_struct() };
        OptionInputStruct {
            a: result.a.into_option(),
            b: result.b.into_option().map(crate::private::char_from_u32),
            c: result.c.into_option(),
        }
    }
}

impl<'view> OptionOpaqueRef<'view> {
    pub fn option_isize(&self) -> Option<isize> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_isize(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_usize(&self) -> Option<usize> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_usize(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_i32(&self) -> Option<i32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_i32(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_u32(&self) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_u32(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn returns_none_self<'a>(&'a self) -> Option<crate::OptionOpaqueRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::OptionOpaque_returns_none_self(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaqueRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn returns_some_self<'a>(&'a self) -> Option<crate::OptionOpaqueRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::OptionOpaque_returns_some_self(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaqueRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn assert_integer(&self, i: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OptionOpaque_assert_integer(self.inner.as_ptr() as *const _, i) };
    }
}

impl<'view> OptionOpaqueRefMut<'view> {
    pub fn option_isize(&self) -> Option<isize> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_isize(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_usize(&self) -> Option<usize> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_usize(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_i32(&self) -> Option<i32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_i32(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn option_u32(&self) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OptionOpaque_option_u32(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn returns_none_self<'a>(&'a self) -> Option<crate::OptionOpaqueRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::OptionOpaque_returns_none_self(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaqueRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn returns_some_self<'a>(&'a self) -> Option<crate::OptionOpaqueRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::OptionOpaque_returns_some_self(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OptionOpaqueRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn assert_integer(&self, i: i32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OptionOpaque_assert_integer(self.inner.as_ptr() as *const _, i) };
    }
}
