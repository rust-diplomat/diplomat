use std::{collections::BTreeSet, fmt::Write};

use askama::{self, Template};
use diplomat_core::hir::{BackendAttrSupport, TypeContext};
use terminus::{RenderTerminusContext, TerminusInfo};

use crate::{
    js::{self, formatter::JSFormatter, FileType},
    ErrorStore, FileMap,
};

mod terminus;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = js::attr_support();

    // For automagical construction detection:
    a.constructors = true;
    a.fallible_constructors = true;

    a
}

/// Per https://docs.google.com/document/d/1xRTmK0YtOfuAe7ClN6kqDaHyv5HpdIRIYQW6Zc_KKFU/edit?usp=sharing
/// Generate markup.
///
/// That is, only generate .js files to be used in final rendering.
/// This JS should include:
/// Render Termini that can be called, and internal functions to construct dependencies that the Render Terminus function needs.
pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    docs: &'tcx diplomat_core::ast::DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let formatter = JSFormatter::new(tcx, docs);
    let errors = ErrorStore::default();
    let files = FileMap::default();

    struct TerminusExport {
        type_name: String,
        js_file_name: String,
    }

    #[derive(Template)]
    #[template(path = "demo_gen/index.js.jinja", escape = "none")]
    struct IndexInfo {
        termini_exports: Vec<TerminusExport>,
        pub termini: Vec<TerminusInfo>,
    }

    let mut out_info = IndexInfo {
        termini_exports: Vec::new(),
        termini: Vec::new(),
    };

    for (id, ty) in tcx.all_types() {
        let _guard = errors.set_context_ty(ty.name().as_str().into());

        let methods = ty.methods();

        const FILE_TYPES: [FileType; 2] = [FileType::Module, FileType::Typescript];

        let mut termini = Vec::new();

        {
            let type_name = formatter.fmt_type_name(id);

            let ty = tcx.resolve_type(id);
            if ty.attrs().disable {
                continue;
            }

            for method in methods {
                if method.attrs.disable || !RenderTerminusContext::is_valid_terminus(method) {
                    continue;
                }

                let _guard = errors
                    .set_context_method(ty.name().as_str().into(), method.name.as_str().into());

                let mut ctx = RenderTerminusContext {
                    tcx,
                    formatter: &formatter,
                    errors: &errors,
                    terminus_info: TerminusInfo {
                        function_name: formatter.fmt_method_name(method),
                        out_params: Vec::new(),

                        type_name: type_name.clone().into(),

                        js_file_name: formatter
                            .fmt_file_name(&type_name, &crate::js::FileType::Module),

                        node_call_stack: String::default(),

                        // We set this in the init function of WebDemoGenerationContext.
                        typescript: false,

                        imports: BTreeSet::new(),
                    },
                };

                ctx.evaluate(type_name.clone().into(), method);

                termini.push(ctx.terminus_info);
            }
        }

        generate_default_renderer_files(&termini, &files);

        if !termini.is_empty() {
            let mut imports = BTreeSet::new();
            for file_type in FILE_TYPES {
                let type_name = formatter.fmt_type_name(id);
                let file_name = formatter.fmt_file_name(&type_name, &file_type);

                let mut method_str = String::new();

                for terminus in &mut termini {
                    terminus.typescript = file_type.is_typescript();
                    writeln!(method_str, "{}", terminus.render().unwrap()).unwrap();

                    imports.append(&mut terminus.imports);
                }

                let mut import_str = String::new();

                for import in imports.iter() {
                    writeln!(import_str, "{}", import).unwrap();
                }

                files.add_file(file_name.to_string(), format!("{import_str}{method_str}"));
            }

            // Only push the first one,
            out_info.termini_exports.push(TerminusExport {
                type_name: termini[0].type_name.clone(),
                js_file_name: termini[0].js_file_name.clone(),
            });

            out_info.termini.append(&mut termini);
        }
    }

    files.add_file("index.mjs".into(), out_info.render().unwrap());

    // TODO: Avoid overwriting these files if one already exists (but update if that is a file present somewhere).
    // I'm thinking of just putting these in their own folder for that.
    // TODO: Some of these files should only be generated with certain command line options.
    files.add_file(
        "rendering.mjs".into(),
        include_str!("../../templates/demo_gen/default_renderer/rendering.mjs").into(),
    );
    files.add_file(
        "runtime.mjs".into(),
        include_str!("../../templates/demo_gen/default_renderer/runtime.mjs").into(),
    );
    files.add_file(
        "template.html".into(),
        include_str!("../../templates/demo_gen/default_renderer/template.html").into(),
    );
    files.add_file(
        "diplomat.config.mjs".into(),
        include_str!("../../templates/demo_gen/default_renderer/config.mjs").into(),
    );

    (files, errors)
}

fn generate_default_renderer_files(termini: &Vec<TerminusInfo>, files: &FileMap) {
    #[derive(Template)]
    #[template(path = "demo_gen/default_renderer/termini.html.jinja")]
    struct TerminiHTML<'a> {
        pub t: &'a TerminusInfo,
    }

    for terminus in termini {
        files.add_file(
            format!(
                "rendering/{}_{}.html",
                terminus.type_name, terminus.function_name
            ),
            TerminiHTML { t: terminus }.render().unwrap(),
        );
    }
}
