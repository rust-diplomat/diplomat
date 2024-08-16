use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::{Attrs, Ident, LifetimeEnv, Method, Mutability, Param, PathType, SelfParam, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct Trait {
    pub name: Ident,
    // pub lifetimes: LifetimeEnv,
    pub fcts: Vec<TraitFct>,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct TraitFct {
    pub name: Ident,
    // corresponds to the types in Function(Vec<Box<TypeName>>, Box<TypeName>)
    // the callback type
    pub input_types: Vec<Box<TypeName>>,
    pub output_type: Box<TypeName>,
}

impl Trait {
    /// Extract an [`Enum`] metadata value from an AST node.
    pub fn new(trt: &syn::ItemTrait, parent_attrs: &Attrs) -> Self {
        Self {
            name: (&trt.ident).into(),
            fcts: vec![],
        }
    }
}
