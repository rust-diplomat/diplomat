use std::collections::HashMap;

use quote::ToTokens;

use super::{methods::Method, types::Type};

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: String,
    pub opaque: bool,
    pub fields: HashMap<String, Type>,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemStruct> for Struct {
    fn from(strct: &syn::ItemStruct) -> Struct {
        if strct
            .attrs
            .iter()
            .any(|a| a.path.to_token_stream().to_string() == "diplomat :: opaque")
        {
            Struct {
                name: strct.ident.to_string(),
                opaque: true,
                fields: HashMap::new(),
                methods: vec![],
            }
        } else {
            Struct {
                name: strct.ident.to_string(),
                opaque: false,
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
}
