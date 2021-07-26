use serde::{Deserialize, Serialize};

use super::utils::get_doc_lines;
use super::Method;

/// A fieldless enum declaration in an FFI module.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Enum {
    pub name: String,
    pub doc_lines: String,
    /// A list of variants of the enum. (name, discriminant, doc_lines)
    pub variants: Vec<(String, isize, String)>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemEnum> for Enum {
    /// Extract an [`Enum`] metadata value from an AST node.
    fn from(strct: &syn::ItemEnum) -> Enum {
        let mut last_discriminant = -1;
        Enum {
            name: strct.ident.to_string(),
            doc_lines: get_doc_lines(&strct.attrs),
            variants: strct
                .variants
                .iter()
                .map(|v| {
                    let new_discriminant = v
                        .discriminant
                        .as_ref()
                        .map(|d| {
                            if let syn::Expr::Lit(syn::ExprLit {
                                attrs: _,
                                lit: syn::Lit::Int(lit_int),
                            }) = &d.1
                            {
                                lit_int.base10_parse::<isize>().unwrap()
                            } else {
                                panic!("Expected a discriminant to be a constant integer");
                            }
                        })
                        .unwrap_or_else(|| last_discriminant + 1);

                    last_discriminant = new_discriminant;

                    (
                        v.ident.to_string(),
                        new_discriminant,
                        get_doc_lines(&v.attrs),
                    )
                })
                .collect(),
            methods: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::{self, Settings};

    use syn;

    use super::Enum;

    #[test]
    fn simple_enum() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Enum::from(&syn::parse_quote! {
                /// Some docs.
                enum MyLocalEnum {
                    Abc,
                    /// Some more docs.
                    Def
                }
            }));
        });
    }
}
