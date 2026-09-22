//! Rendering of generated enums and value structs through Askama templates.
//!
//! Public value types are intentionally emitted one file per type. The module
//! index is rendered separately and re-exports the generated items flat from
//! the crate root.

use std::collections::BTreeSet;

use crate::r#rust::formatter::{
    enum_variant_name, field_name, render_docs, type_def_name, type_module_name,
};
use crate::r#rust::gen::method::render_method;
use crate::r#rust::lifetimes::{struct_generics, struct_lifetime_phantom};
use crate::r#rust::type_map::{safe_struct_field_type, safe_value_type, struct_needs_abi_mirror};
use askama::Template;
use diplomat_core::hir::{
    self, DocsUrlGenerator, Slice, StructPathLike, Type, TypeContext, TypeDef,
};

#[derive(Template)]
#[template(path = "rust/types_mod.rs.jinja", escape = "none")]
struct TypesModTemplate<'a> {
    modules: &'a [String],
    has_modules: bool,
}

#[derive(Template)]
#[template(path = "rust/enum.rs.jinja", escape = "none")]
struct EnumTemplate {
    docs: String,
    imports: Vec<String>,
    has_ffi: bool,
    name: String,
    variants: Vec<EnumVariantView>,
    methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "rust/struct.rs.jinja", escape = "none")]
struct StructTemplate {
    docs: String,
    imports: Vec<String>,
    has_ffi: bool,
    has_phantom: bool,
    name: String,
    generics: String,
    args: String,
    needs_abi_mirror: bool,
    derives_eq: bool,
    fields: Vec<FieldView>,
    phantom: String,
    methods: Vec<String>,
}

struct EnumVariantView {
    docs: String,
    name: String,
    discriminant: String,
}

struct FieldView {
    docs: String,
    name: String,
    ty: String,
}

/// Generate `types/mod.rs` and one source file for every enabled enum/struct.
pub(crate) fn generate_type_files(
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) -> Vec<(String, String)> {
    let mut files = Vec::new();
    let mut modules = Vec::new();

    for enm in tcx.enums().iter().filter(|ty| !ty.attrs.disable) {
        let module = type_module_name(TypeDef::Enum(enm));
        modules.push(module.clone());
        files.push((
            format!("src/types/{module}.rs"),
            render_enum(enm, tcx, docs_url_gen),
        ));
    }
    for strct in tcx.structs().iter().filter(|ty| !ty.attrs.disable) {
        let module = type_module_name(TypeDef::Struct(strct));
        modules.push(module.clone());
        files.push((
            format!("src/types/{module}.rs"),
            render_struct(strct, tcx, docs_url_gen),
        ));
    }

    let index = TypesModTemplate {
        modules: &modules,
        has_modules: !modules.is_empty(),
    }
    .render()
    .expect("Rust types module template rendering cannot fail");
    files.insert(0, ("src/types/mod.rs".into(), index));
    files
}

fn render_enum(enm: &hir::EnumDef, tcx: &TypeContext, docs_url_gen: &DocsUrlGenerator) -> String {
    let name = type_def_name(TypeDef::Enum(enm));
    let methods = enm
        .methods
        .iter()
        .filter(|method| !method.attrs.disable)
        .map(|method| render_method(0, method, tcx, docs_url_gen))
        .collect::<Vec<_>>();
    let imports = referenced_names_for_methods(&enm.methods, tcx, Some(&name));
    let variants = enm
        .variants
        .iter()
        .map(|variant| EnumVariantView {
            docs: render_docs(&variant.docs, docs_url_gen, "    "),
            name: enum_variant_name(variant),
            discriminant: variant.discriminant.to_string(),
        })
        .collect();

    EnumTemplate {
        docs: render_docs(&enm.docs, docs_url_gen, ""),
        imports,
        has_ffi: !methods.is_empty(),
        name,
        variants,
        methods,
    }
    .render()
    .expect("Rust enum template rendering cannot fail")
}

fn render_struct(
    strct: &hir::StructDef,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) -> String {
    let name = type_def_name(TypeDef::Struct(strct));
    let methods = strct
        .methods
        .iter()
        .filter(|method| !method.attrs.disable)
        .map(|method| render_method(strct.lifetimes.num_lifetimes(), method, tcx, docs_url_gen))
        .collect::<Vec<_>>();
    let fields = strct
        .fields
        .iter()
        .map(|field| FieldView {
            docs: render_docs(&field.docs, docs_url_gen, "    "),
            name: field_name(field),
            ty: if struct_needs_abi_mirror(strct, tcx) {
                safe_struct_field_type(&field.ty, strct, tcx)
            } else {
                safe_value_type(&field.ty, tcx)
            },
        })
        .collect();
    let phantom = struct_lifetime_phantom(strct);
    let (generics, args) = struct_generics(strct);
    let imports = referenced_names_for_struct(strct, tcx, &name);

    StructTemplate {
        docs: render_docs(&strct.docs, docs_url_gen, ""),
        imports,
        has_ffi: !methods.is_empty(),
        has_phantom: !phantom.is_empty(),
        name,
        generics,
        args,
        needs_abi_mirror: struct_needs_abi_mirror(strct, tcx),
        derives_eq: derives_eq(strct),
        fields,
        phantom,
        methods,
    }
    .render()
    .expect("Rust struct template rendering cannot fail")
}

/// A struct can only derive `Eq` when every field is `Eq`, and a float is not.
fn derives_eq(strct: &hir::StructDef) -> bool {
    strct
        .fields
        .iter()
        .all(|field| !matches!(&field.ty, Type::Primitive(hir::PrimitiveType::Float(_))))
}

fn referenced_names_for_struct(
    strct: &hir::StructDef,
    tcx: &TypeContext,
    own_name: &str,
) -> Vec<String> {
    let mut names = BTreeSet::new();
    for field in &strct.fields {
        collect_value_type_names(&field.ty, tcx, &mut names);
    }
    collect_method_names(&strct.methods, tcx, &mut names);
    names.remove(own_name);
    names.into_iter().collect()
}

fn referenced_names_for_methods(
    methods: &[hir::Method],
    tcx: &TypeContext,
    own_name: Option<&str>,
) -> Vec<String> {
    let mut names = BTreeSet::new();
    collect_method_names(methods, tcx, &mut names);
    if let Some(own_name) = own_name {
        names.remove(own_name);
    }
    names.into_iter().collect()
}

fn collect_method_names(methods: &[hir::Method], tcx: &TypeContext, names: &mut BTreeSet<String>) {
    for method in methods.iter().filter(|method| !method.attrs.disable) {
        for param in &method.params {
            collect_value_type_names(&param.ty, tcx, names);
        }
        method.output.with_contained_types(|ty| {
            collect_value_type_names(ty, tcx, names);
        });
    }
}

fn collect_value_type_names<P: hir::TyPosition>(
    ty: &Type<P>,
    tcx: &TypeContext,
    names: &mut BTreeSet<String>,
) {
    match ty {
        Type::Enum(path) => {
            names.insert(type_def_name(TypeDef::Enum(path.resolve(tcx))));
        }
        Type::Struct(path) => {
            names.insert(type_def_name(tcx.resolve_type(path.id())));
        }
        Type::DiplomatOption(inner) => collect_value_type_names(inner, tcx, names),
        Type::Slice(Slice::Struct(_, path)) => {
            names.insert(type_def_name(tcx.resolve_type(path.id())));
        }
        _ => {}
    }
}
