use std::collections::HashMap;

use super::{methods::Method, types::TypeName};

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, TypeName>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemStruct> for Struct {
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
