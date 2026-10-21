//! Rust code generation, one module per kind of type:
//!
//! * [`enums`], [`structs`] and [`opaque`] render a single type definition.
//! * [`method`] lowers the methods all three share into their safe wrapper and
//!   `extern "C"` declaration.
//! * This file holds [`ItemGenContext`] and the type lowering the others share.

use super::formatter::RustFormatter;
use crate::ErrorStore;
use diplomat_core::hir::{PrimitiveType, StructPathLike, TyPosition, Type, TypeContext, TypeId};

mod enums;
mod method;
mod opaque;
mod structs;

/// The context used for generating a particular type
pub(super) struct ItemGenContext<'cx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'cx RustFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
}

impl ItemGenContext<'_, '_> {
    /// Types that cross the ABI by value with the same layout on both sides
    fn gen_value_type<P: TyPosition>(&self, ty: &Type<P>) -> Option<String> {
        match ty {
            Type::Primitive(prim) => Self::gen_primitive(prim).map(String::from),
            Type::Enum(path) => Some(self.fmt_type_path(path.tcx_id.into())),
            Type::Struct(path) => Some(self.fmt_type_path(path.id())),
            _ => None,
        }
    }

    fn gen_primitive(prim: &PrimitiveType) -> Option<&'static str> {
        match prim {
            PrimitiveType::Bool
            | PrimitiveType::Int(_)
            | PrimitiveType::IntSize(_)
            | PrimitiveType::Float(_) => Some(prim.as_str()),
            PrimitiveType::Byte => Some("u8"),
            // `char` and orderings need validation before they become Rust values
            _ => None,
        }
    }

    /// Types are re-exported from the crate root
    fn fmt_type_path(&self, id: TypeId) -> String {
        format!("crate::{}", self.formatter.fmt_type_name(id))
    }

    /// The type that opaque pointers point to at the ABI level
    fn fmt_opaque_raw(&self, id: TypeId) -> String {
        format!(
            "crate::{}::{}Raw",
            self.formatter.fmt_module_name(id),
            self.formatter.fmt_type_name(id)
        )
    }
}
