use core::marker::PhantomData;
use core::ops::Deref;

/// This is equivalent to &[T], except it has a stable repr(C) layout
#[repr(C)]
pub struct DiplomatSlice<'a, T> {
    // Invariant: ptr is a valid ptr to the beginning of an &[T] allocation
    ptr: *const T,
    // Invariant: the allocation contains at least `len` elements
    len: usize,
    phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Clone for DiplomatSlice<'a, T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
            phantom: PhantomData,
        }
    }
}
impl<'a, T> Copy for DiplomatSlice<'a, T> {}

impl<'a, T> From<&'a [T]> for DiplomatSlice<'a, T> {
    fn from(x: &'a [T]) -> Self {
        // Safe to construct since we're constructing it from a
        DiplomatSlice {
            ptr: x as *const [T] as *const T,
            len: x.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<DiplomatSlice<'a, T>> for &'a [T] {
    fn from(x: DiplomatSlice<'a, T>) -> Self {
        unsafe {
            // Safety: carrying over safety variants from DiplomatSlice
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

/// This is equivalent to &str, except it has a stable repr(C) layout
// Safety invariant: contained slice must be valid UTF-8
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DiplomatUTF8StrSlice<'a>(DiplomatSlice<'a, u8>);

impl<'a> From<&'a str> for DiplomatUTF8StrSlice<'a> {
    fn from(x: &'a str) -> Self {
        // Safety: invariant upheld; obtained from `str`
        Self(x.as_bytes().into())
    }
}

impl<'a> From<DiplomatUTF8StrSlice<'a>> for &'a str {
    fn from(x: DiplomatUTF8StrSlice<'a>) -> Self {
        unsafe {
            // We can assume this because of the invariant on DiplomatUTF8StrSlice
            core::str::from_utf8_unchecked(<&[u8]>::from(x.0))
        }
    }
}

impl<'a> Deref for DiplomatUTF8StrSlice<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        (*self).into()
    }
}

/// Like &str, but unvalidated
pub type DiplomatStrSlice<'a> = DiplomatSlice<'a, u8>;
/// An unvalidated UTF-16 string
pub type DiplomatStr16Slice<'a> = DiplomatSlice<'a, u16>;
