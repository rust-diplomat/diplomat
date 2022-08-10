//! Store all the types contained in the HIR.

use super::{defs, paths};
use crate::{ast, Env};
use std::collections::BTreeMap;
use std::ops::Index;

/// A context type owning all types exposed to Diplomat.
pub struct TypeContext {
    out_structs: Vec<defs::OutStruct>,
    structs: Vec<defs::Struct>,
    opaques: Vec<defs::Opaque>,
    enums: Vec<defs::Enum>,
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

impl paths::ReturnableStruct {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> defs::ReturnableStruct<'tcx> {
        match self {
            paths::ReturnableStruct::Struct(ty) => defs::ReturnableStruct::Struct(ty.resolve(tcx)),
            paths::ReturnableStruct::OutStruct(ty) => {
                defs::ReturnableStruct::OutStruct(ty.resolve(tcx))
            }
        }
    }
}

impl paths::OutStruct {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::OutStruct {
        tcx.out_structs.index(self.tcx_id.0)
    }
}

impl paths::Struct {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Struct {
        tcx.structs.index(self.tcx_id.0)
    }
}

impl paths::Opaque {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Opaque {
        tcx.opaques.index(self.tcx_id.0)
    }
}

impl paths::Enum {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Enum {
        tcx.enums.index(self.tcx_id.0)
    }
}

impl TypeContext {
    /// Lowers the AST to the HIR while simultaneously performing validation.
    pub fn from_ast(env: &Env) -> Result<Self, Vec<ast::ValidityError>> {
        // this function is very much in progress
        let mut out_structs: Vec<defs::OutStruct> = Vec::with_capacity(0);
        let mut structs: Vec<defs::Struct> = Vec::with_capacity(0);
        let mut opaques: Vec<defs::Opaque> = Vec::with_capacity(0);
        let mut enums: Vec<defs::Enum> = Vec::with_capacity(0);

        let mut struct_map: BTreeMap<&ast::Struct, StructId> = BTreeMap::new();
        let mut out_struct_map: BTreeMap<&ast::Struct, OutStructId> = BTreeMap::new();
        let mut opaque_map: BTreeMap<&ast::OpaqueStruct, OpaqueId> = BTreeMap::new();
        let mut enum_map: BTreeMap<&ast::Enum, EnumId> = BTreeMap::new();

        let mut errors: Vec<ast::ValidityError> = Vec::with_capacity(0);

        // todo: make these prealloc to the right capacity
        let mut ast_structs = vec![];
        let mut ast_opaques = vec![];
        let mut ast_enums = vec![];

        // first go and add all the types, then add struct fields, then methods.
        for (path, _, sym) in env.iter_items() {
            if let ast::ModSymbol::CustomType(custom_type) = sym {
                match custom_type {
                    ast::CustomType::Struct(strct) => ast_structs.push((path, strct)),
                    ast::CustomType::Opaque(opaque) => ast_opaques.push((path, opaque)),
                    ast::CustomType::Enum(enm) => ast_enums.push((path, enm)),
                }
            }
        }

        // lets do opaques and enums first just because they're easy.

        // path_type.resolve(in_path, env)
        for (in_path, strct) in ast_structs {
            // go through fields. if we see another struct, see what that is by
            // checking which map its in. if its not in any, jump over and figure
            // out what that one is.

            // this will eventually end because non-opaque structs aren't cyclic.
            // and if it's behind a pointer, that's an error.
        }

        Ok(Self {
            out_structs,
            structs,
            opaques,
            enums,
        })
    }
}
