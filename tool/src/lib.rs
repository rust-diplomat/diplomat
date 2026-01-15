// Enable once https://github.com/rust-lang/rust/issues/89554 is stable
// #![deny(non_exhaustive_omitted_patterns)] // diplomat_core uses non_exhaustive a lot; we should never miss its patterns

pub mod config;
// Custom askama filters
pub(crate) mod filters;

// Backends
pub mod c;
mod cpp;
mod dart;
mod demo_gen;
mod js;
mod kotlin;
mod nanobind;

use colored::*;
use config::toml_value_from_str;
use config::{find_top_level_attr, Config};
use core::mem;
use core::panic;
use diplomat_core::hir;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

pub use hir::DocsUrlGenerator;

pub fn gen(
    entry: &Path,
    target_language: &str,
    out_folder: &Path,
    docs_url_gen: &DocsUrlGenerator,
    mut config: Config,
    silent: bool,
) -> std::io::Result<()> {
    if !entry.exists() {
        eprintln!(
            "{}{}\n{}",
            "Error: ".red().bold(),
            if entry.file_name().map(|e| e == "lib.rs").unwrap_or_default() {
                "Could not find the lib.rs file to process."
            } else {
                "The entry file does not exist."
            },
            format!("{}", std::env::current_dir().unwrap().join(entry).display()).red()
        );
        std::process::exit(1);
    }

    // Set the default binding location:
    if config
        .shared_config
        .custom_binding_location
        .as_os_str()
        .is_empty()
    {
        config.shared_config.custom_binding_location = entry
            .parent()
            .expect("Could not get parent of lib.rs")
            .to_path_buf();
    }

    // The HIR backends used to be named "c2", "js2", etc
    let target_language = target_language.strip_suffix('2').unwrap_or(target_language);
    let mut attr_validator = hir::BasicAttributeValidator::new(target_language);
    attr_validator.support = match target_language {
        "c" => c::attr_support(),
        "cpp" => cpp::attr_support(),
        "dart" => dart::attr_support(),
        "js" => js::attr_support(),
        "demo_gen" => {
            // So renames and disables are carried across.
            attr_validator.other_backend_names = vec!["js".to_string()];
            demo_gen::attr_support()
        }
        "kotlin" => kotlin::attr_support(),
        "py-nanobind" | "nanobind" => nanobind::attr_support(),
        o => panic!("Unknown target: {}", o),
    };

    let module = syn_inline_mod::parse_and_inline_modules(entry);

    // Config:
    // Just search the top-level lib.rs for the Config attributes for now. We can re-configure this to use AST to search ALL modules if need be.
    let cfg = find_top_level_attr(module.items.clone());
    for attr in cfg {
        for kvp in attr.key_value_pairs {
            config.set(&kvp.key, toml_value_from_str(&kvp.value));
        }
    }

    let config = config.get_overridden(target_language);

    let lowering_config = config.shared_config.lowering_config();

    let tcx =
        hir::TypeContext::from_syn(&module, lowering_config, attr_validator).unwrap_or_else(|e| {
            for (ctx, err) in e {
                eprintln!("Lowering error in {ctx}: {err}");
            }
            std::process::exit(1);
        });

    let (files, errors) = match target_language {
        "c" => c::run(&tcx, &config, docs_url_gen),
        "cpp" => cpp::run(&tcx, &config, docs_url_gen),
        "dart" => dart::run(&tcx, docs_url_gen),
        "js" => js::run(&tcx, config, docs_url_gen),
        "py-nanobind" | "nanobind" => nanobind::run(&tcx, config, docs_url_gen),
        "demo_gen" => {
            // If we don't already have an import path set up, generate our own imports:
            if !(config.demo_gen_config.module_name.is_some()
                || config.demo_gen_config.relative_js_path.is_some())
            {
                gen(
                    entry,
                    "js",
                    &out_folder.join("js"),
                    docs_url_gen,
                    config.clone(),
                    silent,
                )?;
            }
            demo_gen::run(entry, &tcx, docs_url_gen, config.clone())
        }
        "kotlin" => kotlin::run(&tcx, config.clone(), docs_url_gen),
        o => panic!("Unknown target: {}", o),
    };

    let errors = errors.take_all();
    if !errors.is_empty() {
        eprintln!("Found errors whilst generating {target_language}:");
        for error in errors {
            eprintln!("\t{}: {}", error.0, error.1);
        }
        eprintln!("Not generating files due to errors");
        // Eventually this should use eyre or something
        std::process::exit(1);
    }

    if !silent {
        println!(
            "{}",
            format!("Generating {target_language} bindings:")
                .green()
                .bold()
        );
    }
    for (subpath, text) in files.take_files() {
        let out_path = out_folder.join(subpath);
        if !silent {
            println!("{}", format!("  {}", out_path.display()).dimmed());
        }
        std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();
        std::fs::write(&out_path, text)?;
    }

    Ok(())
}

