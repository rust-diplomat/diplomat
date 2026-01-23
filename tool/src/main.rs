use clap::Parser;
use std::path::PathBuf;

use diplomat_tool::config::Config;

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

    /// What features (`#[diplomat::attr(feature=)]`) are supported.
    /// Backend-specific configs set elsewhere can override this.
    #[arg(long, value_parser, action=clap::ArgAction::Append)]
    features_enabled: Vec<String>,

    #[clap(short = 's', long)]
    silent: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::parse();

    // -- Config Parsing --

    // Read file:

    let path = opt.config_file;

    let mut config = Config::default();
    config.read_file(&path).expect("Error loading config");

    // Read CLI:
    config.read_cli_settings(opt.config);

    config.shared_config.features_enabled = opt.features_enabled.iter().cloned().collect();

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
