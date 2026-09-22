use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
#[allow(unused_imports)]
use crate::types::*;

pub struct Foo<'a> {
    pub(crate) inner: NonNull<ffi::Foo>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct FooRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::Foo>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct FooRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::Foo>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait FooSharedArg: crate::private::FooSharedSealed {}

#[doc(hidden)]
pub trait FooMutArg: crate::private::FooMutSealed {}

impl<'a> crate::private::FooSharedSealed for Foo<'a> {
    fn __as_const_ptr(&self) -> *const ffi::Foo {
        self.inner.as_ptr()
    }
}

impl<'a> crate::private::FooMutSealed for Foo<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Foo {
        self.inner.as_ptr()
    }
}

impl<'a> FooSharedArg for Foo<'a> {}
impl<'a> FooMutArg for Foo<'a> {}

impl<'view, 'a> crate::private::FooSharedSealed for FooRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Foo {
        self.inner.as_ptr()
    }
}

impl<'view, 'a> FooSharedArg for FooRef<'view, 'a> {}

impl<'view, 'a> crate::private::FooSharedSealed for FooRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Foo {
        self.inner.as_ptr()
    }
}

impl<'view, 'a> crate::private::FooMutSealed for FooRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Foo {
        self.inner.as_ptr()
    }
}

impl<'view, 'a> FooSharedArg for FooRefMut<'view, 'a> {}
impl<'view, 'a> FooMutArg for FooRefMut<'view, 'a> {}

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Foo_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> fmt::Debug for Foo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Foo").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a> fmt::Debug for FooRef<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FooRef").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a> fmt::Debug for FooRefMut<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FooRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'a> Foo<'a> {
    pub fn new(x: &'a [u8]) -> crate::Foo<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_new(ffi::DiplomatSlice::from(x)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Foo");
            crate::Foo {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn get_bar<'b>(&'b self) -> crate::Bar<'b, 'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_get_bar(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Bar");
            crate::Bar {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn r#static(x: &'static [u8]) -> crate::Foo<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_new_static(ffi::DiplomatSlice::from(x)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Foo");
            crate::Foo {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn as_returning<'anon_0>(&'anon_0 self) -> BorrowedFieldsReturning<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_as_returning(self.inner.as_ptr() as *const _) };
        BorrowedFieldsReturning {
            bytes: result.bytes.into(),
            _lifetimes: PhantomData,
        }
    }
}

impl<'view, 'a> FooRef<'view, 'a> {
    pub fn get_bar<'b>(&'b self) -> crate::Bar<'b, 'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_get_bar(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Bar");
            crate::Bar {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn as_returning<'anon_0>(&'anon_0 self) -> BorrowedFieldsReturning<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_as_returning(self.inner.as_ptr() as *const _) };
        BorrowedFieldsReturning {
            bytes: result.bytes.into(),
            _lifetimes: PhantomData,
        }
    }
}

impl<'view, 'a> FooRefMut<'view, 'a> {
    pub fn get_bar<'b>(&'b self) -> crate::Bar<'b, 'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_get_bar(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Bar");
            crate::Bar {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn as_returning<'anon_0>(&'anon_0 self) -> BorrowedFieldsReturning<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Foo_as_returning(self.inner.as_ptr() as *const _) };
        BorrowedFieldsReturning {
            bytes: result.bytes.into(),
            _lifetimes: PhantomData,
        }
    }
}
