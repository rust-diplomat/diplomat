use super::method::MethodInfo;
use super::ItemGenContext;
use askama::Template;
use diplomat_core::hir;
use std::borrow::Cow;

#[derive(Template)]
#[template(path = "rust/opaque.rs.jinja", escape = "none")]
struct OpaqueTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
    dtor_abi_name: &'a str,
    methods: Vec<MethodInfo>,
}

impl ItemGenContext<'_, '_> {
    pub fn gen_opaque(&self, id: hir::OpaqueId) -> String {
        let def = self.tcx.resolve_opaque(id);
        OpaqueTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs),
            dtor_abi_name: def.dtor_abi_name.as_str(),
            methods: self.gen_methods(id.into(), &def.methods),
        }
        .to_string()
    }
}
