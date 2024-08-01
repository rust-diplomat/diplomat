use alloc::boxed::Box;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
/// This is equivalent to &[T], except it has a stable repr(C) layout
#[repr(C)]
pub struct DiplomatSlice<'a, T> {
    // Invariant: ptr is a valid ptr to the beginning of an &[T] allocation. It may be null if len is null
    ptr: *const T,
    // Invariant: the allocation contains at least `len` elements
    len: usize,
    phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Clone for DiplomatSlice<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for DiplomatSlice<'a, T> {}

impl<'a, T> From<&'a [T]> for DiplomatSlice<'a, T> {
    fn from(x: &'a [T]) -> Self {
        // Safe to construct since we're constructing it from a slice
        DiplomatSlice {
            ptr: x.as_ptr(),
            len: x.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<DiplomatSlice<'a, T>> for &'a [T] {
    fn from(x: DiplomatSlice<'a, T>) -> Self {
        unsafe {
            // It's common in C-land to represent empty slices with NULL, which is not the case in Rust
            // We normalize this
            if x.ptr.is_null() {
                debug_assert!(x.len == 0);
                return &[];
            }
            // Safety: carrying over safety variants from DiplomatSlice, and we null-checked
            core::slice::from_raw_parts(x.ptr, x.len)
        }
    }
}

impl<'a, T> Deref for DiplomatSlice<'a, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        (*self).into()
    }
}

/// This is equivalent to &mut [T], except it has a stable repr(C) layout
#[repr(C)]
pub struct DiplomatSliceMut<'a, T> {
    // Invariant: ptr is a valid ptr to the beginning of an &[T] allocation.  It may be null if len is null
    ptr: *mut T,
    // Invariant: the allocation contains at least `len` elements
    len: usize,
    phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> From<&'a mut [T]> for DiplomatSliceMut<'a, T> {
    fn from(x: &'a mut [T]) -> Self {
        // Safe to construct since we're constructing it from a slice
        DiplomatSliceMut {
            ptr: x.as_mut_ptr(),
            len: x.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<DiplomatSliceMut<'a, T>> for &'a mut [T] {
    fn from(x: DiplomatSliceMut<'a, T>) -> Self {
        unsafe {
            if x.ptr.is_null() {
                debug_assert!(x.len == 0);
                return &mut [];
            }
            // Safety: carrying over safety variants from DiplomatSliceMut
            core::slice::from_raw_parts_mut(x.ptr, x.len)
        }
    }
}

impl<'a, T> Deref for DiplomatSliceMut<'a, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        if self.ptr.is_null() {
            debug_assert!(self.len == 0);
            return &[];
        }
        unsafe {
            // Safety: carrying over safety variants from DiplomatSliceMut

            core::slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl<'a, T> DerefMut for DiplomatSliceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.ptr.is_null() {
            debug_assert!(self.len == 0);
            return &mut [];
        }
        unsafe {
            // Safety: carrying over safety variants from DiplomatSliceMut

            core::slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }
}

#[repr(C)]
pub struct DiplomatOwnedSlice<T> {
    // Invariant: ptr is a valid ptr to the beginning of an owned Box<[T]> allocation.  It may be null if len is null
    ptr: *mut T,
    // Invariant: the allocation contains `len` elements
    len: usize,
    phantom: PhantomData<Box<[T]>>,
}

impl<T> Drop for DiplomatOwnedSlice<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // Safety: This is equivalent to a valid Box
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    self.ptr, self.len,
                )));
            }
        }
    }
}

impl<T> From<Box<[T]>> for DiplomatOwnedSlice<T> {
    fn from(x: Box<[T]>) -> Self {
        // Safe to construct since we're constructing it from a valid Box<[T]>
        let len = x.len();
        DiplomatOwnedSlice {
            ptr: Box::into_raw(x) as *mut T,
            len,
            phantom: PhantomData,
        }
    }
}

