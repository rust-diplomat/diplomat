//! Tools for traversing all borrows in method parameters and return types, shallowly
//!
//! This is useful for backends which wish to figure out the borrowing relationships between parameters
//! and return values,
//! and then delegate how lifetimes get mapped to fields to the codegen for those types respectively.

use std::collections::{BTreeMap, BTreeSet};

use crate::hir::{self, Method, TypeContext};

use crate::hir::lifetimes::{Lifetime, LifetimeEnv, MaybeStatic};

/// A visitor for processing method parameters/returns and understanding their borrowing relationships, shallowly.
///
/// This produces a list of lifetime "edges" per lifetime in the output producing a borrow.
///
/// Each `BorrowingFieldsVisitor` corresponds to the type of an input of a method.
///
/// Obtain from [`Method::borrowing_param_visitor()`].
pub struct BorrowingParamVisitor<'tcx> {
    tcx: &'tcx TypeContext,
    #[allow(unused)] // to be used later
    method: &'tcx Method,
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
            method,
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

    /// Processes a parameter, adding it to the borrow_map for any lifetimes it references. Returns true if the lifetime is referenced.
    ///
    /// This basically boils down to: For each lifetime that is actually relevant to borrowing in this method, check if that
    /// lifetime or lifetimes longer than it are used by this parameter. In other words, check if
    /// it is possible for data in the return type with this lifetime to have been borrowed from this parameter.
    /// If so, add code that will yield the ownership-relevant parts of this object to incoming_edges for that lifetime.
    pub fn visit_param(&mut self, ty: &hir::Type, param_name: &str) -> bool {
        if self.used_method_lifetimes.is_empty() {
            return false;
        }
        let mut edges_pushed = false;

        // Structs have special handling: structs are purely Dart-side, so if you borrow
        // from a struct, you really are borrowing from the internal fields.
        if let hir::Type::Struct(s) = ty {
            let link = s.link_lifetimes(self.tcx);
            for method_lifetime in self.borrow_map.values_mut() {
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
                        if method_lifetime.all_longer_lifetimes.contains(&use_lt) {
                            let edge = LifetimeEdge {
                                param_name: param_name.into(),
                                kind: LifetimeEdgeKind::StructLifetime(link.def_env(), def_lt),
                            };
                            method_lifetime.incoming_edges.push(edge);
                            edges_pushed = true;
                            // Do *not* break the inner loop here: even if we found *one* matching lifetime
                            // in this struct that may not be all of them, there may be some other fields that are borrowed
                        }
                    }
                }
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
                            edges_pushed = true;
                            // Break the inner loop: we've already determined this
                            break;
                        }
                    }
                }
            }
        }

        // return true if the number of edges changed
        edges_pushed
    }
}
