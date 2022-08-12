//! Store all the types contained in the HIR.

use super::{
    Borrow, EnumDef, EnumPath, EnumVariant, IdentBuf, LifetimeEnv, LifetimeNode, LoweringError,
    MaybeOwn, Method, NonOptional, OpaqueDef, OpaquePath, Optional, OutStructDef, OutStructField,
    OutStructPath, OutType, Param, ParamSelf, PrimitiveType, ReturnFallability, ReturnType,
    ReturnableStructPath, SelfType, Slice, StructDef, StructField, StructPath, Type, TypeLifetime,
    TypeLifetimes,
};
#[allow(unused_imports)] // use in docs links
use crate::hir;
use crate::{ast, Env};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::ops::Index;
use strck_ident::IntoCk;

/// A context type owning all types exposed to Diplomat.
pub struct TypeContext {
    out_structs: Vec<OutStructDef>,
    structs: Vec<StructDef>,
    opaques: Vec<OpaqueDef>,
    enums: Vec<EnumDef>,
}

/// Key used to index into a [`TypeContext`] representing a struct.
#[derive(Copy, Clone)]
pub struct StructId(usize);

/// Key used to index into a [`TypeContext`] representing an out struct.
#[derive(Copy, Clone)]
pub struct OutStructId(usize);

/// Key used to index into a [`TypeContext`] representing a opaque.
#[derive(Copy, Clone)]
pub struct OpaqueId(usize);

/// Key used to index into a [`TypeContext`] representing an enum.
#[derive(Copy, Clone)]
pub struct EnumId(usize);

impl TypeContext {
    pub(crate) fn resolve_out_struct(&self, id: OutStructId) -> &OutStructDef {
        self.out_structs.index(id.0)
    }

    pub(crate) fn resolve_struct(&self, id: StructId) -> &StructDef {
        self.structs.index(id.0)
    }

    pub(crate) fn resolve_opaque(&self, id: OpaqueId) -> &OpaqueDef {
        self.opaques.index(id.0)
    }

    pub(crate) fn resolve_enum(&self, id: EnumId) -> &EnumDef {
        self.enums.index(id.0)
    }
}

impl TypeContext {
    /// Lower the AST to the HIR while simultaneously performing validation.
    pub fn from_ast(env: &Env) -> Result<Self, Vec<LoweringError>> {
        let mut ast_out_structs = SmallVec::<[_; 16]>::new();
        let mut ast_structs = SmallVec::<[_; 16]>::new();
        let mut ast_opaques = SmallVec::<[_; 16]>::new();
        let mut ast_enums = SmallVec::<[_; 16]>::new();

        let mut errors = Vec::with_capacity(0);

        for (path, _, sym) in env.iter_items() {
            if let ast::ModSymbol::CustomType(custom_type) = sym {
                match custom_type {
                    ast::CustomType::Struct(strct) => {
                        if strct.output_only {
                            ast_out_structs.push((path, strct));
                        } else {
                            ast_structs.push((path, strct));
                        }
                    }
                    ast::CustomType::Opaque(opaque) => ast_opaques.push((path, opaque)),
                    ast::CustomType::Enum(enm) => ast_enums.push((path, enm)),
                }
            }
        }

        let lookup_id = LookupId::new(
            &ast_out_structs[..],
            &ast_structs[..],
            &ast_opaques[..],
            &ast_enums[..],
        );

        let out_structs = lower_out_structs(&ast_out_structs[..], &lookup_id, env, &mut errors);
        let structs = lower_structs(&ast_structs[..], &lookup_id, env, &mut errors);
        let opaques = lower_opaques(&ast_opaques[..], &lookup_id, env, &mut errors);
        let enums = lower_enums(&ast_enums[..], &lookup_id, env, &mut errors);

        match (out_structs, structs, opaques, enums) {
            (Ok(out_structs), Ok(structs), Ok(opaques), Ok(enums)) => Ok(Self {
                out_structs,
                structs,
                opaques,
                enums,
            }),
            _ => Err(errors),
        }
    }
}

