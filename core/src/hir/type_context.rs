//! Store all the types contained in the HIR.

use super::lowering::ItemAndInfo;
use super::{
    AttributeContext, AttributeValidator, Attrs, EnumDef, LoweringContext, LoweringError,
    OpaqueDef, OutStructDef, StructDef, TypeDef,
};
use crate::ast::attrs::AttrInheritContext;
#[allow(unused_imports)] // use in docs links
use crate::hir;
use crate::{ast, Env};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::ops::Index;

/// A context type owning all types exposed to Diplomat.
#[derive(Debug)]
pub struct TypeContext {
    out_structs: Vec<OutStructDef>,
    structs: Vec<StructDef>,
    opaques: Vec<OpaqueDef>,
    enums: Vec<EnumDef>,
}

/// Key used to index into a [`TypeContext`] representing a struct.
#[derive(Copy, Clone, Debug)]
pub struct StructId(usize);

/// Key used to index into a [`TypeContext`] representing an out struct.
#[derive(Copy, Clone, Debug)]
pub struct OutStructId(usize);

/// Key used to index into a [`TypeContext`] representing a opaque.
#[derive(Copy, Clone, Debug)]
pub struct OpaqueId(usize);

/// Key used to index into a [`TypeContext`] representing an enum.
#[derive(Copy, Clone, Debug)]
pub struct EnumId(usize);

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum TypeId {
    Struct(StructId),
    OutStruct(OutStructId),
    Opaque(OpaqueId),
    Enum(EnumId),
}

impl TypeContext {
    pub fn all_types<'tcx>(&'tcx self) -> impl Iterator<Item = (TypeId, TypeDef<'tcx>)> {
        self.structs
            .iter()
            .enumerate()
            .map(|(i, ty)| (TypeId::Struct(StructId(i)), TypeDef::Struct(ty)))
            .chain(
                self.out_structs
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (TypeId::OutStruct(OutStructId(i)), TypeDef::OutStruct(ty))),
            )
            .chain(
                self.opaques
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (TypeId::Opaque(OpaqueId(i)), TypeDef::Opaque(ty))),
            )
            .chain(
                self.enums
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (TypeId::Enum(EnumId(i)), TypeDef::Enum(ty))),
            )
    }

    pub fn out_structs(&self) -> &[OutStructDef] {
        &self.out_structs
    }

    pub fn structs(&self) -> &[StructDef] {
        &self.structs
    }

    pub fn opaques(&self) -> &[OpaqueDef] {
        &self.opaques
    }

    pub fn enums(&self) -> &[EnumDef] {
        &self.enums
    }

    pub fn resolve_type<'tcx>(&'tcx self, id: TypeId) -> TypeDef<'tcx> {
        match id {
            TypeId::Struct(i) => TypeDef::Struct(self.resolve_struct(i)),
            TypeId::OutStruct(i) => TypeDef::OutStruct(self.resolve_out_struct(i)),
            TypeId::Opaque(i) => TypeDef::Opaque(self.resolve_opaque(i)),
            TypeId::Enum(i) => TypeDef::Enum(self.resolve_enum(i)),
        }
    }

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

    /// Lower the AST to the HIR while simultaneously performing validation.
    pub fn from_ast(
        env: &Env,
        attr_validator: impl AttributeValidator + 'static,
    ) -> Result<Self, Vec<LoweringError>> {
        let mut ast_out_structs = SmallVec::<[_; 16]>::new();
        let mut ast_structs = SmallVec::<[_; 16]>::new();
        let mut ast_opaques = SmallVec::<[_; 16]>::new();
        let mut ast_enums = SmallVec::<[_; 16]>::new();

        let mut errors = Vec::with_capacity(0);

        for (path, mod_env) in env.iter_modules() {
            let mod_attrs = Attrs::from_ast(
                &mod_env.attrs,
                &attr_validator,
                AttributeContext::Module,
                &Default::default(),
                &mut errors,
            );
            let ty_attrs = mod_attrs.for_inheritance(AttrInheritContext::Type);
            let method_attrs =
                mod_attrs.for_inheritance(AttrInheritContext::MethodOrImplFromModule);

            for sym in mod_env.items() {
                if let ast::ModSymbol::CustomType(custom_type) = sym {
                    match custom_type {
                        ast::CustomType::Struct(strct) => {
                            let item = ItemAndInfo {
                                item: strct,
                                in_path: path,
                                ty_parent_attrs: ty_attrs.clone(),
                                method_parent_attrs: method_attrs.clone(),
                            };
                            if strct.output_only {
                                ast_out_structs.push(item);
                            } else {
                                ast_structs.push(item);
                            }
                        }
                        ast::CustomType::Opaque(opaque) => {
                            let item = ItemAndInfo {
                                item: opaque,
                                in_path: path,
                                ty_parent_attrs: ty_attrs.clone(),
                                method_parent_attrs: method_attrs.clone(),
                            };
                            ast_opaques.push(item)
                        }
                        ast::CustomType::Enum(enm) => {
                            let item = ItemAndInfo {
                                item: enm,
                                in_path: path,
                                ty_parent_attrs: ty_attrs.clone(),
                                method_parent_attrs: method_attrs.clone(),
                            };
                            ast_enums.push(item)
                        }
                    }
                }
            }
        }

        let lookup_id = LookupId::new(
            &ast_out_structs[..],
            &ast_structs[..],
            &ast_opaques[..],
            &ast_enums[..],
        );
        let attr_validator = Box::new(attr_validator);

        let mut ctx = LoweringContext {
            lookup_id,
            env,
            errors: &mut errors,
            attr_validator,
        };

        let out_structs = ctx.lower_all_out_structs(ast_out_structs.into_iter());
        let structs = ctx.lower_all_structs(ast_structs.into_iter());
        let opaques = ctx.lower_all_opaques(ast_opaques.into_iter());
        let enums = ctx.lower_all_enums(ast_enums.into_iter());

        match (out_structs, structs, opaques, enums) {
            (Ok(out_structs), Ok(structs), Ok(opaques), Ok(enums)) => {
                assert!(
                    errors.is_empty(),
                    "All lowering succeeded but still found error messages: {errors:?}"
                );
                Ok(Self {
                    out_structs,
                    structs,
                    opaques,
                    enums,
                })
            }
            _ => {
                assert!(!errors.is_empty(), "Lowering failed without error messages");
                Err(errors)
            }
        }
    }
}

