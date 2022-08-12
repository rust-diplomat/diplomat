//! Methods for types and navigating lifetimes within methods.

use smallvec::SmallVec;

use super::{
    paths, Docs, Ident, IdentBuf, LifetimeEnv, MethodLifetime, MethodLifetimes, OutType, SelfType,
    Slice, Type, TypeContext, TypeLifetime, TypeLifetimes,
};

/// A method exposed to Diplomat.
pub struct Method {
    pub docs: Docs,
    pub name: IdentBuf,
    pub lifetime_env: LifetimeEnv,

    pub param_self: Option<ParamSelf>,
    pub params: Vec<Param>,
    pub output: ReturnFallability,
}

/// Type that the method returns.
pub enum ReturnType {
    Writeable,
    OutType(OutType),
}

/// Whether or not the method returns a value or a result.
pub enum ReturnFallability {
    Infallible(Option<ReturnType>),
    Fallible(Option<ReturnType>, OutType),
}

/// The `self` parameter of a method.
pub struct ParamSelf {
    ty: SelfType,
}

/// A parameter in a method.
pub struct Param {
    name: IdentBuf,
    ty: Type,
}

/// An id for indexing into a [`LifetimeTree`].
#[derive(Copy, Clone)]
pub struct ParentId(usize);

/// A tree of lifetimes mapping onto a specific instantiation of a type tree.
///
/// Each `LifetimeTree` corresponds to the type of an input of a method.
pub struct LifetimeTree<'m> {
    parents: SmallVec<[(Option<ParentId>, &'m Ident); 4]>,
    leaves: SmallVec<[LifetimeTreeLeaf<'m>; 8]>,
}

/// Non-recursive input-output types that contain lifetimes
pub enum LifetimeTreeLeaf<'m> {
    Opaque(ParentId, MethodLifetime<'m>, MethodLifetimes<'m>),
    Slice(ParentId, MethodLifetime<'m>),
}

/// A leaf of a lifetime tree capable of tracking its parents.
pub struct UnpackedField<'m> {
    /// All inner nodes in the tree. When tracing from the root, we jump around
    /// this slice based on indices, but don't necessarily use all of them.
    parents: &'m [(Option<ParentId>, &'m Ident)],

    /// The unpacked field that is a leaf on the tree.
    leaf: &'m LifetimeTreeLeaf<'m>,
}

impl ReturnType {
    /// Returns `true` if it's writeable, otherwise `false`.
    pub fn is_writeable(&self) -> bool {
        matches!(self, ReturnType::Writeable)
    }

    /// Returns a return type, if it's not a writeable.
    pub fn as_type(&self) -> Option<&OutType> {
        match self {
            ReturnType::Writeable => None,
            ReturnType::OutType(ty) => Some(ty),
        }
    }
}

impl ReturnFallability {
    /// Returns `true` if it's writeable, otherwise `false`.
    pub fn is_writeable(&self) -> bool {
        self.return_type()
            .map(ReturnType::is_writeable)
            .unwrap_or(false)
    }

    /// Returns the [`ReturnOk`] value, whether it's the single return type or
    /// the `Ok` variant of a result.
    pub fn return_type(&self) -> Option<&ReturnType> {
        match self {
            ReturnFallability::Infallible(ret) | ReturnFallability::Fallible(ret, _) => {
                ret.as_ref()
            }
        }
    }
}

impl ParamSelf {
    pub(super) fn new(ty: SelfType) -> Self {
        Self { ty }
    }

    /// Returns a [`LifetimeTree`] corresponding to this self parameter.
    pub fn lifetime_tree<'m>(
        &'m self,
        self_name: &'m Ident,
        method_lifetimes: &MethodLifetimes<'m>,
        tcx: &'m TypeContext,
    ) -> LifetimeTree<'m> {
        LifetimeTree::from_param_self(self, self_name, tcx, method_lifetimes)
    }

    /// Return the number of fields and leaves that will show up in the [`LifetimeTree`]
    /// returned by [`ParamSelf::lifetime_tree`].
    ///
    /// This method is used to calculate how much space to allocate upfront.
    fn field_leaf_lifetime_counts(&self, tcx: &TypeContext) -> (usize, usize) {
        match self.ty {
            SelfType::Opaque(_) => (0, 1),
            SelfType::Struct(ref ty) => ty.resolve(tcx).fields.iter().fold((1, 0), |acc, field| {
                let inner = field.ty.field_leaf_lifetime_counts(tcx);
                (acc.0 + inner.0, acc.1 + inner.1)
            }),
            SelfType::Enum(_) => (0, 0),
        }
    }
}