/// Struct that just wraps the mapping from AST custom types to their IDs that
/// will show up in the final [`TypeContext`].
///
/// The entire point of this type is to reduce the number of arguments in helper
/// functions which need to look up IDs for structs. It does nothing fancy and
/// is only ever used when constructing a [`TypeContext`].
struct LookupId<'ast> {
    out_struct_map: HashMap<&'ast ast::Struct, OutStructId>,
    struct_map: HashMap<&'ast ast::Struct, StructId>,
    opaque_map: HashMap<&'ast ast::OpaqueStruct, OpaqueId>,
    enum_map: HashMap<&'ast ast::Enum, EnumId>,
}

impl<'ast> LookupId<'ast> {
    /// Returns a new [`LookupId`].
    fn new(
        out_structs: &[(&ast::Path, &'ast ast::Struct)],
        structs: &[(&ast::Path, &'ast ast::Struct)],
        opaques: &[(&ast::Path, &'ast ast::OpaqueStruct)],
        enums: &[(&ast::Path, &'ast ast::Enum)],
    ) -> Self {
        Self {
            out_struct_map: out_structs
                .iter()
                .enumerate()
                .map(|(index, (_, strct))| (*strct, OutStructId(index)))
                .collect(),
            struct_map: structs
                .iter()
                .enumerate()
                .map(|(index, (_, strct))| (*strct, StructId(index)))
                .collect(),
            opaque_map: opaques
                .iter()
                .enumerate()
                .map(|(index, (_, opaque))| (*opaque, OpaqueId(index)))
                .collect(),
            enum_map: enums
                .iter()
                .enumerate()
                .map(|(index, (_, enm))| (*enm, EnumId(index)))
                .collect(),
        }
    }

    fn resolve_out_struct(&self, strct: &ast::Struct) -> Option<OutStructId> {
        self.out_struct_map.get(strct).copied()
    }

    fn resolve_struct(&self, strct: &ast::Struct) -> Option<StructId> {
        self.struct_map.get(strct).copied()
    }

    fn resolve_opaque(&self, opaque: &ast::OpaqueStruct) -> Option<OpaqueId> {
        self.opaque_map.get(opaque).copied()
    }

    fn resolve_enum(&self, enm: &ast::Enum) -> Option<EnumId> {
        self.enum_map.get(enm).copied()
    }
}

/// Lowers an [`ast::Ident`]s into an [`hir::IdentBuf`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_ident(
    ident: &ast::Ident,
    context: &'static str,
    errors: &mut Vec<LoweringError>,
) -> Result<IdentBuf, ()> {
    match ident.as_str().ck() {
        Ok(name) => Ok(name.to_owned()),
        Err(e) => {
            errors.push(LoweringError::Other(format!(
                "Ident `{ident}` from {context} could not be turned into a Rust ident: {e}"
            )));
            Err(())
        }
    }
}

/// Lowers many [`ast::Enum`]s into [`hir::EnumDef`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_enums(
    ast_enums: &[(&ast::Path, &ast::Enum)],
    lookup_id: &LookupId,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<EnumDef>, ()> {
    let mut enums = Ok(Vec::with_capacity(ast_enums.len()));

    for (in_path, enm) in ast_enums {
        // Non short-circuiting
        let name = lower_ident(&enm.name, "enum name", errors);

        let mut variants = Ok(Vec::with_capacity(enm.variants.len()));

        for (ident, discriminant, docs) in enm.variants.iter() {
            let name = lower_ident(ident, "enum variant", errors);

            match (name, &mut variants) {
                (Ok(name), Ok(variants)) => {
                    variants.push(EnumVariant {
                        docs: docs.clone(),
                        name,
                        discriminant: *discriminant,
                    });
                }
                _ => variants = Err(()),
            }
        }

        let methods = lower_many_methods(&enm.methods[..], lookup_id, in_path, env, errors);

        match (name, variants, methods, &mut enums) {
            (Ok(name), Ok(variants), Ok(methods), Ok(enums)) => {
                enums.push(EnumDef {
                    docs: enm.docs.clone(),
                    name,
                    variants,
                    methods,
                });
            }
            _ => enums = Err(()),
        }
    }

    enums
}

/// Lowers many [`ast::OpaqueStruct`]s into [`hir::OpaqueDef`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_opaques(
    ast_opaques: &[(&ast::Path, &ast::OpaqueStruct)],
    lookup_id: &LookupId,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<OpaqueDef>, ()> {
    let mut opaques = Ok(Vec::with_capacity(ast_opaques.len()));

    for (in_path, opaque) in ast_opaques {
        let name = lower_ident(&opaque.name, "opaque name", errors);

        let methods = lower_many_methods(&opaque.methods[..], lookup_id, in_path, env, errors);

        match (name, methods, &mut opaques) {
            (Ok(name), Ok(methods), Ok(opaques)) => {
                opaques.push(OpaqueDef {
                    docs: opaque.docs.clone(),
                    name,
                    methods,
                });
            }
            _ => opaques = Err(()),
        }
    }

    opaques
}

/// Lowers many [`ast::Struct`]s into [`hir::StructDef`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_structs(
    ast_structs: &[(&ast::Path, &ast::Struct)],
    lookup_id: &LookupId,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<StructDef>, ()> {
    let mut structs = Ok(Vec::with_capacity(ast_structs.len()));

    for (in_path, strct) in ast_structs {
        let name = lower_ident(&strct.name, "struct name", errors);

        let fields = if strct.fields.is_empty() {
            errors.push(LoweringError::Other(format!(
                "struct `{}` is a ZST because it has no fields",
                strct.name
            )));
            Err(())
        } else {
            let mut fields = Ok(Vec::with_capacity(strct.fields.len()));

            for (name, ty, docs) in strct.fields.iter() {
                let name = lower_ident(name, "struct field name", errors);
                let ty = lower_type(ty, &strct.lifetimes, lookup_id, in_path, env, errors);

                match (name, ty, &mut fields) {
                    (Ok(name), Ok(ty), Ok(fields)) => fields.push(StructField {
                        docs: docs.clone(),
                        name,
                        ty,
                    }),
                    _ => fields = Err(()),
                }
            }

            fields
        };

        let methods = lower_many_methods(&strct.methods[..], lookup_id, in_path, env, errors);

        match (name, fields, methods, &mut structs) {
            (Ok(name), Ok(fields), Ok(methods), Ok(structs)) => structs.push(StructDef {
                docs: strct.docs.clone(),
                name,
                fields,
                methods,
            }),
            _ => structs = Err(()),
        }
    }

    structs
}

/// Lowers many [`ast::Struct`]s into [`hir::OutStructDef`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
// note: this is basically copy-pasted from above
fn lower_out_structs(
    ast_out_structs: &[(&ast::Path, &ast::Struct)],
    lookup_id: &LookupId,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<OutStructDef>, ()> {
    let mut structs = Ok(Vec::with_capacity(ast_out_structs.len()));

    for (in_path, strct) in ast_out_structs {
        let name = lower_ident(&strct.name, "out-struct name", errors);

        let fields = if strct.fields.is_empty() {
            errors.push(LoweringError::Other(format!(
                "struct `{}` is a ZST because it has no fields",
                strct.name
            )));
            Err(())
        } else {
            let mut fields = Ok(Vec::with_capacity(strct.fields.len()));

            for (name, ty, docs) in strct.fields.iter() {
                let name = lower_ident(name, "out-struct field name", errors);
                let ty = lower_out_type(ty, &strct.lifetimes, lookup_id, in_path, env, errors);

                match (name, ty, &mut fields) {
                    (Ok(name), Ok(ty), Ok(fields)) => fields.push(OutStructField {
                        docs: docs.clone(),
                        name,
                        ty,
                    }),
                    _ => fields = Err(()),
                }
            }

            fields
        };

        let methods = lower_many_methods(&strct.methods[..], lookup_id, in_path, env, errors);

        match (name, fields, methods, &mut structs) {
            (Ok(name), Ok(fields), Ok(methods), Ok(structs)) => structs.push(OutStructDef {
                docs: strct.docs.clone(),
                name,
                fields,
                methods,
            }),
            _ => structs = Err(()),
        }
    }

    structs
}

/// Lowers an [`ast::TypeName`]s into a [`hir::Type`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_type(
    ty: &ast::TypeName,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Type, ()> {
    match ty {
        ast::TypeName::Primitive(prim) => Ok(Type::Primitive(PrimitiveType::from_ast(*prim))),
        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                if let Some(tcx_id) = lookup_id.resolve_struct(strct) {
                    let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                    Ok(Type::Struct(StructPath::new(lifetimes, tcx_id)))
                } else if lookup_id.resolve_out_struct(strct).is_some() {
                    errors.push(LoweringError::Other(format!("found struct in input that is marked with #[diplomat::out]: {path} in {ty}")));
                    Err(())
                } else {
                    unreachable!("struct `{}` wasn't found in the set of structs or out-structs, this is a bug.", strct.name);
                }
            }
            ast::CustomType::Opaque(_) => {
                errors.push(LoweringError::Other(format!("Opaque passed by value in input: {path}")));
                Err(())
            }
            ast::CustomType::Enum(enm) => {
                let tcx_id = lookup_id.resolve_enum(enm).expect("can't find enum in lookup map, which contains all enums from env");

                Ok(Type::Enum(EnumPath::new(tcx_id)))
            }
        },
        ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
            ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                ast::CustomType::Opaque(opaque) => {
                    let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                    let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                    let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                        "can't find opaque in lookup map, which contains all opaques from env",
                    );

                    Ok(Type::Opaque(OpaquePath::new(
                        lifetimes,
                        Optional(false),
                        borrow,
                        tcx_id,
                    )))
                }
                _ => Err(LoweringError::Other(format!("found &T in input where T is a custom type, but not opaque. T = {ref_ty}"))),
            },
            _ => Err(LoweringError::Other(format!("found &T in input where T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
        }
        .map_err(|e| errors.push(e)),
        ast::TypeName::Box(box_ty) => {
            errors.push(match box_ty.as_ref() {
                ast::TypeName::Named(path) => {
                    match path.resolve(in_path, env) {
                        ast::CustomType::Opaque(_) => LoweringError::Other(format!("found Box<T> in input where T is an opaque, but owned opaques aren't allowed in inputs. try &T instead? T = {path}")),
                        _ => LoweringError::Other(format!("found Box<T> in input where T is a custom type but not opaque. non-opaques can't be behind pointers, and opaques in inputs can't be owned. T = {path}")),
                    }
                }
                _ => LoweringError::Other(format!("found Box<T> in input where T isn't a custom type. T = {box_ty}")),
            });
            Err(())
        }
        ast::TypeName::Option(opt_ty) => {
            match opt_ty.as_ref() {
                ast::TypeName::Reference(lifetime, mutability, ref_ty) => {
                    match ref_ty.as_ref() {
                        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                                let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                                let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(Type::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(false),
                                    borrow,
                                    tcx_id,
                                )))
                            }
                            _ => Err(LoweringError::Other(format!("found Option<&T> in input where T is a custom type, but it's not opaque. T = {ref_ty}"))),
                        },
                        _ => Err(LoweringError::Other(format!("found Option<&T> in input, but T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
                    }
                    .map_err(|e| errors.push(e))
                }
                ast::TypeName::Box(box_ty) => {
                    // we could see whats in the box here too
                    errors.push(LoweringError::Other(format!("found Option<Box<T>> in input, but box isn't allowed in inputs. T = {box_ty}")));
                    Err(())
                }
                _ => {
                    errors.push(LoweringError::Other(format!("found Option<T> in input, where T isn't a reference but Option<T> in inputs requires that T is a reference to an opaque. T = {opt_ty}")));
                    Err(())
                }
            }
        }
        ast::TypeName::Result(_, _) => {
            errors.push(LoweringError::Other("Results can only appear as the top-level return type of methods".into()));
            Err(())
        }
        ast::TypeName::Writeable => {
            errors.push(LoweringError::Other("Writeables can only appear as the last parameter of a method".into()));
            Err(())
        }
        ast::TypeName::StrReference(lifetime) => {
            let lifetime = TypeLifetime::from_ast(parent_lifetimes, lifetime);

            Ok(Type::Slice(Slice::Str(lifetime)))
        }
        ast::TypeName::PrimitiveSlice(lifetime, mutability, prim) => {
            let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
            let prim = PrimitiveType::from_ast(*prim);

            Ok(Type::Slice(Slice::Primitive(borrow, prim)))
        }
        ast::TypeName::Unit => {
            errors.push(LoweringError::Other("Unit types can only appear as the return value of a method, or as the Ok/Err variants of a returned result".into()));
            Err(())
        }
    }
}

/// Lowers an [`ast::TypeName`]s into an [`hir::OutType`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_out_type(
    ty: &ast::TypeName,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<OutType, ()> {
    match ty {
        ast::TypeName::Primitive(prim) => Ok(OutType::Primitive(PrimitiveType::from_ast(*prim))),
        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);

                if let Some(tcx_id) = lookup_id.resolve_struct(strct) {
                    Ok(OutType::Struct(ReturnableStructPath::Struct(StructPath::new(lifetimes, tcx_id))))
                } else if let Some(tcx_id) = lookup_id.resolve_out_struct(strct) {
                    Ok(OutType::Struct(ReturnableStructPath::OutStruct(OutStructPath::new(lifetimes, tcx_id))))
                } else {
                    unreachable!("struct `{}` wasn't found in the set of structs or out-structs, this is a bug.", strct.name);
                }
            }
            ast::CustomType::Opaque(_) => {
                errors.push(LoweringError::Other(format!("Opaque passed by value in input: {path}")));
                Err(())
            }
            ast::CustomType::Enum(enm) => {
                let tcx_id = lookup_id.resolve_enum(enm).expect("can't find enum in lookup map, which contains all enums from env");

                Ok(OutType::Enum(EnumPath::new(tcx_id)))
            }
        },
        ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
            ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                ast::CustomType::Opaque(opaque) => {
                    let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                    let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                    let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                        "can't find opaque in lookup map, which contains all opaques from env",
                    );

                    Ok(OutType::Opaque(OpaquePath::new(
                        lifetimes,
                        Optional(false),
                        MaybeOwn::Borrow(borrow),
                        tcx_id,
                    )))
                }
                _ => Err(LoweringError::Other(format!("found &T in output where T is a custom type, but not opaque. T = {ref_ty}"))),
            },
            _ => Err(LoweringError::Other(format!("found &T in output where T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
        }
        .map_err(|e| errors.push(e)),
        ast::TypeName::Box(box_ty) => {
            errors.push(match box_ty.as_ref() {
                ast::TypeName::Named(path) => {
                    match path.resolve(in_path, env) {
                        ast::CustomType::Opaque(_) => LoweringError::Other(format!("found Box<T> in input where T is an opaque, but owned opaques aren't allowed in inputs. try &T instead? T = {path}")),
                        _ => LoweringError::Other(format!("found Box<T> in input where T is a custom type but not opaque. non-opaques can't be behind pointers, and opaques in inputs can't be owned. T = {path}")),
                    }
                }
                _ => LoweringError::Other(format!("found Box<T> in input where T isn't a custom type. T = {box_ty}")),
            });
            Err(())
        }
        ast::TypeName::Option(opt_ty) => {
            match opt_ty.as_ref() {
                ast::TypeName::Reference(lifetime, mutability, ref_ty) => {
                    match ref_ty.as_ref() {
                        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                                let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                                let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(OutType::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    MaybeOwn::Borrow(borrow),
                                    tcx_id,
                                )))
                            }
                            _ => Err(LoweringError::Other(format!("found Option<&T> where T is a custom type, but it's not opaque. T = {ref_ty}"))),
                        },
                        _ => Err(LoweringError::Other(format!("found Option<&T>, but T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
                    }
                    .map_err(|e| errors.push(e))
                }
                ast::TypeName::Box(box_ty) => {
                    match box_ty.as_ref() {
                        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                                let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(OutType::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    MaybeOwn::Own,
                                    tcx_id,
                                )))
                            }
                            _ => Err(LoweringError::Other(format!("found Option<Box<T>> where T is a custom type, but it's not opaque. T = {box_ty}"))),
                        }
                        _ => Err(LoweringError::Other(format!("found Option<Box<T>>, but T isn't a custom type and therefore not opaque. T = {box_ty}"))),
                    }
                    .map_err(|e| errors.push(e))
                }
                _ => {
                    errors.push(LoweringError::Other(format!("found Option<T>, where T isn't a reference but Option<T> in inputs requires that T is a reference to an opaque. T = {opt_ty}")));
                    Err(())
                }
            }
        }
        ast::TypeName::Result(_, _) => {
            errors.push(LoweringError::Other("Results can only appear as the top-level return type of methods".into()));
            Err(())
        }
        ast::TypeName::Writeable => {
            errors.push(LoweringError::Other("Writeables can only appear as the last parameter of a method".into()));
            Err(())
        }
        ast::TypeName::StrReference(lifetime) => {
            let lifetime = TypeLifetime::from_ast(parent_lifetimes, lifetime);

            Ok(OutType::Slice(Slice::Str(lifetime)))
        }
        ast::TypeName::PrimitiveSlice(lifetime, mutability, prim) => {
            let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
            let prim = PrimitiveType::from_ast(*prim);

            Ok(OutType::Slice(Slice::Primitive(borrow, prim)))
        }
        ast::TypeName::Unit => {
            errors.push(LoweringError::Other("Unit types can only appear as the return value of a method, or as the Ok/Err variants of a returned result".into()));
            Err(())
        }
    }
}