/// This type abstracts over files being written to.
#[derive(Default, Debug)]
pub struct FileMap {
    // The context types exist as a way to avoid passing around a billion different
    // parameters. However, passing them around as &mut self restricts the amount of
    // borrowing that can be done. We instead use a RefCell to guard the specifically mutable bits.
    files: RefCell<HashMap<String, String>>,
}

impl FileMap {
    pub fn take_files(self) -> HashMap<String, String> {
        mem::take(&mut *self.files.borrow_mut())
    }

    pub fn add_file(&self, name: String, contents: String) {
        if self.files.borrow().get(&name).is_some() {
            panic!("File map already contains {}", name)
        }
        self.files.borrow_mut().insert(name, contents);
    }
}

/// This type acts as a "store" for errors, which can be appended to.
/// Keeps track of the context in which an error was generated.
///
/// You can use [`set_context_ty()`] and [`set_context_method()`] to set the context
/// as a type or method. They will return scope guards that will automatically pop the stack
/// once they go out of scope, so you don't have to worry about errors originating from code
/// that does not set a context.
#[derive(Default)]
pub struct ErrorStore<'tcx, E> {
    /// The stack of contexts reached so far
    context: RefCell<ErrorContext<'tcx>>,
    errors: RefCell<Vec<(ErrorContext<'tcx>, E)>>,
}

impl<'tcx, E> ErrorStore<'tcx, E> {
    /// Set the context to a named type. Will return a scope guard that will automatically
    /// clear the context on drop.
    pub fn set_context_ty<'a>(&'a self, ty: Cow<'tcx, str>) -> ErrorContextGuard<'a, 'tcx, E> {
        let new = ErrorContext { ty, method: None };
        let old = mem::replace(&mut *self.context.borrow_mut(), new);
        ErrorContextGuard(self, old)
    }

    /// Set the context to a named method. Will return a scope guard that will automatically
    /// clear the context on drop.
    pub fn set_context_method<'a>(
        &'a self,
        method: Cow<'tcx, str>,
    ) -> ErrorContextGuard<'a, 'tcx, E> {
        let new = ErrorContext {
            ty: self.context.borrow().ty.clone(),
            method: Some(method),
        };

        let old = mem::replace(&mut *self.context.borrow_mut(), new);
        ErrorContextGuard(self, old)
    }

    pub fn push_error(&self, error: E) {
        self.errors
            .borrow_mut()
            .push((self.context.borrow().clone(), error));
    }

    pub fn take_all(&self) -> Vec<(impl fmt::Display + 'tcx, E)> {
        mem::take(&mut self.errors.borrow_mut())
    }
}

/// The context in which an error was discovered
#[derive(Default, Clone)]
struct ErrorContext<'tcx> {
    ty: Cow<'tcx, str>,
    method: Option<Cow<'tcx, str>>,
}

impl fmt::Display for ErrorContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty = &self.ty;
        if let Some(ref method) = self.method {
            write!(f, "{ty}::{method}")
        } else {
            ty.fmt(f)
        }
    }
}

/// Scope guard terminating the context created `set_context_*` method on [`ErrorStore`]
#[must_use]
pub struct ErrorContextGuard<'a, 'tcx, E>(&'a ErrorStore<'tcx, E>, ErrorContext<'tcx>);

impl<E> Drop for ErrorContextGuard<'_, '_, E> {
    fn drop(&mut self) {
        let _ = mem::replace(&mut *self.0.context.borrow_mut(), mem::take(&mut self.1));
    }
}

pub(crate) fn read_custom_binding<'a, 'b>(
    source: &hir::IncludeSource,
    config: &Config,
    errors: &'b ErrorStore<'a, String>,
) -> Result<String, ()> {
    match source {
        hir::IncludeSource::File(path) => {
            let path = config.shared_config.custom_binding_location.join(path);
            std::fs::read_to_string(&path).map_err(|e| {
                errors.push_error(format!("Cannot find file {}: {e}", path.display()));
            })
        }
        hir::IncludeSource::Source(s) => Ok(s.clone()),
        _ => panic!("Unrecognized IncludeSource: {:?}", source),
    }
}
