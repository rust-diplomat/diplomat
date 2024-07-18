// Enable once https://github.com/rust-lang/rust/issues/89554 is stable
// #![deny(non_exhaustive_omitted_patterns)] // diplomat_core uses non_exhaustive a lot; we should never miss its patterns

#[doc(hidden)]
pub mod c;
#[doc(hidden)]
pub mod c2;
#[doc(hidden)]
pub mod common;
#[doc(hidden)]
pub mod cpp;
#[doc(hidden)]
pub mod cpp2;
#[doc(hidden)]
pub mod dart;
#[doc(hidden)]
pub mod dotnet;
#[doc(hidden)]
pub mod js;
#[doc(hidden)]
pub mod js2;
#[doc(hidden)]
pub mod kotlin;

mod docs_util;
mod layout;
mod layout_hir;
mod util;

use colored::*;
use core::panic;
use diplomat_core::{ast, hir};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub use ast::DocsUrlGenerator;

#[allow(clippy::too_many_arguments)]
pub fn gen(
    entry: &Path,
    mut target_language: &str,
    out_folder: &Path,
    docs_out_folder: Option<&Path>,
    docs_url_gen: &ast::DocsUrlGenerator,
    library_config: Option<&Path>,
    silent: bool,
) -> std::io::Result<()> {
    // Check that user-provided paths exist. Exit early with a nice error message
    // if anything doesn't exist.
    exit_if_path_missing(
        entry,
        if entry.file_name().map(|e| e == "lib.rs").unwrap_or_default() {
            "Could not find the lib.rs file to process."
        } else {
            "The entry file does not exist."
        },
    );
    exit_if_path_missing(out_folder, "The out folder does not exist.");
    if let Some(docs_out_folder) = docs_out_folder {
        exit_if_path_missing(docs_out_folder, "The docs folder does not exist.");
    }
    if let Some(library_config) = library_config {
        exit_if_path_missing(
            library_config,
            "The library configuration file does not exist.",
        );
    }

    let env = ast::File::from(&syn_inline_mod::parse_and_inline_modules(entry)).all_types();

    // Legacy AST path
    match target_language {
        "c" => {
            let mut files = HashMap::new();
            c::gen_bindings(&env, &mut files).unwrap();
            return write_files(files, out_folder, silent, target_language);
        }
        "cpp" => {
            let mut files = HashMap::new();
            c::gen_bindings(&env, &mut files).unwrap();
            cpp::gen_bindings(&env, library_config, docs_url_gen, &mut files).unwrap();
            if let Some(docs_out_folder) = docs_out_folder {
                let mut docs_files = HashMap::new();
                cpp::docs::gen_docs(&env, library_config, &mut docs_files, docs_url_gen).unwrap();
                write_files(docs_files, docs_out_folder, silent, "cpp-docs")?;
            }
            return write_files(files, out_folder, silent, target_language);
        }
        "dotnet" => {
            let mut files = HashMap::new();
            dotnet::gen_bindings(&env, library_config, docs_url_gen, &mut files).unwrap();
            return write_files(files, out_folder, silent, target_language);
        }
        "js" => {
            let mut files = HashMap::new();
            js::gen_bindings(&env, &mut files, Some(docs_url_gen)).unwrap();
            if let Some(docs_out_folder) = docs_out_folder {
                let mut docs_files = HashMap::new();
                js::docs::gen_docs(&env, &mut docs_files, docs_url_gen).unwrap();
                write_files(docs_files, docs_out_folder, silent, "js-docs")?;
            }
            return write_files(files, out_folder, silent, target_language);
        }
        _hir => {
            target_language = target_language.strip_suffix('2').unwrap_or(target_language);
        }
    }

    let mut attr_validator = hir::BasicAttributeValidator::new(target_language);
    match target_language {
        "c" => {
            attr_validator.support.memory_sharing = true;
            attr_validator.support.disabling = true;
        }

        "cpp" => {
            attr_validator.support.renaming = true;
            attr_validator.support.namespacing = true;
            attr_validator.support.memory_sharing = true;
            attr_validator.support.disabling = true;
        }
        "dart" => {
            attr_validator.support.renaming = true;
            attr_validator.support.disabling = true;
            attr_validator.support.constructors = true;
            attr_validator.support.named_constructors = true;
            attr_validator.support.fallible_constructors = true;
            attr_validator.support.accessors = true;
            attr_validator.support.stringifiers = true;
            attr_validator.support.comparators = true;
            attr_validator.support.iterators = true;
            attr_validator.support.iterables = true;
            attr_validator.support.indexing = true;
        }
        "js" => {
            attr_validator.support.renaming = true;
            attr_validator.support.disabling = true;
            attr_validator.support.accessors = true;
        }
        "kotlin" => {
            attr_validator.support.renaming = true;
            attr_validator.support.disabling = true;
            attr_validator.support.iterators = true;
            attr_validator.support.iterables = true;
            attr_validator.support.indexing = true;
            attr_validator.support.constructors = true;
            attr_validator.support.named_constructors = true;
            attr_validator.support.memory_sharing = true;
            attr_validator.support.accessors = true;
        }
        o => panic!("Unknown target: {}", o),
    };

    let tcx = hir::TypeContext::from_ast(&env, attr_validator).unwrap_or_else(|e| {
        for (ctx, err) in e {
            eprintln!("Lowering error in {ctx}: {err}");
        }
        std::process::exit(1);
    });

    let (files, errors) = match target_language {
        "c" => c2::run(&tcx),
        "cpp" => cpp2::run(&tcx),
        "dart" => dart::run(&tcx, docs_url_gen),
        "js" => js2::run(&tcx, docs_url_gen),
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

/// Provide nice error messages if a folder doesn't exist.
fn exit_if_path_missing(path: &Path, message: &str) {
    if !path.exists() {
        let current_dir = std::env::current_dir().expect("Filed to load current directory.");
        eprintln!(
            "{}{}\n{}",
            "Error: ".red().bold(),
            message,
            format!("{}", Path::new(&current_dir).join(path).display()).red()
        );
        std::process::exit(1);
    }
}
