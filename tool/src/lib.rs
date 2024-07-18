// Enable once https://github.com/rust-lang/rust/issues/89554 is stable
// #![deny(non_exhaustive_omitted_patterns)] // diplomat_core uses non_exhaustive a lot; we should never miss its patterns

// Backends
mod c;
mod cpp;
mod dart;
mod js;
mod demo_gen;
mod kotlin;

use colored::*;
use core::mem;
use core::panic;
use diplomat_core::{ast, hir};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub use ast::DocsUrlGenerator;

#[allow(clippy::too_many_arguments)]
pub fn gen(
    entry: &Path,
    target_language: &str,
    out_folder: &Path,
    docs_url_gen: &ast::DocsUrlGenerator,
    library_config: Option<&Path>,
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

    let env = ast::File::from(&syn_inline_mod::parse_and_inline_modules(entry)).all_types();

    // The HIR backends used to be named "c2", "js2", etc
    let target_language = target_language.strip_suffix('2').unwrap_or(target_language);
    let mut attr_validator = hir::BasicAttributeValidator::new(target_language);
    attr_validator.support = match target_language {
        "c" => c::attr_support(),
        "cpp" => cpp::attr_support(),
        "dart" => dart::attr_support(),
        "js" => js::attr_support(),
        "demo-gen" => demo_gen::attr_support(),
        "kotlin" => kotlin::attr_support(),
        o => panic!("Unknown target: {}", o),
    };

    let tcx = hir::TypeContext::from_ast(&env, attr_validator).unwrap_or_else(|e| {
        for (ctx, err) in e {
            eprintln!("Lowering error in {ctx}: {err}");
        }
        std::process::exit(1);
    });

    let (files, errors) = match target_language {
        "c" => c::run(&tcx),
        "cpp" => cpp::run(&tcx),
        "dart" => dart::run(&tcx, docs_url_gen),
        "js" => js::run(&tcx, docs_url_gen),
        "demo-gen" => demo_gen::run(&tcx, docs_url_gen),
        "kotlin" => kotlin::run(&tcx, library_config),
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

    write_files(files.take_files(), out_folder, silent, target_language)?;

    Ok(())
}

fn write_files(
    files: HashMap<String, String>,
    out_folder: &Path,
    silent: bool,
    target_language: &str,
) -> std::io::Result<()> {
    if !silent {
        println!(
            "{}",
            format!("Generating {} bindings:", target_language)
                .green()
                .bold()
        );
    }
    for (subpath, text) in files {
        let out_path = out_folder.join(subpath);
        let parent = out_path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();
        let mut out_file = File::create(&out_path)?;
        out_file.write_all(text.as_bytes())?;
        if !silent {
            println!("{}", format!("  {}", out_path.display()).dimmed());
        }
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
    #[allow(dead_code)]
    pub fn new(files: HashMap<String, String>) -> Self {
        FileMap {
            files: RefCell::new(files),
        }
    }

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
struct ErrorStore<'tcx, E> {
    /// The stack of contexts reached so far
    context: RefCell<ErrorContext<'tcx>>,
    errors: RefCell<Vec<(ErrorContext<'tcx>, E)>>,
}

impl<'tcx, E> ErrorStore<'tcx, E> {
    /// Set the context to a named type. Will return a scope guard that will automatically
    /// clear the context on drop.
    fn set_context_ty<'a>(&'a self, ty: Cow<'tcx, str>) -> ErrorContextGuard<'a, 'tcx, E> {
        let new = ErrorContext { ty, method: None };
        let old = mem::replace(&mut *self.context.borrow_mut(), new);
        ErrorContextGuard(self, old)
    }
    /// Set the context to a named method. Will return a scope guard that will automatically
    /// clear the context on drop.
    fn set_context_method<'a>(
        &'a self,
        ty: Cow<'tcx, str>,
        method: Cow<'tcx, str>,
    ) -> ErrorContextGuard<'a, 'tcx, E> {
        let new = ErrorContext {
            ty,
            method: Some(method),
        };

        let old = mem::replace(&mut *self.context.borrow_mut(), new);
        ErrorContextGuard(self, old)
    }

    fn push_error(&self, error: E) {
        self.errors
            .borrow_mut()
            .push((self.context.borrow().clone(), error));
    }

    fn take_all(&self) -> Vec<(impl fmt::Display + 'tcx, E)> {
        mem::take(&mut self.errors.borrow_mut())
    }
}

/// The context in which an error was discovered
#[derive(Default, Clone)]
struct ErrorContext<'tcx> {
    ty: Cow<'tcx, str>,
    method: Option<Cow<'tcx, str>>,
}

impl<'tcx> fmt::Display for ErrorContext<'tcx> {
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
struct ErrorContextGuard<'a, 'tcx, E>(&'a ErrorStore<'tcx, E>, ErrorContext<'tcx>);

impl<'a, 'tcx, E> Drop for ErrorContextGuard<'a, 'tcx, E> {
    fn drop(&mut self) {
        let _ = mem::replace(&mut *self.0.context.borrow_mut(), mem::take(&mut self.1));
    }
}
