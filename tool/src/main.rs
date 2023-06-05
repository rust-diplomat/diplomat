use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use colored::*;
use diplomat_core::{ast, hir};
use diplomat_tool::{c, c2, common, cpp, cpp2, dotnet, js};

/// diplomat-tool CLI options, as parsed by [clap-derive].
#[derive(Debug, Parser)]
#[clap(
    name = "diplomat-tool",
    about = "Generate bindings to a target language"
)]
struct Opt {
    /// The target language, "js", "c", "cpp" or "dotnet" (C#).
    #[clap()]
    target_language: String,

    /// The folder that stores the bindings.
    #[clap(value_parser)]
    out_folder: PathBuf,

    #[clap(short, long, value_parser)]
    docs: Option<PathBuf>,

    #[clap(short = 'u', long)]
    docs_base_urls: Vec<String>,

    /// The path to the lib.rs file. Defaults to `src/lib.rs`.
    #[clap(short, long, value_parser)]
    entry: Option<PathBuf>,

    /// The path to an optional config file to override code generation defaults.
    /// This is currently used by the cpp generator to allow for code to be
    /// different libraries.
    #[clap(short, long, value_parser)]
    library_config: Option<PathBuf>,

    #[clap(short = 's', long)]
    silent: bool,
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

fn main() -> std::io::Result<()> {
    let opt = Opt::parse();
    let is_custom_entry = opt.entry.is_some();
    let path = opt.entry.unwrap_or_else(|| PathBuf::from("src/lib.rs"));

    // Check that user-provided paths exist. Exit early with a nice error message
    // if anything doesn't exist.
    exit_if_path_missing(
        &path,
        if is_custom_entry {
            "The entry file specified by --entry does not exist."
        } else {
            "Could not find the lib.rs file to process. Set it manually with the --entry option."
        },
    );
    exit_if_path_missing(
        &opt.out_folder,
        "The out folder (the second argument) does not exist.",
    );
    if let Some(ref docs) = opt.docs {
        exit_if_path_missing(docs, "The docs folder specified by --docs does not exist.");
    }
    if let Some(ref library_config) = opt.library_config {
        exit_if_path_missing(
            library_config,
            "The library configuration file specified by --library-config does not exist.",
        );
    }

    let lib_file = syn_inline_mod::parse_and_inline_modules(path.as_path());
    let diplomat_file = ast::File::from(&lib_file);
    let env = diplomat_file.all_types();

    let errors = diplomat_file.check_validity(&env);
    if !errors.is_empty() {
        for e in errors {
            eprintln!("{e}");
        }
        panic!();
    }

    let docs_url_gen = ast::DocsUrlGenerator::with_base_urls(
        opt.docs_base_urls
            .iter()
            .filter_map(|entry| entry.strip_prefix("*:").map(ToString::to_string))
            .next(),
        opt.docs_base_urls
            .iter()
            .filter(|entry| !entry.starts_with('*'))
            .map(|entry| {
                let mut parts = entry.splitn(2, ':');
                (
                    parts.next().unwrap().to_string(),
                    parts
                        .next()
                        .expect("Expected syntax <crate>|*:<url>")
                        .to_string(),
                )
            })
            .collect(),
    );

    let mut out_texts: HashMap<String, String> = HashMap::new();

    let target_language = opt.target_language.as_str();
    let mut errors_found = false;

    match target_language {
        "js" => js::gen_bindings(&env, &mut out_texts, Some(&docs_url_gen)).unwrap(),
        "c" => c::gen_bindings(&env, &mut out_texts).unwrap(),
        "cpp" => {
            c::gen_bindings(&env, &mut out_texts).unwrap();
            cpp::gen_bindings(&env, &opt.library_config, &docs_url_gen, &mut out_texts).unwrap()
        }
        "dotnet" => {
            dotnet::gen_bindings(&env, &opt.library_config, &docs_url_gen, &mut out_texts).unwrap()
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
            }

            attr_validator.support.disabling = true;
            // cpp-c2 is a testing backend, we're not going to treat it as a real c/cpp backend
            // since the ast-cpp backend doesn't know about attributes.

            let tcx = match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(context) => context,
                Err(e) => {
                    for err in e {
                        eprintln!("Lowering error: {err}");
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
                cpp::gen_bindings(&env, &opt.library_config, &docs_url_gen, &mut out_texts).unwrap()
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
        process::exit(1);
    }

    if !opt.silent {
        println!(
            "{}",
            format!("Generating {} bindings:", opt.target_language)
                .green()
                .bold()
        );
    }

    for (subpath, text) in out_texts {
        let out_path = opt.out_folder.join(subpath);
        let mut out_file = File::create(&out_path)?;
        out_file.write_all(text.as_bytes())?;
        if !opt.silent {
            println!("{}", format!("  {}", out_path.display()).dimmed());
        }
    }

    if let Some(docs) = opt.docs {
        if !opt.silent {
            println!(
                "{}",
                format!("Generating {} docs:", opt.target_language)
                    .green()
                    .bold()
            );
        }

        let mut docs_out_texts: HashMap<String, String> = HashMap::new();

        match opt.target_language.as_str() {
            "js" => js::docs::gen_docs(&env, &mut docs_out_texts, &docs_url_gen).unwrap(),
            "cpp" | "cpp-c2" => cpp::docs::gen_docs(
                &env,
                &opt.library_config,
                &mut docs_out_texts,
                &docs_url_gen,
            )
            .unwrap(),
            "c" => todo!("Docs generation for C"),
            "dotnet" => todo!("Docs generation for .NET?"),
            o => panic!("Unknown target: {}", o),
        }

        for (subpath, text) in docs_out_texts {
            let out_path = docs.join(subpath);
            let mut out_file = File::create(&out_path)?;
            out_file.write_all(text.as_bytes())?;
            if !opt.silent {
                println!("{}", format!("  {}", out_path.display()).dimmed());
            }
        }
    }

    Ok(())
}