/// Struct that just wraps the mapping from AST custom types to their IDs that
/// will show up in the final [`TypeContext`].
///
/// The entire point of this type is to reduce the number of arguments in helper
/// functions which need to look up IDs for structs. It does nothing fancy and
/// is only ever used when constructing a [`TypeContext`].
pub(super) struct LookupId<'ast> {
    out_struct_map: HashMap<&'ast ast::Struct, OutStructId>,
    struct_map: HashMap<&'ast ast::Struct, StructId>,
    opaque_map: HashMap<&'ast ast::OpaqueStruct, OpaqueId>,
    enum_map: HashMap<&'ast ast::Enum, EnumId>,
}

impl<'ast> LookupId<'ast> {
    /// Returns a new [`LookupId`].
    fn new(
        out_structs: &[ItemAndInfo<'ast, ast::Struct>],
        structs: &[ItemAndInfo<'ast, ast::Struct>],
        opaques: &[ItemAndInfo<'ast, ast::OpaqueStruct>],
        enums: &[ItemAndInfo<'ast, ast::Enum>],
    ) -> Self {
        Self {
            out_struct_map: out_structs
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, OutStructId(index)))
                .collect(),
            struct_map: structs
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, StructId(index)))
                .collect(),
            opaque_map: opaques
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, OpaqueId(index)))
                .collect(),
            enum_map: enums
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, EnumId(index)))
                .collect(),
        }
    }

    pub(super) fn resolve_out_struct(&self, strct: &ast::Struct) -> Option<OutStructId> {
        self.out_struct_map.get(strct).copied()
    }

    pub(super) fn resolve_struct(&self, strct: &ast::Struct) -> Option<StructId> {
        self.struct_map.get(strct).copied()
    }

    pub(super) fn resolve_opaque(&self, opaque: &ast::OpaqueStruct) -> Option<OpaqueId> {
        self.opaque_map.get(opaque).copied()
    }

    pub(super) fn resolve_enum(&self, enm: &ast::Enum) -> Option<EnumId> {
        self.enum_map.get(enm).copied()
    }
}

impl From<StructId> for TypeId {
    fn from(x: StructId) -> Self {
        TypeId::Struct(x)
    }
}

impl From<OutStructId> for TypeId {
    fn from(x: OutStructId) -> Self {
        TypeId::OutStruct(x)
    }
}

impl From<OpaqueId> for TypeId {
    fn from(x: OpaqueId) -> Self {
        TypeId::Opaque(x)
    }
}

impl From<EnumId> for TypeId {
    fn from(x: EnumId) -> Self {
        TypeId::Enum(x)
    }
}
