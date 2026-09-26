use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct MyOpaqueEnum {
    pub(crate) inner: NonNull<ffi::MyOpaqueEnum>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MyOpaqueEnumRef<'view> {
    pub(crate) inner: NonNull<ffi::MyOpaqueEnum>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MyOpaqueEnumRefMut<'view> {
    pub(crate) inner: NonNull<ffi::MyOpaqueEnum>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait MyOpaqueEnumSharedArg: crate::private::MyOpaqueEnumSharedSealed {}

#[doc(hidden)]
pub trait MyOpaqueEnumMutArg: crate::private::MyOpaqueEnumMutSealed {}

impl crate::private::MyOpaqueEnumSharedSealed for MyOpaqueEnum {
    fn __as_const_ptr(&self) -> *const ffi::MyOpaqueEnum {
        self.inner.as_ptr()
    }
}

impl crate::private::MyOpaqueEnumMutSealed for MyOpaqueEnum {
    fn __as_mut_ptr(&mut self) -> *mut ffi::MyOpaqueEnum {
        self.inner.as_ptr()
    }
}

impl MyOpaqueEnumSharedArg for MyOpaqueEnum {}
impl MyOpaqueEnumMutArg for MyOpaqueEnum {}

impl<'view> crate::private::MyOpaqueEnumSharedSealed for MyOpaqueEnumRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::MyOpaqueEnum {
        self.inner.as_ptr()
    }
}

impl<'view> MyOpaqueEnumSharedArg for MyOpaqueEnumRef<'view> {}

impl<'view> crate::private::MyOpaqueEnumSharedSealed for MyOpaqueEnumRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::MyOpaqueEnum {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::MyOpaqueEnumMutSealed for MyOpaqueEnumRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::MyOpaqueEnum {
        self.inner.as_ptr()
    }
}

impl<'view> MyOpaqueEnumSharedArg for MyOpaqueEnumRefMut<'view> {}
impl<'view> MyOpaqueEnumMutArg for MyOpaqueEnumRefMut<'view> {}

impl Drop for MyOpaqueEnum {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::MyOpaqueEnum_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for MyOpaqueEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyOpaqueEnum")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for MyOpaqueEnumRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyOpaqueEnumRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for MyOpaqueEnumRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyOpaqueEnumRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl MyOpaqueEnum {
    pub fn new() -> crate::MyOpaqueEnum {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyOpaqueEnum_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null MyOpaqueEnum");
            crate::MyOpaqueEnum {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn to_string(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::MyOpaqueEnum_to_string(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
}

impl<'view> MyOpaqueEnumRef<'view> {
    pub fn to_string(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::MyOpaqueEnum_to_string(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
}

impl<'view> MyOpaqueEnumRefMut<'view> {
    pub fn to_string(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::MyOpaqueEnum_to_string(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
}
