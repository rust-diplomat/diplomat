use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct Utf16Wrap {
    pub(crate) inner: NonNull<ffi::Utf16Wrap>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct Utf16WrapRef<'view> {
    pub(crate) inner: NonNull<ffi::Utf16Wrap>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct Utf16WrapRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Utf16Wrap>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait Utf16WrapSharedArg: crate::private::Utf16WrapSharedSealed {}

#[doc(hidden)]
pub trait Utf16WrapMutArg: crate::private::Utf16WrapMutSealed {}

impl crate::private::Utf16WrapSharedSealed for Utf16Wrap {
    fn __as_const_ptr(&self) -> *const ffi::Utf16Wrap {
        self.inner.as_ptr()
    }
}

impl crate::private::Utf16WrapMutSealed for Utf16Wrap {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Utf16Wrap {
        self.inner.as_ptr()
    }
}

impl Utf16WrapSharedArg for Utf16Wrap {}
impl Utf16WrapMutArg for Utf16Wrap {}

impl<'view> crate::private::Utf16WrapSharedSealed for Utf16WrapRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Utf16Wrap {
        self.inner.as_ptr()
    }
}

impl<'view> Utf16WrapSharedArg for Utf16WrapRef<'view> {}

impl<'view> crate::private::Utf16WrapSharedSealed for Utf16WrapRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Utf16Wrap {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::Utf16WrapMutSealed for Utf16WrapRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Utf16Wrap {
        self.inner.as_ptr()
    }
}

impl<'view> Utf16WrapSharedArg for Utf16WrapRefMut<'view> {}
impl<'view> Utf16WrapMutArg for Utf16WrapRefMut<'view> {}

impl Drop for Utf16Wrap {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Utf16Wrap_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Utf16Wrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Utf16Wrap")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for Utf16WrapRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Utf16WrapRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for Utf16WrapRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Utf16WrapRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Utf16Wrap {
    pub fn from_utf16(input: &[u16]) -> crate::Utf16Wrap {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Utf16Wrap_from_utf16(ffi::DiplomatSlice::from(input)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Utf16Wrap");
            crate::Utf16Wrap {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn get_debug_str(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::Utf16Wrap_get_debug_str(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
    pub fn borrow_cont<'a>(&'a self) -> &'a [u16] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Utf16Wrap_borrow_cont(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view> Utf16WrapRef<'view> {
    pub fn get_debug_str(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::Utf16Wrap_get_debug_str(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
    pub fn borrow_cont<'a>(&'a self) -> &'a [u16] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Utf16Wrap_borrow_cont(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view> Utf16WrapRefMut<'view> {
    pub fn get_debug_str(&self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::Utf16Wrap_get_debug_str(self.inner.as_ptr() as *const _, write) };
        })
        .1
    }
    pub fn borrow_cont<'a>(&'a self) -> &'a [u16] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Utf16Wrap_borrow_cont(self.inner.as_ptr() as *const _) };
        result.into()
    }
}
