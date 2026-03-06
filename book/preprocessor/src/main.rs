use std::io;

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
        let BackendAttrSupport {
            namespacing,
            memory_sharing,
            non_exhaustive_structs,
            method_overloading,
            utf8_strings,
            utf16_strings,
            static_slices,
            defaults,

            constructors,
            named_constructors,
            fallible_constructors,
            accessors,
            static_accessors,
            stringifiers,
            comparators,
            iterators,
            iterables,
            indexing,
            arithmetic,
            option,
            callbacks,
            traits,
            custom_errors,
            traits_are_send,
            traits_are_sync,
            generate_mocking_interface,
            abi_compatibles,
            struct_refs,
            free_functions,
            custom_bindings,
            owned_slices,
            default_args,
            ..
        } = diplomat_tool::get_supported(language);
        chapter.content.push_str(
            format!(
                r#"## Supports
- [{namespacing}] Namespaces

- [{memory_sharing}] Memory Sharing

- [{non_exhaustive_structs}] Non Exhaustive Structs

- [{method_overloading}] Method Overloading

- [{utf8_strings}] UTF8 Strings

- [{utf16_strings}] UTF16 Strings

- [{static_slices}] Static Slices

- [{defaults}] Defaults

- [{constructors}] Constructors

- [{named_constructors}] Named Constructors

- [{fallible_constructors}] Fallible Constructors

- [{accessors}] Getters/Setters

- [{static_accessors}] Static Getters/Setters

- [{stringifiers}] Stringifiers

- [{comparators}] Comparators

- [{iterators}] Iterators

- [{iterables}] Iterables

- [{indexing}] Indexers

- [{arithmetic}] Arithmetic

- [{option}] Options

- [{callbacks}] Callbacks

- [{traits}] Traits

- [{custom_errors}] Custom Errors

- [{traits_are_send}] Traits are Send

- [{traits_are_sync}] Traits are Sync

- [{generate_mocking_interface}] Generate Mocking Interface

- [{abi_compatibles}] ABI Compatible Structs

- [{struct_refs}] Struct Refs

- [{free_functions}] Free Functions

- [{custom_bindings}] Custom Bindings

- [{owned_slices}] Owned Slices

- [{default_args}] Default Arguments
"#
            )
            .replace("true", "X")
            .replace("false", " ")
            .as_str(),
        );
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
