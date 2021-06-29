use proc_macro2::Span;
use quote::ToTokens;
use syn::{punctuated::Punctuated, *};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub full_path_name: String,
    pub self_param: Option<Param>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
}

impl Method {
    pub fn from_syn(m: &ImplItemMethod, self_type: &TypePath) -> Method {
        let self_ident = self_type.path.get_ident().unwrap();
        let method_ident = &m.sig.ident;
        let extern_ident = Ident::new(
            format!("{}_{}", &self_ident.to_string(), method_ident.to_string()).as_str(),
            m.sig.ident.span(),
        );

        let all_params = m
            .sig
            .inputs
            .iter()
            .filter_map(|a| match a {
                FnArg::Receiver(_) => None,
                FnArg::Typed(t) => Some(t.into()),
            })
            .collect::<Vec<_>>();

        let self_param = m.sig.receiver().map(|rec| match rec {
            FnArg::Receiver(rec) => Param {
                name: "self".to_string(),
                ty: if rec.reference.is_some() {
                    Type::Reference(
                        Box::new(Type::Named(self_ident.to_string())),
                        rec.mutability.is_some(),
                    )
                } else {
                    Type::Named(self_ident.to_string())
                },
            },
            _ => panic!("Unexpected self param type"),
        });

        let return_ty = match &m.sig.output {
            ReturnType::Type(_, return_typ) => Some(return_typ.as_ref().into()),
            ReturnType::Default => None,
        };

        Method {
            name: method_ident.to_string(),
            full_path_name: extern_ident.to_string(),
            self_param,
            params: all_params,
            return_type: return_ty,
        }
    }
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

impl From<&syn::PatType> for Param {
    fn from(t: &PatType) -> Param {
        let ident = match t.pat.as_ref() {
            Pat::Ident(ident) => ident.clone(),
            _ => panic!("Unexpected param type"),
        };

        Param {
            name: ident.ident.to_string(),
            ty: t.ty.as_ref().into(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    Named(String),
    Reference(Box<Type>, /* mutable */ bool),
    Box(Box<Type>),
}

impl Type {
    pub fn to_syn(&self) -> syn::Type {
        match self {
            Type::Primitive(name) => {
                syn::Type::Path(syn::parse_str(PRIMITIVE_TO_STRING.get(name).unwrap()).unwrap())
            }
            Type::Named(name) => syn::Type::Path(syn::parse_str(name.as_str()).unwrap()),
            Type::Reference(underlying, mutable) => syn::Type::Reference(TypeReference {
                and_token: syn::token::And(Span::call_site()),
                lifetime: None,
                mutability: if *mutable {
                    Some(syn::token::Mut(Span::call_site()))
                } else {
                    None
                },
                elem: Box::new(underlying.to_syn()),
            }),
            Type::Box(underlying) => syn::Type::Path(TypePath {
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

impl From<&syn::Type> for Type {
    fn from(ty: &syn::Type) -> Type {
        match ty {
            syn::Type::Reference(r) => {
                Type::Reference(Box::new(r.elem.as_ref().into()), r.mutability.is_some())
            }
            syn::Type::Path(p) => {
                if let Some(primitive) = p
                    .path
                    .get_ident()
                    .and_then(|i| STRING_TO_PRIMITIVE.get(i.to_string().as_str()))
                {
                    Type::Primitive(primitive.clone())
                } else if p.path.segments.len() == 1 && p.path.segments[0].ident == "Box" {
                    if let PathArguments::AngleBracketed(type_args) = &p.path.segments[0].arguments
                    {
                        if let GenericArgument::Type(tpe) = &type_args.args[0] {
                            Type::Box(Box::new(tpe.into()))
                        } else {
                            panic!("Expected first type argument for Box to be a type")
                        }
                    } else {
                        panic!("Expected angle brackets for Box type")
                    }
                } else {
                    Type::Named(p.path.to_token_stream().to_string())
                }
            }
            _ => panic!(),
        }
    }
}
