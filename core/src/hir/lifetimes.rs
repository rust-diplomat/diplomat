//! Lifetime information for types.

use super::IdentBuf;
use smallvec::SmallVec;

// TODO(Quinn): This type is going to mainly be recycled from `ast::LifetimeEnv`.
// Not fully sure how that will look like yet, but the ideas of what this will do
// is basically the same.
pub struct LifetimeEnv {
    nodes: SmallVec<[LifetimeNode; 2]>,
}

// TODO(Quinn): see above
pub struct LifetimeNode {
    ident: IdentBuf,
    longer: SmallVec<[usize; 2]>,
    shorter: SmallVec<[usize; 2]>,
}

/// A lifetime that exists as part of a type signature.
///
/// This type can be mapped to a [`MethodLifetime`] by using the
/// [`TypeLifetime::in_method`] method.
#[derive(Copy, Clone)]
pub struct TypeLifetime(usize);

/// A set of lifetimes that exist as generic arguments on [`Struct`]s, [`OutStruct`]s,
/// and [`Opaque`]s.
///
/// By itself, `TypeLifetimes` isn't very useful. However, it can be combined with
/// a [`MethodLifetimes`] using [`TypeLifetime::in_method`] to get the lifetimes
/// in the scope of a method it appears in.
///
/// [`Struct`]: super::Struct
/// [`OutStruct`]: super::OutStruct
/// [`Opaque`]: super::Opaque
pub struct TypeLifetimes {
    indices: SmallVec<[TypeLifetime; 2]>,
}

/// A lifetime in the scope of a method.
// The plan is for this type to be able to have functions like "get all
// the shorter/longer lifetimes than me", since it has access to the `LifetimeEnv`.
#[derive(Copy, Clone)]
pub struct MethodLifetime<'lt> {
    lifetime_env: &'lt LifetimeEnv,
    index: usize,
}

/// Map a lifetime in a nested struct to the original lifetime defined
/// in the method that it refers to.
pub struct MethodLifetimes<'lt> {
    lifetime_env: &'lt LifetimeEnv,
    indices: SmallVec<[usize; 2]>,
}

impl LifetimeEnv {
    pub fn method_lifetimes(&self) -> MethodLifetimes {
        MethodLifetimes {
            lifetime_env: self,
            indices: (0..self.nodes.len()).collect(),
        }
    }
}

impl TypeLifetime {
    /// Returns a new [`MethodLifetime`] representing `self` in the scope of the
    /// method that it appears in.
    pub fn in_method<'m>(&self, method_lifetimes: &MethodLifetimes<'m>) -> MethodLifetime<'m> {
        MethodLifetime {
            lifetime_env: method_lifetimes.lifetime_env,
            index: method_lifetimes.indices[self.0],
        }
    }
}

impl TypeLifetimes {
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
                .map(|lifetime| method_lifetimes.indices[lifetime.0])
                .collect(),
            ..*method_lifetimes
        }
    }
}

impl<'m> MethodLifetimes<'m> {
    pub(super) fn iter(&self) -> impl Iterator<Item = MethodLifetime<'m>> + '_ {
        self.indices.iter().map(|&index| MethodLifetime {
            lifetime_env: self.lifetime_env,
            index,
        })
    }
}
