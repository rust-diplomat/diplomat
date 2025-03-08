//! Backend for creating automatic demonstrations of using FFI functions.
//!
//! Designed to work in conjunction with the JS backend.
//!
//! See docs/demo_gen.md for more.
use std::{
    collections::{BTreeSet, HashMap},
    fmt::Write,
};

use askama::{self, Template};
use diplomat_core::hir::{BackendAttrSupport, TypeContext};
use serde::{Deserialize, Serialize};
use terminus::{RenderTerminusContext, TerminusInfo};

use crate::{
    js::{self, formatter::JSFormatter, FileType},
    Config, ErrorStore, FileMap,
};

mod terminus;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = js::attr_support();

    // For automagical construction detection:
    a.constructors = true;
    a.fallible_constructors = true;
    a.named_constructors = true;

    a
}

/// Configuration for demo_gen generation. Set from a `.toml` file, you can specify the path of the file with `--library-config` option flag.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DemoConfig {
    /// Require specific opt-in for the demo generator trying to work. If set to true, looks for #[diplomat::demo(generate)].
    pub explicit_generation: Option<bool>,

    /// Removes rendering/ folder
    pub hide_default_renderer: Option<bool>,

    /// If we can grab from index.mjs through a module, override imports for index.mjs to the new module name.
    /// Will set [DemoConfig::relative_js_path] to a blank string, unless explicitly overridden.
    ///
    /// Will not generate the js/ folder if this is set.
    pub module_name: Option<String>,

    /// The relative path to Javascript to use in `import` statements for demo files.
    /// If this is set, we do not generate the js/ folder.
    pub relative_js_path: Option<String>,
}

impl DemoConfig {
    pub fn set(&mut self, key: &str, value: toml::Value) {
        match key {
            "explicit_generation" => self.explicit_generation = value.as_bool(),
            "hide_default_renderer" => self.hide_default_renderer = value.as_bool(),
            "module_name" => self.module_name = value.as_str().map(|v| v.to_string()),
            "relative_js_path" => self.relative_js_path = value.as_str().map(|v| v.to_string()),
            _ => {}
        }
    }
}

