use alloc::boxed::Box;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

/// This is equivalent to `&[T]`, except it has a stable `repr(C)` layout.
///
/// In `#[diplomat::bridge]` function signatures, prefer using standard `&[T]` instead.
/// Use `DiplomatSlice<'a, T>` explicitly when storing a slice inside a Diplomat struct.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     pub struct MyStruct<'a> {
///         pub data: DiplomatSlice<'a, u8>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn print(self) {
///             println!("{:?}", &*self.data);
///         }
///     }
///
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // In function signatures, prefer `&[T]`:
///         pub fn print(data: &[u8]) {
///             println!("{data:?}");
///         }
///     }
/// }
/// ```
#[repr(C)]
pub struct DiplomatSlice<'a, T> {
    // Invariant: ptr is a valid ptr to the beginning of an &[T] allocation. It may be null if len is null
    ptr: *const T,
    // Invariant: the allocation contains at least `len` elements
    len: usize,
    phantom: PhantomData<&'a [T]>,
}

impl<T> Clone for DiplomatSlice<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for DiplomatSlice<'_, T> {}

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

impl<T> Deref for DiplomatSlice<'_, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        (*self).into()
    }
}

/// This is equivalent to `&mut [T]`, except it has a stable `repr(C)` layout.
///
/// In `#[diplomat::bridge]` function signatures, prefer using standard `&mut [T]` instead.
/// Use `DiplomatSliceMut<'a, T>` explicitly when storing a mutable slice inside a Diplomat struct.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     pub struct MyStruct<'a> {
///         pub data: DiplomatSliceMut<'a, u8>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn fill_zeros(mut self) {
///             self.data.fill(0);
///         }
///     }
///
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // In function signatures, prefer `&mut [T]`:
///         pub fn fill_zeros(data: &mut [u8]) {
///             data.fill(0);
///         }
///     }
/// }
/// ```
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

impl<T> Deref for DiplomatSliceMut<'_, T> {
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

impl<T> DerefMut for DiplomatSliceMut<'_, T> {
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

