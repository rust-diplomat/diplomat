//! Decides which opaques must carry `#[diplomat::attr(dotnet, manually_disposable)]`.
//!
//! A generated wrapper can hold something on another object that only its
//! `Dispose()` releases before the garbage collector gets to it: a borrow on
//! the source, a lifetime reference to it, or a pin on a managed buffer. If the
//! wrapper's type has no `Dispose()`, the source stays borrowed, alive, or
//! pinned until finalization. So a type that can land in one of these positions
//! has to opt into `IDisposable`, and generation fails when it does not:
//!
//! - mutable borrow: a method returns `&mut T`. The wrapper holds an exclusive
//!   borrow on its source.
//! - owned borrow: a method returns `Box<T<'a>>` borrowing the receiver or an
//!   opaque parameter. The wrapper holds a real borrow, so the source cannot be
//!   mutated until that borrow is released. The `Err` arm counts too.
//! - transitive retention: the returned value retains, directly or through
//!   other views, a source that is (or must be) manually disposable. A view
//!   must not defer a disposable source's release to the GC.
//! - pinned input: a method returns `Box<T<'a>>` borrowing a slice or string
//!   parameter, so the wrapper pins managed memory.
//!
//! The facts come from the same dependency and pin lists that become the
//! generated `edges`, recorded while each method is lowered.
//! [`DisposalRequirements::finish`] then propagates transitive retention as a
//! fixpoint over the union of marked and required types, so a whole chain is
//! reported in one run instead of one link per generation.

use std::collections::{BTreeMap, BTreeSet};

use diplomat_core::hir::{Mutability, OpaqueId, TypeContext, TypeDef, TypeId};

use crate::{dotnet::formatter::DotnetFormatter, ErrorStore};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ReturnArm {
    Success,
    Error,
}

/// Where a returned value's borrow comes from, named the way the Rust author
/// wrote it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum BorrowSource {
    Receiver,
    Parameter(String),
}

