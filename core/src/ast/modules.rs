use std::collections::BTreeMap;
use std::collections::HashSet;

use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{ImplItem, Item, ItemMod, UseTree, Visibility};

use super::{
    CustomType, Enum, Ident, Method, ModSymbol, OpaqueStruct, Path, PathType, RustLink, Struct,
    ValidityError,
};
use crate::environment::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: Ident,
    pub imports: Vec<(Path, Ident)>,
    pub declared_types: BTreeMap<Ident, CustomType>,
    pub sub_modules: Vec<Module>,
}

impl Module {
    pub fn check_validity(&self, in_path: &Path, env: &Env, errors: &mut Vec<ValidityError>) {
        self.declared_types.values().for_each(|t| {
            t.check_validity(&in_path.sub_path(self.name.clone()), env, errors);
        });

        self.sub_modules.iter().for_each(|t| {
            t.check_validity(&in_path.sub_path(self.name.clone()), env, errors);
        });
    }

    pub fn all_rust_links(&self) -> HashSet<&RustLink> {
        let mut rust_links = self
            .declared_types
            .values()
            .filter_map(|t| t.docs().rust_link())
            .collect::<HashSet<_>>();

        self.sub_modules.iter().for_each(|m| {
            rust_links.extend(m.all_rust_links().iter());
        });
        rust_links
    }

    fn insert_all_types(&self, in_path: Path, out: &mut Env) {
        let mut mod_symbols = ModuleEnv::default();

        self.imports.iter().for_each(|(path, name)| {
            mod_symbols.insert(name.clone(), ModSymbol::Alias(path.clone()));
        });

        self.declared_types.iter().for_each(|(k, v)| {
            if mod_symbols
                .insert(k.clone(), ModSymbol::CustomType(v.clone()))
                .is_some()
            {
                panic!("Two types were declared with the same name, this needs to be implemented");
            }
        });

        let path_to_self = in_path.sub_path(self.name.clone());
        self.sub_modules.iter().for_each(|m| {
            m.insert_all_types(path_to_self.clone(), out);
            mod_symbols.insert(m.name.clone(), ModSymbol::SubModule(m.name.clone()));
        });

        out.insert(path_to_self, mod_symbols);
    }

    pub fn from_syn(input: &ItemMod, force_analyze: bool) -> Module {
        let mut custom_types_by_name = BTreeMap::new();
        let mut sub_modules = Vec::new();
        let mut imports = Vec::new();

        let analyze_types = force_analyze
            || input
                .attrs
                .iter()
                .any(|a| a.path.to_token_stream().to_string() == "diplomat :: bridge");

        input
            .content
            .as_ref()
            .map(|t| &t.1[..])
            .unwrap_or_default()
            .iter()
            .for_each(|a| match a {
                Item::Use(u) => {
                    if analyze_types {
                        extract_imports(&Path::empty(), &u.tree, &mut imports);
                    }
                }
                Item::Struct(strct) => {
                    if analyze_types {
                        if strct
                            .attrs
                            .iter()
                            .any(|a| a.path.to_token_stream().to_string() == "diplomat :: opaque")
                        {
                            custom_types_by_name.insert(
                                (&strct.ident).into(),
                                CustomType::Opaque(OpaqueStruct::from(strct)),
                            );
                        } else {
                            custom_types_by_name.insert(
                                (&strct.ident).into(),
                                CustomType::Struct(Struct::from(strct)),
                            );
                        }
                    }
                }

                Item::Enum(enm) => {
                    if analyze_types {
                        custom_types_by_name
                            .insert((&enm.ident).into(), CustomType::Enum(Enum::from(enm)));
                    }
                }

                Item::Impl(imp) => {
                    if analyze_types {
                        assert!(imp.trait_.is_none());

                        let self_path = match imp.self_ty.as_ref() {
                            syn::Type::Path(s) => PathType::from(s),
                            _ => panic!("Self type not found"),
                        };

                        let mut new_methods = imp
                            .items
                            .iter()
                            .filter_map(|i| match i {
                                ImplItem::Method(m) => Some(m),
                                _ => None,
                            })
                            .filter(|m| matches!(m.vis, Visibility::Public(_)))
                            .map(|m| Method::from_syn(m, self_path.clone(), Some(&imp.generics)))
                            .collect();

                        let self_ident = self_path.path.elements.last().unwrap();

                        match custom_types_by_name.get_mut(self_ident).unwrap() {
                            CustomType::Struct(strct) => {
                                strct.methods.append(&mut new_methods);
                            }
                            CustomType::Opaque(strct) => {
                                strct.methods.append(&mut new_methods);
                            }
                            CustomType::Enum(enm) => {
                                enm.methods.append(&mut new_methods);
                            }
                        }
                    }
                }
                Item::Mod(item_mod) => {
                    sub_modules.push(Module::from_syn(item_mod, false));
                }
                _ => {}
            });

        Module {
            name: (&input.ident).into(),
            imports,
            declared_types: custom_types_by_name,
            sub_modules,
        }
    }
}