/// Per docs/demo_gen.md
/// Generate markup.
///
/// That is, only generate .js files to be used in final rendering.
/// This JS should include:
/// Render Termini that can be called, and internal functions to construct dependencies that the Render Terminus function needs.
pub(crate) fn run<'tcx>(
    entry: &std::path::Path,
    tcx: &'tcx TypeContext,
    docs: &'tcx diplomat_core::ast::DocsUrlGenerator,
    conf: Config,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let formatter = JSFormatter::new(tcx, docs);
    let errors = ErrorStore::default();
    let files = FileMap::default();

    let root = entry.parent().unwrap();

    let unwrapped_conf = conf.demo_gen_config;

    let import_path_exists =
        unwrapped_conf.relative_js_path.is_some() || unwrapped_conf.module_name.is_some();

    let import_path = unwrapped_conf
        .relative_js_path
        .unwrap_or(match unwrapped_conf.module_name {
            Some(_) => "".into(),
            None => "./js/".into(),
        });

    let module_name = unwrapped_conf.module_name.unwrap_or("index.mjs".into());

    struct TerminusExport {
        type_name: String,
        js_file_name: String,
    }

    #[derive(Template)]
    #[template(path = "demo_gen/index.js.jinja", escape = "none")]
    struct IndexInfo {
        termini_exports: Vec<TerminusExport>,
        pub termini: Vec<TerminusInfo>,
        pub js_out: String,

        pub imports: Vec<String>,
        pub custom_func_objs: Vec<String>,
    }

    let mut out_info = IndexInfo {
        termini_exports: Vec::new(),
        termini: Vec::new(),
        js_out: format!("{import_path}{module_name}"),

        imports: Vec::new(),
        custom_func_objs: Vec::new(),
    };

    let is_explicit = unwrapped_conf.explicit_generation.unwrap_or(false);

    for (id, ty) in tcx.all_types() {
        let _guard = errors.set_context_ty(ty.name().as_str().into());

        let methods = ty.methods();

        const FILE_TYPES: [FileType; 2] = [FileType::Module, FileType::Typescript];

        let mut termini = Vec::new();

        {
            let ty_name = formatter.fmt_type_name(id);
            let type_name: String = ty_name.into();

            let js_file_name =
                formatter.fmt_file_name(&type_name.clone(), &crate::js::FileType::Module);

            let ty = tcx.resolve_type(id);

            let attrs = ty.attrs();
            if attrs.disable {
                continue;
            }

            if let Some(custom_func) = &attrs.demo_attrs.custom_func {
                let custom_func_filename = custom_func.to_string();

                let file_path = root.join(custom_func_filename.clone());

                let file_name: String =
                    String::from(file_path.file_name().unwrap().to_str().unwrap());

                // Copy the custom function file from where it is relative to the FFI definition to our output directory.
                let read = std::fs::read(file_path.clone());

                if let Ok(r) = read {
                    let from_utf = String::from_utf8(r);
                    if let Ok(contents) = from_utf {
                        files.add_file(file_name.clone(), contents);
                    } else if let Err(e) = from_utf {
                        errors.push_error(format!(
                            "Could not convert contents of {custom_func_filename} to UTF-8: {e}"
                        ));
                        continue;
                    }
                } else if let Err(e) = read {
                    errors.push_error(format!("Could not read {custom_func_filename} as a custom function file path ({file_path:?}): {e}"));
                    continue;
                }

                // Then add it to our imports for `index.mjs`:
                out_info.imports.push(format!(
                    r#"import RenderTermini{type_name} from "./{file_name}";"#
                ));

                // Finally, make sure the user-defined RenderTermini is added to the terminus object:
                out_info
                    .custom_func_objs
                    .push(format!("RenderTermini{type_name}"));
            }

            for method in methods {
                if method.attrs.disable
                    || (is_explicit && !method.attrs.demo_attrs.generate)
                    || !RenderTerminusContext::is_valid_terminus(method)
                {
                    continue;
                }

                let _guard = errors
                    .set_context_method(ty.name().as_str().into(), method.name.as_str().into());

                let function_name = formatter.fmt_method_name(method);

                let mut ctx = RenderTerminusContext {
                    tcx,
                    formatter: &formatter,
                    errors: &errors,
                    terminus_info: TerminusInfo {
                        function_name: function_name.clone(),
                        out_params: Vec::new(),

                        type_name: type_name.clone(),

                        js_file_name: js_file_name.clone(),

                        node_call_stack: Vec::default(),

                        // We set this in the init function of WebDemoGenerationContext.
                        typescript: false,

                        imports: BTreeSet::new(),
                    },

                    name_collision: HashMap::new(),

                    relative_import_path: import_path.clone(),
                    module_name: module_name.clone(),
                };

                ctx.evaluate(type_name.clone(), method);

                termini.push(ctx.terminus_info);
            }
        }

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

    let hide_default_renderer = unwrapped_conf.hide_default_renderer.unwrap_or(false);

    if !hide_default_renderer {
        files.add_file(
            "rendering/rendering.mjs".into(),
            include_str!("../../templates/demo_gen/default_renderer/rendering.mjs").into(),
        );
        files.add_file(
            "rendering/runtime.mjs".into(),
            include_str!("../../templates/demo_gen/default_renderer/runtime.mjs").into(),
        );
        files.add_file(
            "rendering/template.html".into(),
            include_str!("../../templates/demo_gen/default_renderer/template.html").into(),
        );
    }

    if !import_path_exists {
        files.add_file(
            "diplomat.config.mjs".into(),
            include_str!("../../templates/demo_gen/default_renderer/config.mjs").into(),
        );
    }

    (files, errors)
}
