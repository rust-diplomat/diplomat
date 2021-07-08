use proc_macro2::Span;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{punctuated::Punctuated, *};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::FromIterator;

use super::{Method, OpaqueStruct, Struct};

/// A type declared inside a Diplomat-annotated module.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CustomType {
    /// A non-opaque struct whose fields will be visible across the FFI boundary.
    Struct(Struct),
    // TODO(shadaj): Enum
    /// A struct annotated with [`diplomat::opaque`] whose fields are not visible.
    Opaque(OpaqueStruct),
}

impl CustomType {
    /// Get the name of the custom type, which is unique within a module.
    pub fn name(&self) -> &String {
        match self {
            CustomType::Struct(strct) => &strct.name,
            CustomType::Opaque(strct) => &strct.name,
        }
    }

    /// Get the methods declared in impls of the custom type.
    pub fn methods(&self) -> &Vec<Method> {
        match self {
            CustomType::Struct(strct) => &strct.methods,
            CustomType::Opaque(strct) => &strct.methods,
        }
    }

    /// Checks that any references to opaque structs in parameters or return values
    /// are always behind a box or reference.
    ///
    /// Any references to opaque structs that are invalid are pushed into the `errors` vector.
    pub fn check_opaque<'a>(
        &'a self,
        env: &HashMap<String, CustomType>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        match self {
            CustomType::Struct(strct) => {
                for (_, field) in strct.fields.iter() {
                    field.check_opaque(env, errors);
                }
            }
            CustomType::Opaque(_) => {}
        }

        for method in self.methods().iter() {
            method.check_opaque(env, errors);
        }
    }
}

/// A local type reference, such as the type of a field, parameter, or return value.
/// Unlike [`CustomType`], which represents a type declaration, [`TypeName`]s can compose
/// types through references and boxing, and can also capture unresolved paths.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TypeName {
    /// A built-in Rust scalar primitive.
    Primitive(PrimitiveType),
    /// An unresolved path to a custom type, which can be resolved after all types
    /// are collected with [`TypeName::resolve()`].
    Named(String),
    /// An optionally mutable reference to another type.
    Reference(Box<TypeName>, /* mutable */ bool),
    /// A `Box<T>` type.
    Box(Box<TypeName>),
    /// A `diplomat_runtime::DiplomatWriteable` type.
    Writeable,
}

impl TypeName {
    /// Converts the [`TypeName`] back into an AST node that can be spliced into a program.
    pub fn to_syn(&self) -> syn::Type {
        match self {
            TypeName::Primitive(name) => {
                syn::Type::Path(syn::parse_str(PRIMITIVE_TO_STRING.get(name).unwrap()).unwrap())
            }
            TypeName::Named(name) => syn::Type::Path(syn::parse_str(name.as_str()).unwrap()),
            TypeName::Reference(underlying, mutable) => syn::Type::Reference(TypeReference {
                and_token: syn::token::And(Span::call_site()),
                lifetime: None,
                mutability: if *mutable {
                    Some(syn::token::Mut(Span::call_site()))
                } else {
                    None
                },
                elem: Box::new(underlying.to_syn()),
            }),
            TypeName::Box(underlying) => syn::Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(vec![PathSegment {
                        ident: Ident::new("Box", Span::call_site()),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: syn::token::Lt(Span::call_site()),
                            args: Punctuated::from_iter(vec![GenericArgument::Type(
                                underlying.to_syn(),
                            )]),
                            gt_token: syn::token::Gt(Span::call_site()),
                        }),
                    }]),
                },
            }),
            TypeName::Writeable => syn::Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(vec![
                        PathSegment {
                            ident: Ident::new("diplomat_runtime", Span::call_site()),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: Ident::new("DiplomatWriteable", Span::call_site()),
                            arguments: PathArguments::None,
                        },
                    ]),
                },
            }),
        }
    }

    /// If this is a [`TypeName::Named`], grab the [`CustomType`] it points to from
    /// the `env`, which contains all [`CustomType`]s across all FFI modules.
    pub fn resolve<'a>(&self, env: &'a HashMap<String, CustomType>) -> &'a CustomType {
        match self {
            TypeName::Named(name) => env.get(name).unwrap(),
            _ => panic!(),
        }
    }

    fn check_opaque_internal<'a>(
        &'a self,
        env: &HashMap<String, CustomType>,
        behind_reference: bool,
        errors: &mut Vec<&'a TypeName>,
    ) {
        match self {
            TypeName::Reference(underlying, _) => {
                underlying.check_opaque_internal(env, true, errors)
            }
            TypeName::Box(underlying) => underlying.check_opaque_internal(env, true, errors),
            TypeName::Primitive(_) => {}
            TypeName::Named(_) => {
                if let CustomType::Opaque(_) = self.resolve(env) {
                    if !behind_reference {
                        errors.push(self)
                    }
                }
            }
            TypeName::Writeable => {}
        }
    }

    /// Checks that any references to opaque structs in parameters or return values
    /// are always behind a box or reference.
    ///
    /// Any references to opaque structs that are invalid are pushed into the `errors` vector.
    pub fn check_opaque<'a>(
        &'a self,
        env: &HashMap<String, CustomType>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        self.check_opaque_internal(env, false, errors);
    }
}