impl Param {
    pub(super) fn new(name: IdentBuf, ty: Type) -> Self {
        Self { name, ty }
    }

    /// Returns a [`LifetimeTree`] corresponding to this parameter.
    pub fn lifetime_tree<'m>(
        &'m self,
        method_lifetimes: &MethodLifetimes<'m>,
        tcx: &'m TypeContext,
    ) -> LifetimeTree<'m> {
        LifetimeTree::from_param(self, tcx, method_lifetimes)
    }
}

impl Method {
    /// Returns `true` if the method takes a writeable as an out parameter,
    /// otherwise `false`.
    pub fn is_writeable(&self) -> bool {
        self.output.is_writeable()
    }

    /// Returns a fresh [`MethodLifetimes`] corresponding to `self`.
    pub fn method_lifetimes(&self) -> MethodLifetimes {
        self.lifetime_env.method_lifetimes()
    }
}

impl ParentId {
    /// Pushes a new parent to the vec, returning the corresponding [`ParentId`].
    fn new<'m>(
        parent: Option<ParentId>,
        name: &'m Ident,
        parents: &mut SmallVec<[(Option<ParentId>, &'m Ident); 4]>,
    ) -> Self {
        let this = ParentId(parents.len());
        parents.push((parent, name));
        this
    }
}

impl<'m> LifetimeTree<'m> {
    /// Returns a new [`LifetimeTree`] corresponding to the type of a [`Param`].
    fn from_param(
        param: &'m Param,
        tcx: &'m TypeContext,
        method_lifetimes: &MethodLifetimes<'m>,
    ) -> Self {
        let (num_fields, num_leaves) = param.ty.field_leaf_lifetime_counts(tcx);
        let mut parents = SmallVec::with_capacity(num_fields + 1);
        let mut leaves = SmallVec::with_capacity(num_leaves);
        let parent = ParentId::new(None, param.name.as_ref(), &mut parents);
        Self::from_type(
            &param.ty,
            tcx,
            parent,
            method_lifetimes,
            &mut parents,
            &mut leaves,
        );
        Self { parents, leaves }
    }

    /// Returns a new [`LifetimeTree`] corresponding to the type of `self`.
    ///
    /// This method takes an extra `self_name` argument, which dictates that the
    /// root of the returned [`LifetimeTree`] should be referred to as.
    fn from_param_self(
        param_self: &'m ParamSelf,
        self_name: &'m Ident,
        tcx: &'m TypeContext,
        method_lifetimes: &MethodLifetimes<'m>,
    ) -> Self {
        let (num_fields, num_leaves) = param_self.field_leaf_lifetime_counts(tcx);
        let mut parents = SmallVec::with_capacity(num_fields + 1);
        let mut leaves = SmallVec::with_capacity(num_leaves);
        let parent = ParentId::new(None, self_name, &mut parents);
        match &param_self.ty {
            SelfType::Opaque(ty) => {
                Self::visit_opaque(
                    &ty.lifetimes,
                    &ty.borrowed().lifetime,
                    parent,
                    method_lifetimes,
                    &mut leaves,
                );
            }
            SelfType::Struct(ty) => {
                Self::visit_struct(ty, tcx, parent, method_lifetimes, &mut parents, &mut leaves);
            }
            SelfType::Enum(_) => {}
        }
        Self { parents, leaves }
    }