impl<T> From<DiplomatOwnedSlice<T>> for Box<[T]> {
    fn from(x: DiplomatOwnedSlice<T>) -> Self {
        let x = ManuallyDrop::new(x);
        unsafe {
            if x.ptr.is_null() {
                debug_assert!(x.len == 0);
                let dangling = core::ptr::NonNull::dangling().as_ptr();
                return Box::from_raw(core::ptr::slice_from_raw_parts_mut(dangling, x.len));
            }
            // Safety: carrying over safety variants from DiplomatOwnedSlice
            Box::from_raw(core::ptr::slice_from_raw_parts_mut(x.ptr, x.len))
        }
    }
}

impl<T> Deref for DiplomatOwnedSlice<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        if self.ptr.is_null() {
            debug_assert!(self.len == 0);
            return &[];
        }
        // Safety: The type invariants allow us to treat is as a valid Box<[T]>
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for DiplomatOwnedSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.ptr.is_null() {
            debug_assert!(self.len == 0);
            return &mut [];
        }
        // Safety: The type invariants allow us to treat is as a valid Box<[T]>
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

/// This is equivalent to &str, except it has a stable repr(C) layout
// Safety invariant: contained slice must be valid UTF-8
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DiplomatUtf8StrSlice<'a>(DiplomatSlice<'a, u8>);

impl<'a> From<&'a str> for DiplomatUtf8StrSlice<'a> {
    fn from(x: &'a str) -> Self {
        // Safety: invariant upheld; obtained from `str`
        Self(x.as_bytes().into())
    }
}

impl<'a> From<DiplomatUtf8StrSlice<'a>> for &'a str {
    fn from(x: DiplomatUtf8StrSlice<'a>) -> Self {
        unsafe {
            // We can assume this because of the invariant on DiplomatUtf8StrSlice
            core::str::from_utf8_unchecked(<&[u8]>::from(x.0))
        }
    }
}

impl<'a> Deref for DiplomatUtf8StrSlice<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        (*self).into()
    }
}

/// This is equivalent to Box<str>, except it has a stable repr(C) layout
// Safety invariant: contained slice must be valid UTF-8
#[repr(transparent)]
pub struct DiplomatOwnedUTF8StrSlice(DiplomatOwnedSlice<u8>);

impl From<Box<str>> for DiplomatOwnedUTF8StrSlice {
    fn from(x: Box<str>) -> Self {
        // Safety: invariant upheld; obtained from `str`
        Self(Box::<[u8]>::from(x).into())
    }
}

impl From<DiplomatOwnedUTF8StrSlice> for Box<str> {
    fn from(x: DiplomatOwnedUTF8StrSlice) -> Self {
        let buf = Box::<[u8]>::from(x.0);
        let len = buf.len();
        let raw = Box::into_raw(buf);
        let raw = if raw.is_null() {
            debug_assert!(len == 0);
            NonNull::<u8>::dangling().as_ptr()
        } else {
            raw as *mut u8
        };
        unsafe {
            // We deconstructed the Box already so we don't have to worry about provenance
            // We're technically making an &mut [u8] here, because there isn't an easy route to construct
            // string types that are owned
            let slice = core::slice::from_raw_parts_mut(raw, len);
            // We can assume this because of the invariant on DiplomatOwnedUTF8StrSlice
            let strslice = core::str::from_utf8_unchecked_mut(slice);
            Box::from_raw(strslice as *mut str)
        }
    }
}

impl Deref for DiplomatOwnedUTF8StrSlice {
    type Target = str;
    fn deref(&self) -> &str {
        let slice = &self.0;
        unsafe {
            // We can assume this because of the invariant on DiplomatOwnedUTF8StrSlice
            core::str::from_utf8_unchecked(slice)
        }
    }
}

/// Like &str, but unvalidated
pub type DiplomatStrSlice<'a> = DiplomatSlice<'a, u8>;
/// Like Box<str>, but unvalidated
pub type DiplomatOwnedStrSlice = DiplomatOwnedSlice<u8>;
/// An unvalidated UTF-16 string
pub type DiplomatStr16Slice<'a> = DiplomatSlice<'a, u16>;
/// An unvalidated, owned UTF-16 string
pub type DiplomatOwnedStr16Slice<'a> = DiplomatOwnedSlice<u16>;
