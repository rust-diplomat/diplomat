//! Lifetime information for types.
#![allow(dead_code)]

use super::IdentBuf;
use crate::ast;
use core::fmt::{self, Debug};
use core::hash::Hash;
use core::marker::PhantomData;
use smallvec::{smallvec, SmallVec};
use std::borrow::{Borrow, Cow};

/// Convenience const representing the number of lifetimes a [`LifetimeEnv`]
/// can hold inline before needing to dynamically allocate.
pub(crate) const INLINE_NUM_LIFETIMES: usize = 4;

/// The lifetimes and bounds found on a method or type definition (determined by
/// Kind parameter, which will be one of [`LifetimeKind`])
// TODO(Quinn): This type is going to mainly be recycled from `ast::LifetimeEnv`.
// Not fully sure how that will look like yet, but the ideas of what this will do
// is basically the same.
#[derive(Debug)]
pub struct LifetimeEnv<Kind> {
    /// List of named lifetimes in scope of the method, and their bounds
    nodes: SmallVec<[BoundedLifetime<Kind>; INLINE_NUM_LIFETIMES]>,

    /// Only relevant for method LifetimeEnvs (otherwise this is nodes.len())
    ///
    /// The number of named _and_ anonymous lifetimes in the method.
    /// We store the sum since it represents the upper bound on what indices
    /// are in range of the graph. If we make a [`MethodLifetimes`] with
    /// `num_lifetimes` entries, then `TypeLifetime`s that convert into
    /// `MethodLifetime`s will fall into this range, and we'll know that it's
    /// a named lifetime if it's < `nodes.len()`, or that it's an anonymous
    /// lifetime if it's < `num_lifetimes`. Otherwise, we'd have to make a
    /// distinction in `TypeLifetime` about which kind it refers to.
    num_lifetimes: usize,
}

impl<Kind: LifetimeKind> LifetimeEnv<Kind> {
    /// Format a lifetime indexing this env for use in code
    pub fn fmt_lifetime(&self, lt: impl Borrow<Lifetime<Kind>>) -> Cow<str> {
        // we use Borrow here so that this can be used in templates where there's autoborrowing
        let lt = *lt.borrow();
        if let Some(lt) = self.nodes.get(lt.0) {
            Cow::from(lt.ident.as_str())
        } else if lt.0 < self.num_lifetimes {
            format!("anon_{}", lt.0 - self.nodes.len()).into()
        } else {
            panic!("Found out of range lifetime: Got {lt:?} for env with {} nodes and {} total lifetimes", self.nodes.len(), self.num_lifetimes);
        }
    }

    /// Get an iterator of all lifetimes that this must live as long as (including itself)
    /// with the first lifetime always being returned first
    ///
    /// The kind *can* be different: e.g. the Type paths in a method signature will
    /// still have Lifetime<Type> even though they're in a method context.
    ///
    /// In the medium term we may want to get rid of Type vs Method lifetimes, OR
    /// make them a parameter on Type.
    pub fn all_shorter_lifetimes<K2: LifetimeKind>(
        &self,
        lt: impl Borrow<Lifetime<K2>>,
    ) -> impl Iterator<Item = Lifetime<Kind>> + '_ {
        // we use Borrow here so that this can be used in templates where there's autoborrowing
        let lt = *lt.borrow();
        // longer = true, since we are looking for lifetimes this is longer than
        LifetimeTransitivityIterator::new(self, lt.0, false)
    }

    /// Same as all_shorter_lifetimes but the other way
    pub fn all_longer_lifetimes<K2: LifetimeKind>(
        &self,
        lt: impl Borrow<Lifetime<K2>>,
    ) -> impl Iterator<Item = Lifetime<Kind>> + '_ {
        // we use Borrow here so that this can be used in templates where there's autoborrowing
        let lt = *lt.borrow();
        LifetimeTransitivityIterator::new(self, lt.0, true)
    }

    // List all named and unnamed lifetimes
    pub fn num_lifetimes(&self) -> usize {
        self.num_lifetimes
    }

    pub fn all_lifetimes(&self) -> impl ExactSizeIterator<Item = Lifetime<Kind>> {
        (0..self.num_lifetimes()).map(|i| Lifetime::new(i))
    }

    /// Returns a new [`LifetimeEnv`].
    pub(super) fn new(
        nodes: SmallVec<[BoundedLifetime<Kind>; INLINE_NUM_LIFETIMES]>,
        num_lifetimes: usize,
    ) -> Self {
        Self {
            nodes,
            num_lifetimes,
        }
    }

    /// Returns a fresh [`MethodLifetimes`] corresponding to `self`.
    pub fn lifetimes(&self) -> Lifetimes<Kind> {
        let indices = (0..self.num_lifetimes)
            .map(|index| MaybeStatic::NonStatic(Lifetime::new(index)))
            .collect();

        Lifetimes { indices }
    }

    /// Returns a new [`SubtypeLifetimeVisitor`], which can visit all reachable
    /// lifetimes
    pub fn subtype_lifetimes_visitor<F>(&self, visit_fn: F) -> SubtypeLifetimeVisitor<'_, Kind, F>
    where
        F: FnMut(Lifetime<Kind>),
    {
        SubtypeLifetimeVisitor::new(self, visit_fn)
    }
}

