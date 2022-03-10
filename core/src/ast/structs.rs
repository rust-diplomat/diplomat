use serde::{Deserialize, Serialize};

use super::utils::*;
use super::{Method, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Struct {
    pub name: String,
    pub doc_lines: String,
    pub rust_link: Option<RustLink>,
    /// A list of fields in the struct. (name, type, doc_lines)
    pub fields: Vec<(String, TypeName, String)>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemStruct> for Struct {
    /// Extract a [`Struct`] metadata value from an AST node.
    fn from(strct: &syn::ItemStruct) -> Struct {
        Struct {
            name: strct.ident.to_string(),
            doc_lines: get_doc_lines(&strct.attrs),
            rust_link: get_rust_link(&strct.attrs),
            fields: strct
                .fields
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    (
                        f.ident
                            .as_ref()
                            .map(|i| i.to_string())
                            .unwrap_or(format!("{}", i)),
                        (&f.ty).into(),
                        get_doc_lines(&f.attrs),
                    )
                })
                .collect(),
            methods: vec![],
        }
    }
}

/// A struct annotated with [`diplomat::opaque`] whose fields are not visible.
/// Opaque structs cannot be passed by-value across the FFI boundary, so they
/// must be boxed or passed as references.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct OpaqueStruct {
    pub name: String,
    pub doc_lines: String,
    pub rust_link: Option<RustLink>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemStruct> for OpaqueStruct {
    /// Extract a [`OpaqueStruct`] metadata value from an AST node.
    fn from(strct: &syn::ItemStruct) -> OpaqueStruct {
        OpaqueStruct {
            name: strct.ident.to_string(),
            doc_lines: get_doc_lines(&strct.attrs),
            rust_link: get_rust_link(&strct.attrs),
            methods: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::{self, Settings};

    use syn;

    use super::Struct;

    #[test]
    fn simple_struct() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Struct::from(&syn::parse_quote! {
                /// Some docs.
                struct MyLocalStruct {
                    a: i32,
                    b: Box<MyLocalStruct>
                }
            }));
        });
    }
}
