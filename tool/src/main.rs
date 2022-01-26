use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use colored::*;
use diplomat_core::ast;
use diplomat_tool::{c, cpp, dotnet, js};
use structopt::StructOpt;

/// diplomat-tool CLI options, as parsed by [structopt].
#[derive(Debug, StructOpt)]
#[structopt(
    name = "diplomat-tool",
    about = "Generate bindings to a target language"
)]
struct Opt {
    /// The target language, "js", "c", "cpp" or "dotnet" (C#).
    #[structopt()]
    target_language: String,

    /// The folder that stores the bindings.
    #[structopt(parse(from_os_str))]
    out_folder: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    docs: Option<PathBuf>,

    /// The path to the lib.rs file. Defaults to src/lib.rs
    #[structopt(short, long, parse(from_os_str))]
    entry: Option<PathBuf>,

    /// The path to an optional config file to override code generation defaults.
    /// This is currently used by the cpp generator to allow for code to be
    /// different libraries.
    #[structopt(short, long, parse(from_os_str))]
    library_config: Option<PathBuf>,
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
    let opt = Opt::from_args();
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
            eprintln!("{}", e);
        }
        panic!();
    }

    let mut out_texts: HashMap<String, String> = HashMap::new();

    match opt.target_language.as_str() {
        "js" => js::gen_bindings(&env, &mut out_texts).unwrap(),
        "c" => c::gen_bindings(&env, &mut out_texts).unwrap(),
        "cpp" => cpp::gen_bindings(&env, &opt.library_config, &mut out_texts).unwrap(),
        "dotnet" => dotnet::gen_bindings(&env, &opt.library_config, &mut out_texts).unwrap(),
        o => panic!("Unknown target: {}", o),
    }

    println!(
        "{}",
        format!("Generating {} bindings:", opt.target_language)
            .green()
            .bold()
    );

    for (subpath, text) in out_texts {
        let out_path = opt.out_folder.join(subpath);
        let mut out_file = File::create(&out_path)?;
        out_file.write_all(text.as_bytes())?;
        println!("{}", format!("  {}", out_path.display()).dimmed());
    }

    if let Some(docs) = opt.docs {
        println!(
            "{}",
            format!("Generating {} docs:", opt.target_language)
                .green()
                .bold()
        );
        let mut docs_out_texts: HashMap<String, String> = HashMap::new();

        match opt.target_language.as_str() {
            "js" => js::docs::gen_docs(&env, &mut docs_out_texts).unwrap(),
            "cpp" => cpp::docs::gen_docs(&env, &opt.library_config, &mut docs_out_texts).unwrap(),
            "c" => todo!("Docs generation for C"),
            "dotnet" => todo!("Docs generation for .NET?"),
            o => panic!("Unknown target: {}", o),
        }

        for (subpath, text) in docs_out_texts {
            let out_path = docs.join(subpath);
            let mut out_file = File::create(&out_path)?;
            out_file.write_all(text.as_bytes())?;
            println!("{}", format!("  {}", out_path.display()).dimmed());
        }
    }

    Ok(())
}
