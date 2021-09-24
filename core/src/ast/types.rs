use proc_macro2::Span;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{punctuated::Punctuated, *};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::FromIterator;

use super::{Enum, Method, OpaqueStruct, Path, Struct};

/// A type declared inside a Diplomat-annotated module.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CustomType {
    /// A non-opaque struct whose fields will be visible across the FFI boundary.
    Struct(Struct),
    /// A struct annotated with [`diplomat::opaque`] whose fields are not visible.
    Opaque(OpaqueStruct),
    /// A fieldless enum.
    Enum(Enum),
}

impl CustomType {
    /// Get the name of the custom type, which is unique within a module.
    pub fn name(&self) -> &String {
        match self {
            CustomType::Struct(strct) => &strct.name,
            CustomType::Opaque(strct) => &strct.name,
            CustomType::Enum(enm) => &enm.name,
        }
    }

    /// Get the methods declared in impls of the custom type.
    pub fn methods(&self) -> &Vec<Method> {
        match self {
            CustomType::Struct(strct) => &strct.methods,
            CustomType::Opaque(strct) => &strct.methods,
            CustomType::Enum(enm) => &enm.methods,
        }
    }

    /// Get the doc lines of the custom type.
    pub fn doc_lines(&self) -> &String {
        match self {
            CustomType::Struct(strct) => &strct.doc_lines,
            CustomType::Opaque(strct) => &strct.doc_lines,
            CustomType::Enum(enm) => &enm.doc_lines,
        }
    }

    pub fn self_path(&self, in_path: &Path) -> Path {
        in_path.sub_path(self.name().clone())
    }

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
        match self {
            CustomType::Struct(strct) => {
                for (_, field, _) in strct.fields.iter() {
                    field.check_opaque(in_path, env, errors);
                }
            }
            CustomType::Opaque(_) => {}
            CustomType::Enum(_) => {}
        }

        for method in self.methods().iter() {
            method.check_opaque(in_path, env, errors);
        }
    }

    /// Ensures that we are not exporting any non-opaque zero-sized types
    pub fn check_zst<'a>(&'a self, in_path: &Path, errors: &mut Vec<Path>) {
        match self {
            CustomType::Struct(strct) => {
                if !strct.fields.iter().any(|f| !f.1.is_zst()) {
                    errors.push(self.self_path(in_path))
                }
            }
            CustomType::Opaque(_) => {}
            CustomType::Enum(e) => {
                if e.variants.is_empty() {
                    errors.push(self.self_path(in_path))
                }
            }
        }
    }
}

/// A symbol declared in a module, which can either be a pointer to another path,
/// or a custom type defined directly inside that module
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ModSymbol {
    /// A symbol that is a pointer to another path.
    Alias(Path),
    /// A symbol that is a submodule.
    SubModule(String),
    /// A symbol that is a custom type.
    CustomType(CustomType),
}

/// A local type reference, such as the type of a field, parameter, or return value.
/// Unlike [`CustomType`], which represents a type declaration, [`TypeName`]s can compose
/// types through references and boxing, and can also capture unresolved paths.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TypeName {
    /// A built-in Rust scalar primitive.
    Primitive(PrimitiveType),
    /// An unresolved path to a custom type, which can be resolved after all types
    /// are collected with [`TypeName::resolve()`].
    Named(Path),
    /// An optionally mutable reference to another type.
    Reference(Box<TypeName>, /* mutable */ bool),
    /// A `Box<T>` type.
    Box(Box<TypeName>),
    /// A `Option<T>` type.
    Option(Box<TypeName>),
    /// A `diplomat_runtime::DiplomatResult<T, E>` type.
    Result(Box<TypeName>, Box<TypeName>),
    /// A `diplomat_runtime::DiplomatWriteable` type.
    Writeable,
    /// A `&str` type.
    StrReference,
    /// A `&[T]` type, where `T` is a primitive.
    PrimitiveSlice(PrimitiveType),
    /// The `()` type.
    Unit,
}