fn extract_imports(base_path: &Path, use_tree: &UseTree, out: &mut Vec<(Path, Ident)>) {
    match use_tree {
        UseTree::Name(name) => out.push((
            base_path.sub_path((&name.ident).into()),
            (&name.ident).into(),
        )),
        UseTree::Path(path) => {
            extract_imports(&base_path.sub_path((&path.ident).into()), &path.tree, out)
        }
        UseTree::Glob(_) => todo!("Glob imports are not yet supported"),
        UseTree::Group(group) => {
            group
                .items
                .iter()
                .for_each(|i| extract_imports(base_path, i, out));
        }
        UseTree::Rename(rename) => out.push((
            base_path.sub_path((&rename.ident).into()),
            (&rename.rename).into(),
        )),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub modules: BTreeMap<String, Module>,
}

impl File {
    /// Performs all necessary validity checks and returns any errors
    ///
    /// Environment should be passed in from `.all_types()`
    pub fn check_validity(&self, env: &Env) -> Vec<ValidityError> {
        let mut errors = vec![];
        self.modules
            .values()
            .for_each(|t| t.check_validity(&Path::empty(), env, &mut errors));
        errors
    }

    /// Fuses all declared types into a single environment `HashMap`.
    pub fn all_types(&self) -> Env {
        let mut out = Env::default();
        let mut top_symbols = ModuleEnv::default();

        self.modules.values().for_each(|m| {
            m.insert_all_types(Path::empty(), &mut out);
            top_symbols.insert(m.name.clone(), ModSymbol::SubModule(m.name.clone()));
        });

        out.insert(Path::empty(), top_symbols);

        out
    }

    pub fn all_rust_links(&self) -> HashSet<&RustLink> {
        self.modules
            .values()
            .flat_map(|m| m.all_rust_links().into_iter())
            .collect()
    }
}

impl From<&syn::File> for File {
    /// Get all custom types across all modules defined in a given file.
    fn from(file: &syn::File) -> File {
        let mut out = BTreeMap::new();
        file.items.iter().for_each(|i| {
            if let Item::Mod(item_mod) = i {
                out.insert(
                    item_mod.ident.to_string(),
                    Module::from_syn(item_mod, false),
                );
            }
        });

        File { modules: out }
    }
}

#[cfg(test)]
mod tests {
    use insta::{self, Settings};

    use syn;

    use crate::ast::{File, Module};

    #[test]
    fn simple_mod() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Module::from_syn(
                &syn::parse_quote! {
                    mod ffi {
                        struct NonOpaqueStruct {
                            a: i32,
                            b: Box<NonOpaqueStruct>
                        }

                        impl NonOpaqueStruct {
                            pub fn new(x: i32) -> NonOpaqueStruct {
                                unimplemented!();
                            }

                            pub fn set_a(&mut self, new_a: i32) {
                                self.a = new_a;
                            }
                        }

                        #[diplomat::opaque]
                        struct OpaqueStruct {
                            a: SomeExternalType
                        }

                        impl OpaqueStruct {
                            pub fn new() -> Box<OpaqueStruct> {
                                unimplemented!();
                            }

                            pub fn get_string(&self) -> String {
                                unimplemented!()
                            }
                        }
                    }
                },
                true
            ));
        });
    }

    #[test]
    fn method_visibility() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Module::from_syn(
                &syn::parse_quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        struct Foo {}

                        impl Foo {
                            pub fn pub_fn() {
                                unimplemented!()
                            }
                            pub(crate) fn pub_crate_fn() {
                                unimplemented!()
                            }
                            pub(super) fn pub_super_fn() {
                                unimplemented!()
                            }
                            fn priv_fn() {
                                unimplemented!()
                            }
                        }
                    }
                },
                true
            ));
        });
    }

    #[test]
    fn import_in_non_diplomat_not_analyzed() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(File::from(&syn::parse_quote! {
                #[diplomat::bridge]
                mod ffi {
                    struct Foo {}
                }

                mod other {
                    use something::*;
                }
            }));
        });
    }
}
