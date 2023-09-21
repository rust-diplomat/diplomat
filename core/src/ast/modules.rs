use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;

use quote::ToTokens;
use serde::Serialize;
use syn::{ImplItem, Item, ItemMod, UseTree, Visibility};

use super::{
    Attrs, CustomType, Enum, Ident, Method, ModSymbol, Mutability, OpaqueStruct, Path, PathType,
    RustLink, Struct, ValidityError,
};
use crate::environment::*;

/// Custom Diplomat attribute that can be placed on a struct definition.
#[derive(Debug)]
enum DiplomatStructAttribute {
    /// The `#[diplomat::out]` attribute, used for non-opaque structs that
    /// contain an owned opaque in the form of a `Box`.
    Out,
    /// The `#[diplomat::opaque]` attribute, used for marking a struct as opaque.
    /// Note that opaque structs can be borrowed in return types, but cannot
    /// be passed into a function behind a mutable reference.
    Opaque,
    /// The `#[diplomat::opaque_mut]` attribute, used for marking a struct as
    /// opaque and mutable.
    /// Note that mutable opaque structs can never be borrowed in return types
    /// (even immutably!), but can be passed into a function behind a mutable
    /// reference.
    OpaqueMut,
}

impl DiplomatStructAttribute {
    /// Parses a [`DiplomatStructAttribute`] from an array of [`syn::Attribute`]s.
    /// If more than one kind is found, an error is returned containing all the
    /// ones encountered, since all the current attributes are disjoint.
    fn parse(attrs: &[syn::Attribute]) -> Result<Option<Self>, Vec<Self>> {
        let mut buf = String::with_capacity(32);
        let mut res = Ok(None);
        for attr in attrs {
            buf.clear();
            write!(&mut buf, "{}", attr.path().to_token_stream()).unwrap();
            let parsed = match buf.as_str() {
                "diplomat :: out" => Some(Self::Out),
                "diplomat :: opaque" => Some(Self::Opaque),
                "diplomat :: opaque_mut" => Some(Self::OpaqueMut),
                _ => None,
            };

            if let Some(parsed) = parsed {
                match res {
                    Ok(None) => res = Ok(Some(parsed)),
                    Ok(Some(first)) => res = Err(vec![first, parsed]),
                    Err(ref mut errors) => errors.push(parsed),
                }
            }
        }

        res
    }
}

#[derive(Clone, Serialize, Debug)]
#[non_exhaustive]
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
            .flat_map(|t| t.all_rust_links())
            .collect::<HashSet<_>>();

        self.sub_modules.iter().for_each(|m| {
            rust_links.extend(m.all_rust_links().iter());
        });
        rust_links
    }

    pub fn insert_all_types(&self, in_path: Path, out: &mut Env) {
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
                .any(|a| a.path().to_token_stream().to_string() == "diplomat :: bridge");

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
                        let custom_type = match DiplomatStructAttribute::parse(&strct.attrs[..]) {
                            Ok(None) => CustomType::Struct(Struct::new(strct, false)),
                            Ok(Some(DiplomatStructAttribute::Out)) => {
                                CustomType::Struct(Struct::new(strct, true))
                            }
                            Ok(Some(DiplomatStructAttribute::Opaque)) => {
                                CustomType::Opaque(OpaqueStruct::new(strct, Mutability::Immutable))
                            }
                            Ok(Some(DiplomatStructAttribute::OpaqueMut)) => {
                                CustomType::Opaque(OpaqueStruct::new(strct, Mutability::Mutable))
                            }
                            Err(errors) => {
                                panic!("Multiple conflicting Diplomat struct attributes, there can be at most one: {errors:?}");
                            }
                        };

                        custom_types_by_name.insert(Ident::from(&strct.ident), custom_type);
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
                        let attrs = Attrs::from(&*imp.attrs);

                        let mut new_methods = imp
                            .items
                            .iter()
                            .filter_map(|i| match i {
                                ImplItem::Fn(m) => Some(m),
                                _ => None,
                            })
                            .filter(|m| matches!(m.vis, Visibility::Public(_)))
                            .map(|m| Method::from_syn(m, self_path.clone(), Some(&imp.generics), &attrs))
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

#[derive(Serialize, Clone, Debug)]
#[non_exhaustive]
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
