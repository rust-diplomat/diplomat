//! Safe downstream consumer of the generated bindings.
//!
//! This crate depends only on `diplomat-rust-backend-generated` and never links
//! the provider implementation. Its integration tests exercise the generated
//! safe API with `#![forbid(unsafe_code)]`:
//!
//! - `tests/runtime.rs` — runtime behaviour and ownership/destruction.
//! - `tests/compile_fail.rs` — unsound uses must fail to compile.
#![forbid(unsafe_code)]
