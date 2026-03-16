mod filters;

use std::io;

use askama::Template;
use clap::{Parser, Subcommand};
use diplomat_core::hir::BackendAttrSupport;
use mdbook_preprocessor::book::{BookItem, Chapter};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::Preprocessor;

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
    fn find_languages_item(items: &mut Vec<BookItem>) -> Option<&mut Chapter> {
        for i in items {
            if let BookItem::Chapter(c) = i {
                if c.path == Some(std::path::PathBuf::from("backends/intro.md")) {
                    return Some(c);
                } else if let Some(c) = Self::find_languages_item(&mut c.sub_items) {
                    return Some(c);
                }
            }
        }
        None
    }

    fn generate_language_supports(language: &str, chapter: &mut Chapter) {
        #[derive(Template)]
        #[template(path="supports.md.jinja")]
        struct SupportsBlock {
            attr_support : BackendAttrSupport,
        }
        chapter.content.push('\n');
        chapter.content.push_str(&SupportsBlock { attr_support: diplomat_tool::get_supported(language) }.render().expect("Could not render supports block."));
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
        let mut languages = vec!["c", "cpp", "dart", "demo_gen", "js", "kotlin", "nanobind"];

        let book_item: &mut Chapter = Self::find_languages_item(&mut book.items).unwrap();
        for i in &mut book_item.sub_items {
            if let BookItem::Chapter(c) = i {
                if let Some(p) = c.path.clone() {
                    let name = p.file_stem().unwrap().to_str().unwrap();
                    let language = languages.iter().position(|i| *i == name.to_lowercase());
                    if let Some(l) = language {
                        languages.remove(l);
                        Self::generate_language_supports(name, c);
                    }
                }
            }
        }
        // Create new pages:
        for l in languages {
            let mut nm = book_item.number.clone().unwrap();
            nm.push(book_item.sub_items.len() as u32);
            let mut ch = Chapter {
                name: l.to_string(),
                parent_names: vec![book_item.name.clone()],
                path: Some(format!("backends/{l}").into()),
                number: Some(nm),
                content: format!("# {l}"),
                ..Default::default()
            };
            Self::generate_language_supports(l, &mut ch);
            book_item.sub_items.push(BookItem::Chapter(ch));
        }
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
