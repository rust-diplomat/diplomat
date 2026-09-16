//! Safe Rust bindings generated over a Diplomat native ABI.
//!
//! The unsafe ABI layer lives in the private `ffi` module; the public API is
//! re-exported from this crate root.

#![allow(clippy::needless_lifetimes)]
#![allow(clippy::new_without_default)]

mod ffi;
mod opaques;
mod private;
mod types;

pub use opaques::*;
pub use types::*;
