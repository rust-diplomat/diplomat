//! Lifetime information for types.
#![allow(dead_code)]

use super::{IdentBuf, ImplicitLifetimeGenerator};
use crate::ast;
use smallvec::SmallVec;

// TODO(Quinn): This type is going to mainly be recycled from `ast::LifetimeEnv`.
// Not fully sure how that will look like yet, but the ideas of what this will do
// is basically the same.
#[derive(Debug)]
pub struct LifetimeEnv {
    nodes: SmallVec<[LifetimeNode; 2]>,
}

#[derive(Debug)]
pub enum LifetimeNode {
    Explicit(ExplicitLifetime),
    Implicit(ImplicitLifetime),
}

/// A named, boundable lifetime.
#[derive(Debug)]
pub struct ExplicitLifetime {
    ident: IdentBuf,
    longer: SmallVec<[usize; 2]>,
    shorter: SmallVec<[usize; 2]>,
}

impl ExplicitLifetime {
    /// Returns a new [`ExplicitLifetime`].
    pub(super) fn new(
        ident: IdentBuf,
        longer: SmallVec<[usize; 2]>,
        shorter: SmallVec<[usize; 2]>,
    ) -> Self {
        Self {
            ident,
            longer,
            shorter,
        }
    }
}

/// An anonymous lifetime.
#[derive(Debug)]
pub struct ImplicitLifetime(u32);

impl ImplicitLifetime {
    /// Returns a new [`ImplicitLifetime`].
    pub(super) fn new(label: u32) -> Self {
        Self(label)
    }
}

/// Wrapper type for `TypeLifetime` and `MethodLifetime`, indicating that it may
/// be the `'static` lifetime.
#[derive(Copy, Clone, Debug)]
pub enum MaybeStatic<T> {
    Static,
    NonStatic(T),
}

impl<T> MaybeStatic<T> {
    /// Maps the lifetime, if it's not the `'static` lifetime, to another
    /// non-static lifetime.
    pub(super) fn map_nonstatic<F, R>(self, f: F) -> MaybeStatic<R>
    where
        F: FnOnce(T) -> R,
    {
        match self {
            MaybeStatic::Static => MaybeStatic::Static,
            MaybeStatic::NonStatic(lifetime) => MaybeStatic::NonStatic(f(lifetime)),
        }
    }

    /// Maps the lifetime, if it's not the `'static` lifetime, to a potentially
    /// static lifetime.
    pub(super) fn flat_map_nonstatic<R, F>(self, f: F) -> MaybeStatic<R>
    where
        F: FnOnce(T) -> MaybeStatic<R>,
    {
        match self {
            MaybeStatic::Static => MaybeStatic::Static,
            MaybeStatic::NonStatic(lifetime) => f(lifetime),
        }
    }
}

/// A lifetime that exists as part of a type signature.
///
/// This type can be mapped to a [`MethodLifetime`] by using the
/// [`TypeLifetime::as_method_lifetime`] method.
#[derive(Copy, Clone, Debug)]
pub struct TypeLifetime {
    index: usize,
}

/// A set of lifetimes that exist as generic arguments on [`StructPath`]s,
/// [`OutStructPath`]s, and [`OpaquePath`]s.
///
/// By itself, `TypeLifetimes` isn't very useful. However, it can be combined with
/// a [`MethodLifetimes`] using [`TypeLifetimes::as_method_lifetimes`] to get the lifetimes
/// in the scope of a method it appears in.
///
/// [`StructPath`]: super::StructPath
/// [`OutStructPath`]: super::OutStructPath
/// [`OpaquePath`]: super::OpaquePath
#[derive(Clone, Debug)]
pub struct TypeLifetimes {
    indices: SmallVec<[MaybeStatic<TypeLifetime>; 2]>,
}

/// A lifetime that exists as part of a method signature.
#[derive(Copy, Clone)]
pub struct MethodLifetime<'m> {
    lifetime_env: &'m LifetimeEnv,
    index: usize,
}

/// Map a lifetime in a nested struct to the original lifetime defined
/// in the method that it refers to.
pub struct MethodLifetimes<'m> {
    lifetime_env: &'m LifetimeEnv,
    indices: SmallVec<[MaybeStatic<usize>; 2]>,
}

impl LifetimeEnv {
    /// Returns a new [`LifetimeEnv`].
    pub(super) fn new(nodes: SmallVec<[LifetimeNode; 2]>) -> Self {
        Self { nodes }
    }

    /// Returns a fresh [`MethodLifetimes`] corresponding to `self`.
    pub fn method_lifetimes(&self) -> MethodLifetimes {
        MethodLifetimes {
            lifetime_env: self,
            indices: (0..self.nodes.len()).map(MaybeStatic::NonStatic).collect(),
        }
    }
}

