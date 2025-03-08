use clap::Parser;
use std::path::PathBuf;
use toml::value::Table;

use diplomat_tool::config::{toml_value_from_str, Config};

/// diplomat-tool CLI options, as parsed by [clap-derive].
#[derive(Debug, Parser)]
#[clap(
    name = "diplomat-tool",
    about = "Generate bindings to a target language"
)]
struct Opt {
    /// The target language, "c", "cpp", "js", "demo_gen", or "kotlin" (JVM)
    #[clap()]
    target_language: String,

    /// The folder that stores the bindings.
    #[clap(value_parser)]
    out_folder: PathBuf,

    #[clap(short = 'u', long)]
    docs_base_urls: Vec<String>,

    /// The path to the lib.rs file.
    #[clap(short, long, value_parser, default_value = "src/lib.rs")]
    entry: PathBuf,

    /// The path to an optional config file to override code generation defaults.
    /// This is where [`config::Config`] is filled in.
    ///
    /// We assume by default that this is located in the root directory.
    #[clap(short, long, value_parser, default_value = "config.toml")]
    config_file: PathBuf,

    #[arg(long, value_parser, action=clap::ArgAction::Append)]
    config: Vec<String>,

    #[clap(short = 's', long)]
    silent: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::parse();

    // -- Config Parsing --

    // Read file:
    let path = opt.config_file;
    let config_table: Table = if path.exists() {
        let file_buf = std::fs::read(path)?;
        toml::from_slice(&file_buf)?
    } else {
        Table::default()
    };

    let mut config = Config::default();

    for (key, value) in config_table {
        // Quick way to take config.toml from kebab to snake case.
        // This technically means that someone could also just as easily do CamelCase and have it translated,
        // but I'm not sure I want to bother writing validation code for such a scenario.
        let key = heck::AsSnakeCase(key).to_string();
        if let toml::Value::Table(t) = value {
            for (subkey, subvalue) in t {
                let subkey = heck::AsSnakeCase(subkey).to_string();
                config.set(&format!("{}.{}", key, subkey), subvalue);
            }
        } else {
            config.set(&key, value);
        }
    }

    // Read CLI:
    for c in opt.config {
        let split = c.split_once("=");
        if let Some((key, value)) = split {
            config.set(key, toml_value_from_str(value));
        } else {
            eprintln!("Could not read {c}, expected =");
        }
    }

    // -- Config Parsing --

    diplomat_tool::gen(
        &opt.entry,
        &opt.target_language,
        &opt.out_folder,
        &diplomat_core::hir::DocsUrlGenerator::with_base_urls(
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
        ),
        config,
        opt.silent,
    )
}
