//! Tools for traversing all borrows in method parameters and return types, shallowly
//!
//! This is useful for backends which wish to figure out the borrowing relationships between parameters
//! and return values,
//! and then delegate how lifetimes get mapped to fields to the codegen for those types respectively.

use std::collections::{BTreeMap, BTreeSet};

use crate::hir::{self, Method, StructDef, TyPosition, TypeContext};

use crate::hir::lifetimes::{Lifetime, LifetimeEnv, MaybeStatic};
use crate::hir::ty_position::StructPathLike;

/// A visitor for processing method parameters/returns and understanding their borrowing relationships, shallowly.
///
/// This produces a list of lifetime "edges" per lifetime in the output producing a borrow.
///
/// Each `BorrowingFieldsVisitor` corresponds to the type of an input of a method.
///
/// Obtain from [`Method::borrowing_param_visitor()`].
pub struct BorrowingParamVisitor<'tcx> {
    tcx: &'tcx TypeContext,
    used_method_lifetimes: BTreeSet<Lifetime>,
    borrow_map: BTreeMap<Lifetime, BorrowedLifetimeInfo<'tcx>>,
}

/// A single lifetime "edge" from a parameter to a value
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct LifetimeEdge<'tcx> {
    pub param_name: String,
    pub kind: LifetimeEdgeKind<'tcx>,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum LifetimeEdgeKind<'tcx> {
    /// Just an opaque parameter directly being borrowed.
    OpaqueParam,
    /// A slice being converted and then borrowed. These often need to be handled differently
    /// when they are borrowed as the borrow will need to create an edge
    SliceParam,
    /// A lifetime parameter of a struct, given the lifetime context and the struct-def lifetime for that struct
    ///
    /// Using this, you can generate code that "asks" the struct for the lifetime-relevant field edges
    StructLifetime(&'tcx LifetimeEnv, Lifetime),
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct BorrowedLifetimeInfo<'tcx> {
    // Initializers for all inputs to the edge array from parameters, except for slices (slices get handled
    // differently)
    pub incoming_edges: Vec<LifetimeEdge<'tcx>>,
    // All lifetimes longer than this. When this lifetime is borrowed from, data corresponding to
    // the other lifetimes may also be borrowed from.
    pub all_longer_lifetimes: BTreeSet<Lifetime>,
}

impl<'tcx> BorrowingParamVisitor<'tcx> {
    pub(crate) fn new(method: &'tcx Method, tcx: &'tcx TypeContext) -> Self {
        let used_method_lifetimes = method.output.used_method_lifetimes();
        let borrow_map = used_method_lifetimes
            .iter()
            .map(|lt| {
                (
                    *lt,
                    BorrowedLifetimeInfo {
                        incoming_edges: Vec::new(),
                        all_longer_lifetimes: method
                            .lifetime_env
                            .all_longer_lifetimes(lt)
                            .collect(),
                    },
                )
            })
            .collect();
        BorrowingParamVisitor {
            tcx,
            used_method_lifetimes,
            borrow_map,
        }
    }

    /// Get the cached list of used method lifetimes. Same as calling `.used_method_lifetimes()` on `method.output`
    pub fn used_method_lifetimes(&self) -> &BTreeSet<Lifetime> {
        &self.used_method_lifetimes
    }

    /// Get the final borrow map, listing lifetime edges for each output lfietime
    pub fn borrow_map(self) -> BTreeMap<Lifetime, BorrowedLifetimeInfo<'tcx>> {
        self.borrow_map
    }

    /// Processes a parameter, adding it to the borrow_map for any lifetimes it references. Returns further information about the type of borrow.
    ///
    /// This basically boils down to: For each lifetime that is actually relevant to borrowing in this method, check if that
    /// lifetime or lifetimes longer than it are used by this parameter. In other words, check if
    /// it is possible for data in the return type with this lifetime to have been borrowed from this parameter.
    /// If so, add code that will yield the ownership-relevant parts of this object to incoming_edges for that lifetime.
    pub fn visit_param(&mut self, ty: &hir::Type, param_name: &str) -> ParamBorrowInfo<'tcx> {
        let mut is_borrowed = false;
        if self.used_method_lifetimes.is_empty() {
            if let hir::Type::Slice(..) = *ty {
                return ParamBorrowInfo::TemporarySlice;
            } else {
                return ParamBorrowInfo::NotBorrowed;
            }
        }

        // Structs have special handling: structs are purely Dart-side, so if you borrow
        // from a struct, you really are borrowing from the internal fields.
        if let hir::Type::Struct(s) = ty {
            let mut borrowed_struct_lifetime_map = BTreeMap::<Lifetime, BTreeSet<Lifetime>>::new();
            let link = s.link_lifetimes(self.tcx);
            for (method_lifetime, method_lifetime_info) in &mut self.borrow_map {
                // Note that ty.lifetimes()/s.lifetimes() is lifetimes
                // in the *use* context, i.e. lifetimes on the Type that reference the
                // indices of the method's lifetime arrays. Their *order* references
                // the indices of the underlying struct def. We need to link the two,
                // since the _fields_for_lifetime_foo() methods are named after
                // the *def* context lifetime.
                //
                // Concretely, if we have struct `Foo<'a, 'b>` and our method
                // accepts `Foo<'x, 'y>`, we need to output _fields_for_lifetime_a()/b not x/y.
                //
                // This is a struct so lifetimes_def_only() is fine to call
                for (use_lt, def_lt) in link.lifetimes_def_only() {
                    if let MaybeStatic::NonStatic(use_lt) = use_lt {
                        if method_lifetime_info.all_longer_lifetimes.contains(&use_lt) {
                            let edge = LifetimeEdge {
                                param_name: param_name.into(),
                                kind: LifetimeEdgeKind::StructLifetime(link.def_env(), def_lt),
                            };
                            method_lifetime_info.incoming_edges.push(edge);

                            is_borrowed = true;

                            borrowed_struct_lifetime_map
                                .entry(def_lt)
                                .or_default()
                                .insert(*method_lifetime);
                            // Do *not* break the inner loop here: even if we found *one* matching lifetime
                            // in this struct that may not be all of them, there may be some other fields that are borrowed
                        }
                    }
                }
            }
            if is_borrowed {
                ParamBorrowInfo::Struct(StructBorrowInfo {
                    env: link.def_env(),
                    borrowed_struct_lifetime_map,
                })
            } else {
                ParamBorrowInfo::NotBorrowed
            }
        } else {
            for method_lifetime in self.borrow_map.values_mut() {
                for lt in ty.lifetimes() {
                    if let MaybeStatic::NonStatic(lt) = lt {
                        if method_lifetime.all_longer_lifetimes.contains(&lt) {
                            let kind = match ty {
                                hir::Type::Slice(..) => LifetimeEdgeKind::SliceParam,
                                hir::Type::Opaque(..) => LifetimeEdgeKind::OpaqueParam,
                                _ => unreachable!("Types other than slices, opaques, and structs cannot have lifetimes")
                            };

                            let edge = LifetimeEdge {
                                param_name: param_name.into(),
                                kind,
                            };

                            method_lifetime.incoming_edges.push(edge);
                            is_borrowed = true;
                            // Break the inner loop: we've already determined this
                            break;
                        }
                    }
                }
            }
            match (is_borrowed, ty) {
                (true, &hir::Type::Slice(..)) => ParamBorrowInfo::BorrowedSlice,
                (false, &hir::Type::Slice(..)) => ParamBorrowInfo::TemporarySlice,
                (false, _) => ParamBorrowInfo::NotBorrowed,
                (true, _) => ParamBorrowInfo::BorrowedOpaque,
            }
        }
    }
}