/// Lowers an [`ast::Method`]s into an [`hir::Method`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_method(
    method: &ast::Method,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Method, ()> {
    let name = lower_ident(&method.name, "method name", errors);
    let full_path_name = lower_ident(&method.full_path_name, "method full_path_name", errors);

    let param_self = method
        .self_param
        .as_ref()
        .map(|self_param| {
            lower_self_param(
                self_param,
                &method.lifetime_env,
                lookup_id,
                &method.full_path_name,
                in_path,
                env,
                errors,
            )
        })
        .transpose();

    let (params, takes_writeable) = match method.params.split_last() {
        Some((last, remaining)) if last.is_writeable() => (remaining, true),
        _ => (&method.params[..], false),
    };

    let params = lower_many_params(
        params,
        &method.lifetime_env,
        lookup_id,
        in_path,
        env,
        errors,
    );

    let output = lower_return_type(
        &method.return_type,
        takes_writeable,
        &method.lifetime_env,
        lookup_id,
        in_path,
        env,
        errors,
    );

    let lifetime_env = lower_lifetime_env(&method.lifetime_env, errors);

    match (name, lifetime_env, param_self, params, output) {
        (Ok(name), Ok(lifetime_env), Ok(param_self), Ok(params), Ok(output)) => Ok(Method {
            docs: method.docs.clone(),
            name,
            lifetime_env,
            param_self,
            params,
            output,
        }),
        _ => Err(()),
    }
}

