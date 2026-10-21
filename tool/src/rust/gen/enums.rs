use super::method::MethodInfo;
use super::ItemGenContext;
use askama::Template;
use diplomat_core::hir;
use std::borrow::Cow;

#[derive(Template)]
#[template(path = "rust/enum.rs.jinja", escape = "none")]
struct EnumTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
    variants: Vec<EnumVariant>,
    methods: Vec<MethodInfo>,
}

struct EnumVariant {
    name: String,
    discriminant: isize,
    docs: String,
}

impl ItemGenContext<'_, '_> {
    pub fn gen_enum(&self, id: hir::EnumId) -> String {
        let def = self.tcx.resolve_enum(id);
        EnumTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs),
            variants: def
                .variants
                .iter()
                .map(|v| EnumVariant {
                    name: v.name.to_string(),
                    discriminant: v.discriminant,
                    docs: self.formatter.fmt_docs(&v.docs),
                })
                .collect(),
            methods: self.gen_methods(id.into(), &def.methods),
        }
        .to_string()
    }
}
