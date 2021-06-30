use proc_macro2::Span;
use quote::ToTokens;
use syn::{punctuated::Punctuated, *};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::FromIterator;

use super::{methods::Method, structs::Struct};

#[derive(Clone, Debug)]
pub enum CustomType {
    Struct(Struct),
    // TODO(shadaj): Enum
    Opaque(String, Vec<Method>),
}

impl CustomType {
    pub fn name(&self) -> &String {
        match self {
            CustomType::Struct(strct) => &strct.name,
            CustomType::Opaque(name, _) => name,
        }
    }

    pub fn methods(&self) -> &Vec<Method> {
        match self {
            CustomType::Struct(strct) => &strct.methods,
            CustomType::Opaque(_, methods) => methods,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeName {
    Primitive(PrimitiveType),
    Named(String),
    Reference(Box<TypeName>, /* mutable */ bool),
    Box(Box<TypeName>),
}

impl TypeName {
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
        }
    }

    pub fn resolve<'a>(&self, env: &'a HashMap<String, CustomType>) -> &'a CustomType {
        match self {
            TypeName::Named(name) => env.get(name).unwrap(),
            _ => panic!(),
        }
    }
}

impl From<&syn::Type> for TypeName {
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
                } else {
                    TypeName::Named(p.path.to_token_stream().to_string())
                }
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
