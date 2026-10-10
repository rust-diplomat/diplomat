use super::formatter::RustFormatter;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::{self, TyPosition, TypeContext};
use std::borrow::Cow;

#[derive(Template)]
#[template(path = "rust/enum.rs.jinja", escape = "none")]
struct EnumTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
}

#[derive(Template)]
#[template(path = "rust/opaque.rs.jinja", escape = "none")]
struct OpaqueTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
}

#[derive(Template)]
#[template(path = "rust/struct.rs.jinja", escape = "none")]
struct StructTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
}

/// The context used for generating a particular type
pub(super) struct ItemGenContext<'cx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'cx RustFormatter<'tcx>,
    #[allow(dead_code)] // Used once types with fallible codegen are supported
    pub errors: &'cx ErrorStore<'tcx, String>,
}

impl ItemGenContext<'_, '_> {
    pub fn gen_enum(&self, id: hir::EnumId) -> String {
        let def = self.tcx.resolve_enum(id);
        EnumTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs),
        }
        .to_string()
    }

    pub fn gen_opaque(&self, id: hir::OpaqueId) -> String {
        let def = self.tcx.resolve_opaque(id);
        OpaqueTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs),
        }
        .to_string()
    }

    pub fn gen_struct<P: TyPosition>(&self, id: P::StructId) -> String {
        let def = P::resolve_struct(self.tcx, id);
        StructTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs),
        }
        .to_string()
    }
}
