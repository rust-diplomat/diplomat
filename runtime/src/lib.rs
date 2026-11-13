#![no_std]

//! The [Diplomat](https://rust-diplomat.github.io/diplomat/) runtime crate.
//!
//! This crate provides Diplomat-specific types for crates writing `#[diplomat::bridge]` modules.
//! Include this in any crate that also depends on `diplomat`, since `#[diplomat::bridge]`
//! will generate code that relies on types from here.
//!
//! # Type Overview
//!
//! - **Option and Result**: [`DiplomatResult<T, E>`] (FFI-safe `Result`) and [`DiplomatOption<T>`] (type alias for [`DiplomatResult<T, ()>`]).
//! - **Slices**: [`DiplomatSlice<'a, T>`] (`&'a [T]`), [`DiplomatSliceMut<'a, T>`] (`&'a mut [T]`), and [`DiplomatOwnedSlice<T>`] (`Box<[T]>`).
//! - **String input**: These all map to appropriate string types across FFI.
//!    - Unvalidated UTF-8: [`DiplomatStrSlice<'a>`] (`&'a [u8]`), [`DiplomatOwnedStrSlice`] (`Box<[u8]>`)
//!    - Unvalidated UTF-16: [`DiplomatStr16Slice<'a>`] (`&'a [u16]`), [`DiplomatOwnedStr16Slice`]  (`Box<[u16]>`)
//!    - Validated UTF-8: [`DiplomatUtf8StrSlice<'a>`] (`&'a str`), [`DiplomatOwnedUtf8StrSlice`] (`Box<str>`)
//!    - Unvalidated stringy DSTs: [`DiplomatStr`] (`[u8]`), [`DiplomatStr16`] (`[u16]`), for convenient use with `&` and `Box` as function parameters.
//! - **String output**: [`DiplomatWrite`] and [`DiplomatWrite16`]
//! - **Primitive buffer output**: [`DiplomatWriteGeneric<T>`]
//! - **Callbacks**: [`support::Callback<ReturnType>`] (FFI-safe callback handle).
//! - **Scalars**: [`DiplomatChar`] (`u32`) and [`DiplomatByte`] (`u8`).
//!
//! Note that many of these are type aliases; using these types instead of the type alias signals that Diplomat is expected to
//! generate the equivalent type on the other side. For example, using `DiplomatChar` will signal that a language's `char` type is
//! to be used, even if the actual type seen by rust is an unvalidated `u32`.
//!
//! In `#[diplomat::bridge]` function signatures, standard Rust types ([`Result<T, E>`], [`Option<T>`],
//! `&[T]`, `&mut [T]`, `Box<[T]>`, `&str`, and `Box<str>`) suffice: Diplomat will automatically
//! convert them to their FFI-safe `diplomat_runtime` equivalents. The `Diplomat*` slice and option
//! types only need to be written explicitly when used inside Diplomat structs (or in slices of strings).
//!
//! # Features
//!
//! The `log` feature enables logging support, currently enabled via the wasm-only `diplomat_init()`.
//!
//! The `jvm-callback-support` feature should be enabled if building Diplomat for use in the JVM, for
//! a Diplomat-based library that uses callbacks.

extern crate alloc;

use alloc::alloc::Layout;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm_glue;

mod write;
pub use write::{
    diplomat_array_write_create, diplomat_buffer_write_create, diplomat_buffer_write_destroy,
    DiplomatWrite, DiplomatWrite16, DiplomatWriteGeneric,
};
mod slices;
pub use slices::{
    DiplomatOwnedSlice, DiplomatOwnedStr16Slice, DiplomatOwnedStrSlice, DiplomatOwnedUtf8StrSlice,
    DiplomatSlice, DiplomatSliceMut, DiplomatStr16Slice, DiplomatStrSlice, DiplomatUtf8StrSlice,
};

// TODO(#958) remove this before 1.0
#[doc(hidden)]
#[deprecated(note = "use DiplomatOwnedUtf8StrSlice instead")]
pub use DiplomatOwnedUtf8StrSlice as DiplomatOwnedUTF8StrSlice;
mod callback;

/// Types that Diplomat uses internally
///
/// Users should not need to reference these directly.
pub mod support {
    pub use crate::callback::Callback;
    pub use crate::write::AbiCompatibleForWrite;

    /// 🔌 \[C API\] Frees a `Box<[u8]>` that was returned across FFI as a raw `(ptr, len)`
    /// pair (e.g. via `DiplomatOwnedSlice<u8>`).
    ///
    /// Currently this is used by the `.NET` backend when Rust returns `Box<[u8]>` across FFI. Other
    /// backends may eventually use this as well for highly efficient byte slice transfer.
    ///
    /// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
    ///
    /// # Safety
    /// - `ptr`/`len` must be the raw parts of a `Box<[u8]>` that Rust allocated and handed across
    ///   FFI; this reconstructs that box and drops it, so the same allocator that made it frees it.
    /// - Must not be called more than once for the same `ptr`.
    #[no_mangle]
    pub unsafe extern "C" fn diplomat_owned_slice_u8_destroy(ptr: *mut u8, len: usize) {
        if !ptr.is_null() {
            drop(alloc::boxed::Box::from_raw(
                core::ptr::slice_from_raw_parts_mut(ptr, len),
            ));
        }
    }

