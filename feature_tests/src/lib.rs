//! Various integration tests for Diplomat features

// We're not trying to write good code here, just tests
#![allow(clippy::style, dead_code)]
// Diplomat limitations
#![allow(
    clippy::needless_lifetimes,
    clippy::result_unit_err,
    clippy::should_implement_trait
)]

#[diplomat::config(lib_name = "this is also a test value, see below")]
struct Config;

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

// Feel free to add overrides for other languages for the lib_name, this is meant to showcase overriding SharedConfig.
#[diplomat::config(kotlin.lib_name = somelib)]
#[diplomat::config(nanobind.lib_name = somelib)]
struct KotlinConfig;
