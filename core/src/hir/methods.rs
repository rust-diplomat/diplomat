//! Methods for types and navigating lifetimes within methods.

use std::fmt::{self, Write};

use smallvec::SmallVec;

use super::{
    paths, Attrs, Docs, Ident, IdentBuf, LifetimeEnv, MaybeStatic, MethodLifetime, MethodLifetimes,
    OutType, SelfType, Slice, Type, TypeContext, TypeLifetime, TypeLifetimes,
};

/// A method exposed to Diplomat.
#[derive(Debug)]
#[non_exhaustive]
pub struct Method {
    pub docs: Docs,
    pub name: IdentBuf,
    pub lifetime_env: LifetimeEnv,

    pub param_self: Option<ParamSelf>,
    pub params: Vec<Param>,
    pub output: ReturnType,
    pub attrs: Attrs,
}

/// Type that the method returns.
#[derive(Debug)]
#[non_exhaustive]
pub enum SuccessType {
    Writeable,
    OutType(OutType),
}

/// Whether or not the method returns a value or a result.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // this only exists for fallible/infallible, breaking changes for more complex returns are ok
pub enum ReturnType {
    Infallible(Option<SuccessType>),
    Fallible(Option<SuccessType>, Option<OutType>),
}

/// The `self` parameter of a method.
#[derive(Debug)]
#[non_exhaustive]
pub struct ParamSelf {
    pub ty: SelfType,
}

/// A parameter in a method.
#[derive(Debug)]
#[non_exhaustive]
pub struct Param {
    pub name: IdentBuf,
    pub ty: Type,
}

/// An id for indexing into a [`BorrowingFieldsVisitor`].
#[derive(Copy, Clone, Debug)]
struct ParentId(usize);

/// Convenience const representing the number of nested structs a [`BorrowingFieldVisitor`]
/// can hold inline before needing to dynamically allocate.
const INLINE_NUM_PARENTS: usize = 4;

/// Convenience const representing the number of borrowed fields a [`BorrowingFieldVisitor`]
/// can hold inline before needing to dynamically allocate.
const INLINE_NUM_LEAVES: usize = 8;

/// A tree of lifetimes mapping onto a specific instantiation of a type tree.
///
/// Each `BorrowingFieldsVisitor` corresponds to the type of an input of a method.
pub struct BorrowingFieldVisitor<'m> {
    parents: SmallVec<[(Option<ParentId>, &'m Ident); INLINE_NUM_PARENTS]>,
    leaves: SmallVec<[BorrowingFieldVisitorLeaf; INLINE_NUM_LEAVES]>,
}

/// Non-recursive input-output types that contain lifetimes
enum BorrowingFieldVisitorLeaf {
    Opaque(ParentId, MaybeStatic<MethodLifetime>, MethodLifetimes),
    Slice(ParentId, MaybeStatic<MethodLifetime>),
}

/// A leaf of a lifetime tree capable of tracking its parents.
#[derive(Copy, Clone)]
pub struct BorrowingField<'m> {
    /// All inner nodes in the tree. When tracing from the root, we jump around
    /// this slice based on indices, but don't necessarily use all of them.
    parents: &'m [(Option<ParentId>, &'m Ident)],

    /// The unpacked field that is a leaf on the tree.
    leaf: &'m BorrowingFieldVisitorLeaf,
}

impl SuccessType {
    /// Returns `true` if it's writeable, otherwise `false`.
    pub fn is_writeable(&self) -> bool {
        matches!(self, SuccessType::Writeable)
    }

    /// Returns a return type, if it's not a writeable.
    pub fn as_type(&self) -> Option<&OutType> {
        match self {
            SuccessType::Writeable => None,
            SuccessType::OutType(ty) => Some(ty),
        }
    }
}

impl ReturnType {
    /// Returns `true` if it's writeable, otherwise `false`.
    pub fn is_writeable(&self) -> bool {
        self.return_type()
            .map(SuccessType::is_writeable)
            .unwrap_or(false)
    }

    /// Returns the [`ReturnOk`] value, whether it's the single return type or
    /// the `Ok` variant of a result.
    pub fn return_type(&self) -> Option<&SuccessType> {
        match self {
            ReturnType::Infallible(ret) | ReturnType::Fallible(ret, _) => ret.as_ref(),
        }
    }

    /// Returns `true` if the FFI function returns a value (such that it may be assigned to a variable).
    pub fn returns_value(&self) -> bool {
        match self {
            ReturnType::Fallible(_, _) => true,
            ReturnType::Infallible(Some(SuccessType::OutType(_))) => true,
            ReturnType::Infallible(Some(SuccessType::Writeable)) => false,
            ReturnType::Infallible(None) => false,
        }
    }
}

impl ParamSelf {
    pub(super) fn new(ty: SelfType) -> Self {
        Self { ty }
    }

