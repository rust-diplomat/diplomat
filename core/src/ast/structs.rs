use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::{methods::Method, types::TypeName};

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