impl TypeLifetime {
    /// Returns a [`TypeLifetime`] from its AST counterparts.
    pub(super) fn from_ast(named: &ast::NamedLifetime, lifetime_env: &ast::LifetimeEnv) -> Self {
        let index = lifetime_env
            .id(named)
            .unwrap_or_else(|| panic!("lifetime `{}` not found in lifetime env", named));
        Self::new(index)
    }

    /// Returns a [`TypeLifetime`] representing a new anonymous lifetime.
    pub(super) fn new_elided(
        elided_node_gen: &mut ImplicitLifetimeGenerator,
        nodes: &mut SmallVec<[LifetimeNode; 2]>,
    ) -> Self {
        let index = nodes.len();
        nodes.push(LifetimeNode::Implicit(elided_node_gen.gen()));
        Self::new(index)
    }

    /// Returns a [`TypeLifetime`] from an index.
    fn new(index: usize) -> Self {
        Self { index }
    }

    /// Returns the index that `self` appears in a [`MethodLifetimes`] wrapped
    /// in `MaybeStatic::NonStatic`, or `MaybeStatic::Static` if it's substituted
    /// as `'static`.
    fn as_method_index(self, method_lifetimes: &MethodLifetimes<'_>) -> MaybeStatic<usize> {
        method_lifetimes.indices[self.index]
    }

    /// Returns a new [`MaybeStatic<MethodLifetime>`] representing `self` in the
    /// scope of the method that it appears in.
    ///
    /// For example, if we have some `Foo<'a>` type with a field `&'a Bar`, then
    /// we can call this on the `'a` on the field. If `Foo` was `Foo<'static>`
    /// in the method, then this will return `MaybeStatic::Static`. But if it
    /// was `Foo<'b>`, then this will return `MaybeStatic::NonStatic` containing
    /// the `MethodLifetime` corresponding to `'b`.
    pub fn as_method_lifetime<'m>(
        self,
        method_lifetimes: &MethodLifetimes<'m>,
    ) -> MaybeStatic<MethodLifetime<'m>> {
        self.as_method_index(method_lifetimes)
            .map_nonstatic(|index| MethodLifetime::new(index, method_lifetimes.lifetime_env))
    }
}

impl TypeLifetimes {
    pub(super) fn from_fn<F>(lifetimes: &[ast::Lifetime], lower_fn: F) -> Self
    where
        F: FnMut(&ast::Lifetime) -> MaybeStatic<TypeLifetime>,
    {
        Self {
            indices: lifetimes.iter().map(lower_fn).collect(),
        }
    }

    /// Returns a new [`MethodLifetimes`] representing the lifetimes in the scope
    /// of the method this type appears in.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # struct Alice<'a>(&'a ());
    /// # struct Bob<'b>(&'b ());
    /// struct Foo<'a, 'b> {
    ///     alice: Alice<'a>,
    ///     bob: Bob<'b>,
    /// }
    ///
    /// fn bar<'x, 'y>(arg: Foo<'x, 'y>) {}
    /// ```
    /// Here, `Foo` will have a [`TypeLifetimes`] containing `['a, 'b]`,
    /// and `bar` will have a [`MethodLifetimes`] containing `{'x: 'x, 'y: 'y}`.
    /// When we enter the scope of `Foo` as a type, we use this method to combine
    /// the two to get a new [`MethodLifetimes`] representing the mapping from
    /// lifetimes in `Foo`'s scope to lifetimes in `bar`s scope: `{'a: 'x, 'b: 'y}`.
    ///
    /// This tells us that `arg.alice` has lifetime `'x` in the method, and
    /// that `arg.bob` has lifetime `'y`.
    pub fn as_method_lifetimes<'m>(
        &self,
        method_lifetimes: &MethodLifetimes<'m>,
    ) -> MethodLifetimes<'m> {
        MethodLifetimes {
            indices: self
                .indices
                .iter()
                .map(|maybe_static_lt| {
                    maybe_static_lt.flat_map_nonstatic(|lt| lt.as_method_index(method_lifetimes))
                })
                .collect(),
            lifetime_env: method_lifetimes.lifetime_env,
        }
    }
}

impl<'m> MethodLifetime<'m> {
    /// Returns a new [`MethodLifetime`].
    fn new(index: usize, lifetime_env: &'m LifetimeEnv) -> Self {
        Self {
            lifetime_env,
            index,
        }
    }
}

impl<'m> MethodLifetimes<'m> {
    /// Returns an iterator over the contained [`MethodLifetime`]s.
    pub(super) fn lifetimes(&self) -> impl Iterator<Item = MaybeStatic<MethodLifetime<'m>>> + '_ {
        self.indices.iter().map(|maybe_static| {
            maybe_static.map_nonstatic(|index| MethodLifetime::new(index, self.lifetime_env))
        })
    }
}
