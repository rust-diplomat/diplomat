use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// Owned `Box<[u8]>` returns: the provider allocates and hands ownership across.
pub struct Bytes {
    pub(crate) inner: NonNull<ffi::Bytes>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct BytesRef<'view> {
    pub(crate) inner: NonNull<ffi::Bytes>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct BytesRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Bytes>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait BytesSharedArg: crate::private::BytesSharedSealed {}

#[doc(hidden)]
pub trait BytesMutArg: crate::private::BytesMutSealed {}

impl crate::private::BytesSharedSealed for Bytes {
    fn __as_const_ptr(&self) -> *const ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl crate::private::BytesMutSealed for Bytes {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl BytesSharedArg for Bytes {}
impl BytesMutArg for Bytes {}
impl<'view> crate::private::BytesSharedSealed for BytesRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl<'view> BytesSharedArg for BytesRef<'view> {}
impl<'view> crate::private::BytesSharedSealed for BytesRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::BytesMutSealed for BytesRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl<'view> BytesSharedArg for BytesRefMut<'view> {}
impl<'view> BytesMutArg for BytesRefMut<'view> {}

impl Drop for Bytes {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Bytes_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Bytes").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view> fmt::Debug for BytesRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BytesRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for BytesRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BytesRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Bytes {
    /// An owned byte-slice return: ownership of the provider's allocation crosses
    /// the ABI, which is what `owned_byte_slice_returns` promises.
    pub fn make(len: u32) -> Box<[u8]> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Bytes_make(len) };
        Box::from(result)
    }
    /// The same flag with borrowed-slice parameters alongside the owned return.
    pub fn join(a: &[u8], b: &[u8]) -> Box<[u8]> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Bytes_join(ffi::DiplomatSlice::from(a), ffi::DiplomatSlice::from(b)) };
        Box::from(result)
    }
}

impl<'view> BytesRef<'view> {}

impl<'view> BytesRefMut<'view> {}
