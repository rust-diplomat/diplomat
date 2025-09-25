//! JS code generation backend.
//!
//! This module specifically handles the overview of generating all the necessary `.mjs` and `.d.ts` files that [`gen`] creates content for.
use std::collections::BTreeSet;
use std::{borrow::Cow, cell::RefCell};

use crate::config::Config;
use crate::{ErrorStore, FileMap};
use diplomat_core::hir::{BackendAttrSupport, DocsUrlGenerator, TypeContext, TypeDef};

use askama::Template;

pub(crate) mod formatter;
use formatter::JSFormatter;

mod gen;
use gen::{ItemGenContext, MethodsInfo};
use serde::{Deserialize, Serialize};
mod converter;

mod layout;

/// Since the main difference between .mjs and .d.ts is typing, we just want a differentiator for our various helper functions as to what's being generated: .d.ts, or .mjs?
pub(crate) enum FileType {
    Module,
    Typescript,
}

impl FileType {
    pub fn is_typescript(&self) -> bool {
        match self {
            FileType::Module => false,
            FileType::Typescript => true,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JsConfig {}

impl JsConfig {
    pub fn set(&mut self, key: &str, _value: toml::Value) {
        panic!("JS does not support any backend-specific configs, found {key}");
    }
}

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = false;
    a.memory_sharing = false;
    a.non_exhaustive_structs = true;
    a.method_overloading = false;
    a.utf8_strings = false;
    a.utf16_strings = true;
    a.static_slices = false;

    a.constructors = true;
    a.named_constructors = false;
    a.fallible_constructors = true;
    a.accessors = true;
    a.static_accessors = false;
    a.comparators = false;
    a.stringifiers = false; // TODO
    a.iterators = true;
    a.iterables = true;
    a.indexing = false;
    a.callbacks = false;
    a.option = true;
    a.traits = false;
    a.custom_errors = false; // TODO
    a.traits_are_send = false;
    a.traits_are_sync = false;
    a.generate_mocking_interface = false;

    a
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    config: Config,
    docs: &'tcx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let formatter = JSFormatter::new(tcx, docs);
    let errors = ErrorStore::default();
    let files = FileMap::default();
    let mut exports = Vec::new();
    let mut ts_exports = Vec::new();

    files.add_file(
        "diplomat-runtime.mjs".into(),
        include_str!("../../templates/js/runtime.mjs").into(),
    );
    files.add_file(
        "diplomat-runtime.d.ts".into(),
        include_str!("../../templates/js/runtime.d.ts").into(),
    );
    files.add_file(
        "diplomat-wasm.mjs".into(),
        include_str!("../../templates/js/wasm.mjs").into(),
    );

    for (id, ty) in tcx.all_types() {
        let _guard = errors.set_context_ty(ty.name().as_str().into());

        if ty.attrs().disable {
            continue;
        }

        let type_def = tcx.resolve_type(id);

        let _guard = errors.set_context_ty(type_def.name().as_str().into());

        let type_name = formatter.fmt_type_name(id);

        let context = ItemGenContext {
            tcx,
            type_name,
            formatter: &formatter,
            errors: &errors,
            imports: RefCell::new(gen::Imports {
                js: BTreeSet::new(),
                ts: BTreeSet::new(),
            }),
            config: config.js_config.clone(),
        };

        let (m, special_method_presence, fields, fields_out) = match type_def {
            TypeDef::Enum(e) => (&e.methods, &e.special_method_presence, None, None),
            TypeDef::Opaque(o) => (&o.methods, &o.special_method_presence, None, None),
            TypeDef::Struct(s) => (
                &s.methods,
                &s.special_method_presence,
                Some(context.generate_fields(s)),
                None,
            ),
            TypeDef::OutStruct(s) => (
                &s.methods,
                &s.special_method_presence,
                None,
                Some(context.generate_fields(s)),
            ),
            _ => unreachable!("HIR/AST variant {:?} is unknown.", type_def),
        };

        let mut special_methods = context.generate_special_method(special_method_presence);

        let methods = m
            .iter()
            .flat_map(|method| {
                let inf = context.generate_method(method);
                if inf.is_some() {
                    if let Some(diplomat_core::hir::SpecialMethod::Constructor) =
                        method.attrs.special_method
                    {
                        special_methods.constructor.replace(inf.clone().unwrap());
                    }
                }
                inf
            })
            .collect::<Vec<_>>();

        let mut methods_info = MethodsInfo {
            methods,
            special_methods,
        };

        for file_type in [FileType::Module, FileType::Typescript] {
            let ts = file_type.is_typescript();

            for m in &mut methods_info.methods {
                m.typescript = ts;
            }
            methods_info.special_methods.typescript = ts;

            let contents = match type_def {
                TypeDef::Enum(e) => context.gen_enum(ts, e, &methods_info),
                TypeDef::Opaque(o) => context.gen_opaque(ts, o, &methods_info),
                TypeDef::Struct(s) => {
                    let (fields, layout) = fields.clone().unwrap();
                    context.gen_struct(ts, s, &fields, &methods_info, false, layout)
                }
                TypeDef::OutStruct(s) => {
                    let (fields, layout) = fields_out.clone().unwrap();
                    context.gen_struct(ts, s, &fields, &methods_info, true, layout)
                }
                _ => unreachable!("HIR/AST variant {:?} is unknown.", type_def),
            };

            let file_name = formatter.fmt_file_name(&context.type_name, &file_type);

            // Remove our self reference:
            context.remove_import(context.type_name.clone(), None, gen::ImportUsage::Both);

            // If we're a struct, remove importing our own StructType_obj definition if it exists.
            if matches!(type_def, TypeDef::Struct(..)) {
                context.remove_import(
                    format!("{}_obj", context.type_name).into(),
                    None,
                    gen::ImportUsage::Typescript,
                );
            }

            files.add_file(file_name, context.generate_base(ts, contents));
        }

        let export_filename = formatter.fmt_file_name_extensionless(&context.type_name);

        exports.push(
            formatter
                .fmt_export_statement(&context.type_name, false, "./".into(), &export_filename)
                .into(),
        );
        ts_exports.push(
            formatter
                .fmt_export_statement(
                    &match type_def {
                        TypeDef::Struct(s) if !s.fields.is_empty() => {
                            format!("{}, {}_obj", context.type_name, context.type_name).into()
                        }
                        _ => context.type_name,
                    },
                    true,
                    "./".into(),
                    &export_filename,
                )
                .into(),
        );
    }

    /// Represents the `index.mjs` file that `export`s all classes that we generate.
    #[derive(Template)]
    #[template(path = "js/index.js.jinja", escape = "none")]
    struct IndexTemplate<'a> {
        exports: &'a Vec<Cow<'a, str>>,
        typescript: bool,
    }

    let mut out_index = IndexTemplate {
        exports: &exports,
        typescript: false,
    };

    files.add_file("index.mjs".into(), out_index.render().unwrap());

    out_index.typescript = true;
    out_index.exports = &ts_exports;

    files.add_file("index.d.ts".into(), out_index.render().unwrap());

    (files, errors)
}
