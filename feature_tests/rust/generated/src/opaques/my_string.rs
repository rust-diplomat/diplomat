use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct MyString {
    pub(crate) inner: NonNull<ffi::MyString>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MyStringRef<'view> {
    pub(crate) inner: NonNull<ffi::MyString>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MyStringRefMut<'view> {
    pub(crate) inner: NonNull<ffi::MyString>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait MyStringSharedArg: crate::private::MyStringSharedSealed {}

#[doc(hidden)]
pub trait MyStringMutArg: crate::private::MyStringMutSealed {}

impl crate::private::MyStringSharedSealed for MyString {
    fn __as_const_ptr(&self) -> *const ffi::MyString {
        self.inner.as_ptr()
    }
}

impl crate::private::MyStringMutSealed for MyString {
    fn __as_mut_ptr(&mut self) -> *mut ffi::MyString {
        self.inner.as_ptr()
    }
}

impl MyStringSharedArg for MyString {}
impl MyStringMutArg for MyString {}

impl<'view> crate::private::MyStringSharedSealed for MyStringRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::MyString {
        self.inner.as_ptr()
    }
}

impl<'view> MyStringSharedArg for MyStringRef<'view> {}

impl<'view> crate::private::MyStringSharedSealed for MyStringRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::MyString {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::MyStringMutSealed for MyStringRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::MyString {
        self.inner.as_ptr()
    }
}

impl<'view> MyStringSharedArg for MyStringRefMut<'view> {}
impl<'view> MyStringMutArg for MyStringRefMut<'view> {}

impl Drop for MyString {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::MyString_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyString")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for MyStringRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyStringRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for MyStringRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyStringRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl MyString {
    pub fn new(v: &[u8]) -> crate::MyString {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyString_new(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null MyString");
            crate::MyString {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn r#unsafe(v: &str) -> crate::MyString {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyString_new_unsafe(ffi::DiplomatSlice::from(v.as_bytes())) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null MyString");
            crate::MyString {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn set_str(&mut self, new_str: &[u8]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::MyString_set_str(self.inner.as_ptr(), ffi::DiplomatSlice::from(new_str)) };
    }
    pub fn get_str(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::MyString_get_str(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
    pub fn get_static_str() -> &'static str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyString_get_static_str() };
        unsafe { crate::private::utf8_str_from_slice(result) }
    }
    pub fn string_transform(foo: &str) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe {
                ffi::MyString_string_transform(ffi::DiplomatSlice::from(foo.as_bytes()), write)
            };
        })
        .1
    }
    pub fn borrow<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyString_borrow(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view> MyStringRef<'view> {
    pub fn get_str(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::MyString_get_str(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
    pub fn borrow<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyString_borrow(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view> MyStringRefMut<'view> {
    pub fn set_str(&mut self, new_str: &[u8]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::MyString_set_str(self.inner.as_ptr(), ffi::DiplomatSlice::from(new_str)) };
    }
    pub fn get_str(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::MyString_get_str(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
    pub fn borrow<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyString_borrow(self.inner.as_ptr() as *const _) };
        result.into()
    }
}