impl TypeName {
    /// Converts the [`TypeName`] back into an AST node that can be spliced into a program.
    pub fn to_syn(&self) -> syn::Type {
        match self {
            TypeName::Primitive(name) => {
                syn::Type::Path(syn::parse_str(PRIMITIVE_TO_STRING.get(name).unwrap()).unwrap())
            }
            TypeName::Named(name) => syn::Type::Path(syn::TypePath {
                qself: None,
                path: name.to_syn(),
            }),
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
                path: syn::Path {
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
            TypeName::Option(underlying) => syn::Type::Path(TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(vec![PathSegment {
                        ident: Ident::new("Option", Span::call_site()),
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
            TypeName::Result(ok, err) => syn::Type::Path(TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(vec![
                        PathSegment {
                            ident: Ident::new("diplomat_runtime", Span::call_site()),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: Ident::new("DiplomatResult", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: syn::token::Lt(Span::call_site()),
                                    args: Punctuated::from_iter(vec![
                                        GenericArgument::Type(ok.to_syn()),
                                        GenericArgument::Type(err.to_syn()),
                                    ]),
                                    gt_token: syn::token::Gt(Span::call_site()),
                                },
                            ),
                        },
                    ]),
                },
            }),
            TypeName::Writeable => syn::parse_quote! {
                diplomat_runtime::DiplomatWriteable
            },
            TypeName::StrReference => syn::parse_quote! {
                &str
            },
            TypeName::PrimitiveSlice(name) => {
                let primitive_name = PRIMITIVE_TO_STRING.get(name).unwrap();
                syn::parse_str(&format!("&[{}]", primitive_name)).unwrap()
            },
            TypeName::Unit => syn::parse_quote! {
                ()
            },
        }
    }

    /// If this is a [`TypeName::Named`], grab the [`CustomType`] it points to from
    /// the `env`, which contains all [`CustomType`]s across all FFI modules.
    pub fn resolve_with_path<'a>(
        &self,
        in_path: &Path,
        env: &'a HashMap<Path, HashMap<String, ModSymbol>>,
    ) -> (Path, &'a CustomType) {
        match self {
            TypeName::Named(local_path) => {
                let mut cur_path = in_path.clone();
                for (i, elem) in local_path.elements.iter().enumerate() {
                    match elem.as_ref() {
                        "crate" => {
                            // TODO(#34): get the name of enclosing crate from env when we support multiple crates
                            cur_path = Path::empty()
                        }

                        "super" => cur_path = cur_path.get_super(),

                        o => match env.get(&cur_path).and_then(|env| env.get(o)) {
                            Some(ModSymbol::Alias(p)) => {
                                let mut remaining_elements: Vec<String> =
                                    local_path.elements.iter().skip(i + 1).cloned().collect();
                                let mut new_path = p.elements.clone();
                                new_path.append(&mut remaining_elements);
                                return TypeName::Named(Path { elements: new_path })
                                    .resolve_with_path(&cur_path.clone(), env);
                            }
                            Some(ModSymbol::SubModule(name)) => {
                                cur_path.elements.push(name.clone());
                            }
                            Some(ModSymbol::CustomType(t)) => {
                                if i == local_path.elements.len() - 1 {
                                    return (cur_path, t);
                                } else {
                                    panic!(
                                        "Unexpected custom type when resolving symbol {} in {}",
                                        o,
                                        cur_path.elements.join("::")
                                    )
                                }
                            }
                            None => panic!(
                                "Could not resolve symbol {} in {}",
                                o,
                                cur_path.elements.join("::")
                            ),
                        },
                    }
                }

                panic!(
                    "Path {} does not point to a custom type",
                    in_path.elements.join("::")
                )
            }
            _ => panic!(),
        }
    }

    pub fn resolve<'a>(
        &self,
        in_path: &Path,
        env: &'a HashMap<Path, HashMap<String, ModSymbol>>,
    ) -> &'a CustomType {
        self.resolve_with_path(in_path, env).1
    }

    fn check_opaque_internal<'a>(
        &'a self,
        in_path: &Path,
        env: &HashMap<Path, HashMap<String, ModSymbol>>,
        behind_reference: bool,
        errors: &mut Vec<&'a TypeName>,
    ) {
        match self {
            TypeName::Reference(underlying, _) => {
                underlying.check_opaque_internal(in_path, env, true, errors)
            }
            TypeName::Box(underlying) => {
                underlying.check_opaque_internal(in_path, env, true, errors)
            }
            TypeName::Option(underlying) => {
                underlying.check_opaque_internal(in_path, env, false, errors)
            }
            TypeName::Result(ok, err) => {
                ok.check_opaque_internal(in_path, env, false, errors);
                err.check_opaque_internal(in_path, env, false, errors);
            }
            TypeName::Primitive(_) => {}
            TypeName::Named(_) => {
                if let CustomType::Opaque(_) = self.resolve(in_path, env) {
                    if !behind_reference {
                        errors.push(self)
                    }
                }
            }
            TypeName::Writeable => {}
            TypeName::StrReference => {}
            TypeName::PrimitiveSlice(_) => {}
            TypeName::Unit => {}
        }
    }

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
        self.check_opaque_internal(in_path, env, false, errors);
    }

    pub fn is_zst(&self) -> bool {
        // check_zst() prevents non-unit types from being ZSTs
        matches!(*self, TypeName::Unit)
    }
}

