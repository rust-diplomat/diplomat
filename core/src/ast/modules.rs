use std::collections::HashMap;

use quote::ToTokens;
use syn::{ImplItem, Item, ItemMod};

use super::methods::Method;
use super::structs::{OpaqueStruct, Struct};
use super::types::CustomType;

#[derive(Clone, Debug)]
pub struct Module {
    pub declared_types: HashMap<String, CustomType>,
}

impl From<&ItemMod> for Module {
    /// Get all custom types defined in a module as a mapping from their name to
    /// the extracted metadata.
    fn from(input: &ItemMod) -> Module {
        let mut custom_types_by_name = HashMap::new();
        input
            .content
            .as_ref()
            .unwrap()
            .1
            .iter()
            .for_each(|a| match a {
                Item::Struct(strct) => {
                    if strct
                        .attrs
                        .iter()
                        .any(|a| a.path.to_token_stream().to_string() == "diplomat :: opaque")
                    {
                        custom_types_by_name.insert(
                            strct.ident.to_string(),
                            CustomType::Opaque(OpaqueStruct {
                                name: strct.ident.to_string(),
                                methods: vec![],
                            }),
                        );
                    } else {
                        custom_types_by_name.insert(
                            strct.ident.to_string(),
                            CustomType::Struct(Struct::from(strct)),
                        );
                    }
                }
                Item::Impl(ipl) => {
                    assert!(ipl.trait_.is_none());

                    let self_typ = match ipl.self_ty.as_ref() {
                        syn::Type::Path(s) => s,
                        _ => panic!("Self type not found"),
                    };

                    let mut new_methods = ipl
                        .items
                        .iter()
                        .filter_map(|i| match i {
                            ImplItem::Method(m) => Some(Method::from_syn(m, self_typ)),
                            _ => None,
                        })
                        .collect();

                    match custom_types_by_name
                        .get_mut(&self_typ.path.get_ident().unwrap().to_string())
                        .unwrap()
                    {
                        CustomType::Struct(strct) => {
                            strct.methods.append(&mut new_methods);
                        }
                        CustomType::Opaque(strct) => {
                            strct.methods.append(&mut new_methods);
                        }
                    }
                }
                _ => {}
            });

        Module {
            declared_types: custom_types_by_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub modules: HashMap<String, Module>,
}

impl From<&syn::File> for File {
    /// Get all custom types across all modules defined in a given file.
    fn from(file: &syn::File) -> File {
        let mut out = HashMap::new();
        file.items.iter().for_each(|i| {
            if let Item::Mod(item_mod) = i {
                if item_mod
                    .attrs
                    .iter()
                    .any(|a| a.path.to_token_stream().to_string() == "diplomat :: bridge")
                {
                    out.insert(item_mod.ident.to_string(), Module::from(item_mod));
                }
            }
        });

        File { modules: out }
    }
}