/// Lowers many [`ast::Method`]s into a vector of [`hir::Method`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_many_methods(
    ast_methods: &[ast::Method],
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<Method>, ()> {
    let mut methods = Ok(Vec::with_capacity(ast_methods.len()));

    for method in ast_methods {
        let method = lower_method(method, lookup_id, in_path, env, errors);
        match (method, &mut methods) {
            (Ok(method), Ok(methods)) => {
                methods.push(method);
            }
            _ => methods = Err(()),
        }
    }

    methods
}

/// Lowers an [`ast::LifetimeEnv`] into an [`hir::LifetimeEnv`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_lifetime_env(
    lifetime_env: &ast::LifetimeEnv,
    errors: &mut Vec<LoweringError>,
) -> Result<LifetimeEnv, ()> {
    let mut nodes = Ok(SmallVec::new());

    for node in lifetime_env.nodes.iter() {
        let name = lower_ident(node.lifetime.name(), "lifetime", errors);
        match (name, &mut nodes) {
            (Ok(name), Ok(nodes)) => nodes.push(LifetimeNode::new(
                name,
                node.longer.iter().copied().collect(),
                node.shorter.iter().copied().collect(),
            )),
            _ => nodes = Err(()),
        }
    }

    nodes.map(LifetimeEnv::new)
}

