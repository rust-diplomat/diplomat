//! Askama rendering of the generated crate facade and private ABI files.
//!
//! Semantic lowering stays in `type_map`, `lifetimes` and `validate`; this
//! module only builds render views and sends them to `.jinja` templates.

mod method;
mod opaque;
mod types;

pub(super) use opaque::generate_opaque_file;
pub(super) use types::generate_type_files;

use askama::Template;
use diplomat_core::hir::{self, TypeContext, TypeDef};

use super::formatter::{field_name, opaque_module_name, type_def_name};
use super::lifetimes::{
    lifetime_generics, lifetime_witness_type, render_generics, struct_generics,
};
use super::type_map::{
    ffi_input_type, ffi_return_generics, ffi_return_type, ffi_self_type, ffi_struct_field_type,
    struct_needs_abi_mirror,
};

#[derive(Template)]
#[template(path = "rust/ffi.rs.jinja", escape = "none")]
struct FfiTemplate {
    dylib_name: String,
    opaque_types: Vec<String>,
    mirrors: Vec<AbiStructView>,
    opaque_methods: Vec<AbiOpaqueMethodsView>,
    value_methods: Vec<AbiMethodView>,
}

#[derive(Template)]
#[template(path = "rust/lib.rs.jinja", escape = "none")]
struct LibTemplate;

#[derive(Template)]
#[template(path = "rust/owned_slice.rs.jinja", escape = "none")]
struct OwnedSliceTemplate;

#[derive(Template)]
#[template(path = "rust/private.rs.jinja", escape = "none")]
struct PrivateTemplate {
    opaques: Vec<OpaqueCapabilityView>,
}

#[derive(Template)]
#[template(path = "rust/opaques_mod.rs.jinja", escape = "none")]
struct OpaquesModTemplate {
    modules: Vec<String>,
    has_modules: bool,
}

struct AbiStructView {
    name: String,
    params: String,
    fields: Vec<AbiFieldView>,
}

struct AbiFieldView {
    name: String,
    ty: String,
}

struct AbiOpaqueMethodsView {
    name: String,
    dtor_abi_name: String,
    methods: Vec<AbiMethodView>,
}

struct AbiMethodView {
    name: String,
    generics: String,
    params: String,
    return_suffix: String,
}

struct OpaqueCapabilityView {
    name: String,
    type_params: String,
    type_args: String,
    has_type_lifetimes: bool,
    lifetime_witness: String,
}

pub(super) fn generate_ffi(tcx: &TypeContext, dylib_name: &str) -> String {
    let opaque_types = tcx
        .opaques()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .map(|opaque| type_def_name(TypeDef::Opaque(opaque)))
        .collect();

    let mirrors = tcx
        .structs()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .filter(|strct| struct_needs_abi_mirror(strct, tcx))
        .map(|strct| AbiStructView {
            name: type_def_name(TypeDef::Struct(strct)),
            params: struct_generics(strct).0,
            fields: strct
                .fields
                .iter()
                .map(|field| AbiFieldView {
                    name: field_name(field),
                    ty: ffi_struct_field_type(&field.ty, strct, tcx),
                })
                .collect(),
        })
        .collect();

    let opaque_methods = tcx
        .opaques()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .map(|opaque| AbiOpaqueMethodsView {
            name: type_def_name(TypeDef::Opaque(opaque)),
            dtor_abi_name: opaque.dtor_abi_name.to_string(),
            methods: abi_methods(&opaque.methods, tcx),
        })
        .collect();
    let value_methods = tcx
        .structs()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .flat_map(|strct| abi_methods(&strct.methods, tcx))
        .chain(
            tcx.enums()
                .iter()
                .filter(|ty| !ty.attrs.disable)
                .flat_map(|enm| abi_methods(&enm.methods, tcx)),
        )
        .collect();

    FfiTemplate {
        dylib_name: dylib_name.to_string(),
        opaque_types,
        mirrors,
        opaque_methods,
        value_methods,
    }
    .render()
    .expect("Rust ffi template rendering cannot fail")
}

fn abi_methods(methods: &[hir::Method], tcx: &TypeContext) -> Vec<AbiMethodView> {
    methods
        .iter()
        .filter(|method| !method.attrs.disable)
        .map(|method| {
            let mut params = Vec::new();
            if let Some(param_self) = &method.param_self {
                params.push(format!("this: {}", ffi_self_type(&param_self.ty, tcx)));
            }
            params.extend(
                method
                    .params
                    .iter()
                    .map(|param| format!("{}: {}", param.name, ffi_input_type(&param.ty, tcx))),
            );
            if method.output.is_write() {
                params.push("write: *mut DiplomatWrite".into());
            }
            let ret = ffi_return_type(&method.output, tcx);
            AbiMethodView {
                name: method.abi_name.to_string(),
                generics: ffi_return_generics(&method.output, tcx),
                params: params.join(", "),
                return_suffix: if ret == "()" {
                    String::new()
                } else {
                    format!(" -> {ret}")
                },
            }
        })
        .collect()
}

pub(super) fn generate_lib() -> String {
    LibTemplate
        .render()
        .expect("Rust lib template rendering cannot fail")
}

pub(super) fn generate_owned_slice() -> String {
    OwnedSliceTemplate
        .render()
        .expect("Rust owned-slice template rendering cannot fail")
}

/// Sealed capability traits plus the unsafe ABI reconstruction helpers.
pub(super) fn generate_private(tcx: &TypeContext) -> String {
    let opaques = tcx
        .opaques()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .map(|opaque| {
            let (type_params, type_args) = lifetime_generics(&opaque.lifetimes);
            let has_type_lifetimes = !type_args.is_empty();
            OpaqueCapabilityView {
                name: type_def_name(TypeDef::Opaque(opaque)),
                type_params: render_generics(&type_params),
                type_args: render_generics(&type_args),
                has_type_lifetimes,
                lifetime_witness: if has_type_lifetimes {
                    lifetime_witness_type(&type_args)
                } else {
                    String::new()
                },
            }
        })
        .collect();
    PrivateTemplate { opaques }
        .render()
        .expect("Rust private template rendering cannot fail")
}

/// One module per opaque type, re-exported flat from the `opaques` module.
pub(super) fn generate_opaques_index(tcx: &TypeContext) -> String {
    let modules = tcx
        .opaques()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .map(opaque_module_name)
        .collect::<Vec<_>>();
    OpaquesModTemplate {
        has_modules: !modules.is_empty(),
        modules,
    }
    .render()
    .expect("Rust opaque module template rendering cannot fail")
}
