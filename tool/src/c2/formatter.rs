//! This module contains functions for formatting types

use diplomat_core::hir::TypeId;
use std::borrow::Cow;

impl super::CContext {
    pub fn fmt_type_name<'tcx>(&'tcx self, id: TypeId) -> Cow<'tcx, str> {
        // Currently don't do anything fancy
        // Eventually apply rename rules and such
        self.tcx.resolve_type(id).name().as_str().into()
    }
}