impl From<&syn::Type> for TypeName {
    /// Extract a [`TypeName`] from a [`syn::Type`] AST node.
    /// The following rules are used to infer [`TypeName`] variants:
    /// - If the type is a path with a single element that is the name of a Rust primitive, returns a [`TypeName::Primitive`]
    /// - If the type is a path with a single element [`Box`], returns a [`TypeName::Box`] with the type paramter recursively converted
    /// - If the type is a reference (`&` or `&mut`), returns a [`TypeName::Reference`] with the referenced type recursively converted
    /// - Otherwise, assume that the reference is to a [`CustomType`] in either the current module or another one, returns a [`TypeName::Named`]
    fn from(ty: &syn::Type) -> TypeName {
        match ty {
            syn::Type::Reference(r) => {
                TypeName::Reference(Box::new(r.elem.as_ref().into()), r.mutability.is_some())
            }
            syn::Type::Path(p) => {
                if let Some(primitive) = p
                    .path
                    .get_ident()
                    .and_then(|i| STRING_TO_PRIMITIVE.get(i.to_string().as_str()))
                {
                    TypeName::Primitive(primitive.clone())
                } else if p.path.segments.len() == 1 && p.path.segments[0].ident == "Box" {
                    if let PathArguments::AngleBracketed(type_args) = &p.path.segments[0].arguments
                    {
                        if let GenericArgument::Type(tpe) = &type_args.args[0] {
                            TypeName::Box(Box::new(tpe.into()))
                        } else {
                            panic!("Expected first type argument for Box to be a type")
                        }
                    } else {
                        panic!("Expected angle brackets for Box type")
                    }
                } else if p.path.to_token_stream().to_string()
                    == "diplomat_runtime :: DiplomatWriteable"
                {
                    TypeName::Writeable
                } else {
                    TypeName::Named(p.path.to_token_stream().to_string())
                }
            }
            _ => panic!(),
        }
    }
}

/// A built-in Rust primitive scalar type.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PrimitiveType {
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    isize,
    usize,
    f32,
    f64,
    bool,
    char,
}

lazy_static! {
    static ref PRIMITIVES_MAPPING: [(&'static str, PrimitiveType); 16] = [
        ("i8", PrimitiveType::i8),
        ("u8", PrimitiveType::u8),
        ("i16", PrimitiveType::i16),
        ("u16", PrimitiveType::u16),
        ("i32", PrimitiveType::i32),
        ("u32", PrimitiveType::u32),
        ("i64", PrimitiveType::i64),
        ("u64", PrimitiveType::u64),
        ("i128", PrimitiveType::i128),
        ("u128", PrimitiveType::u128),
        ("isize", PrimitiveType::isize),
        ("usize", PrimitiveType::usize),
        ("f32", PrimitiveType::f32),
        ("f64", PrimitiveType::f64),
        ("bool", PrimitiveType::bool),
        ("char", PrimitiveType::char),
    ];
    static ref STRING_TO_PRIMITIVE: HashMap<&'static str, PrimitiveType> =
        PRIMITIVES_MAPPING.iter().cloned().collect();
    static ref PRIMITIVE_TO_STRING: HashMap<PrimitiveType, &'static str> = PRIMITIVES_MAPPING
        .iter()
        .map(|t| (t.1.clone(), t.0))
        .collect();
}

#[cfg(test)]
mod tests {
    use insta;

    use quote::quote;
    use syn;

    use super::TypeName;

    #[test]
    fn typename_primitives() {
        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                i32
            })
            .unwrap()
        ));

        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                usize
            })
            .unwrap()
        ));

        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                bool
            })
            .unwrap()
        ));
    }

    #[test]
    fn typename_named() {
        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                MyLocalStruct
            })
            .unwrap()
        ));
    }

    #[test]
    fn typename_references() {
        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                &i32
            })
            .unwrap()
        ));

        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                &mut MyLocalStruct
            })
            .unwrap()
        ));
    }

    #[test]
    fn typename_boxes() {
        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                Box<i32>
            })
            .unwrap()
        ));

        insta::assert_yaml_snapshot!(TypeName::from(
            &syn::parse2(quote! {
                Box<MyLocalStruct>
            })
            .unwrap()
        ));
    }
}