/// This is equivalent to `Box<[T]>`, except it has a stable `repr(C)` layout.
///
/// This type is intended to be accepted as an input argument in Diplomat functions
/// when a garbage-collected backend like JS or Dart needs to allocate a new slice
/// in Rust memory to pass data across FFI *anyway*, so Rust can take ownership of
/// that allocation instead of copying it again. (For backends that share memory like
/// C or C++, accepting a borrowed `&[T]` suffices.)
///
/// In `#[diplomat::bridge]` function inputs, standard `Box<[T]>` suffices and should
/// be preferred. Owned slices cannot currently be used in Diplomat structs.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // Preferred: use `Box<[T]>` in function inputs
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept(data: Box<[u8]>) {
///             println!("{data:?}");
///         }
///
///         // Equivalent without sugar:
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept_explicit(data: DiplomatOwnedSlice<u8>) {
///             println!("{:?}", &*data);
///         }
///     }
/// }
/// ```
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

/// This is equivalent to `&str`, except it has a stable `repr(C)` layout.
///
/// In `#[diplomat::bridge]` function signatures, prefer using `&str` instead (or
/// [`&DiplomatStr`](crate::DiplomatStr) if UTF-8 validation is not required).
/// Use `DiplomatUtf8StrSlice<'a>` explicitly when storing a validated UTF-8 string
/// slice inside a Diplomat struct, or as the element type of `&[DiplomatUtf8StrSlice]`.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     pub struct MyStruct<'a> {
///         pub text: DiplomatUtf8StrSlice<'a>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn print(self) {
///             println!("{}", &*self.text);
///         }
///     }
///
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // In function signatures, prefer `&str`:
///         pub fn print(text: &str) {
///             println!("{text}");
///         }
///     }
/// }
/// ```
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

impl Deref for DiplomatUtf8StrSlice<'_> {
    type Target = str;
    fn deref(&self) -> &str {
        (*self).into()
    }
}

/// This is equivalent to `Box<str>`, except it has a stable `repr(C)` layout.
///
/// This type is intended to be accepted as an input argument in Diplomat functions
/// when a garbage-collected backend like JS or Dart needs to allocate a new buffer
/// in Rust memory to pass a string across FFI *anyway*, so Rust can take ownership
/// of that allocation instead of copying it again. (When an owned allocation is not
/// needed, accepting a borrowed `&str` suffices.)
///
/// In `#[diplomat::bridge]` function inputs, standard `Box<str>` suffices and should
/// be preferred. Owned string slices cannot currently be used in Diplomat structs.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // Preferred: use `Box<str>` in function inputs
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept(text: Box<str>) {
///             println!("{text}");
///         }
///
///         // Equivalent without sugar:
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept_explicit(text: DiplomatOwnedUtf8StrSlice) {
///             println!("{}", &*text);
///         }
///     }
/// }
/// ```
// Safety invariant: contained slice must be valid UTF-8
#[repr(transparent)]
pub struct DiplomatOwnedUtf8StrSlice(DiplomatOwnedSlice<u8>);

impl From<Box<str>> for DiplomatOwnedUtf8StrSlice {
    fn from(x: Box<str>) -> Self {
        // Safety: invariant upheld; obtained from `str`
        Self(Box::<[u8]>::from(x).into())
    }
}

impl From<DiplomatOwnedUtf8StrSlice> for Box<str> {
    fn from(x: DiplomatOwnedUtf8StrSlice) -> Self {
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
            // We can assume this because of the invariant on DiplomatOwnedUtf8StrSlice
            let strslice = core::str::from_utf8_unchecked_mut(slice);
            Box::from_raw(strslice as *mut str)
        }
    }
}

impl Deref for DiplomatOwnedUtf8StrSlice {
    type Target = str;
    fn deref(&self) -> &str {
        let slice = &self.0;
        unsafe {
            // We can assume this because of the invariant on DiplomatOwnedUtf8StrSlice
            core::str::from_utf8_unchecked(slice)
        }
    }
}

/// This is like `&str`, but unvalidated and safe to use over FFI.
///
/// This type will usually map to some string type in the target language, and
/// you will not need to worry about the safety of mismatched string invariants.
///
/// In `#[diplomat::bridge]` function inputs, prefer using [`&DiplomatStr`](crate::DiplomatStr)
/// instead (or standard `&str` if UTF-8 validation is acceptable). Use `DiplomatStrSlice<'a>`
/// explicitly when storing an unvalidated UTF-8 string slice inside a Diplomat struct, or as
/// the element type of `&[DiplomatStrSlice]`.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     pub struct MyStruct<'a> {
///         pub text: DiplomatStrSlice<'a>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn print(self) {
///             println!("{:?}", &*self.text);
///         }
///     }
///
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // In function inputs, prefer `&DiplomatStr`:
///         pub fn print(text: &DiplomatStr) {
///             println!("{text:?}");
///         }
///     }
/// }
/// ```
pub type DiplomatStrSlice<'a> = DiplomatSlice<'a, u8>;

/// This is like `Box<str>`, but unvalidated and safe to use over FFI.
///
/// This type is intended to be accepted as an input argument in Diplomat functions
/// when a garbage-collected backend like JS or Dart needs to allocate a new buffer
/// in Rust memory to pass a string across FFI *anyway*, so Rust can take ownership
/// of that allocation instead of copying it again. (When an owned allocation is not
/// needed, accepting [`&DiplomatStr`](crate::DiplomatStr) or `&str` suffices.)
///
/// In `#[diplomat::bridge]` function inputs, [`Box<DiplomatStr>`](crate::DiplomatStr)
/// suffices and should be preferred. Owned string slices cannot currently be used in
/// Diplomat structs.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // Preferred: use `Box<DiplomatStr>` in function inputs
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept(text: Box<DiplomatStr>) {
///             println!("{text:?}");
///         }
///
///         // Equivalent without sugar:
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept_explicit(text: DiplomatOwnedStrSlice) {
///             println!("{:?}", &*text);
///         }
///     }
/// }
/// ```
pub type DiplomatOwnedStrSlice = DiplomatOwnedSlice<u8>;

/// An unvalidated UTF-16 string that is safe to use over FFI.
///
/// This type will usually map to some string type in the target language, and
/// you will not need to worry about the safety of mismatched string invariants.
///
/// In `#[diplomat::bridge]` function inputs, prefer using [`&DiplomatStr16`](crate::DiplomatStr16)
/// instead. Use `DiplomatStr16Slice<'a>` explicitly when storing an unvalidated UTF-16 string
/// slice inside a Diplomat struct, or as the element type of `&[DiplomatStr16Slice]`.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     pub struct MyStruct<'a> {
///         pub text: DiplomatStr16Slice<'a>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn print(self) {
///             println!("{:?}", &*self.text);
///         }
///     }
///
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // In function inputs, prefer `&DiplomatStr16`:
///         pub fn print(text: &DiplomatStr16) {
///             println!("{text:?}");
///         }
///     }
/// }
/// ```
pub type DiplomatStr16Slice<'a> = DiplomatSlice<'a, u16>;

/// An unvalidated, owned UTF-16 string that is safe to use over FFI.
///
/// This type is intended to be accepted as an input argument in Diplomat functions
/// when a garbage-collected backend like JS or Dart needs to allocate a new buffer
/// in Rust memory to pass a UTF-16 string across FFI *anyway*, so Rust can take
/// ownership of that allocation instead of copying it again. (When an owned allocation
/// is not needed, accepting [`&DiplomatStr16`](crate::DiplomatStr16) suffices.)
///
/// In `#[diplomat::bridge]` function inputs, [`Box<DiplomatStr16>`](crate::DiplomatStr16)
/// suffices and should be preferred. Owned string slices cannot currently be used in
/// Diplomat structs.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // Preferred: use `Box<DiplomatStr16>` in function inputs
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept(text: Box<DiplomatStr16>) {
///             println!("{text:?}");
///         }
///
///         // Equivalent without sugar:
///         #[diplomat::cfg(supports = owned_slices)]
///         pub fn accept_explicit(text: DiplomatOwnedStr16Slice) {
///             println!("{:?}", &*text);
///         }
///     }
/// }
/// ```
pub type DiplomatOwnedStr16Slice<'a> = DiplomatOwnedSlice<u16>;