impl BorrowSource {
    fn describe(&self) -> String {
        match self {
            Self::Receiver => "the receiver".to_string(),
            Self::Parameter(name) => format!("parameter `{name}`"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BorrowMode {
    Shared,
    Exclusive,
}

impl From<Mutability> for BorrowMode {
    fn from(mutability: Mutability) -> Self {
        match mutability {
            Mutability::Immutable => Self::Shared,
            Mutability::Mutable => Self::Exclusive,
        }
    }
}

impl BorrowMode {
    fn describe(self) -> &'static str {
        match self {
            Self::Shared => "a shared",
            Self::Exclusive => "an exclusive",
        }
    }

    fn blocked_calls(self) -> &'static str {
        match self {
            Self::Shared => "mutable calls",
            Self::Exclusive => "read-only and mutable calls",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TriggerKind {
    MutableBorrow,
    OwnedBorrow {
        source: BorrowSource,
        source_type: OpaqueId,
        borrow_mode: BorrowMode,
    },
    Retains {
        source: OpaqueId,
        root: OpaqueId,
        source_is_marked: bool,
    },
    PinnedSlice(String),
}

impl TriggerKind {
    fn label(&self) -> &'static str {
        match self {
            Self::MutableBorrow => "mutable borrow",
            Self::OwnedBorrow { .. } => "owned borrow",
            Self::Retains { .. } => "transitive retention",
            Self::PinnedSlice(_) => "pinned input",
        }
    }

    fn rank(&self) -> u8 {
        match self {
            Self::MutableBorrow => 0,
            Self::OwnedBorrow { .. } => 1,
            Self::PinnedSlice(_) => 2,
            Self::Retains { .. } => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Trigger {
    method: String,
    arm: ReturnArm,
    kind: TriggerKind,
}

#[derive(Clone, Debug)]
struct Retain {
    dependent: OpaqueId,
    source: OpaqueId,
    method: String,
    arm: ReturnArm,
}

#[derive(Default)]
pub(in crate::dotnet) struct DisposalRequirements {
    required: BTreeMap<OpaqueId, Vec<Trigger>>,
    retains: Vec<Retain>,
}

impl DisposalRequirements {
    fn add_trigger(&mut self, opaque: OpaqueId, trigger: Trigger) {
        let triggers = self.required.entry(opaque).or_default();
        if !triggers.contains(&trigger) {
            triggers.push(trigger);
        }
    }

    pub(super) fn record_mutable_borrow(&mut self, opaque: OpaqueId, method: &str, arm: ReturnArm) {
        self.add_trigger(
            opaque,
            Trigger {
                method: method.to_string(),
                arm,
                kind: TriggerKind::MutableBorrow,
            },
        );
    }

    pub(super) fn record_owned_borrow(
        &mut self,
        opaque: OpaqueId,
        source: BorrowSource,
        source_type: OpaqueId,
        borrow_mode: BorrowMode,
        method: &str,
        arm: ReturnArm,
    ) {
        self.add_trigger(
            opaque,
            Trigger {
                method: method.to_string(),
                arm,
                kind: TriggerKind::OwnedBorrow {
                    source,
                    source_type,
                    borrow_mode,
                },
            },
        );
    }

    pub(super) fn record_pin(
        &mut self,
        opaque: OpaqueId,
        parameter: &str,
        method: &str,
        arm: ReturnArm,
    ) {
        self.add_trigger(
            opaque,
            Trigger {
                method: method.to_string(),
                arm,
                kind: TriggerKind::PinnedSlice(parameter.to_string()),
            },
        );
    }

    pub(super) fn record_retain(
        &mut self,
        dependent: OpaqueId,
        source: OpaqueId,
        method: &str,
        arm: ReturnArm,
    ) {
        self.retains.push(Retain {
            dependent,
            source,
            method: method.to_string(),
            arm,
        });
    }

    /// Reports every type that must be `manually_disposable` but is not.
    ///
    /// Propagation runs over marked *and* required types. Propagating from the
    /// marked set alone would report an outer view only after the author marks
    /// the middle one, one generation per link.
    pub(super) fn finish<'tcx>(
        &self,
        tcx: &'tcx TypeContext,
        errors: &ErrorStore<'tcx, String>,
        formatter: &DotnetFormatter<'tcx>,
    ) {
        let marked: BTreeSet<OpaqueId> = tcx
            .all_types()
            .filter_map(|(id, ty)| match (id, ty) {
                (TypeId::Opaque(id), TypeDef::Opaque(def))
                    if !def.attrs.disable && def.attrs.manually_disposable =>
                {
                    Some(id)
                }
                _ => None,
            })
            .collect();
        let mut required: BTreeSet<OpaqueId> = self.required.keys().copied().collect();
        let mut roots: BTreeMap<OpaqueId, OpaqueId> = marked
            .iter()
            .chain(required.iter())
            .map(|id| (*id, *id))
            .collect();

        loop {
            let mut changed = false;
            for retain in &self.retains {
                if (marked.contains(&retain.source) || required.contains(&retain.source))
                    && required.insert(retain.dependent)
                {
                    let root = roots.get(&retain.source).copied().unwrap_or(retain.source);
                    roots.insert(retain.dependent, root);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        let mut triggers = self.required.clone();
        for retain in &self.retains {
            if !marked.contains(&retain.source) && !required.contains(&retain.source) {
                continue;
            }
            let trigger = Trigger {
                method: retain.method.clone(),
                arm: retain.arm,
                kind: TriggerKind::Retains {
                    source: retain.source,
                    root: roots.get(&retain.source).copied().unwrap_or(retain.source),
                    source_is_marked: marked.contains(&retain.source),
                },
            };
            let dependent_triggers = triggers.entry(retain.dependent).or_default();
            if !dependent_triggers.contains(&trigger) {
                dependent_triggers.push(trigger);
            }
        }

        let mut missing: Vec<(&str, OpaqueId)> = required
            .difference(&marked)
            .map(|id| (*id, tcx.resolve_opaque(*id)))
            .filter(|(_, def)| !def.attrs.disable)
            .map(|(id, def)| (def.name.as_str(), id))
            .collect();
        missing.sort_by(|a, b| a.0.cmp(b.0));

        for (rust_name, id) in missing {
            let mut type_triggers = triggers.remove(&id).unwrap_or_default();
            type_triggers.sort_by(|a, b| {
                a.kind
                    .rank()
                    .cmp(&b.kind.rank())
                    .then_with(|| a.method.cmp(&b.method))
            });
            let mut message = format!(
                "[.NET backend] `{rust_name}` must be `#[diplomat::attr(dotnet, manually_disposable)]`:"
            );
            for trigger in &type_triggers {
                message.push_str("\n  - ");
                message.push_str(&trigger.describe(tcx, rust_name));
            }
            let _guard = errors.set_context_ty(formatter.fmt_type_name(id.into()));
            errors.push_error(message);
        }
    }
}

impl Trigger {
    fn describe(&self, tcx: &TypeContext, returned_type: &str) -> String {
        let label = self.kind.label();
        let error_arm = if self.arm == ReturnArm::Error {
            " from the `Result` error arm"
        } else {
            ""
        };
        match &self.kind {
            TriggerKind::MutableBorrow => format!(
                "{label}: `{}` returns it as a mutable borrow; only Dispose() ends the exclusive borrow",
                self.method
            ),
            TriggerKind::OwnedBorrow {
                source,
                source_type,
                borrow_mode,
            } => {
                let source_type = tcx.resolve_opaque(*source_type).name.as_str();
                let blocked_calls = borrow_mode.blocked_calls();
                let borrow_mode = borrow_mode.describe();
                let error_retention = if self.arm == ReturnArm::Error {
                    format!(
                        " The generated exception retains `{returned_type}` through its `Inner` property."
                    )
                } else {
                    String::new()
                };
                format!(
                    "{label}: `{}` returns owned `{returned_type}`{error_arm}, which holds {borrow_mode} borrow of `{source_type}` from {}.{error_retention} Until `{returned_type}` is disposed, {blocked_calls} on `{source_type}` are rejected and its native value is kept alive. Without `manually_disposable`, .NET cannot release this borrow deterministically",
                    self.method,
                    source.describe()
                )
            }
            TriggerKind::Retains {
                source,
                root,
                source_is_marked,
            } => {
                let source_name = tcx.resolve_opaque(*source).name.as_str();
                let relation = if *source_is_marked {
                    "which is manually_disposable".to_string()
                } else if source == root {
                    "which must be manually_disposable".to_string()
                } else {
                    format!(
                        "which retains the manually_disposable `{}`",
                        tcx.resolve_opaque(*root).name.as_str()
                    )
                };
                format!(
                    "{label}: `{}` returns it{error_arm} borrowing from `{source_name}`, {relation}; a view must not defer a disposable source's release",
                    self.method
                )
            }
            TriggerKind::PinnedSlice(parameter) => format!(
                "{label}: `{}` returns it borrowing from slice parameter `{parameter}`; the wrapper pins managed memory it must be able to unpin",
                self.method
            ),
        }
    }
}