    /// Returns a new [`LifetimeTree`] corresponding to a type. This method is to
    /// be used internally by [`LifetimeTree::from_param`] and [`LifetimeTree::from_param_self`].
    fn from_type(
        ty: &'m Type,
        tcx: &'m TypeContext,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes<'m>,
        parents: &mut SmallVec<[(Option<ParentId>, &'m Ident); 4]>,
        leaves: &mut SmallVec<[LifetimeTreeLeaf<'m>; 8]>,
    ) {
        match ty {
            Type::Opaque(path) => {
                Self::visit_opaque(
                    &path.lifetimes,
                    &path.borrowed().lifetime,
                    parent,
                    method_lifetimes,
                    leaves,
                );
            }
            Type::Slice(slice) => {
                Self::visit_slice(slice, parent, method_lifetimes, leaves);
            }
            Type::Struct(path) => {
                Self::visit_struct(path, tcx, parent, method_lifetimes, parents, leaves);
            }
            _ => {}
        }
    }

    /// Add an opaque as a leaf during construction of a [`LifetimeTree`].
    fn visit_opaque(
        lifetimes: &'m TypeLifetimes,
        borrow: &'m TypeLifetime,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes<'m>,
        leaves: &mut SmallVec<[LifetimeTreeLeaf<'m>; 8]>,
    ) {
        let method_borrow_lifetime = borrow.in_method(method_lifetimes);
        let method_type_lifetimes = lifetimes.in_method(method_lifetimes);
        leaves.push(LifetimeTreeLeaf::Opaque(
            parent,
            method_borrow_lifetime,
            method_type_lifetimes,
        ));
    }

    /// Add a slice as a leaf during construction of a [`LifetimeTree`].
    fn visit_slice(
        slice: &Slice,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes<'m>,
        leaves: &mut SmallVec<[LifetimeTreeLeaf<'m>; 8]>,
    ) {
        let method_lifetime = slice.lifetime().in_method(method_lifetimes);
        leaves.push(LifetimeTreeLeaf::Slice(parent, method_lifetime));
    }

    /// Add a struct as a parent an recurse down leaves during construction of a
    /// [`LifetimeTree`].
    fn visit_struct(
        ty: &paths::StructPath,
        tcx: &'m TypeContext,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes<'m>,
        parents: &mut SmallVec<[(Option<ParentId>, &'m Ident); 4]>,
        leaves: &mut SmallVec<[LifetimeTreeLeaf<'m>; 8]>,
    ) {
        let method_type_lifetimes = ty.lifetimes.in_method(method_lifetimes);
        for field in ty.resolve(tcx).fields.iter() {
            Self::from_type(
                &field.ty,
                tcx,
                ParentId::new(Some(parent), field.name.as_ref(), parents),
                &method_type_lifetimes,
                parents,
                leaves,
            );
        }
    }

    /// Returns an iterator over the leaves of a [`LifetimeTree`], which correspond
    /// to the unpacked fields that contain lifetimes within a type.
    pub fn unpacked_fields(&'m self) -> impl Iterator<Item = UnpackedField<'m>> {
        self.leaves.iter().map(|leaf| UnpackedField {
            parents: self.parents.as_slice(),
            leaf,
        })
    }
}

impl<'m> UnpackedField<'m> {
    /// Iterate over the [`MethodLifetime`]s of an unpacked field.
    pub fn lifetimes(&self) -> impl Iterator<Item = MethodLifetime> + '_ {
        let (lifetime, lifetimes) = match self.leaf {
            LifetimeTreeLeaf::Opaque(_, lifetime, lifetimes) => (lifetime, Some(lifetimes.iter())),
            LifetimeTreeLeaf::Slice(_, lifetime) => (lifetime, None),
        };

        Some(*lifetime)
            .into_iter()
            .chain(lifetimes.into_iter().flatten())
    }

    /// Visit fields in order.
    ///
    /// If `self` represents the field `param.first.second`, then calling [`UnpackedField::trace`]
    /// will visit the following in order: `"param"`, `"first"`, `"second"`.
    pub fn trace<F>(&self, visit: &mut F)
    where
        F: FnMut(&'m Ident),
    {
        let (parent, ident) = match self.leaf {
            LifetimeTreeLeaf::Opaque(id, ..) | LifetimeTreeLeaf::Slice(id, _) => self.parents[id.0],
        };

        self._trace(parent, ident, visit);
    }

    /// Recursively visits fields in order from root to leaf by building up the
    /// stack, and then visiting fields as it unwinds.
    fn _trace<F>(&self, parent: Option<ParentId>, ident: &'m Ident, visit: &mut F)
    where
        F: FnMut(&'m Ident),
    {
        if let Some(id) = parent {
            let (parent, ident) = self.parents[id.0];
            self._trace(parent, ident, visit);
        }

        visit(ident);
    }
}