/// Lowers an [`ast::SelfParam`] into an [`hir::ParamSelf`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_self_param(
    self_param: &ast::SelfParam,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    method_full_path: &ast::Ident, // for better error msg
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<ParamSelf, ()> {
    match self_param.path_type.resolve(in_path, env) {
        ast::CustomType::Struct(strct) => {
            if let Some(tcx_id) = lookup_id.resolve_struct(strct) {
                if self_param.reference.is_some() {
                    errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes a reference to a struct as a self parameter, which isn't allowed")));
                    Err(())
                } else {
                    let lifetimes =
                        TypeLifetimes::from_ast(parent_lifetimes, &self_param.path_type);

                    Ok(ParamSelf::new(SelfType::Struct(StructPath::new(
                        lifetimes, tcx_id,
                    ))))
                }
            } else if lookup_id.resolve_out_struct(strct).is_some() {
                if self_param.reference.is_some() {
                    errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an out-struct as the self parameter, which isn't allowed. Also, it's also behind a reference, but only opaques can be behind references")));
                } else {
                    errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an out-struct as the self parameter, which isn't allowed")));
                }
                Err(())
            } else {
                unreachable!(
                    "struct `{}` wasn't found in the set of structs or out-structs, this is a bug.",
                    strct.name
                );
            }
        }
        ast::CustomType::Opaque(opaque) => {
            let tcx_id = lookup_id.resolve_opaque(opaque).expect("opaque is in env");

            if let Some((lifetime, mutability)) = &self_param.reference {
                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, &self_param.path_type);
                let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);

                Ok(ParamSelf::new(SelfType::Opaque(OpaquePath::new(
                    lifetimes,
                    NonOptional,
                    borrow,
                    tcx_id,
                ))))
            } else {
                errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an opaque by value as the self parameter, but opaques as inputs must be behind refs")));
                Err(())
            }
        }
        ast::CustomType::Enum(enm) => {
            let tcx_id = lookup_id.resolve_enum(enm).expect("enum is in env");

            Ok(ParamSelf::new(SelfType::Enum(EnumPath::new(tcx_id))))
        }
    }
}

