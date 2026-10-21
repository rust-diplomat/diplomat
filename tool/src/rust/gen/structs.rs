use super::method::MethodInfo;
use super::ItemGenContext;
use askama::Template;
use diplomat_core::hir::TyPosition;
use std::borrow::Cow;

#[derive(Template)]
#[template(path = "rust/struct.rs.jinja", escape = "none")]
struct StructTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
    fields: Vec<StructField>,
    methods: Vec<MethodInfo>,
}

struct StructField {
    name: String,
    ty: String,
    docs: String,
}

impl ItemGenContext<'_, '_> {
    pub fn gen_struct<P: TyPosition>(&self, id: P::StructId) -> String {
        let def = P::resolve_struct(self.tcx, id);
        let fields = def
            .fields
            .iter()
            .map(|field| {
                let ty = self.gen_value_type(&field.ty).unwrap_or_else(|| {
                    self.errors.push_error(format!(
                        "Unsupported type for struct field `{}`",
                        field.name
                    ));
                    String::new()
                });
                StructField {
                    name: field.name.to_string(),
                    ty,
                    docs: self.formatter.fmt_docs(&field.docs),
                }
            })
            .collect();
        StructTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs),
            fields,
            methods: self.gen_methods(id.into(), &def.methods),
        }
        .to_string()
    }
}
