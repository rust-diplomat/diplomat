use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::{Ident, Method};

/// A fieldless enum declaration in an FFI module.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Enum {
    pub name: Ident,
    pub docs: Docs,
    /// A list of variants of the enum. (name, discriminant, docs)
    pub variants: Vec<(Ident, isize, Docs)>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemEnum> for Enum {
    /// Extract an [`Enum`] metadata value from an AST node.
    fn from(enm: &syn::ItemEnum) -> Enum {
        let mut last_discriminant = -1;
        if !enm.generics.params.is_empty() {
            // Generic types are not allowed.
            // Assuming all enums cannot have lifetimes? We don't even have a
            // `lifetimes` field. If we change our minds we can adjust this later
            // and update the `CustomType::lifetimes` API accordingly.
            panic!("Enums cannot have generic parameters");
        }
        Enum {
            name: (&enm.ident).into(),
            docs: Docs::from_attrs(&enm.attrs),
            variants: enm
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
                        (&v.ident).into(),
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