/// A lifetime in a [`LifetimeEnv`], which keeps track of which lifetimes it's
/// longer and shorter than.
///
/// Invariant: for a BoundedLifetime found inside a LifetimeEnv, all short/long connections
/// should be bidirectional.
#[derive(Debug)]
pub(super) struct BoundedLifetime<Kind> {
    pub(super) ident: IdentBuf,
    /// Lifetimes longer than this (not transitive)
    ///
    /// These are the inverse graph edges compared to `shorter`
    pub(super) longer: SmallVec<[Lifetime<Kind>; 2]>,
    /// Lifetimes this is shorter than (not transitive)
    ///
    /// These match `'a: 'b + 'c` bounds
    pub(super) shorter: SmallVec<[Lifetime<Kind>; 2]>,
}

impl<Kind> BoundedLifetime<Kind> {
    /// Returns a new [`BoundedLifetime`].
    pub(super) fn new(
        ident: IdentBuf,
        longer: SmallVec<[Lifetime<Kind>; 2]>,
        shorter: SmallVec<[Lifetime<Kind>; 2]>,
    ) -> Self {
        Self {
            ident,
            longer,
            shorter,
        }
    }
}

/// Visit subtype lifetimes recursively, keeping track of which have already
/// been visited.
pub struct SubtypeLifetimeVisitor<'lt, Kind, F> {
    lifetime_env: &'lt LifetimeEnv<Kind>,
    visited: SmallVec<[bool; INLINE_NUM_LIFETIMES]>,
    visit_fn: F,
}

impl<'lt, Kind: LifetimeKind, F> SubtypeLifetimeVisitor<'lt, Kind, F>
where
    F: FnMut(Lifetime<Kind>),
    Kind: LifetimeKind,
{
    fn new(lifetime_env: &'lt LifetimeEnv<Kind>, visit_fn: F) -> Self {
        Self {
            lifetime_env,
            visited: smallvec![false; lifetime_env.nodes.len()],
            visit_fn,
        }
    }

    /// Visit more sublifetimes. This method tracks which lifetimes have already
    /// been visited, and uses this to not visit the same lifetime twice.
    pub fn visit_subtypes(&mut self, method_lifetime: Lifetime<Kind>) {
        if let Some(visited @ false) = self.visited.get_mut(method_lifetime.0) {
            *visited = true;

            (self.visit_fn)(method_lifetime);

            for longer in self.lifetime_env.nodes[method_lifetime.0].longer.iter() {
                self.visit_subtypes(*longer)
            }
        } else {
            debug_assert!(
                method_lifetime.0 > self.lifetime_env.num_lifetimes,
                "method lifetime has an internal index that's not in range of the lifetime env"
            );
        }
    }
}

/// Wrapper type for `TypeLifetime` and `MethodLifetime`, indicating that it may
/// be the `'static` lifetime.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_enums)] // this will only ever have two variants
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

/// The [`LifetimeKind`] of [`TypeLifetimes`]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // marker type
pub struct Type;
/// The [`LifetimeKind`] of [`MethodLifetimes`]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // marker type
pub struct Method;

/// Abstraction over where lifetimes can occur
pub trait LifetimeKind: Copy + Clone + Debug + Hash + PartialEq + Eq + PartialOrd + Ord {}

impl LifetimeKind for Type {}
impl LifetimeKind for Method {}

/// A lifetime that exists as part of a type or method signature (determined by
/// Kind parameter, which will be one of [`LifetimeKind`]).
///
/// This index only makes sense in the context of a surrounding type or method; since
/// this is essentially an index into that type/method's lifetime list.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lifetime<Kind>(usize, PhantomData<Kind>);

impl<Kind> Debug for Lifetime<Kind> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}Lifetime({})", std::any::type_name::<Kind>(), self.0)
    }
}

