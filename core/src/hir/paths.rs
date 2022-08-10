use super::{
    Borrow, EnumDef, EnumId, OpaqueDef, OpaqueId, OutStructDef, OutStructId, ReturnableStructDef,
    StructDef, StructId, TypeContext, TypeLifetimes,
};

/// Path to a struct that may appear as an output.
pub enum ReturnableStructPath {
    Struct(StructPath),
    OutStruct(OutStructPath),
}

/// Path to a struct that can only be used as an output.
pub struct OutStructPath {
    pub lifetimes: TypeLifetimes,
    tcx_id: OutStructId,
}

/// Path to a struct that can be used in inputs and outputs.
pub struct StructPath {
    pub lifetimes: TypeLifetimes,
    tcx_id: StructId,
}

/// Path to an opaque.
///
/// There are three kinds of opaques that Diplomat uses, so this type has two
/// generic arguments to differentiate between the three, while still showing
/// that the three are all paths to opaques. The monomorphized versions that
/// Diplomat uses are:
///
/// 1. `OpaquePath<Optional, MaybeOwn>`: Opaques in return types,
/// which can be optional and either owned or borrowed.
/// 2. `OpaquePath<Optional, Borrow>`: Opaques in method parameters, which can
/// be optional but must be borrowed, since most languages don't have a way to
/// entirely give up ownership of a value.
/// 3. `OpaquePath<NonOptional, Borrow>`: Opaques in the `&self` position, which
/// cannot be optional and must be borrowed for the same reason as above.
pub struct OpaquePath<Opt, Owner> {
    pub lifetimes: TypeLifetimes,
    optional: Opt,
    owner: Owner,
    tcx_id: OpaqueId,
}

pub struct Optional(bool);

pub struct NonOptional;

impl<Owner> OpaquePath<Optional, Owner> {
    pub fn is_optional(&self) -> bool {
        self.optional.0
    }
}

impl<Opt> OpaquePath<Opt, MaybeOwn> {
    pub fn as_borrowed(&self) -> Option<&Borrow> {
        self.owner.as_borrowed()
    }
}

impl<Opt> OpaquePath<Opt, Borrow> {
    pub fn borrowed(&self) -> &Borrow {
        &self.owner
    }
}

/// Path to an enum.
pub struct EnumPath {
    tcx_id: EnumId,
}

/// Determine whether a pointer to an opaque type is owned or borrowed.
///
/// Since owned opaques cannot be used as inputs, this only appears in output types.
#[derive(Copy, Clone)]
pub enum MaybeOwn {
    Own,
    Borrow(Borrow),
}

impl MaybeOwn {
    pub fn as_borrowed(&self) -> Option<&Borrow> {
        match self {
            MaybeOwn::Own => None,
            MaybeOwn::Borrow(borrow) => Some(borrow),
        }
    }
}

impl ReturnableStructPath {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> ReturnableStructDef<'tcx> {
        match self {
            ReturnableStructPath::Struct(path) => ReturnableStructDef::Struct(path.resolve(tcx)),
            ReturnableStructPath::OutStruct(path) => {
                ReturnableStructDef::OutStruct(path.resolve(tcx))
            }
        }
    }
}

impl OutStructPath {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx OutStructDef {
        tcx.resolve_out_struct(self.tcx_id)
    }
}

impl StructPath {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx StructDef {
        tcx.resolve_struct(self.tcx_id)
    }
}

impl<Opt, Owner> OpaquePath<Opt, Owner> {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx OpaqueDef {
        tcx.resolve_opaque(self.tcx_id)
    }
}

impl EnumPath {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx EnumDef {
        tcx.resolve_enum(self.tcx_id)
    }
}
