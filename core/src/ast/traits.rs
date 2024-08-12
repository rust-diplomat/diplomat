use serde::Serialize;

use super::docs::Docs;
use super::{Attrs, Ident, LifetimeEnv, Method, Mutability, PathType, TypeName, Param, SelfParam};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
#[non_exhaustive]
pub struct Trait {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fcts: Vec<TraitFct>,
    pub attrs: Attrs,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
#[non_exhaustive]
pub struct TraitFct {
    pub name: Ident,
    pub docs: Docs,
    /// The `self` param of the function, if any.
    pub self_param: Option<SelfParam>,
    /// All non-`self` params taken by the function.
    pub params: Vec<Param>,
    /// The return type of the function, if any.
    pub return_type: Option<TypeName>,
    /// The lifetimes introduced in this function and surrounding impl block.
    pub lifetime_env: LifetimeEnv,
    /// The list of `cfg` attributes (if any).
    ///
    /// These are strings instead of `syn::Attribute` or `proc_macro2::TokenStream`
    /// because those types are not `PartialEq`, `Hash`, `Serialize`, etc.
    pub attrs: Attrs,
}


impl Trait {
}