//! Lifetime information for types.
#![allow(dead_code)]

use super::{IdentBuf, ImplicitLifetimeGenerator};
use crate::ast;
use smallvec::SmallVec;

// TODO(Quinn): This type is going to mainly be recycled from `ast::LifetimeEnv`.
// Not fully sure how that will look like yet, but the ideas of what this will do
// is basically the same.
pub struct LifetimeEnv {
    nodes: SmallVec<[LifetimeNode; 2]>,
}

pub enum LifetimeNode {
    Explicit(ExplicitLifetime),
    Implicit(ImplicitLifetime),
}

/// A named, boundable lifetime.
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
pub struct ImplicitLifetime(u32);

impl ImplicitLifetime {
    /// Returns a new [`ImplicitLifetime`].
    pub(super) fn new(label: u32) -> Self {
        Self(label)
    }
}

/// A lifetime that exists as part of a type signature.
///
/// This type can be mapped to a [`MethodLifetime`] by using the
/// [`TypeLifetime::in_method`] method.
#[derive(Copy, Clone)]
pub struct TypeLifetime {
    /// The index of the lifetime in a type's generic arguments,
    /// or `None` if `'static`.
    index: Option<usize>,
}

/// A set of lifetimes that exist as generic arguments on [`StructPath`]s,
/// [`OutStructPath`]s, and [`OpaquePath`]s.
///
/// By itself, `TypeLifetimes` isn't very useful. However, it can be combined with
/// a [`MethodLifetimes`] using [`TypeLifetimes::in_method`] to get the lifetimes
/// in the scope of a method it appears in.
///
/// [`StructPath`]: super::StructPath
/// [`OutStructPath`]: super::OutStructPath
/// [`OpaquePath`]: super::OpaquePath
#[derive(Clone)]
pub struct TypeLifetimes {
    indices: SmallVec<[TypeLifetime; 2]>,
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
    indices: SmallVec<[Option<usize>; 2]>,
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
            indices: (0..self.nodes.len()).map(Some).collect(),
        }
    }
}

impl TypeLifetime {
    /// Returns a [`TypeLifetime`] from its AST counterparts.
    pub(super) fn from_ast(named: &ast::NamedLifetime, lifetime_env: &ast::LifetimeEnv) -> Self {
        let index = lifetime_env
            .id(named)
            .expect("named lifetime in lifetime env");
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

    /// Returns a [`TypeLifetime`] representing the `'static` lifetime.
    pub(super) fn new_static() -> Self {
        Self { index: None }
    }

    /// Returns a [`TypeLifetime`] from an index.
    ///
    /// This method is used internally by [`TypeLifetime::from_ast`] and
    /// [`TypeLifetime::new_elided`].
    fn new(index: usize) -> Self {
        Self { index: Some(index) }
    }

    /// Returns the index that `self` appears in a [`MethodLifetimes`],
    /// or `None` if it is, or is substituted as, the `'static` lifetime.
    fn as_method_index(self, method_lifetimes: &MethodLifetimes<'_>) -> Option<usize> {
        self.index.and_then(|index| method_lifetimes.indices[index])
    }

    /// Returns a new [`MethodLifetime`] representing `self` in the scope of the
    /// method that it appears in.
    pub fn in_method<'m>(
        self,
        method_lifetimes: &MethodLifetimes<'m>,
    ) -> Option<MethodLifetime<'m>> {
        self.as_method_index(method_lifetimes)
            .map(|index| MethodLifetime::new(index, method_lifetimes.lifetime_env))
    }
}

impl TypeLifetimes {
    pub(super) fn from_fn<F>(lifetimes: &[ast::Lifetime], lower_fn: F) -> Self
    where
        F: FnMut(&ast::Lifetime) -> TypeLifetime,
    {
        let indices = lifetimes.iter().map(lower_fn).collect();

        Self { indices }
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
    pub fn in_method<'m>(&self, method_lifetimes: &MethodLifetimes<'m>) -> MethodLifetimes<'m> {
        MethodLifetimes {
            indices: self
                .indices
                .iter()
                .map(|lifetime| lifetime.as_method_index(method_lifetimes))
                .collect(),
            ..*method_lifetimes
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
    pub(super) fn iter(&self) -> impl Iterator<Item = MethodLifetime<'m>> + '_ {
        self.indices
            .iter()
            .flatten()
            .map(|&index| MethodLifetime::new(index, self.lifetime_env))
    }
}
