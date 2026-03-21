mod filters;

use std::{
    collections::HashMap,
    io,
    panic::catch_unwind,
    sync::{LazyLock, RwLock},
};

use askama::Template;
use clap::{Parser, Subcommand};
use diplomat_core::hir::BackendAttrSupport;
use diplomat_tool::{config::Config, DocsUrlGenerator};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::Preprocessor;
use minijinja::{context, value::Kwargs, Environment, State};
use tempfile::tempdir;

#[derive(Debug, Subcommand)]
enum Supports {
    Supports {
        #[arg()]
        renderer: String,
    },
}

#[derive(Debug, Parser)]
#[clap(name = "mdbook-diplomat", about = "Diplomat mdbook preprocessor")]
struct Opt {
    #[command(subcommand)]
    supports: Option<Supports>,
}

fn main() {
    let opt = Opt::parse();

    if opt.supports.is_some() {
        return;
    }

    handle_preprocessing().expect("Could not preprocess book");
}

struct Module {
    /// Key is file path relative to out dir, result is the file contents.
    files: HashMap<String, String>,
}

struct ModuleTracker {
    modules: HashMap<String, Module>,
}

impl ModuleTracker {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
}

static MODULES: LazyLock<RwLock<ModuleTracker>> =
    LazyLock::new(|| RwLock::new(ModuleTracker::new()));

struct DiplomatPreprocessor;

const LANGUAGES: [&str; 7] = ["c", "cpp", "dart", "demo_gen", "js", "kotlin", "nanobind"];

impl DiplomatPreprocessor {
    fn generate_language_supports(language: &str) -> String {
        #[derive(Template)]
        #[template(path = "supports.md.jinja")]
        struct SupportsBlock {
            attr_support: BackendAttrSupport,
        }
        SupportsBlock {
            attr_support: diplomat_tool::get_supported(language),
        }
        .render()
        .expect("Could not render supports block.")
    }

    fn get_which_languages_supports(attr: &str) -> String {
        let mut language_list = Vec::new();
        for l in LANGUAGES {
            let supports = diplomat_tool::get_supported(l);
            if supports.check_string(attr).unwrap_or(false) {
                language_list.push(l);
            }
        }

        #[derive(Template)]
        #[template(path = "supported_by.md.jinja")]
        struct LanguagesSupported<'a> {
            languages: Vec<&'a str>,
            supports_query: &'a str,
        }

        LanguagesSupported {
            languages: language_list,
            supports_query: attr,
        }
        .render()
        .expect("Could not render languages that this is supported by.")
    }

    fn add_module_dir(dir_path: &std::path::Path, base_module: &mut Module) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir_path)? {
            let dir_entry = entry?;
            let ty = dir_entry.file_type()?;
            if ty.is_file() {
                let pth = dir_entry.path();
                let file_contents = String::from_utf8(std::fs::read(&pth)?)
                    .map_err(|e| std::io::Error::other(e.to_string()))?;

                let diff = pathdiff::diff_paths(&pth, dir_path);

                if let Some(pth) = diff {
                    base_module
                        .files
                        .insert(pth.to_string_lossy().to_string(), file_contents);
                } else {
                    return Err(std::io::Error::other(format!(
                        "Could not get diff paths between {pth:?} and {dir_path:?}"
                    )));
                }
            }
            // TODO: Add submodule reading.
        }
        Ok(())
    }

    fn generate_module(
        state: &State,
        language: &str,
        key: String,
        kwargs: Kwargs,
    ) -> std::io::Result<String> {
        let tmp = tempdir()?;
        let entry = tmp.path().join("lib.rs");
        let caller = kwargs
            .get::<minijinja::Value>("caller")
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        let contents = caller
            .call(state, minijinja::args!())
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        let contents_str = contents.to_string();
        std::fs::write(&entry, &contents_str)?;

        let out_dir = tmp.path().join("out");
        std::fs::create_dir(&out_dir)?;
        let res = catch_unwind(|| {
            diplomat_tool::gen(
                &entry,
                language,
                &out_dir,
                &DocsUrlGenerator::with_base_urls(None, HashMap::new()),
                Config {
                    ..Default::default()
                },
                true,
            )
        });

        let tool_res = res.map_err(|e| {
            std::io::Error::other(if let Ok(msg) = e.downcast::<String>() {
                msg.to_string()
            } else {
                "Unknown Error".to_string()
            })
        })?;

        tool_res.map_err(|e| std::io::Error::other(e.to_string()))?;

        let mut modules = MODULES
            .write()
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        let existing_mod = if let Some(m) = modules.modules.get_mut(&key) {
            m
        } else {
            modules.modules.insert(
                key.clone(),
                Module {
                    files: HashMap::new(),
                },
            );
            modules.modules.get_mut(&key).unwrap()
        };

        Self::add_module_dir(&out_dir, existing_mod)?;
        Ok(contents_str)
    }

    fn get_module_file(module_name: String, file_path: String) -> Result<String, String> {
        let modules = MODULES.read().map_err(|e| e.to_string())?;

        if let Some(m) = modules.modules.get(&module_name) {
            if let Some(f) = m.files.get(&file_path) {
                Ok(f.clone())
            } else {
                Err(format!("Could not find file {file_path} in module {module_name}. Available files: {:?}", m.files.keys()))
            }
        } else {
            Err(format!("Could not find module {module_name}"))
        }
    }
}

impl Preprocessor for DiplomatPreprocessor {
    fn name(&self) -> &str {
        "mdbook-diplomat"
    }

    fn run(
        &self,
        _ctx: &mdbook_preprocessor::PreprocessorContext,
        mut book: mdbook_preprocessor::book::Book,
    ) -> mdbook_preprocessor::errors::Result<mdbook_preprocessor::book::Book> {
        let mut env = Environment::new();
        env.add_function("supports", Self::generate_language_supports);
        env.add_function("get_supports", Self::get_which_languages_supports);
        env.add_function(
            "generate_module",
            |state: &State, language: &str, key: String, kwargs: Kwargs| -> String {
                let res = Self::generate_module(state, language, key, kwargs);
                if let Ok(c) = res {
                    c
                } else {
                    format!("\n{}", res.err().unwrap())
                }
            },
        );
        env.add_function(
            "get_module_file",
            |module_name: String, file_path: String| -> String {
                let res = Self::get_module_file(module_name, file_path);
                if let Ok(c) = res {
                    c
                } else {
                    res.err().unwrap()
                }
            },
        );
        // Evaluate each page as an askama template:
        book.for_each_chapter_mut(|ch| {
            let expr = env
                .template_from_named_str(&ch.name, &ch.content)
                .expect("Could not compile book expression.");
            ch.content = expr
                .render(context! {})
                .expect("Could not render template.");
        });
        Ok(book)
    }
}

fn handle_preprocessing() -> Result<()> {
    let pre = DiplomatPreprocessor;
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}
