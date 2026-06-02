//! Type-leaf lowering.
//!
//! Pure functions that map a single HIR type fragment to its **bare** C#
//! name (e.g. `"Color"`, `"Point2D"`). Callers compose the form they need:
//! append `"*"` for the FFI pointer, prefix `"Raw."` for the raw namespace.
//!
//! Shared across opaque / struct / enum codegen.

use diplomat_core::hir::{
    Borrow, EnumPath, MaybeOwn, OpaquePath, Optional, ReturnableStructPath, StructPath,
};

use super::ItemGenContext;

impl<'ctx, 'tcx> ItemGenContext<'ctx, 'tcx> {
    /// Bare C# name of an *owned* opaque (`Box<T>` in return position).
    /// Returns `"Color"`; append `"*"` for the FFI pointer form. Routes
    /// through `fmt_type_name` so `#[diplomat::rename]` attrs and C#
    /// keyword collisions are handled.
    pub(super) fn opaque_name(&self, opaque_path: &OpaquePath<Optional, MaybeOwn>) -> String {
        self.formatter
            .fmt_type_name(opaque_path.tcx_id.into())
            .into_owned()
    }

    pub(super) fn enum_name(&self, enum_path: &EnumPath) -> String {
        self.formatter
            .fmt_type_name(enum_path.tcx_id.into())
            .into_owned()
    }

    pub(super) fn returnable_struct_name(
        &self,
        struct_path: &ReturnableStructPath,
    ) -> Option<String> {
        Some(match struct_path {
            ReturnableStructPath::Struct(p) => {
                self.formatter.fmt_type_name(p.tcx_id.into()).into_owned()
            }
            ReturnableStructPath::OutStruct(p) => {
                let name = self.formatter.fmt_type_name(p.tcx_id.into());
                self.errors.push_error(format!(
                    "[.NET backend] out struct (`#[diplomat::out] struct {name}`) return is \
                     not yet supported"
                ));
                return None;
            }
            _ => {
                self.errors.push_error(
                    "[.NET backend] unsupported struct return type".to_string(),
                );
                return None;
            }
        })
    }

    /// Bare C# name of a *borrowed* opaque (`&T` / `&mut T`). Returns `"Color"`;
    /// callers append `"*"` for the FFI pointer form on params and self.
    pub(super) fn opaque_name_borrowed<O>(&self, opaque_path: &OpaquePath<O, Borrow>) -> String {
        self.formatter
            .fmt_type_name(opaque_path.tcx_id.into())
            .into_owned()
    }

    /// Bare C# name of a struct path (e.g. `"Point2D"`). Used for struct-self
    /// params; the raw extern takes the struct by value.
    pub(super) fn struct_name(&self, struct_path: &StructPath) -> String {
        self.formatter
            .fmt_type_name(struct_path.tcx_id.into())
            .into_owned()
    }
}
