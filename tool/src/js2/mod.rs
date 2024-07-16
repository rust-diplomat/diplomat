use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::Display;

use diplomat_core::ast::DocsUrlGenerator;

use diplomat_core::hir::{self, TypeContext, TypeDef, TypeId};

use askama::{self, Template};
use type_generation::TypeGenerationContext;

use crate::common::{ErrorStore, FileMap};

use self::formatter::JSFormatter;

mod formatter;
mod type_generation;

/// Wrapper for generating all export types.
///
/// .d.ts definitions, basically. Although we include .mjs so we can do actual conversions to WebAssembly friendly definitions.
pub struct JSGenerationContext<'tcx> {
    tcx: &'tcx TypeContext,
    formatter: JSFormatter<'tcx>,

    errors: ErrorStore<'tcx, String>,

    files: FileMap,

    /// Exports for the root level index.js file.
    exports: Vec<Cow<'tcx, str>>,
    /// Exports for typescript index.d.ts file.
    ts_exports: Vec<Cow<'tcx, str>>,
}

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

impl<'tcx> JSGenerationContext<'tcx> {
    pub fn run(
        tcx: &'tcx TypeContext,
        docs: &'tcx DocsUrlGenerator,
    ) -> Result<FileMap, Vec<(impl Display + 'tcx, String)>> {
        let mut this = Self {
            tcx,
            formatter: JSFormatter::new(tcx, docs),

            errors: ErrorStore::default(),

            files: FileMap::default(),

            exports: Vec::new(),
            ts_exports: Vec::new(),
        };
        this.init();

        let errors = this.errors.take_all();
        if errors.is_empty() {
            Ok(this.files)
        } else {
            Err(errors)
        }
    }

    /// Setup. Write out all the pre-written files.
    ///
    /// Then iterate through all the types we get from the TypeContext to create separate out files.
    pub fn init(&mut self) {
        self.files.add_file(
            "diplomat-runtime.mjs".into(),
            include_str!("../../templates/js2/runtime.mjs").into(),
        );
        self.files.add_file(
            "diplomat-runtime.d.ts".into(),
            include_str!("../../templates/js2/runtime.d.ts").into(),
        );
        self.files.add_file(
            "diplomat-wasm.mjs".into(),
            include_str!("../../templates/js2/wasm.mjs").into(),
        );

        for (id, ty) in self.tcx.all_types() {
            self.generate_file_from_type(id, ty);
        }

        #[derive(Template)]
        #[template(path = "js2/index.js.jinja", escape = "none")]
        struct IndexTemplate<'a> {
            exports: &'a Vec<Cow<'a, str>>,
            typescript: bool,
        }

        let mut out_index = IndexTemplate {
            exports: &self.exports,
            typescript: false,
        };

        self.files
            .add_file("index.mjs".into(), out_index.render().unwrap());

        out_index.typescript = true;
        out_index.exports = &self.ts_exports;

        self.files
            .add_file("index.d.ts".into(), out_index.render().unwrap());
    }

    /// Generate a file's name and body from its given [`TypeId`]
    fn generate_file_from_type(&mut self, type_id: TypeId, ty: hir::TypeDef<'tcx>) {
        let _guard = self.errors.set_context_ty(ty.name().as_str().into());

        if ty.attrs().disable {
            return;
        }

        let type_def = self.tcx.resolve_type(type_id);

        let _guard = self.errors.set_context_ty(type_def.name().as_str().into());

        let name = self.formatter.fmt_type_name(type_id);

        const FILE_TYPES: [FileType; 2] = [FileType::Module, FileType::Typescript];

        for file_type in FILE_TYPES {
            let mut context = TypeGenerationContext {
                js_ctx: self,
                typescript: file_type.is_typescript(),
                imports: BTreeSet::new(),
            };

            // TODO: A lot of this could go faster if we cached info for typescript, instead of re-generating it.
            let contents = match type_def {
                TypeDef::Enum(enum_def) => context.generate_enum_from_def(enum_def, type_id, &name),
                TypeDef::Opaque(opaque_def) => {
                    context.generate_opaque_from_def(opaque_def, type_id, &name)
                }
                TypeDef::Struct(struct_def) => {
                    context.generate_struct_from_def(struct_def, type_id, false, &name, true)
                }
                TypeDef::OutStruct(struct_def) => {
                    context.generate_struct_from_def(struct_def, type_id, true, &name, false)
                }
                _ => unreachable!("HIR/AST variant {:?} is unknown.", type_def),
            };

            let file_name = self.formatter.fmt_file_name(&name, &file_type);

            // Remove our self reference:
            context.imports.remove(&self.formatter.fmt_import_statement(
                &name,
                context.typescript,
                "./".into(),
            ));

            self.files
                .add_file(file_name, context.generate_base(contents));
        }

        self.exports.push(
            self.formatter
                .fmt_export_statement(&name, false, "./".into())
                .into(),
        );
        self.ts_exports.push(
            self.formatter
                .fmt_export_statement(&name, true, "./".into())
                .into(),
        )
    }
}
