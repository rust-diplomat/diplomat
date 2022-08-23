use super::Path;
use core::fmt;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::Attribute;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Docs(String, Vec<RustLink>);

impl Docs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        Self(Self::get_doc_lines(attrs), Self::get_rust_link(attrs))
    }

    fn get_doc_lines(attrs: &[Attribute]) -> String {
        let mut lines: String = String::new();

        attrs.iter().for_each(|attr| {
            let maybe_ident = attr.path.get_ident();
            if maybe_ident.is_some() && *maybe_ident.unwrap() == "doc" {
                let literal_token = attr.tokens.clone().into_iter().nth(1).unwrap();
                let node: syn::LitStr = syn::parse2(literal_token.to_token_stream()).unwrap();
                let line = node.value().trim().to_string();

                if !lines.is_empty() {
                    lines.push('\n');
                }

                lines.push_str(&line);
            }
        });

        lines
    }

    fn get_rust_link(attrs: &[Attribute]) -> Vec<RustLink> {
        let mut vec = vec![];
        for attr in attrs {
            if attr.path.to_token_stream().to_string() == "diplomat :: rust_link" {
                if let Ok(syn::Meta::List(syn::MetaList { nested, .. })) = attr.parse_meta() {
                    let (path, typ, maybe_display) = if nested.len() == 2 {
                        if let (
                            syn::NestedMeta::Meta(syn::Meta::Path(path)),
                            syn::NestedMeta::Meta(syn::Meta::Path(typ)),
                        ) = (&nested[0], &nested[1])
                        {
                            (path, typ, None)
                        } else {
                            panic!("Malformed attribute: {}", attr.to_token_stream());
                        }
                    } else if nested.len() == 3 {
                        if let (
                            syn::NestedMeta::Meta(syn::Meta::Path(path)),
                            syn::NestedMeta::Meta(syn::Meta::Path(typ)),
                            syn::NestedMeta::Meta(syn::Meta::Path(display)),
                        ) = (&nested[0], &nested[1], &nested[2])
                        {
                            (path, typ, Some(display))
                        } else {
                            panic!("Malformed attribute: {}", attr.to_token_stream());
                        }
                    } else {
                        panic!(
                            "#[diplomat::rust_link()] can take two or three parameters, found: {}",
                            attr.to_token_stream()
                        );
                    };

                    if let Some(typ) = typ.get_ident() {
                        let display = if let Some(display) = maybe_display {
                            if let Some(ident) = display.get_ident() {
                                let s = ident.to_string();
                                match &*s {
                                    "normal" => RustLinkDisplay::Normal,
                                    "compact" => RustLinkDisplay::Compact,
                                    "hidden" => RustLinkDisplay::Hidden,
                                    _ => panic!("#[diplomat::rust_link()]'s third argument must be `normal`, `compact`, \
                                                 or `hidden`, found: {}", display.to_token_stream()),
                                }
                            } else {
                                panic!("#[diplomat::rust_link()]'s third argument must be a path, found: {}", display.to_token_stream());
                            }
                        } else {
                            RustLinkDisplay::Normal
                        };
                        vec.push(RustLink {
                            path: Path::from_syn(path),
                            typ: match typ.to_token_stream().to_string().as_str() {
                                "Struct" => DocType::Struct,
                                "StructField" => DocType::StructField,
                                "Enum" => DocType::Enum,
                                "EnumVariant" => DocType::EnumVariant,
                                "EnumVariantField" => DocType::EnumVariantField,
                                "Trait" => DocType::Trait,
                                "FnInStruct" => DocType::FnInStruct,
                                "FnInEnum" => DocType::FnInEnum,
                                "FnInTrait" => DocType::FnInTrait,
                                "DefaultFnInTrait" => DocType::DefaultFnInTrait,
                                "Fn" => DocType::Fn,
                                "Mod" => DocType::Mod,
                                "Constant" => DocType::Constant,
                                "Macro" => DocType::Macro,
                                x => panic!("Invalid doc type {:?}", x),
                            },
                            display,
                        });
                        continue;
                    };
                }
                panic!("Malformed attribute: {}", attr.to_token_stream());
            }
        }
        vec
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    /// Convert to markdown
    ///
    /// `in_rst` will avoid using complicated formatting combinations disallowed in ReStructuredText
    pub fn to_markdown(&self, docs_url_gen: &DocsUrlGenerator, in_rst: bool) -> String {
        use std::fmt::Write;
        let mut lines = self.0.clone();
        let mut has_compact = false;
        let backtick = if in_rst {
            ""
        } else {
            "`"
        };
        for rust_link in &self.1 {
            if rust_link.display == RustLinkDisplay::Compact {
                has_compact = true;
            } else if rust_link.display == RustLinkDisplay::Normal {
                write!(
                    lines,
                    "\n\nSee the [Rust documentation for {backtick}{name}{backtick}]({link}) for more information.",
                    name = rust_link.path.elements.last().unwrap(),
                    link = docs_url_gen.gen_for_rust_link(rust_link)
                )
                .unwrap();
            }
        }
        if has_compact {
            write!(lines, "\n\n Additional information: ").unwrap();
            for (i, rust_link) in self
                .1
                .iter()
                .filter(|r| r.display == RustLinkDisplay::Compact)
                .enumerate()
            {
                if i != 0 {
                    write!(lines, ", ").unwrap();
                }
                write!(
                    lines,
                    "`{}`]({})",
                    i + 1,
                    docs_url_gen.gen_for_rust_link(rust_link)
                )
                .unwrap();
            }
        }
        lines
    }

    pub fn rust_link(&self) -> &[RustLink] {
        &self.1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RustLinkDisplay {
    /// A nice expanded representation that includes the type name
    ///
    /// e.g. "See the \[link to Rust documentation\] for more details"
    Normal,
    /// A compact representation that will fit multiple rust_link entries in one line
    ///
    /// E.g. "For further information, see: 1, 2, 3, 4" (all links)
    Compact,
    /// Hidden. Useful for programmatically annotating an API as related without showing a link to the user
    Hidden,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, PartialOrd, Ord)]
pub struct RustLink {
    pub path: Path,
    pub typ: DocType,
    pub display: RustLinkDisplay,
}

impl fmt::Display for RustLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{:?}", self.path, self.typ)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, PartialOrd, Ord)]
pub enum DocType {
    Struct,
    StructField,
    Enum,
    EnumVariant,
    EnumVariantField,
    Trait,
    FnInStruct,
    FnInEnum,
    FnInTrait,
    DefaultFnInTrait,
    Fn,
    Mod,
    Constant,
    AssociatedConstantInEnum,
    AssociatedConstantInTrait,
    AssociatedConstantInStruct,
    Macro,
    AssociatedTypeInEnum,
    AssociatedTypeInTrait,
    AssociatedTypeInStruct,
    Typedef,
}

#[derive(Default)]
pub struct DocsUrlGenerator {
    default_url: Option<String>,
    base_urls: HashMap<String, String>,
}

impl DocsUrlGenerator {
    pub fn with_base_urls(default_url: Option<String>, base_urls: HashMap<String, String>) -> Self {
        Self {
            default_url,
            base_urls,
        }
    }

    fn gen_for_rust_link(&self, rust_link: &RustLink) -> String {
        use DocType::*;

        let mut r = String::new();

        let base = self
            .base_urls
            .get(rust_link.path.elements[0].as_str())
            .map(String::as_str)
            .or(self.default_url.as_deref())
            .unwrap_or("https://docs.rs/");

        r.push_str(base);
        if !base.ends_with('/') {
            r.push('/');
        }
        if r == "https://docs.rs/" {
            r.push_str(rust_link.path.elements[0].as_str());
            r.push_str("/latest/");
        }

        let mut elements = rust_link.path.elements.iter().peekable();

        let module_depth = rust_link.path.elements.len()
            - match rust_link.typ {
                Mod => 0,
                Struct | Enum | Trait | Fn | Macro | Constant | Typedef => 1,
                FnInEnum
                | FnInStruct
                | FnInTrait
                | DefaultFnInTrait
                | EnumVariant
                | StructField
                | AssociatedTypeInEnum
                | AssociatedTypeInStruct
                | AssociatedTypeInTrait
                | AssociatedConstantInEnum
                | AssociatedConstantInStruct
                | AssociatedConstantInTrait => 2,
                EnumVariantField => 3,
            };

        for _ in 0..module_depth {
            r.push_str(elements.next().unwrap().as_str());
            r.push('/');
        }

        if elements.peek() == None {
            r.push_str("index.html");
            return r;
        }

        r.push_str(match rust_link.typ {
            Typedef => "type.",
            Struct
            | StructField
            | FnInStruct
            | AssociatedTypeInStruct
            | AssociatedConstantInStruct => "struct.",
            Enum
            | EnumVariant
            | EnumVariantField
            | FnInEnum
            | AssociatedTypeInEnum
            | AssociatedConstantInEnum => "enum.",
            Trait
            | FnInTrait
            | DefaultFnInTrait
            | AssociatedTypeInTrait
            | AssociatedConstantInTrait => "trait.",
            Fn => "fn.",
            Constant => "constant.",
            Macro => "macro.",
            Mod => unreachable!(),
        });

        r.push_str(elements.next().unwrap().as_str());

        r.push_str(".html");

        match rust_link.typ {
            FnInStruct | FnInEnum | DefaultFnInTrait => {
                r.push_str("#method.");
                r.push_str(elements.next().unwrap().as_str());
            }
            AssociatedTypeInStruct | AssociatedTypeInEnum | AssociatedTypeInTrait => {
                r.push_str("#associatedtype.");
                r.push_str(elements.next().unwrap().as_str());
            }
            AssociatedConstantInStruct | AssociatedConstantInEnum | AssociatedConstantInTrait => {
                r.push_str("#associatedconstant.");
                r.push_str(elements.next().unwrap().as_str());
            }
            FnInTrait => {
                r.push_str("#tymethod.");
                r.push_str(elements.next().unwrap().as_str());
            }
            EnumVariant => {
                r.push_str("#variant.");
                r.push_str(elements.next().unwrap().as_str());
            }
            StructField => {
                r.push_str("#structfield.");
                r.push_str(elements.next().unwrap().as_str());
            }
            EnumVariantField => {
                r.push_str("#variant.");
                r.push_str(elements.next().unwrap().as_str());
                r.push_str(".field.");
                r.push_str(elements.next().unwrap().as_str());
            }
            _ => {}
        }
        r
    }
}

#[test]
fn test_docs_url_generator() {
    let test_cases = [
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Struct)] },
            "https://docs.rs/std/latest/std/foo/bar/struct.batz.html",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, StructField)] },
            "https://docs.rs/std/latest/std/foo/struct.bar.html#structfield.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Enum)] },
            "https://docs.rs/std/latest/std/foo/bar/enum.batz.html",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, EnumVariant)] },
            "https://docs.rs/std/latest/std/foo/enum.bar.html#variant.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, EnumVariantField)] },
            "https://docs.rs/std/latest/std/enum.foo.html#variant.bar.field.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Trait)] },
            "https://docs.rs/std/latest/std/foo/bar/trait.batz.html",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, FnInStruct)] },
            "https://docs.rs/std/latest/std/foo/struct.bar.html#method.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, FnInEnum)] },
            "https://docs.rs/std/latest/std/foo/enum.bar.html#method.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, FnInTrait)] },
            "https://docs.rs/std/latest/std/foo/trait.bar.html#tymethod.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, DefaultFnInTrait)] },
            "https://docs.rs/std/latest/std/foo/trait.bar.html#method.batz",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Fn)] },
            "https://docs.rs/std/latest/std/foo/bar/fn.batz.html",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Mod)] },
            "https://docs.rs/std/latest/std/foo/bar/batz/index.html",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Constant)] },
            "https://docs.rs/std/latest/std/foo/bar/constant.batz.html",
        ),
        (
            syn::parse_quote! { #[diplomat::rust_link(std::foo::bar::batz, Macro)] },
            "https://docs.rs/std/latest/std/foo/bar/macro.batz.html",
        ),
    ];

    for (attr, expected) in test_cases.clone() {
        assert_eq!(
            DocsUrlGenerator::default().gen_for_rust_link(&Docs::from_attrs(&[attr]).1[0]),
            expected
        );
    }

    assert_eq!(
        DocsUrlGenerator::with_base_urls(
            None,
            [("std".to_string(), "http://std-docs.biz/".to_string())]
                .into_iter()
                .collect()
        )
        .gen_for_rust_link(&Docs::from_attrs(&[test_cases[0].0.clone()]).1[0]),
        "http://std-docs.biz/std/foo/bar/struct.batz.html"
    );

    assert_eq!(
        DocsUrlGenerator::with_base_urls(Some("http://std-docs.biz/".to_string()), HashMap::new())
            .gen_for_rust_link(&Docs::from_attrs(&[test_cases[0].0.clone()]).1[0]),
        "http://std-docs.biz/std/foo/bar/struct.batz.html"
    );
}
