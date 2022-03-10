use super::Path;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::Attribute;

pub fn get_doc_lines(attrs: &[Attribute]) -> String {
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

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct RustLink {
    path: Path,
    typ: DocType,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
enum DocType {
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
    Macro,
}

pub fn get_rust_link(attrs: &[Attribute]) -> Option<RustLink> {
    for attr in attrs {
        if attr.path.to_token_stream().to_string() == "diplomat :: rust_link" {
            if let Ok(syn::Meta::List(syn::MetaList { nested, .. })) = attr.parse_meta() {
                if nested.len() == 2 {
                    if let (
                        syn::NestedMeta::Meta(syn::Meta::Path(path)),
                        syn::NestedMeta::Meta(syn::Meta::Path(typ)),
                    ) = (&nested[0], &nested[1])
                    {
                        if let Some(typ) = typ.get_ident() {
                            return Some(RustLink {
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
                            });
                        }
                    }
                }
            }
            panic!("Malformed attribute: {}", attr.to_token_stream());
        }
    }

    None
}

impl RustLink {
    pub fn http(&self, base_urls: &HashMap<String, String>) -> String {
        use DocType::*;

        let mut r = String::new();

        if let Some(base) = base_urls.get(&self.path.elements[0]) {
            r.push_str(base);
        } else {
            r.push_str("https://docs.rs/");
            r.push_str(&self.path.elements[0]);
            r.push_str("/latest/");
        }

        let mut elements = self.path.elements.iter().peekable();

        let module_depth = self.path.elements.len()
            - match self.typ {
                Mod => 0,
                Struct | Enum | Trait | Fn | Macro | Constant => 1,
                FnInEnum | FnInStruct | FnInTrait | DefaultFnInTrait | EnumVariant
                | StructField => 2,
                EnumVariantField => 3,
            };

        for _ in 0..module_depth {
            r.push_str(elements.next().unwrap());
            r.push('/');
        }

        if elements.peek() == None {
            r.push_str("index.html");
            return r;
        }

        r.push_str(match self.typ {
            Struct | StructField | FnInStruct => "struct.",
            Enum | EnumVariant | EnumVariantField | FnInEnum => "enum.",
            Trait | FnInTrait | DefaultFnInTrait => "trait.",
            Fn => "fn.",
            Constant => "constant.",
            Macro => "macro.",
            Mod => unreachable!(),
        });

        r.push_str(elements.next().unwrap());

        r.push_str(".html");

        match self.typ {
            FnInStruct | FnInEnum | DefaultFnInTrait => {
                r.push_str("#method.");
                r.push_str(elements.next().unwrap());
            }
            FnInTrait => {
                r.push_str("#tymethod.");
                r.push_str(elements.next().unwrap());
            }
            EnumVariant => {
                r.push_str("#variant.");
                r.push_str(elements.next().unwrap());
            }
            StructField => {
                r.push_str("#structfield.");
                r.push_str(elements.next().unwrap());
            }
            EnumVariantField => {
                r.push_str("#variant.");
                r.push_str(elements.next().unwrap());
                r.push_str(".field.");
                r.push_str(elements.next().unwrap());
            }
            _ => {}
        }
        r
    }
}

#[test]
fn test_rust_link_http() {
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

    let mut base_urls = HashMap::new();

    for (attr, expected) in test_cases.clone() {
        assert_eq!(get_rust_link(&[attr]).unwrap().http(&base_urls), expected,);
    }

    base_urls.insert("std".to_string(), "http://std-docs.biz/".to_string());

    assert_eq!(
        get_rust_link(&[test_cases[0].0.clone()])
            .unwrap()
            .http(&base_urls),
        "http://std-docs.biz/std/foo/bar/struct.batz.html"
    );
}
