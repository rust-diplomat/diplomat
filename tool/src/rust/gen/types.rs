//! Rendering of generated enums and value structs.

use std::fmt::Write as _;

use diplomat_core::hir::{DocsUrlGenerator, TypeContext, TypeDef};

use crate::r#rust::formatter::{emit_docs, enum_variant_name, field_name, type_def_name};
use crate::r#rust::gen::method::emit_method;
use crate::r#rust::lifetimes::{struct_generics, struct_lifetime_phantom};
use crate::r#rust::type_map::{is_lifetime_struct, safe_struct_field_type, safe_value_type};

/// Whether a type has at least one method that will be emitted.
fn has_methods(methods: &[diplomat_core::hir::Method]) -> bool {
    methods.iter().any(|method| !method.attrs.disable)
}

pub(in crate::r#rust) fn generate_types(
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) -> String {
    let mut out = String::from("//! Generated enums and value structs.\n\n");
    if tcx
        .structs()
        .iter()
        .any(|strct| !strct.attrs.disable && is_lifetime_struct(strct))
    {
        out.push_str("use core::marker::PhantomData;\n\n");
    }
    // An inherent method on a value type calls straight into the ABI layer; a value
    // type has no wrapper between the two.
    if tcx
        .enums()
        .iter()
        .any(|enm| !enm.attrs.disable && has_methods(&enm.methods))
        || tcx
            .structs()
            .iter()
            .any(|strct| !strct.attrs.disable && has_methods(&strct.methods))
    {
        out.push_str("use crate::ffi;\n\n");
    }
    for enm in tcx.enums().iter().filter(|ty| !ty.attrs.disable) {
        emit_docs(&mut out, &enm.docs, docs_url_gen, "");
        let name = type_def_name(TypeDef::Enum(enm));
        writeln!(
            out,
            "#[repr(C)]\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum {name} {{"
        )
        .unwrap();
        for variant in &enm.variants {
            emit_docs(&mut out, &variant.docs, docs_url_gen, "    ");
            writeln!(
                out,
                "    {} = {},",
                enum_variant_name(variant),
                variant.discriminant
            )
            .unwrap();
        }
        out.push_str("}\n\n");
        if has_methods(&enm.methods) {
            writeln!(out, "impl {name} {{").unwrap();
            for method in enm.methods.iter().filter(|method| !method.attrs.disable) {
                emit_method(&mut out, 0, method, tcx, docs_url_gen);
            }
            out.push_str("}\n\n");
        }
    }
    for strct in tcx.structs().iter().filter(|ty| !ty.attrs.disable) {
        emit_docs(&mut out, &strct.docs, docs_url_gen, "");
        let name = type_def_name(TypeDef::Struct(strct));
        if is_lifetime_struct(strct) {
            let (params, _) = struct_generics(strct);
            writeln!(
                out,
                "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct {name}{params} {{"
            )
            .unwrap();
            for field in &strct.fields {
                emit_docs(&mut out, &field.docs, docs_url_gen, "    ");
                writeln!(
                    out,
                    "    pub {}: {},",
                    field_name(field),
                    safe_struct_field_type(&field.ty, strct, tcx)
                )
                .unwrap();
            }
            out.push_str(&struct_lifetime_phantom(strct));
            out.push_str("}\n\n");
        } else {
            writeln!(
                out,
                "#[repr(C)]\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct {name} {{"
            )
            .unwrap();
            for field in &strct.fields {
                emit_docs(&mut out, &field.docs, docs_url_gen, "    ");
                writeln!(
                    out,
                    "    pub {}: {},",
                    field_name(field),
                    safe_value_type(&field.ty, tcx)
                )
                .unwrap();
            }
            out.push_str("}\n\n");
        }
        if has_methods(&strct.methods) {
            let (params, args) = struct_generics(strct);
            writeln!(out, "impl{params} {name}{args} {{").unwrap();
            for method in strct.methods.iter().filter(|method| !method.attrs.disable) {
                emit_method(
                    &mut out,
                    strct.lifetimes.num_lifetimes(),
                    method,
                    tcx,
                    docs_url_gen,
                );
            }
            out.push_str("}\n\n");
        }
    }
    out
}
