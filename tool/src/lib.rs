// Enable once https://github.com/rust-lang/rust/issues/89554 is stable
// #![deny(non_exhaustive_omitted_patterns)] // diplomat_core uses non_exhaustive a lot; we should never miss its patterns

// Backends
mod c2;
mod cpp2;
mod dart;
mod js2;
mod kotlin;

mod common;
#[allow(unused)] // HIR backends don't do doc work, let's allow(unused) for now
mod docs_util;
mod layout_hir;

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
    target_language: &str,
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

    // The HIR backends used to be named "c2", "js2", etc
    let target_language = target_language.strip_suffix('2').unwrap_or(target_language);
    let mut attr_validator = hir::BasicAttributeValidator::new(target_language);
    attr_validator.support = match target_language {
        "c" => c2::attr_support(),
        "cpp" => cpp2::attr_support(),
        "dart" => dart::attr_support(),
        "js" => js2::attr_support(),
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
