use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::Method;

/// A fieldless enum declaration in an FFI module.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub docs: Docs,
    /// A list of variants of the enum. (name, discriminant, docs)
    pub variants: Vec<(String, isize, Docs)>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemEnum> for Enum {
    /// Extract an [`Enum`] metadata value from an AST node.
    fn from(strct: &syn::ItemEnum) -> Enum {
        let mut last_discriminant = -1;
        Enum {
            name: strct.ident.to_string(),
            docs: Docs::from_attrs(&strct.attrs),
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
                        Docs::from_attrs(&v.attrs),
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
                #[diplomat::rust_link(foo::Bar, Enum)]
                enum MyLocalEnum {
                    Abc,
                    /// Some more docs.
                    Def
                }
            }));
        });
    }
}
