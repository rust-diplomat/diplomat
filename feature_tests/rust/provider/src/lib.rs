//! Owner-side cdylib for the Safe Rust backend fixture.
//!
//! The generated consumer is a separate Cargo package and has no dependency
//! on this crate. All allocation and destruction stay in this owner.

#![allow(clippy::needless_lifetimes)]

use std::sync::atomic::AtomicUsize;

static COUNTER_DROPS: AtomicUsize = AtomicUsize::new(0);
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// The bridge module lives in its own file rather than inline in this one, which is
/// the layout every other backend's provider uses: `feature_tests/src/lib.rs`
/// declares `pub mod X;` and each `X.rs` holds its `#[diplomat::bridge]` module.
///
/// It matters for diagnostics. For a module declared inline in the *entry* file the
/// tool records the entry file's parent directory as the module's source location,
/// so a rejected shape can only be reported with its message and not with a file,
/// line and excerpt. Declaring it as `<name>.rs` gives the module a real file to
/// point at, exactly as the inline module inside `X.rs` gets `X.rs`.
pub mod bridge;
