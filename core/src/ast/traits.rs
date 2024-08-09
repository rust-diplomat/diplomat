use serde::Serialize;

use super::docs::Docs;
use super::{Attrs, Ident, LifetimeEnv, Method, Mutability, PathType, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
#[non_exhaustive]
pub struct Trait {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    // pub fcts: Callback,
    pub attrs: Attrs,
}

impl Trait {
}