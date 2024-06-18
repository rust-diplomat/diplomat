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
pub mod demo_gen;
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
    target_language: &str,
    out_folder: &Path,
    docs_out_folder: Option<&Path>,
    docs_url_gen: &ast::DocsUrlGenerator,
    library_config: Option<&Path>,
    silent: bool,
    strip_prefix: Option<String>,
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

    let lib_file = syn_inline_mod::parse_and_inline_modules(entry);
    let diplomat_file = ast::File::from(&lib_file);
    let env = diplomat_file.all_types();

    let mut out_texts: HashMap<String, String> = HashMap::new();

    let mut errors_found = false;

    match target_language {
        "js2" => {
            let mut attr_validator = hir::BasicAttributeValidator::new("js2");
            attr_validator.other_backend_names.push("js".into());

            attr_validator.support.renaming = true;
            attr_validator.support.disabling = true;

            attr_validator.support.iterables = true;
            attr_validator.support.iterators = true;
            
            attr_validator.support.accessors = true;

            // Not possible since Javascript only allows us to provide one `constructor()` function for the `new` keyword.
            // For opaques, we need the constructor to ensure that a pointer has actually been allocated.
            // For structs, we want to allow the user to set their own values in the struct.
            attr_validator.support.constructors = false;

            let tcx = match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(context) => context,
                Err(e) => {
                    eprintln!("Lowering AST to HIR for Javascript backend failed:");
                    for (ctx, err) in e {
                        eprintln!("\tLowering error in {ctx}: {err}");
                    }
                    std::process::exit(-1);
                }
            };
            match js2::JSGenerationContext::run(&tcx, docs_url_gen, strip_prefix) {
                Ok(mut files) => {
                    out_texts = files.take_files()
                },
                Err(errors) => {
                    eprintln!("Found errors whilst generating {target_language}:");
                    for error in errors {
                        eprintln!("\t{}: {}", error.0, error.1);
                    }
                    errors_found = true;
                }
            };
        }
        "js" => js::gen_bindings(&env, &mut out_texts, Some(docs_url_gen)).unwrap(),
        "demo-gen" => {
            let mut attr_validator = hir::BasicAttributeValidator::new("demo-gen");

            // For finding default constructors of opaques:
            attr_validator.support.constructors = true;
            attr_validator.support.fallible_constructors = true;
            
            let tcx = match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(context) => context,
                Err(e) => {
                    eprintln!("Lowering AST to HIR for Demo Generator backend failed:");
                    for (ctx, err) in e {
                        eprintln!("\tLowering error in {ctx}: {err}");
                    }
                    std::process::exit(-1);
                }
            };

            match demo_gen::WebDemoGenerationContext::run(&tcx, docs_url_gen, strip_prefix) {
                Ok(mut files) => {
                    out_texts = files.take_files();
                },
                Err(errors) => {
                    eprintln!("Found errors whilst generating {target_language}:");
                    for error in errors {
                        eprintln!("\t{}: {}", error.0, error.1);
                    }
                    errors_found = true;
                }
            };
        },
        "kotlin" => {
            let mut attr_validator = hir::BasicAttributeValidator::new("kotlin");
            attr_validator.support.renaming = true;
            attr_validator.support.disabling = true;
            let tcx = match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(context) => context,

                Err(e) => {
                    for (ctx, err) in e {
                        eprintln!("Lowering error in {ctx}: {err}");
                    }
                    std::process::exit(1);
                }
            };
            out_texts = kotlin::run(&tcx, library_config).take_files();
        }
        "dart" => {
            let mut attr_validator = hir::BasicAttributeValidator::new("dart");
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
            let tcx = match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(context) => context,
                Err(e) => {
                    for (ctx, err) in e {
                        eprintln!("Lowering error in {ctx}: {err}");
                    }
                    std::process::exit(1);
                }
            };
            match dart::run(&tcx, docs_url_gen, strip_prefix) {
                Ok(mut files) => out_texts = files.take_files(),
                Err(errors) => {
                    eprintln!("Found errors whilst generating {target_language}:");
                    for error in errors {
                        eprintln!("\t{}: {}", error.0, error.1);
                    }
                    errors_found = true;
                }
            };
        }
        "c" => c::gen_bindings(&env, &mut out_texts).unwrap(),
        "cpp" => {
            c::gen_bindings(&env, &mut out_texts).unwrap();
            cpp::gen_bindings(&env, library_config, docs_url_gen, &mut out_texts).unwrap()
        }
        "dotnet" => {
            dotnet::gen_bindings(&env, library_config, docs_url_gen, &mut out_texts).unwrap()
        }
        "c2" | "cpp-c2" | "cpp2" => {
            let mut attr_validator = hir::BasicAttributeValidator::new(target_language);

            if target_language == "c2" {
                attr_validator.other_backend_names.push("c".into());
            } else {
                attr_validator.other_backend_names.push("cpp".into());
                // C backends cannot rename types using backend attributes
                // In the future we may add a c_rename attribute
                attr_validator.support.renaming = true;

                attr_validator.support.namespacing = true;
            }

            attr_validator.support.memory_sharing = true;
            attr_validator.support.disabling = true;
            // cpp-c2 is a testing backend, we're not going to treat it as a real c/cpp backend
            // since the ast-cpp backend doesn't know about attributes.

            let tcx = match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(context) => context,
                Err(e) => {
                    for (ctx, err) in e {
                        eprintln!("Lowering error in {ctx}: {err}");
                    }
                    std::process::exit(1);
                }
            };
            let files = common::FileMap::default();
            let mut context = c2::CContext::new(&tcx, files);
            context.run();

            let errors = context.errors.take_all();

            if !errors.is_empty() {
                eprintln!("Found errors whilst generating {target_language}:");
                for error in errors {
                    eprintln!("\t{}: {}", error.0, error.1);
                }
                errors_found = true;
            }

            out_texts = context.files.take_files();

            if target_language == "cpp-c2" {
                cpp::gen_bindings(&env, library_config, docs_url_gen, &mut out_texts).unwrap()
            }
            if target_language == "cpp2" {
                let files = common::FileMap::default();
                let mut context = cpp2::Cpp2Context::new(&tcx, files);
                context.run();
                out_texts.extend(context.files.take_files());

                let errors = context.errors.take_all();

                if !errors.is_empty() {
                    eprintln!("Found errors whilst generating {target_language}:");
                    for error in errors {
                        eprintln!("\t{}: {}", error.0, error.1);
                    }
                    errors_found = true;
                }
            }
        }
        o => panic!("Unknown target: {}", o),
    }

    if errors_found {
        eprintln!("Not generating files due to errors");
        // Eventually this should use eyre or something
        std::process::exit(1);
    }

    if !silent {
        println!(
            "{}",
            format!("Generating {} bindings:", target_language)
                .green()
                .bold()
        );
    }

    for (subpath, text) in out_texts {
        let out_path = out_folder.join(subpath);
        let parent = out_path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();
        let mut out_file = File::create(&out_path)?;
        out_file.write_all(text.as_bytes())?;
        if !silent {
            println!("{}", format!("  {}", out_path.display()).dimmed());
        }
    }

    if let Some(docs_out_folder) = docs_out_folder {
        if !silent {
            println!(
                "{}",
                format!("Generating {} docs:", target_language)
                    .green()
                    .bold()
            );
        }

        let mut docs_out_texts: HashMap<String, String> = HashMap::new();

        match target_language {
            "js" => js::docs::gen_docs(&env, &mut docs_out_texts, docs_url_gen).unwrap(),
            "cpp" | "cpp-c2" => {
                cpp::docs::gen_docs(&env, library_config, &mut docs_out_texts, docs_url_gen)
                    .unwrap()
            }
            "c" => todo!("Docs generation for C"),
            "dotnet" => todo!("Docs generation for .NET?"),
            o => panic!("Unknown target: {}", o),
        }

        for (subpath, text) in docs_out_texts {
            let out_path = docs_out_folder.join(subpath);
            let mut out_file = File::create(&out_path)?;
            out_file.write_all(text.as_bytes())?;
            if !silent {
                println!("{}", format!("  {}", out_path.display()).dimmed());
            }
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
