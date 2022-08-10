use super::{Borrow, EnumId, OpaqueId, OutStructId, StructId, TypeLifetimes};

/// Path to a struct that may appear as an output.
pub enum ReturnableStruct {
    Struct(Struct),
    OutStruct(OutStruct),
}

/// Path to a struct that can only be used as an output.
pub struct OutStruct {
    pub lifetimes: TypeLifetimes,
    pub(crate) tcx_id: OutStructId,
}

/// Path to a struct that can be used in inputs and outputs.
pub struct Struct {
    pub lifetimes: TypeLifetimes,
    pub(crate) tcx_id: StructId,
}

/// Path to an opaque that may be owned (`Box<T>`) or borrowed (`&T`), and may
/// also be nullable (`Option<Box<T>>`/`Option<&T>`).
pub struct ReturnedOpaque {
    pub lifetimes: TypeLifetimes,
    pub nullable: bool,
    pub ownership: Ownership,
    pub(crate) tcx_id: OpaqueId,
}

/// Path to an opaque that is borrowed (`&T`), and may
/// also be nullable (`Option<&T>`).
pub struct OpaqueRef {
    pub lifetimes: TypeLifetimes,
    pub nullable: bool,
    pub borrow: Borrow,
    pub(crate) tcx_id: OpaqueId,
}

/// Path to an opaque that can appear in the `&self` position and is always borrowed
/// (`&T`), but not nullable.
pub struct SelfOpaqueRef {
    pub lifetimes: TypeLifetimes,
    pub borrow: Borrow,
    pub(crate) tcx_id: OpaqueId,
}

/// Path to an enum.
pub struct Enum {
    pub(crate) tcx_id: EnumId,
}

/// Determine whether a pointer to an opaque type is owned or borrowed.
///
/// Since owned opaques cannot be used as inputs, this only appears in output types.
#[derive(Copy, Clone)]
pub enum Ownership {
    Owned,
    Borrowed(Borrow),
}
