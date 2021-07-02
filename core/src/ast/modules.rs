use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use quote::ToTokens;
use syn::{ImplItem, Item, ItemMod};

use super::{CustomType, Method, OpaqueStruct, Struct, TypeName};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Module {
    pub declared_types: HashMap<String, CustomType>,
}

impl Module {
    /// Checks that any references to opaque structs in parameters or return values
    /// are always behind a box or reference.
    ///
    /// Any references to opaque structs that are invalid are pushed into the `errors` vector.
    pub fn check_opaque<'a>(
        &'a self,
        env: &HashMap<String, CustomType>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        self.declared_types
            .values()
            .for_each(|t| t.check_opaque(env, errors));
    }
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

impl File {
    /// Checks that any references to opaque structs in parameters or return values
    /// are always behind a box or reference.
    ///
    /// Any references to opaque structs that are invalid are pushed into the `errors` vector.
    pub fn check_opaque<'a>(
        &'a self,
        env: &HashMap<String, CustomType>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        self.modules
            .values()
            .for_each(|t| t.check_opaque(env, errors));
    }

    /// Fuses all declared types into a single environment `HashMap`.
    pub fn all_types(&self) -> HashMap<String, CustomType> {
        let mut out = HashMap::new();
        self.modules.values().for_each(|m| {
            m.declared_types.iter().for_each(|(k, v)| {
                if out.insert(k.clone(), v.clone()).is_some() {
                    panic!(
                        "Two types were declared with the same name, this needs to be implemented"
                    );
                }
            })
        });
        out
    }
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

#[cfg(test)]
mod tests {
    use insta::{self, Settings};

    use quote::quote;
    use syn;

    use super::Module;

    #[test]
    fn simple_mod() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Module::from(
                &syn::parse2(quote! {
                    mod ffi {
                        struct NonOpaqueStruct {
                            a: i32,
                            b: Box<NonOpaqueStruct>
                        }

                        impl NonOpaqueStruct {
                            fn new(x: i32) -> NonOpaqueStruct {
                                unimplemented!();
                            }

                            fn set_a(&mut self, new_a: i32) {
                                self.a = new_a;
                            }
                        }

                        #[diplomat::opaque]
                        struct OpaqueStruct {
                            a: SomeExternalType
                        }

                        impl OpaqueStruct {
                            fn new() -> Box<OpaqueStruct> {
                                unimplemented!();
                            }

                            fn get_string(&self) -> String {
                                unimplemented!()
                            }
                        }
                    }
                })
                .unwrap()
            ));
        });
    }
}