    /// 🔌 \[C API\] Whether a `&[u8]` is a `&str`.
    ///
    /// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
    ///
    /// # Safety
    /// - `ptr` and `size` must be a valid `&[u8]`
    #[no_mangle]
    pub unsafe extern "C" fn diplomat_is_utf8_str(ptr: *const u8, size: usize) -> bool {
        core::str::from_utf8(core::slice::from_raw_parts(ptr, size)).is_ok()
    }
}

// TODO(#958) remove these before 1.0
#[doc(hidden)]
#[deprecated(note = "use support::diplomat_is_utf8_str instead")]
pub use support::diplomat_is_utf8_str as diplomat_is_str;
#[doc(hidden)]
#[deprecated(note = "use support::diplomat_owned_slice_u8_destroy instead")]
pub use support::diplomat_owned_slice_u8_destroy;
#[doc(hidden)]
#[deprecated(note = "use support::AbiCompatibleForWrite instead")]
pub use support::AbiCompatibleForWrite as DiplomatAbiCompatible;
#[doc(hidden)]
#[deprecated(note = "use support::Callback instead")]
pub use support::Callback as DiplomatCallback;
#[doc(hidden)]
#[deprecated(note = "use support::diplomat_array_write_create instead")]
pub unsafe extern "C" fn diplomat_simple_write(buf: *mut u8, buf_size: usize) -> DiplomatWrite {
    diplomat_array_write_create(buf, buf_size)
}

mod result;
pub use result::{DiplomatOption, DiplomatResult};

pub mod rust_interop;

/// Like [`char`], but unvalidated.
///
/// Rust's [`char`] has validity invariants (it must be a valid Unicode scalar value)
/// that are not safe to assume across FFI. Diplomat users can use `DiplomatChar` (`u32`)
/// to avoid validation overhead across FFI while still mapping to a character type in
/// the target language.
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
///         pub fn print_char(ch: DiplomatChar) {
///             println!("{:?}", char::from_u32(ch));
///         }
///     }
/// }
/// ```
pub type DiplomatChar = u32;

/// Like [`str`], but unvalidated.
///
/// This is a dynamically sized type (`[u8]`) meant to be used behind `&` or `Box<T>`
/// in `#[diplomat::bridge]` function inputs (`&DiplomatStr` or `Box<DiplomatStr>`).
/// It maps to the target language's string type without requiring UTF-8 validation
/// (if validated UTF-8 is acceptable, standard `&str` or `Box<str>` suffices).
///
/// Because `&DiplomatStr` is not `repr(C)`, it cannot be placed directly in a
/// Diplomat struct; use [`DiplomatStrSlice`] in structs (or in slices of strings)
/// instead.
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
///         pub fn print(text: &DiplomatStr) {
///             println!("{text:?}");
///         }
///     }
///
///     // In structs, use `DiplomatStrSlice`:
///     pub struct MyStruct<'a> {
///         pub text: DiplomatStrSlice<'a>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn print(self) {
///             println!("{:?}", &*self.text);
///         }
///     }
/// }
/// ```
pub type DiplomatStr = [u8];

/// Like `Wstr`, but unvalidated.
///
/// This is a dynamically sized type (`[u16]`) meant to be used behind `&` or `Box<T>`
/// in `#[diplomat::bridge]` function inputs (`&DiplomatStr16` or `Box<DiplomatStr16>`).
/// It maps to the target language's string type without requiring UTF-16 validation.
///
/// Because `&DiplomatStr16` is not `repr(C)`, it cannot be placed directly in a
/// Diplomat struct; use [`DiplomatStr16Slice`] in structs (or in slices of strings)
/// instead.
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
///         pub fn print(text: &DiplomatStr16) {
///             println!("{text:?}");
///         }
///     }
///
///     // In structs, use `DiplomatStr16Slice`:
///     pub struct MyStruct<'a> {
///         pub text: DiplomatStr16Slice<'a>,
///     }
///
///     impl<'a> MyStruct<'a> {
///         pub fn print(self) {
///             println!("{:?}", &*self.text);
///         }
///     }
/// }
/// ```
pub type DiplomatStr16 = [u16];

/// Like [`u8`], but interpreted explicitly as a raw byte as opposed to a numerical value.
///
/// This matters for languages like JavaScript or Dart, where there's only a single numeric
/// type, but special types for byte buffers. When a regular numerical 8-bit integer
/// suffices, use standard [`u8`] instead.
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
///         pub fn print_bytes(bytes: &[DiplomatByte]) {
///             println!("{bytes:?}");
///         }
///     }
/// }
/// ```
pub type DiplomatByte = u8;

/// 🔌 \[C API\] Allocates a buffer of a given size in Rust's memory.
///
/// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
///
/// # Safety
/// - The allocated buffer must be freed with [`diplomat_free()`].
#[no_mangle]
pub unsafe extern "C" fn diplomat_alloc(size: usize, align: usize) -> *mut u8 {
    alloc::alloc::alloc(Layout::from_size_align(size, align).unwrap())
}

/// 🔌 \[C API\] Frees a buffer that was allocated in Rust's memory.
///
/// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
///
/// # Safety
/// - `ptr` must be a pointer to a valid buffer allocated by [`diplomat_alloc()`].
#[no_mangle]
pub unsafe extern "C" fn diplomat_free(ptr: *mut u8, size: usize, align: usize) {
    alloc::alloc::dealloc(ptr, Layout::from_size_align(size, align).unwrap())
}
