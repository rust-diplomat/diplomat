//! An owned byte buffer allocated by the provider cdylib.
//!
//! `Drop` calls the provider's `diplomat_owned_slice_u8_destroy`, so the
//! allocator that created the buffer is the one that frees it. Reading is
//! free. Turning it into a `Box<[u8]>` either copies (`clone_to_box`) or
//! reuses the allocation when the caller promises the allocators match
//! (`into_box`).

use core::borrow::Borrow;
use core::mem::ManuallyDrop;
use core::ops::Deref;

use crate::ffi;

/// Bytes owned by the provider's allocator.
pub struct DiplomatBoxU8 {
    ptr: *mut u8,
    len: usize,
}

impl DiplomatBoxU8 {
    pub(crate) fn from_abi(slice: ffi::DiplomatOwnedSlice<u8>) -> Self {
        // `DiplomatOwnedSlice`'s fields are private. `Box::from` reads the
        // pointer out without freeing, and `into_raw` hands that same pointer
        // back before the temporary box can be dropped.
        let boxed: Box<[u8]> = Box::from(slice);
        let len = boxed.len();
        let ptr = Box::into_raw(boxed) as *mut u8;
        Self { ptr, len }
    }

    /// Borrow the bytes. Does not allocate.
    pub fn as_slice(&self) -> &[u8] {
        if self.ptr.is_null() || self.len == 0 {
            return &[];
        }
        // SAFETY: `ptr`/`len` describe the provider allocation until `Drop`.
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Copy the bytes into a `Box<[u8]>` allocated by this crate.
    ///
    /// The original buffer stays owned by `self` and is freed by the provider
    /// when `self` is dropped.
    pub fn clone_to_box(&self) -> Box<[u8]> {
        Box::from(self.as_slice())
    }

    /// Take the provider allocation as a `Box<[u8]>` without copying.
    ///
    /// # Safety
    ///
    /// This crate and the provider cdylib must use the same global allocator.
    /// The returned box frees the buffer with this crate's allocator, and
    /// `self` does not free it again.
    pub unsafe fn into_box(self) -> Box<[u8]> {
        let this = ManuallyDrop::new(self);
        if this.ptr.is_null() {
            return Box::from([]);
        }
        // SAFETY: caller guarantees this allocator owns `ptr`, and `self`'s
        // destructor is suppressed above.
        unsafe { Box::from_raw(core::ptr::slice_from_raw_parts_mut(this.ptr, this.len)) }
    }
}

impl Drop for DiplomatBoxU8 {
    fn drop(&mut self) {
        // SAFETY: `ptr`/`len` came from the provider, and this is the one free.
        // The symbol is the provider cdylib's copy, so the free runs in the
        // allocator that created the buffer.
        unsafe { ffi::diplomat_owned_slice_u8_destroy(self.ptr, self.len) };
    }
}

impl Deref for DiplomatBoxU8 {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<[u8]> for DiplomatBoxU8 {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Borrow<[u8]> for DiplomatBoxU8 {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl core::fmt::Debug for DiplomatBoxU8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DiplomatBoxU8")
            .field(&self.as_slice())
            .finish()
    }
}
