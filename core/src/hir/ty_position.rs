use super::{Borrow, MaybeOwn, OutStructId, ReturnableStructPath, StructId, StructPath, TypeId};
use core::fmt::Debug;

pub trait TyPosition: Debug + Copy {
    const IS_OUT_ONLY: bool;
    type OpaqueOwnership: Debug;
    type StructId: Debug;
    type StructPath: Debug;

    fn id_for_path(p: &Self::StructPath) -> TypeId;
}

#[derive(Debug, Copy, Clone)]
pub struct Everywhere;
#[derive(Debug, Copy, Clone)]
pub struct OutputOnly;

impl TyPosition for Everywhere {
    const IS_OUT_ONLY: bool = false;
    type OpaqueOwnership = Borrow;
    type StructId = StructId;
    type StructPath = StructPath;

    fn id_for_path(p: &Self::StructPath) -> TypeId {
        p.tcx_id.into()
    }
}

impl TyPosition for OutputOnly {
    const IS_OUT_ONLY: bool = true;
    type OpaqueOwnership = MaybeOwn;
    type StructId = OutStructId;
    type StructPath = ReturnableStructPath;
    fn id_for_path(p: &Self::StructPath) -> TypeId {
        match p {
            ReturnableStructPath::Struct(p) => p.tcx_id.into(),
            ReturnableStructPath::OutStruct(p) => p.tcx_id.into(),
        }
    }
}