/// Information relevant to borrowing for producing conversions
#[derive(Clone, Debug)]
pub enum ParamBorrowInfo<'tcx> {
    /// No borrowing constraints. This means the parameter
    /// is not borrowed by the output and also does not need temporary borrows
    NotBorrowed,
    /// A slice that is not borrowed by the output (but will still need temporary allocation)
    TemporarySlice,
    /// A slice that is borrowed by the output
    BorrowedSlice,
    /// A struct parameter that is borrowed by the output
    Struct(StructBorrowInfo<'tcx>),
    /// An opaque type that is borrowed
    BorrowedOpaque,
}

/// Information about the lifetimes of a struct parameter that are borrowed by a method output or by a wrapping struct
#[derive(Clone, Debug)]
pub struct StructBorrowInfo<'tcx> {
    /// This is the struct's lifetime environment
    pub env: &'tcx LifetimeEnv,
    /// A map from (borrow-relevant) struct lifetimes to lifetimes in the method (or wrapping struct) that may flow from it
    pub borrowed_struct_lifetime_map: BTreeMap<Lifetime, BTreeSet<Lifetime>>,
}

impl<'tcx> StructBorrowInfo<'tcx> {
    /// Get borrowing info for a struct field, if it does indeed borrow
    ///
    /// The lifetime map produced here does not handle lifetime dependencies: the expectation is that the struct
    /// machinery generated by this will be called by method code that handles these dependencies. We try to handle
    /// lifetime dependencies in ONE place.
    pub fn compute_for_struct_field<P: TyPosition>(
        struc: &StructDef<P>,
        field: &P::StructPath,
        tcx: &'tcx TypeContext,
    ) -> Option<Self> {
        if field.lifetimes().as_slice().is_empty() {
            return None;
        }

        let mut borrowed_struct_lifetime_map = BTreeMap::<Lifetime, BTreeSet<Lifetime>>::new();

        let link = field.link_lifetimes(tcx);

        for outer_lt in struc.lifetimes.all_lifetimes() {
            // Note that field.lifetimes()
            // in the *use* context, i.e. lifetimes on the Type that reference the
            // indices of the outer struct's lifetime arrays. Their *order* references
            // the indices of the underlying struct def. We need to link the two,
            // since the _fields_for_lifetime_foo() methods are named after
            // the *def* context lifetime.
            //
            // This is a struct so lifetimes_def_only() is fine to call
            for (use_lt, def_lt) in link.lifetimes_def_only() {
                if let MaybeStatic::NonStatic(use_lt) = use_lt {
                    // We do *not* need to transitively check for longer lifetimes here:
                    //
                    if outer_lt == use_lt {
                        borrowed_struct_lifetime_map
                            .entry(def_lt)
                            .or_default()
                            .insert(outer_lt);
                    }
                }
            }
        }
        if borrowed_struct_lifetime_map.is_empty() {
            // if the inner struct is only statics
            None
        } else {
            Some(StructBorrowInfo {
                env: link.def_env(),
                borrowed_struct_lifetime_map,
            })
        }
    }
}
