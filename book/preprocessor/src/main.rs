mod filters;

use std::io;

use askama::Template;
use clap::{Parser, Subcommand};
use diplomat_core::hir::BackendAttrSupport;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::Preprocessor;
use minijinja::{Environment, context};

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

struct DiplomatPreprocessor;

impl DiplomatPreprocessor {
    fn generate_language_supports(language: &str) -> String {
        #[derive(Template)]
        #[template(path="supports.md.jinja")]
        struct SupportsBlock {
            attr_support : BackendAttrSupport,
        }
        SupportsBlock { attr_support: diplomat_tool::get_supported(language) }.render().expect("Could not render supports block.")
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
        // Evaluate each page as an askama template:
        book.for_each_chapter_mut(|ch| {
            let expr = env.template_from_named_str(&ch.name, &ch.content).expect("Could not compile book expression.");
            ch.content = expr.render(context! {}).expect("Could not render template.");
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
