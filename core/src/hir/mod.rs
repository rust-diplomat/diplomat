//! Experimental high-level representation (HIR) for Diplomat.
//!
//! Enabled with the `"hir"` Cargo feature

mod attrs;
mod defs;
mod elision;
// We don't reexport this for two reasons.
// 
// One is that these are somewhat more niche types and we don't want to clutter the main module too
// much. You only need these if you're dealing with lifetimes, which not all backends may wish to
// do.
//
// Two is that this module contains types named Type and Method which will conflict with others.
pub mod lifetimes;
mod lowering;
mod methods;
mod paths;
mod primitives;
mod ty_position;
mod type_context;
mod types;
pub use attrs::*;
pub use defs::*;
pub(super) use elision::*;
pub(super) use lowering::*;
pub use methods::*;
pub use paths::*;
pub use primitives::*;
pub use ty_position::*;
pub use type_context::*;
pub use types::*;

pub use lowering::LoweringError;

pub use crate::ast::Docs;
pub use strck_ident::rust::{Ident, IdentBuf};
