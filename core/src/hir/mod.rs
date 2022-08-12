//! Experiment high-level representation (HIR) for Diplomat.

#![allow(unused_mut, unused_variables, dead_code)]
mod defs;
mod error;
mod lifetimes;
mod methods;
mod paths;
mod primitives;
mod type_context;
mod types;
pub use defs::*;
pub use error::*;
pub use lifetimes::*;
pub use methods::*;
pub use paths::*;
pub use primitives::*;
pub use type_context::*;
pub use types::*;

pub use crate::ast::Docs;
use strck_ident::rust::{Ident, IdentBuf};