    /// Return the number of fields and leaves that will show up in the [`BorrowingFieldVisitor`].
    ///
    /// This method is used to calculate how much space to allocate upfront.
    fn field_leaf_lifetime_counts(&self, tcx: &TypeContext) -> (usize, usize) {
        match self.ty {
            SelfType::Opaque(_) => (1, 1),
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

    /// Returns a new [`BorrowingFieldVisitor`], which allocates memory to
    /// efficiently represent all fields (and their paths!) of the inputs that
    /// have a lifetime.
    /// ```ignore
    /// # use std::collections::BTreeMap;
    /// let visitor = method.borrowing_field_visitor(&tcx, "this".ck().unwrap());
    /// let mut map = BTreeMap::new();
    /// visitor.visit_borrowing_fields(|lifetime, field| {
    ///     map.entry(lifetime).or_default().push(field);
    /// })
    /// ```
    pub fn borrowing_field_visitor<'m>(
        &'m self,
        tcx: &'m TypeContext,
        self_name: &'m Ident,
    ) -> BorrowingFieldVisitor<'m> {
        BorrowingFieldVisitor::new(self, tcx, self_name)
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

impl<'m> BorrowingFieldVisitor<'m> {
    /// Visits every borrowing field and method lifetime that it uses.
    ///
    /// The idea is that you could use this to construct a mapping from
    /// `MethodLifetime`s to `BorrowingField`s. We choose to use a visitor
    /// pattern to avoid having to
    ///
    /// This would be convenient in the JavaScript backend where if you're
    /// returning an `NonOpaque<'a, 'b>` from Rust, you need to pass a
    /// `[[all input borrowing fields with 'a], [all input borrowing fields with 'b]]`
    /// array into `NonOpaque`'s constructor.
    ///
    /// Alternatively, you could use such a map in the C++ backend by recursing
    /// down the return type and keeping track of which fields you've recursed
    /// into so far, and when you hit some lifetime 'a, generate docs saying
    /// "path.to.current.field must be outlived by {borrowing fields of input that
    /// contain 'a}".
    pub fn visit_borrowing_fields<'a, F>(&'a self, mut visit: F)
    where
        F: FnMut(MaybeStatic<MethodLifetime>, BorrowingField<'a>),
    {
        for leaf in self.leaves.iter() {
            let borrowing_field = BorrowingField {
                parents: self.parents.as_slice(),
                leaf,
            };

            match leaf {
                BorrowingFieldVisitorLeaf::Opaque(_, lt, method_lifetimes) => {
                    visit(*lt, borrowing_field);
                    for lt in method_lifetimes.lifetimes() {
                        visit(lt, borrowing_field);
                    }
                }
                BorrowingFieldVisitorLeaf::Slice(_, lt) => {
                    visit(*lt, borrowing_field);
                }
            }
        }
    }

    /// Returns a new `BorrowingFieldsVisitor` containing all the lifetime trees of the arguments
    /// in only two allocations.
    fn new(method: &'m Method, tcx: &'m TypeContext, self_name: &'m Ident) -> Self {
        let (parents, leaves) = method
            .param_self
            .as_ref()
            .map(|param_self| param_self.field_leaf_lifetime_counts(tcx))
            .into_iter()
            .chain(
                method
                    .params
                    .iter()
                    .map(|param| param.ty.field_leaf_lifetime_counts(tcx)),
            )
            .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
            .map(|(num_fields, num_leaves)| {
                let num_params = method.params.len() + usize::from(method.param_self.is_some());
                let mut parents = SmallVec::with_capacity(num_fields + num_params);
                let mut leaves = SmallVec::with_capacity(num_leaves);
                let method_lifetimes = method.method_lifetimes();

                if let Some(param_self) = method.param_self.as_ref() {
                    let parent = ParentId::new(None, self_name, &mut parents);
                    match &param_self.ty {
                        SelfType::Opaque(ty) => {
                            Self::visit_opaque(
                                &ty.lifetimes,
                                &ty.borrowed().lifetime,
                                parent,
                                &method_lifetimes,
                                &mut leaves,
                            );
                        }
                        SelfType::Struct(ty) => {
                            Self::visit_struct(
                                ty,
                                tcx,
                                parent,
                                &method_lifetimes,
                                &mut parents,
                                &mut leaves,
                            );
                        }
                        SelfType::Enum(_) => {}
                    }
                }

                for param in method.params.iter() {
                    let parent = ParentId::new(None, param.name.as_ref(), &mut parents);
                    Self::from_type(
                        &param.ty,
                        tcx,
                        parent,
                        &method_lifetimes,
                        &mut parents,
                        &mut leaves,
                    );
                }

                // sanity check that the preallocations were correct
                debug_assert_eq!(
                    parents.capacity(),
                    std::cmp::max(INLINE_NUM_PARENTS, num_fields + num_params)
                );
                debug_assert_eq!(
                    leaves.capacity(),
                    std::cmp::max(INLINE_NUM_LEAVES, num_leaves)
                );
                (parents, leaves)
            })
            .unwrap_or_default();

        Self { parents, leaves }
    }

    /// Returns a new [`BorrowingFieldsVisitor`] corresponding to a type.
    fn from_type(
        ty: &'m Type,
        tcx: &'m TypeContext,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes,
        parents: &mut SmallVec<[(Option<ParentId>, &'m Ident); 4]>,
        leaves: &mut SmallVec<[BorrowingFieldVisitorLeaf; 8]>,
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

    /// Add an opaque as a leaf during construction of a [`BorrowingFieldsVisitor`].
    fn visit_opaque(
        lifetimes: &'m TypeLifetimes,
        borrow: &'m MaybeStatic<TypeLifetime>,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes,
        leaves: &mut SmallVec<[BorrowingFieldVisitorLeaf; 8]>,
    ) {
        let method_borrow_lifetime =
            borrow.flat_map_nonstatic(|lt| lt.as_method_lifetime(method_lifetimes));
        let method_type_lifetimes = lifetimes.as_method_lifetimes(method_lifetimes);
        leaves.push(BorrowingFieldVisitorLeaf::Opaque(
            parent,
            method_borrow_lifetime,
            method_type_lifetimes,
        ));
    }

    /// Add a slice as a leaf during construction of a [`BorrowingFieldsVisitor`].
    fn visit_slice(
        slice: &Slice,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes,
        leaves: &mut SmallVec<[BorrowingFieldVisitorLeaf; 8]>,
    ) {
        let method_lifetime = slice
            .lifetime()
            .flat_map_nonstatic(|lt| lt.as_method_lifetime(method_lifetimes));
        leaves.push(BorrowingFieldVisitorLeaf::Slice(parent, method_lifetime));
    }

    /// Add a struct as a parent and recurse down leaves during construction of a
    /// [`BorrowingFieldsVisitor`].
    fn visit_struct(
        ty: &paths::StructPath,
        tcx: &'m TypeContext,
        parent: ParentId,
        method_lifetimes: &MethodLifetimes,
        parents: &mut SmallVec<[(Option<ParentId>, &'m Ident); 4]>,
        leaves: &mut SmallVec<[BorrowingFieldVisitorLeaf; 8]>,
    ) {
        let method_type_lifetimes = ty.lifetimes.as_method_lifetimes(method_lifetimes);
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
}

impl<'m> BorrowingField<'m> {
    /// Visit fields in order.
    ///
    /// If `self` represents the field `param.first.second`, then calling [`BorrowingField::trace`]
    /// will visit the following in order: `"param"`, `"first"`, `"second"`.
    pub fn backtrace<F>(&self, mut visit: F)
    where
        F: FnMut(usize, &'m Ident),
    {
        let (parent, ident) = match self.leaf {
            BorrowingFieldVisitorLeaf::Opaque(id, ..) | BorrowingFieldVisitorLeaf::Slice(id, _) => {
                self.parents[id.0]
            }
        };

        self.backtrace_rec(parent, ident, &mut visit);
    }

    /// Recursively visits fields in order from root to leaf by building up the
    /// stack, and then visiting fields as it unwinds.
    fn backtrace_rec<F>(&self, parent: Option<ParentId>, ident: &'m Ident, visit: &mut F) -> usize
    where
        F: FnMut(usize, &'m Ident),
    {
        let from_end = if let Some(id) = parent {
            let (parent, ident) = self.parents[id.0];
            self.backtrace_rec(parent, ident, visit)
        } else {
            0
        };

        visit(from_end, ident);

        from_end + 1
    }

    /// Fallibly visits fields in order.
    ///
    /// This method is similar to [`BorrowinfField::backtrace`], but short-circuits
    /// when an `Err` is returned.
    pub fn try_backtrace<F, E>(&self, mut visit: F) -> Result<(), E>
    where
        F: FnMut(usize, &'m Ident) -> Result<(), E>,
    {
        let (parent, ident) = match self.leaf {
            BorrowingFieldVisitorLeaf::Opaque(id, ..) | BorrowingFieldVisitorLeaf::Slice(id, _) => {
                self.parents[id.0]
            }
        };

        self.try_backtrace_rec(parent, ident, &mut visit)?;

        Ok(())
    }

    /// Recursively visits fields in order from root to leaf by building up the
    /// stack, and then visiting fields as it unwinds.
    fn try_backtrace_rec<F, E>(
        &self,
        parent: Option<ParentId>,
        ident: &'m Ident,
        visit: &mut F,
    ) -> Result<usize, E>
    where
        F: FnMut(usize, &'m Ident) -> Result<(), E>,
    {
        let from_end = if let Some(id) = parent {
            let (parent, ident) = self.parents[id.0];
            self.try_backtrace_rec(parent, ident, visit)?
        } else {
            0
        };

        visit(from_end, ident)?;

        Ok(from_end + 1)
    }
}

impl<'m> fmt::Display for BorrowingField<'m> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.try_backtrace(|i, ident| {
            if i != 0 {
                f.write_char('.')?;
            }
            f.write_str(ident.as_str())
        })
    }
}
