use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// Opaque owning a string, exercising `DiplomatStr`/`&str` borrows.
pub struct Message {
    pub(crate) inner: NonNull<ffi::Message>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MessageRef<'view> {
    pub(crate) inner: NonNull<ffi::Message>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MessageRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Message>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait MessageSharedArg: crate::private::MessageSharedSealed {}

#[doc(hidden)]
pub trait MessageMutArg: crate::private::MessageMutSealed {}

impl crate::private::MessageSharedSealed for Message {
    fn __as_const_ptr(&self) -> *const ffi::Message {
        self.inner.as_ptr()
    }
}
impl crate::private::MessageMutSealed for Message {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Message {
        self.inner.as_ptr()
    }
}
impl MessageSharedArg for Message {}
impl MessageMutArg for Message {}
impl<'view> crate::private::MessageSharedSealed for MessageRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Message {
        self.inner.as_ptr()
    }
}
impl<'view> MessageSharedArg for MessageRef<'view> {}
impl<'view> crate::private::MessageSharedSealed for MessageRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Message {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::MessageMutSealed for MessageRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Message {
        self.inner.as_ptr()
    }
}
impl<'view> MessageSharedArg for MessageRefMut<'view> {}
impl<'view> MessageMutArg for MessageRefMut<'view> {}

impl Drop for Message {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Message_destroy(self.inner.as_ptr()) };
    }
}

impl Message {
    pub fn new(v: &[u8]) -> super::Message {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_new(ffi::DiplomatSlice::from(v)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Message");
            super::Message {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_bytes(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn text<'a>(&'a self) -> &'a str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_text(self.inner.as_ptr() as *const _) };
        unsafe { crate::private::utf8_str_from_slice(result) }
    }
    pub fn utf8_len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Message_utf8_len(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> MessageRef<'view> {
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_bytes(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn text<'a>(&'a self) -> &'a str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_text(self.inner.as_ptr() as *const _) };
        unsafe { crate::private::utf8_str_from_slice(result) }
    }
    pub fn utf8_len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Message_utf8_len(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> MessageRefMut<'view> {
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_bytes(self.inner.as_ptr() as *const _) };
        result.into()
    }
    pub fn text<'a>(&'a self) -> &'a str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_text(self.inner.as_ptr() as *const _) };
        unsafe { crate::private::utf8_str_from_slice(result) }
    }
    pub fn utf8_len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Message_utf8_len(self.inner.as_ptr() as *const _) }
    }
}
