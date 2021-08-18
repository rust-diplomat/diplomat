use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use quote::ToTokens;
use syn::{ImplItem, Item, ItemMod, UseTree, Visibility};

use super::{CustomType, Enum, Method, ModSymbol, OpaqueStruct, Path, Struct, TypeName};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub imports: Vec<(Path, String)>,
    pub declared_types: HashMap<String, CustomType>,
    pub sub_modules: Vec<Module>,
}

impl Module {
    /// Checks that any references to opaque structs in parameters or return values
    /// are always behind a box or reference.
    ///
    /// Any references to opaque structs that are invalid are pushed into the `errors` vector.
    pub fn check_opaque<'a>(
        &'a self,
        in_path: &Path,
        env: &HashMap<Path, HashMap<String, ModSymbol>>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        self.declared_types
            .values()
            .for_each(|t| t.check_opaque(&in_path.sub_path(self.name.clone()), env, errors));

        self.sub_modules
            .iter()
            .for_each(|m| m.check_opaque(&in_path.sub_path(self.name.clone()), env, errors));
    }

    /// Ensures that we are not exporting any non-opaque zero-sized types
    pub fn check_zst<'a>(&'a self, in_path: &Path, errors: &mut Vec<Path>) {
        self.declared_types
            .values()
            .for_each(|t| t.check_zst(&in_path.sub_path(self.name.clone()), errors));

        self.sub_modules
            .iter()
            .for_each(|m| m.check_zst(&in_path.sub_path(self.name.clone()), errors));
    }

    fn insert_all_types(&self, in_path: Path, out: &mut HashMap<Path, HashMap<String, ModSymbol>>) {
        let mut mod_symbols = HashMap::new();

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
        let mut custom_types_by_name = HashMap::new();
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
            .unwrap()
            .1
            .iter()
            .for_each(|a| match a {
                Item::Use(u) => {
                    extract_imports(&Path::empty(), &u.tree, &mut imports);
                }
                Item::Struct(strct) => {
                    if analyze_types {
                        if strct
                            .attrs
                            .iter()
                            .any(|a| a.path.to_token_stream().to_string() == "diplomat :: opaque")
                        {
                            custom_types_by_name.insert(
                                strct.ident.to_string(),
                                CustomType::Opaque(OpaqueStruct::from(strct)),
                            );
                        } else {
                            custom_types_by_name.insert(
                                strct.ident.to_string(),
                                CustomType::Struct(Struct::from(strct)),
                            );
                        }
                    }
                }

                Item::Enum(enm) => {
                    if analyze_types {
                        custom_types_by_name
                            .insert(enm.ident.to_string(), CustomType::Enum(Enum::from(enm)));
                    }
                }

                Item::Impl(ipl) => {
                    if analyze_types {
                        assert!(ipl.trait_.is_none());

                        let self_typ = match ipl.self_ty.as_ref() {
                            syn::Type::Path(s) => Path::from_syn(&s.path),
                            _ => panic!("Self type not found"),
                        };

                        let mut new_methods = ipl
                            .items
                            .iter()
                            .filter_map(|i| match i {
                                ImplItem::Method(m) => Some(m),
                                _ => None,
                            })
                            .filter(|m| matches!(m.vis, Visibility::Public(_)))
                            .map(|m| Method::from_syn(m, &self_typ))
                            .collect();

                        let self_ident = self_typ.elements.last().unwrap();

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
            name: input.ident.to_string(),
            imports,
            declared_types: custom_types_by_name,
            sub_modules,
        }
    }
}

fn extract_imports(base_path: &Path, use_tree: &UseTree, out: &mut Vec<(Path, String)>) {
    match use_tree {
        UseTree::Name(name) => out.push((
            base_path.sub_path(name.ident.to_string()),
            name.ident.to_string(),
        )),
        UseTree::Path(path) => {
            extract_imports(&base_path.sub_path(path.ident.to_string()), &path.tree, out)
        }
        UseTree::Glob(_) => todo!("Glob imports are not yet supported"),
        UseTree::Group(group) => {
            group
                .items
                .iter()
                .for_each(|i| extract_imports(base_path, i, out));
        }
        UseTree::Rename(rename) => out.push((
            base_path.sub_path(rename.ident.to_string()),
            rename.rename.to_string(),
        )),
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
        env: &HashMap<Path, HashMap<String, ModSymbol>>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        self.modules
            .values()
            .for_each(|t| t.check_opaque(&Path::empty(), env, errors));
    }

    /// Ensures that we are not exporting any non-opaque zero-sized types
    pub fn check_zst(&self, errors: &mut Vec<Path>) {
        self.modules
            .values()
            .for_each(|t| t.check_zst(&Path::empty(), errors));
    }

    /// Fuses all declared types into a single environment `HashMap`.
    pub fn all_types(&self) -> HashMap<Path, HashMap<String, ModSymbol>> {
        let mut out = HashMap::new();
        let mut top_symbols = HashMap::new();

        self.modules.values().for_each(|m| {
            m.insert_all_types(Path::empty(), &mut out);
            top_symbols.insert(m.name.clone(), ModSymbol::SubModule(m.name.clone()));
        });

        out.insert(Path::empty(), top_symbols);

        out
    }
}

impl From<&syn::File> for File {
    /// Get all custom types across all modules defined in a given file.
    fn from(file: &syn::File) -> File {
        let mut out = HashMap::new();
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

    use crate::ast::Path;

    use super::{File, Module, TypeName};

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
    fn opaque_checks_with_safe_use() {
        let file_with_safe_opaque = File::from(&syn::parse_quote! {
            #[diplomat::bridge]
            mod ffi {
                struct NonOpaqueStruct {}

                impl NonOpaqueStruct {
                    fn new(x: i32) -> NonOpaqueStruct {
                        unimplemented!();
                    }
                }

                #[diplomat::opaque]
                struct OpaqueStruct {}

                impl OpaqueStruct {
                    pub fn new() -> Box<OpaqueStruct> {
                        unimplemented!();
                    }

                    pub fn get_i32(&self) -> i32 {
                        unimplemented!()
                    }
                }
            }
        });

        let mut errors = Vec::new();
        file_with_safe_opaque.check_opaque(&file_with_safe_opaque.all_types(), &mut errors);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn opaque_checks_with_error() {
        let file_with_error_opaque = File::from(&syn::parse_quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct OpaqueStruct {}

                impl OpaqueStruct {
                    pub fn new() -> OpaqueStruct {
                        unimplemented!();
                    }

                    pub fn get_i32(self) -> i32 {
                        unimplemented!()
                    }
                }
            }
        });

        let mut errors = Vec::new();
        file_with_error_opaque.check_opaque(&file_with_error_opaque.all_types(), &mut errors);
        assert_eq!(
            errors,
            vec![
                &TypeName::Named(Path::empty().sub_path("OpaqueStruct".to_string())),
                &TypeName::Named(Path::empty().sub_path("OpaqueStruct".to_string()))
            ]
        );
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
}
