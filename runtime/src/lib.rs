#![cfg_attr(not(any(target_arch = "wasm32")), no_std)]

extern crate alloc;

use alloc::vec::Vec;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;

mod writeable;
pub use writeable::DiplomatWriteable;

mod result;
pub use result::DiplomatResult;

/// Allocates a buffer of a given size in Rust's memory.
///
/// # Safety
/// - The allocated buffer must be freed with [`diplomat_free()`].
#[no_mangle]
pub unsafe extern "C" fn diplomat_alloc(size: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(size);
    let ret = vec.as_mut_ptr();
    core::mem::forget(vec);
    ret
}

/// Frees a buffer that was allocated in Rust's memory.
/// # Safety
/// - `ptr` must be a pointer to a valid buffer allocated by [`diplomat_alloc()`].
#[no_mangle]
pub unsafe extern "C" fn diplomat_free(ptr: *mut u8, size: usize) {
    let vec = Vec::from_raw_parts(ptr, size, size);
    drop(vec);
}
