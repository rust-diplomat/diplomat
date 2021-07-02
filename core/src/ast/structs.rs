use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Method, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, TypeName>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemStruct> for Struct {
    /// Extract a [`Struct`] metadata value from an AST node.
    fn from(strct: &syn::ItemStruct) -> Struct {
        Struct {
            name: strct.ident.to_string(),
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpaqueStruct {
    pub name: String,
    pub methods: Vec<Method>,
}

#[cfg(test)]
mod tests {
    use insta::{self, Settings};

    use quote::quote;
    use syn;

    use super::Struct;

    #[test]
    fn simple_struct() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Struct::from(
                &syn::parse2(quote! {
                    struct MyLocalStruct {
                        a: i32,
                        b: Box<MyLocalStruct>
                    }
                })
                .unwrap()
            ));
        });
    }
}