/// Lowers many [`ast::Param`]s into a vector of [`hir::Param`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
///
/// Note that this expects that if there was a writeable param at the end in
/// the method, `ast_params` was sliced to not include it. This happens in
/// `lower_method`, the caller of this function.
fn lower_many_params(
    ast_params: &[ast::Param],
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<Param>, ()> {
    let mut params = Ok(Vec::with_capacity(ast_params.len()));

    for param in ast_params {
        let name = lower_ident(&param.name, "param name", errors);
        let ty = lower_type(&param.ty, parent_lifetimes, lookup_id, in_path, env, errors);

        match (name, ty, &mut params) {
            (Ok(name), Ok(ty), Ok(params)) => {
                params.push(Param::new(name, ty));
            }
            _ => params = Err(()),
        }
    }

    params
}

/// Lowers the return type of an [`ast::Method`] into a [`hir::ReturnFallability`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_return_type(
    return_type: &Option<ast::TypeName>,
    takes_writeable: bool,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<ReturnFallability, ()> {
    let return_type = return_type.as_ref().unwrap_or(&ast::TypeName::Unit);

    match return_type {
        ast::TypeName::Result(ok_ty, err_ty) => {
            let ok_ty = match ok_ty.as_ref() {
                ast::TypeName::Unit => {
                    if takes_writeable {
                        Ok(Some(ReturnType::Writeable))
                    } else {
                        Ok(None)
                    }
                }
                ty => lower_out_type(ty, parent_lifetimes, lookup_id, in_path, env, errors)
                    .map(|ty| Some(ReturnType::OutType(ty))),
            };

            let err_ty = lower_out_type(err_ty, parent_lifetimes, lookup_id, in_path, env, errors);

            match (ok_ty, err_ty) {
                (Ok(ok_ty), Ok(err_ty)) => Ok(ReturnFallability::Fallible(ok_ty, err_ty)),
                _ => Err(()),
            }
        }
        ast::TypeName::Unit => {
            if takes_writeable {
                Ok(ReturnFallability::Infallible(Some(ReturnType::Writeable)))
            } else {
                Ok(ReturnFallability::Infallible(None))
            }
        }
        _ => lower_out_type(
            return_type,
            parent_lifetimes,
            lookup_id,
            in_path,
            env,
            errors,
        )
        .map(|ty| ReturnFallability::Infallible(Some(ReturnType::OutType(ty)))),
    }
}
