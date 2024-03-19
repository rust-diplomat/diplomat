//! Types that can be exposed in Diplomat APIs.

use super::lifetimes::{Lifetime, MaybeStatic};
use super::{
    EnumPath, Everywhere, NonOptional, OpaqueOwner, OpaquePath, Optional, OutputOnly,
    PrimitiveType, StructPath, StructPathLike, TyPosition, TypeContext, TypeId,
};
use crate::ast;
pub use ast::Mutability;
pub use ast::StringEncoding;
use either::Either;

/// Type that can only be used as an output.
pub type OutType = Type<OutputOnly>;

/// Type that may be used as input or output.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Type<P: TyPosition = Everywhere> {
    Primitive(PrimitiveType),
    Opaque(OpaquePath<Optional, P::OpaqueOwnership>),
    Struct(P::StructPath),
    Enum(EnumPath),
    Slice(Slice),
}

/// Type that can appear in the `self` position.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SelfType {
    Opaque(OpaquePath<NonOptional, Borrow>),
    Struct(StructPath),
    Enum(EnumPath),
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Slice {
    /// A string slice, e.g. `&DiplomatStr` or `Box<DiplomatStr>`.
    ///
    /// Owned slices are useful for garbage-collected languages that have to
    /// reallocate into non-gc memory anyway. For example for Dart it's more
    /// efficient to accept `Box<str>` than to accept `&str` and then
    /// allocate in Rust, as Dart will have to create the `Box<str`> to
    /// pass `&str` anyway.
    Str(Option<MaybeStatic<Lifetime>>, StringEncoding),

    /// A primitive slice, e.g. `&mut [u8]` or `Box<[usize]>`.
    ///
    /// Owned slices are useful for garbage-collected languages that have to
    /// reallocate into non-gc memory anyway. For example for Dart it's more
    /// efficient to accept `Box<[bool]>` than to accept `&[bool]` and then
    /// allocate in Rust, as Dart will have to create the `Box<[bool]`> to
    /// pass `&[bool]` anyway.
    Primitive(Option<Borrow>, PrimitiveType),

    /// A `&[&DiplomatStr]]`. This type of slice always needs to be
    /// allocated before passing it into Rust, as it has to conform to the
    /// Rust ABI. In other languages this is the idiomatic list of string
    /// views, i.e. `std::span<std::string_view>` or `core.List<core.String>`.
    Strs(StringEncoding),
}

// For now, the lifetime in not optional. This is because when you have references
// as fields of structs, the lifetime must always be present, and we want to uphold
// this invariant at the type level within the HIR.
//
// The only time when a lifetime is optional in Rust code is in function signatures,
// where implicit lifetimes are allowed. Getting this to all fit together will
// involve getting the implicit lifetime thing to be understood by Diplomat, but
// should be doable.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub struct Borrow {
    pub lifetime: MaybeStatic<Lifetime>,
    pub mutability: Mutability,
}

impl Type {
    /// Return the number of fields and leaves that will show up in the [`LifetimeTree`]
    /// returned by [`Param::lifetime_tree`] and [`ParamSelf::lifetime_tree`].
    ///
    /// This method is used to calculate how much space to allocate upfront.
    pub(super) fn field_leaf_lifetime_counts(&self, tcx: &TypeContext) -> (usize, usize) {
        match self {
            Type::Struct(ty) => ty.resolve(tcx).fields.iter().fold((1, 0), |acc, field| {
                let inner = field.ty.field_leaf_lifetime_counts(tcx);
                (acc.0 + inner.0, acc.1 + inner.1)
            }),
            Type::Opaque(_) | Type::Slice(_) => (1, 1),
            Type::Primitive(_) | Type::Enum(_) => (0, 0),
        }
    }
}

impl<P: TyPosition> Type<P> {
    /// Get all lifetimes "contained" in this type
    pub fn lifetimes(&self) -> impl Iterator<Item = MaybeStatic<Lifetime>> + '_ {
        match self {
            Type::Opaque(opaque) => Either::Right(
                opaque
                    .lifetimes
                    .as_slice()
                    .iter()
                    .copied()
                    .chain(opaque.owner.lifetime()),
            ),
            Type::Struct(struct_) => Either::Left(struct_.lifetimes().as_slice().iter().copied()),
            Type::Slice(slice) => Either::Left(
                slice
                    .lifetime()
                    .map(|lt| std::slice::from_ref(lt).iter().copied())
                    .unwrap_or([].iter().copied()),
            ),
            _ => Either::Left([].iter().copied()),
        }
    }

    // For custom types, get the type id
    pub fn id(&self) -> Option<TypeId> {
        Some(match self {
            Self::Opaque(p) => TypeId::Opaque(p.tcx_id),
            Self::Enum(p) => TypeId::Enum(p.tcx_id),
            Self::Struct(p) => p.id(),
            _ => return None,
        })
    }
}

impl SelfType {
    /// Returns whether the self parameter is borrowed immutably.
    ///
    /// Curently this can only happen with opaque types.
    pub fn is_immutably_borrowed(&self) -> bool {
        match self {
            SelfType::Opaque(opaque_path) => opaque_path.owner.mutability == Mutability::Immutable,
            _ => false,
        }
    }
}

impl Slice {
    /// Returns the [`Lifetime`] contained in either the `Str` or `Primitive`
    /// variant.
    pub fn lifetime(&self) -> Option<&MaybeStatic<Lifetime>> {
        match self {
            Slice::Str(lifetime, ..) => lifetime.as_ref(),
            Slice::Primitive(Some(reference), ..) => Some(&reference.lifetime),
            Slice::Primitive(..) => None,
            Slice::Strs(..) => Some({
                const X: MaybeStatic<Lifetime> = MaybeStatic::NonStatic(Lifetime::new(usize::MAX));
                &X
            }),
        }
    }
}

impl Borrow {
    pub(super) fn new(lifetime: MaybeStatic<Lifetime>, mutability: Mutability) -> Self {
        Self {
            lifetime,
            mutability,
        }
    }
}

impl From<SelfType> for Type {
    fn from(s: SelfType) -> Type {
        match s {
            SelfType::Opaque(o) => Type::Opaque(o.wrap_optional()),
            SelfType::Struct(s) => Type::Struct(s),
            SelfType::Enum(e) => Type::Enum(e),
        }
    }
}
