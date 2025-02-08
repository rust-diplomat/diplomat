//! Various integration tests for Diplomat features

// We're not trying to write good code here, just tests
#![allow(clippy::style, dead_code)]
// Diplomat limitations
#![allow(
    clippy::needless_lifetimes,
    clippy::result_unit_err,
    clippy::should_implement_trait
)]

#[diplomat::bridge]
mod ffi {
    #[diplomat::config(lib_name = somelib)]
    struct Dummy;
}

extern crate alloc;

pub mod attrs;
pub mod callbacks;
pub mod imports;
pub mod lifetimes;
pub mod option;
pub mod result;
pub mod selftype;
pub mod slices;
pub mod structs;
pub mod traits;