impl From<&syn::Type> for TypeName {
    /// Extract a [`TypeName`] from a [`syn::Type`] AST node.
    /// The following rules are used to infer [`TypeName`] variants:
    /// - If the type is a path with a single element that is the name of a Rust primitive, returns a [`TypeName::Primitive`]
    /// - If the type is a path with a single element [`Box`], returns a [`TypeName::Box`] with the type parameter recursively converted
    /// - If the type is a path with a single element [`Option`], returns a [`TypeName::Option`] with the type parameter recursively converted
    /// - If the type is a path equal to [`diplomat_runtime::DiplomatResult`], returns a [`TypeName::Result`] with the type parameters recursively converted
    /// - If the type is a path equal to [`diplomat_runtime::DiplomatWriteable`], returns a [`TypeName::Writeable`]
    /// - If the type is a reference to `str`, returns a [`TypeName::StrReference`]
    /// - If the type is a reference to a slice of a Rust primitive, returns a [`TypeName::PrimitiveSlice`]
    /// - If the type is a reference (`&` or `&mut`), returns a [`TypeName::Reference`] with the referenced type recursively converted
    /// - Otherwise, assume that the reference is to a [`CustomType`] in either the current module or another one, returns a [`TypeName::Named`]
    fn from(ty: &syn::Type) -> TypeName {
        match ty {
            syn::Type::Reference(r) => {
                if r.elem.to_token_stream().to_string() == "str" {
                    return TypeName::StrReference;
                }
                if let syn::Type::Slice(slice) = &*r.elem {
                    if let syn::Type::Path(p) = &*slice.elem {
                        if let Some(primitive) = p
                            .path
                            .get_ident()
                            .and_then(|i| STRING_TO_PRIMITIVE.get(i.to_string().as_str()))
                        {
                            return TypeName::PrimitiveSlice(primitive.clone());
                        }
                    }
                }
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
                } else if p.path.segments.len() == 1 && p.path.segments[0].ident == "Option" {
                    if let PathArguments::AngleBracketed(type_args) = &p.path.segments[0].arguments
                    {
                        if let GenericArgument::Type(tpe) = &type_args.args[0] {
                            TypeName::Option(Box::new(tpe.into()))
                        } else {
                            panic!("Expected first type argument for Option to be a type")
                        }
                    } else {
                        panic!("Expected angle brackets for Option type")
                    }
                } else if is_runtime_type(p, "DiplomatResult") {
                    if let PathArguments::AngleBracketed(type_args) =
                        &p.path.segments.last().unwrap().arguments
                    {
                        if let (GenericArgument::Type(ok), GenericArgument::Type(err)) =
                            (&type_args.args[0], &type_args.args[1])
                        {
                            TypeName::Result(Box::new(ok.into()), Box::new(err.into()))
                        } else {
                            panic!("Expected both type arguments for Result to be a type")
                        }
                    } else {
                        panic!("Expected angle brackets for Result type")
                    }
                } else if is_runtime_type(p, "DiplomatWriteable") {
                    TypeName::Writeable
                } else {
                    TypeName::Named(Path::from_syn(&p.path))
                }
            }
            syn::Type::Tuple(tup) => {
                if tup.elems.is_empty() {
                    TypeName::Unit
                } else {
                    todo!("Tuples are not currently supported")
                }
            }
            other => panic!("Unsupported type: {}", other.to_token_stream()),
        }
    }
}

fn is_runtime_type(p: &TypePath, name: &str) -> bool {
    (p.path.segments.len() == 1 && p.path.segments[0].ident == name)
        || (p.path.segments.len() == 2
            && p.path.segments[0].ident == "diplomat_runtime"
            && p.path.segments[1].ident == name)
}

/// A built-in Rust primitive scalar type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
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

    use syn;

    use super::TypeName;

    #[test]
    fn typename_primitives() {
        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            i32
        }));

        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            usize
        }));

        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            bool
        }));
    }

    #[test]
    fn typename_named() {
        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            MyLocalStruct
        }));
    }

    #[test]
    fn typename_references() {
        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            &i32
        }));

        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            &mut MyLocalStruct
        }));
    }

    #[test]
    fn typename_boxes() {
        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            Box<i32>
        }));

        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            Box<MyLocalStruct>
        }));
    }

    #[test]
    fn typename_option() {
        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            Option<i32>
        }));

        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            Option<MyLocalStruct>
        }));
    }

    #[test]
    fn typename_result() {
        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            DiplomatResult<MyLocalStruct, i32>
        }));

        insta::assert_yaml_snapshot!(TypeName::from(&syn::parse_quote! {
            DiplomatResult<(), MyLocalStruct>
        }));
    }
}
