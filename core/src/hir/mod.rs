//! Experimental high-level representation (HIR) for Diplomat.

mod defs;
mod elision;
mod lifetimes;
mod lowering;
mod methods;
mod paths;
mod primitives;
mod type_context;
mod types;
pub use defs::*;
pub(super) use elision::*;
pub use lifetimes::*;
pub(super) use lowering::*;
pub use methods::*;
pub use paths::*;
pub use primitives::*;
pub use type_context::*;
pub use types::*;

pub use crate::ast::Docs;
use strck_ident::rust::{Ident, IdentBuf};
