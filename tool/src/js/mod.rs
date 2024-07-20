use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::{ErrorStore, FileMap};
use diplomat_core::hir::{BackendAttrSupport, DocsUrlGenerator, TypeContext, TypeDef};

use askama::Template;

mod formatter;
use formatter::JSFormatter;

mod type_generation;
use type_generation::TyGenContext;

mod layout;

/// Since the main difference between .mjs and .d.ts is typing, we just want a differentiator for our various helper functions as to what's being generated: .d.ts, or .mjs?
enum FileType {
    Module,
    Typescript,
}

impl FileType {
    fn is_typescript(&self) -> bool {
        match self {
            FileType::Module => false,
            FileType::Typescript => true,
        }
    }
}

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.renaming = true;
    a.namespacing = false;
    a.memory_sharing = false;
    a.non_exhaustive_structs = true;
    a.method_overloading = false;
    a.utf8_strings = false;
    a.utf16_strings = true;

    a.constructors = false;
    a.named_constructors = false;
    a.fallible_constructors = false;
    a.accessors = true;
    a.comparators = false;
    a.stringifiers = false; // TODO
    a.iterators = true;
    a.iterables = true;
    a.indexing = false;

    a
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
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

        let name = formatter.fmt_type_name(id);

        for file_type in [FileType::Module, FileType::Typescript] {
            let mut context = TyGenContext {
                tcx,
                formatter: &formatter,
                errors: &errors,
                typescript: file_type.is_typescript(),
                imports: BTreeSet::new(),
            };

            // TODO: A lot of this could go faster if we cached info for typescript, instead of re-generating it.
            let contents = match type_def {
                TypeDef::Enum(e) => context.gen_enum(e, id, &name),
                TypeDef::Opaque(o) => context.gen_opaque(o, id, &name),
                TypeDef::Struct(s) => context.gen_struct(s, id, false, &name, true),
                TypeDef::OutStruct(s) => context.gen_struct(s, id, true, &name, false),
                _ => unreachable!("HIR/AST variant {:?} is unknown.", type_def),
            };

            let file_name = formatter.fmt_file_name(&name, &file_type);

            // Remove our self reference:
            context.imports.remove(&formatter.fmt_import_statement(
                &name,
                context.typescript,
                "./".into(),
            ));

            files.add_file(file_name, context.generate_base(contents));
        }

        exports.push(
            formatter
                .fmt_export_statement(&name, false, "./".into())
                .into(),
        );
        ts_exports.push(
            formatter
                .fmt_export_statement(&name, true, "./".into())
                .into(),
        )
    }

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
