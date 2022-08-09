//! Types that can be exposed in Diplomat APIs.

use super::{
    EnumId, OpaqueId, PrimitiveType, StructId, StructIdKind, TypeContext, TypeLifetime,
    TypeLifetimes,
};

/// Type that may be used as an output.
pub enum TypeKind {
    OutType(OutType),
    Type(Type),
}

/// Type that can only be used as an output.
pub enum OutType {
    Primitive(PrimitiveType),
    Opaque(TypeLifetimes, Optionality, Ownership, OpaqueId),
    Struct(TypeLifetimes, StructIdKind),
    Enum(EnumId),
    Slice(Slice),
}

/// Type that may be used as input or output.
pub enum Type {
    Primitive(PrimitiveType),
    Opaque(TypeLifetimes, Optionality, TypeLifetime, OpaqueId),
    Struct(TypeLifetimes, StructId),
    Enum(EnumId),
    Slice(Slice),
}

#[derive(Copy, Clone)]
pub enum Slice {
    /// A string slice, e.g. `&str`.
    Str(TypeLifetime),

    /// A primitive slice, e.g. `&mut [u8]`.
    Primitive(TypeLifetime, Mutability, PrimitiveType),
}

#[derive(Copy, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

/// Flag type determining whether or not a pointer to an opaque type is nullable.
#[derive(Copy, Clone)]
pub enum Optionality {
    Optional,
    NonOptional,
}

/// Determine whether a pointer to an opaque type is owned or borrowed.
/// 
/// Since owned opaques cannot be used as inputs, this only appears in output types.
#[derive(Copy, Clone)]
pub enum Ownership {
    Owned,
    Borrowed(TypeLifetime),
}

impl Type {
    /// Return the number of fields and leaves that will show up in the [`LifetimeTree`]
    /// returned by [`Param::lifetime_tree`] and [`ParamSelf::lifetime_tree`].
    ///
    /// This method is used to calculate how much space to allocate upfront.
    pub(super) fn field_leaf_lifetime_counts(&self, tcx: &TypeContext) -> (usize, usize) {
        match self {
            Type::Struct(_, id) => tcx[*id].fields.iter().fold((1, 0), |acc, field| {
                let inner = field.ty.field_leaf_lifetime_counts(tcx);
                (acc.0 + inner.0, acc.1 + inner.1)
            }),
            Type::Opaque(..) | Type::Slice(_) => (0, 1),
            Type::Primitive(_) | Type::Enum(_) => (0, 0),
        }
    }
}

impl Slice {
    /// Returns the [`TypeLifetime`] contained in either the `Str` or `Primitive`
    /// variant.
    pub fn lifetime(&self) -> TypeLifetime {
        match self {
            Slice::Str(lifetime) | Slice::Primitive(lifetime, ..) => *lifetime,
        }
    }
}