/// A set of lifetimes found on a type name or method signature (determined by
/// Kind parameter, which will be one of [`LifetimeKind`])
#[derive(Clone, Debug)]
pub struct Lifetimes<Kind> {
    indices: SmallVec<[MaybeStatic<Lifetime<Kind>>; 2]>,
}

/// A lifetime that exists as part of a type signature.
///
/// This type can be mapped to a [`MethodLifetime`] by using the
/// [`TypeLifetime::as_method_lifetime`] method.
pub type TypeLifetime = Lifetime<Type>;

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
pub type TypeLifetimes = Lifetimes<Type>;

/// A lifetime that exists as part of a method signature, e.g. `'a` or an
/// anonymous lifetime.
///
/// This type is intended to be used as a key into a map to keep track of which
/// borrowed fields depend on which method lifetimes.
pub type MethodLifetime = Lifetime<Method>;

/// Map a lifetime in a nested struct to the original lifetime defined
/// in the method that it refers to.
pub type MethodLifetimes = Lifetimes<Method>;

impl<Kind: LifetimeKind> Lifetime<Kind> {
    pub(super) fn new(index: usize) -> Self {
        Self(index, PhantomData)
    }

    /// Cast between lifetime kinds. See all_longer_lifetimes() as to why this can be necessary.
    ///
    /// Hopefully can be removed in the long run.
    pub fn cast<K2: LifetimeKind>(self) -> Lifetime<K2> {
        Lifetime::new(self.0)
    }
}

impl<Kind: LifetimeKind> Lifetimes<Kind> {
    /// Returns an iterator over the contained [`Lifetime`]s.
    pub fn lifetimes(&self) -> impl ExactSizeIterator<Item = MaybeStatic<Lifetime<Kind>>> + '_ {
        self.indices.iter().copied()
    }

    pub(super) fn as_slice(&self) -> &[MaybeStatic<Lifetime<Kind>>] {
        self.indices.as_slice()
    }
}

impl TypeLifetime {
    /// Returns a [`TypeLifetime`] from its AST counterparts.
    pub(super) fn from_ast(named: &ast::NamedLifetime, lifetime_env: &ast::LifetimeEnv) -> Self {
        let index = lifetime_env
            .id(named)
            .unwrap_or_else(|| panic!("lifetime `{named}` not found in lifetime env"));
        Self::new(index)
    }

    /// Returns a new [`MaybeStatic<MethodLifetime>`] representing `self` in the
    /// scope of the method that it appears in.
    ///
    /// For example, if we have some `Foo<'a>` type with a field `&'a Bar`, then
    /// we can call this on the `'a` on the field. If `Foo` was `Foo<'static>`
    /// in the method, then this will return `MaybeStatic::Static`. But if it
    /// was `Foo<'b>`, then this will return `MaybeStatic::NonStatic` containing
    /// the `MethodLifetime` corresponding to `'b`.
    pub fn as_method_lifetime(
        self,
        method_lifetimes: &MethodLifetimes,
    ) -> MaybeStatic<MethodLifetime> {
        method_lifetimes.indices[self.0]
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
    pub fn as_method_lifetimes(&self, method_lifetimes: &MethodLifetimes) -> MethodLifetimes {
        let indices = self
            .indices
            .iter()
            .map(|maybe_static_lt| {
                maybe_static_lt.flat_map_nonstatic(|lt| lt.as_method_lifetime(method_lifetimes))
            })
            .collect();

        MethodLifetimes { indices }
    }
}

struct LifetimeTransitivityIterator<'env, Kind> {
    env: &'env LifetimeEnv<Kind>,
    visited: Vec<bool>,
    queue: Vec<usize>,
    longer: bool,
}

impl<'env, Kind: LifetimeKind> LifetimeTransitivityIterator<'env, Kind> {
    // Longer is whether we are looking for lifetimes longer or shorter than this
    fn new(env: &'env LifetimeEnv<Kind>, starting: usize, longer: bool) -> Self {
        Self {
            env,
            visited: vec![false; env.num_lifetimes()],
            queue: vec![starting],
            longer,
        }
    }
}

impl<'env, Kind: LifetimeKind> Iterator for LifetimeTransitivityIterator<'env, Kind> {
    type Item = Lifetime<Kind>;

    fn next(&mut self) -> Option<Lifetime<Kind>> {
        while let Some(next) = self.queue.pop() {
            if self.visited[next] {
                continue;
            }
            self.visited[next] = true;

            if let Some(named) = self.env.nodes.get(next) {
                let edge_dir = if self.longer {
                    &named.longer
                } else {
                    &named.shorter
                };
                self.queue.extend(edge_dir.iter().map(|i| i.0));
            }

            return Some(Lifetime::new(next));
        }
        None
    }
}
