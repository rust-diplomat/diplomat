use super::{defs, Borrow, EnumId, OpaqueId, OutStructId, StructId, TypeContext, TypeLifetimes};

/// Path to a struct that may appear as an output.
pub enum ReturnableStruct {
    Struct(Struct),
    OutStruct(OutStruct),
}

/// Path to a struct that can only be used as an output.
pub struct OutStruct {
    pub lifetimes: TypeLifetimes,
    tcx_id: OutStructId,
}

/// Path to a struct that can be used in inputs and outputs.
pub struct Struct {
    pub lifetimes: TypeLifetimes,
    tcx_id: StructId,
}

/// Path to an opaque that may be owned (`Box<T>`) or borrowed (`&T`), and may
/// also be nullable (`Option<Box<T>>`/`Option<&T>`).
pub struct ReturnedOpaque {
    pub lifetimes: TypeLifetimes,
    pub nullable: bool,
    pub ownership: Ownership,
    tcx_id: OpaqueId,
}

/// Path to an opaque that is borrowed (`&T`), and may
/// also be nullable (`Option<&T>`).
pub struct OpaqueRef {
    pub lifetimes: TypeLifetimes,
    pub nullable: bool,
    pub borrow: Borrow,
    tcx_id: OpaqueId,
}

/// Path to an opaque that can appear in the `&self` position and is always borrowed
/// (`&T`), but not nullable.
pub struct SelfOpaqueRef {
    pub lifetimes: TypeLifetimes,
    pub borrow: Borrow,
    tcx_id: OpaqueId,
}

/// Path to an enum.
pub struct Enum {
    tcx_id: EnumId,
}

/// Determine whether a pointer to an opaque type is owned or borrowed.
///
/// Since owned opaques cannot be used as inputs, this only appears in output types.
#[derive(Copy, Clone)]
pub enum Ownership {
    Owned,
    Borrowed(Borrow),
}

impl ReturnableStruct {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> defs::ReturnableStruct<'tcx> {
        match self {
            ReturnableStruct::Struct(ty) => defs::ReturnableStruct::Struct(ty.resolve(tcx)),
            ReturnableStruct::OutStruct(ty) => defs::ReturnableStruct::OutStruct(ty.resolve(tcx)),
        }
    }
}

impl OutStruct {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::OutStruct {
        tcx.resolve_out_struct(self.tcx_id)
    }
}

impl Struct {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Struct {
        tcx.resolve_struct(self.tcx_id)
    }
}

impl ReturnedOpaque {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Opaque {
        tcx.resolve_opaque(self.tcx_id)
    }
}

impl OpaqueRef {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Opaque {
        tcx.resolve_opaque(self.tcx_id)
    }
}

impl SelfOpaqueRef {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Opaque {
        tcx.resolve_opaque(self.tcx_id)
    }
}

impl Enum {
    pub fn resolve<'tcx>(&self, tcx: &'tcx TypeContext) -> &'tcx defs::Enum {
        tcx.resolve_enum(self.tcx_id)
    }
}
