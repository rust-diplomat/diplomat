use clap::Parser;
use std::path::PathBuf;
use toml::value::Table;

use diplomat_tool::config::{merge_config, table_from_values, Config};

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
    let mut config_table: Table = if path.exists() {
        let file_buf = std::fs::read(path)?;
        toml::from_slice(&file_buf)?
    } else {
        Table::default()
    };

    // Read CLI:
    let mut key_values = Vec::new();
    for c in opt.config {
        let split = c.split_once("=");
        if let Some((key, value)) = split {
            key_values.push((key.to_string(), value.to_string()));
        } else {
            eprintln!("Could not read {c}, expected =");
        }
    }

    let (cli_config, errors) = table_from_values(key_values);

    for e in errors {
        eprintln!("{e}");
    }

    // CLI takes priority over `config.toml`.
    merge_config(&mut config_table, cli_config);

    // Convert into config (somewhat hacky, need to convert to a string then BACK to the required type):
    let config_parse = toml::from_slice(&toml::to_vec(&toml::Value::Table(config_table)).unwrap());

    if let Err(e) = config_parse {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Could not parse config: {} ", e),
        ));
    }
    let config: Config = config_parse.unwrap();

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
