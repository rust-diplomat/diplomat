//! Accessors and the properties they become.
//!
//! A C# property has exactly one type, but its getter and setter can cross the
//! FFI boundary differently — a `DiplomatWrite` getter has Rust append UTF-8 to
//! a buffer we then decode, while the matching setter can pin a C# string's
//! UTF-16 in place and copy nothing. So marshalling and the exposed type are
//! kept apart: [`AccessorMarshal`] is per-accessor, [`PropertyType`] is shared.
//!
//! That split is what makes the rule enforceable. A getter and a setter may
//! share a property only when their `PropertyType`s are equal, so the two can
//! never end up with different C# types. The one place the mapping collapses is
//! text — every string encoding surfaces as `string` — and it is spelled out one
//! arm at a time in [`AccessorMarshal::shape`], so that collapse is reviewable
//! rather than assumed.

use std::collections::BTreeMap;
use std::fmt::{self, Display};

use super::method::{BorrowedSpanElement, MemberDocs, MethodInfo};
use super::DotnetPrimitives;
use crate::ErrorStore;

// ─────────────────────────────────────────────────────────────────────────────
// One accessor
// ─────────────────────────────────────────────────────────────────────────────

/// Which half of a property a method is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AccessorKind {
    Getter,
    Setter,
}

impl AccessorKind {
    /// The C# keyword, which is also the word the diagnostics use.
    fn keyword(self) -> &'static str {
        match self {
            Self::Getter => "get",
            Self::Setter => "set",
        }
    }
}

/// How one accessor crosses the FFI boundary.
///
/// The four text marshals are separate variants because they are not
/// interchangeable, even though all four present as `string`:
///
/// * `WrittenUtf8` — Rust appends UTF-8 to a caller-provided `DiplomatWrite`,
///   which the binding then decodes. Getter only.
/// * `ValidatedUtf8Param` — `&str`. Rust may assume well-formed UTF-8, so the
///   binding has to transcode a real `System.String`; it can never pass caller
///   bytes straight through.
/// * `UnvalidatedUtf8Param` — `&DiplomatStr`. Opaque bytes with no validity
///   contract, which is why outside a property this is a zero-copy `byte[]`.
/// * `Utf16Param` — `&DiplomatStr16`. A C# string is already a flat UTF-16
///   buffer, so this pins it in place and allocates nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum AccessorMarshal {
    Primitive(DotnetPrimitives),
    Enum(String),
    Struct(String),
    Opaque(String),
    WrittenUtf8,
    ValidatedUtf8Param,
    UnvalidatedUtf8Param,
    Utf16Param,
    /// `&'a [T]` / `&'a str` return — a zero-copy view over Rust-owned memory.
    BorrowedSpanReturn(BorrowedSpanElement),
    /// `Box<[u8]>` return — a Rust allocation the caller disposes.
    OwnedBytesReturn,
    /// Slice parameter Rust only reads during the call: a managed array pinned
    /// for the duration of the call and released after it.
    ManagedArrayParam(BorrowedSpanElement),
    /// Slice parameter the return borrows from, so the buffer stays pinned past
    /// the call and the caller keeps owning the memory.
    PinnedMemoryParam(BorrowedSpanElement),
}

impl AccessorMarshal {
    /// The property surface this marshal presents.
    fn shape(&self) -> PropertyShape {
        match self {
            Self::Primitive(p) => PropertyShape::Primitive(*p),
            Self::Enum(name) => PropertyShape::Enum(name.clone()),
            Self::Struct(name) => PropertyShape::Struct(name.clone()),
            Self::Opaque(name) => PropertyShape::Opaque(name.clone()),
            Self::WrittenUtf8 => PropertyShape::Text,
            Self::ValidatedUtf8Param => PropertyShape::Text,
            Self::UnvalidatedUtf8Param => PropertyShape::Text,
            Self::Utf16Param => PropertyShape::Text,
            Self::BorrowedSpanReturn(elem) => PropertyShape::BorrowedSpan(*elem),
            Self::OwnedBytesReturn => PropertyShape::OwnedBytes,
            Self::ManagedArrayParam(elem) => PropertyShape::ManagedArray(*elem),
            Self::PinnedMemoryParam(elem) => PropertyShape::PinnedMemory(*elem),
        }
    }

    /// How a diagnostic names this marshal. The C# type alone cannot explain a
    /// refusal — `string` against `string` reads as agreement — so this names the
    /// Rust shape that produced it.
    fn describe(&self) -> &'static str {
        match self {
            Self::Primitive(_) => "a primitive",
            Self::Enum(_) => "an enum",
            Self::Struct(_) => "a struct by value",
            Self::Opaque(_) => "an opaque handle",
            Self::WrittenUtf8 => "UTF-8 written into a `DiplomatWrite`",
            Self::ValidatedUtf8Param => "a validated UTF-8 string (`&str`)",
            Self::UnvalidatedUtf8Param => "an unvalidated UTF-8 string (`&DiplomatStr`)",
            Self::Utf16Param => "a UTF-16 string (`&DiplomatStr16`)",
            Self::BorrowedSpanReturn(_) => "a borrowed view over Rust-owned memory",
            Self::OwnedBytesReturn => "an owned `Box<[u8]>` the caller disposes",
            Self::ManagedArrayParam(_) => "a managed array Rust reads during the call",
            Self::PinnedMemoryParam(_) => "a pinned buffer the return borrows from",
        }
    }
}

/// One accessor's C#-facing value: how it marshals, and whether it can be null.
/// Both axes have to match for a getter and setter to share a property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct AccessorValue {
    marshal: AccessorMarshal,
    nullable: bool,
}

impl AccessorValue {
    pub(super) fn plain(marshal: AccessorMarshal) -> Self {
        Self {
            marshal,
            nullable: false,
        }
    }

    pub(super) fn nullable_if(nullable: bool, marshal: AccessorMarshal) -> Self {
        Self { marshal, nullable }
    }

    pub(super) fn property_type(&self) -> PropertyType {
        PropertyType {
            shape: self.marshal.shape(),
            nullable: self.nullable,
        }
    }
}

/// An accessor as the type-level codegen sees it: where it belongs and what it
/// puts there.
pub(super) struct AccessorInfo {
    /// C# property name — see `DotnetFormatter::fmt_accessor_name`.
    pub(super) name: String,
    pub(super) kind: AccessorKind,
    /// Rust method name, so a diagnostic can point at the source, not the
    /// generated C#.
    pub(super) rust_name: String,
    pub(super) value: AccessorValue,
}

// ─────────────────────────────────────────────────────────────────────────────
// The property type
// ─────────────────────────────────────────────────────────────────────────────

/// What a C# property's single type is made of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PropertyType {
    shape: PropertyShape,
    /// `Option<T>` on either side — the C# type gains `?`.
    nullable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PropertyShape {
    Primitive(DotnetPrimitives),
    Enum(String),
    Struct(String),
    Opaque(String),
    /// `string` — the one surface several marshals share.
    Text,
    OwnedBytes,
    BorrowedSpan(BorrowedSpanElement),
    ManagedArray(BorrowedSpanElement),
    PinnedMemory(BorrowedSpanElement),
}

impl PropertyType {
    /// The C# type written in the property declaration.
    fn cs_type(&self) -> String {
        let shape = match &self.shape {
            PropertyShape::Primitive(p) => p.to_string(),
            PropertyShape::Enum(name)
            | PropertyShape::Struct(name)
            | PropertyShape::Opaque(name) => name.clone(),
            PropertyShape::Text => "string".to_string(),
            PropertyShape::OwnedBytes => "RustVec".to_string(),
            PropertyShape::BorrowedSpan(elem) => {
                format!("DiplomatBorrowedSpan<{}>", elem.element_type())
            }
            PropertyShape::ManagedArray(elem) => format!("{}[]", elem.element_type()),
            PropertyShape::PinnedMemory(elem) => {
                format!("ReadOnlyMemory<{}>", elem.element_type())
            }
        };
        if self.nullable {
            format!("{shape}?")
        } else {
            shape
        }
    }
}

impl Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.cs_type())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pairing accessors into properties
// ─────────────────────────────────────────────────────────────────────────────

/// A finished property. The accessor methods live inside it — an accessor *is*
/// the property, the same single member Dart and JS emit, not a property
/// forwarding to a second method under another name.
pub(super) struct PropertyInfo<'ctx> {
    pub(super) name: String,
    pub(super) property_type: PropertyType,
    pub(super) getter: Option<MethodInfo<'ctx>>,
    pub(super) setter: Option<MethodInfo<'ctx>>,
}

impl PropertyInfo<'_> {
    /// The doc comments this property carries — the union of both accessors',
    /// each said once.
    pub(super) fn docs(&self) -> MemberDocs {
        MemberDocs::for_accessors(self.getter.as_ref(), self.setter.as_ref())
    }
}

/// Accessors collected under one property name, before the pair is checked.
struct PropertyDraft<'ctx> {
    getter: Option<Slot<'ctx>>,
    setter: Option<Slot<'ctx>>,
}

struct Slot<'ctx> {
    value: AccessorValue,
    rust_name: String,
    method: MethodInfo<'ctx>,
}

impl<'ctx> PropertyDraft<'ctx> {
    fn new() -> Self {
        Self {
            getter: None,
            setter: None,
        }
    }

    /// Fill this property's getter or setter slot. Two methods claiming the same
    /// slot would be two C# members with one name (CS0102), so the second is
    /// refused rather than silently dropped or emitted.
    fn fill(
        &mut self,
        property: &str,
        accessor: AccessorInfo,
        method: MethodInfo<'ctx>,
        errors: &ErrorStore<'_, String>,
    ) {
        let slot = match accessor.kind {
            AccessorKind::Getter => &mut self.getter,
            AccessorKind::Setter => &mut self.setter,
        };
        if let Some(taken) = slot {
            errors.push_error(format!(
                "[.NET backend] two Rust methods both claim the `{}` of property `{property}`: \
                 `{}` and `{}`. C# allows one of each, so rename one accessor or drop its \
                 attribute.",
                accessor.kind.keyword(),
                taken.rust_name,
                accessor.rust_name,
            ));
            return;
        }
        *slot = Some(Slot {
            value: accessor.value,
            rust_name: accessor.rust_name,
            method,
        });
    }

    /// Agree on one type, or refuse the property.
    ///
    /// A write-only or read-only property is fine — a Rust config object often
    /// has setters and no getters, and C# expresses that directly. What C#
    /// cannot express is a getter and setter of different types, so that is
    /// where this gives up instead of guessing which side to honour.
    fn finish(self, name: String, errors: &ErrorStore<'_, String>) -> Option<PropertyInfo<'ctx>> {
        let Self { getter, setter } = self;

        let property_type = match (&getter, &setter) {
            (Some(g), Some(s)) => {
                let (get, set) = (g.value.property_type(), s.value.property_type());
                if get != set {
                    errors.push_error(format!(
                        "[.NET backend] property `{name}` would need two types: `{}` \
                         from getter `{}` ({}), and `{}` from setter `{}` ({}). A C# property \
                         has a single type. Make the two Rust signatures agree, or disable one \
                         of them for .NET.",
                        get,
                        g.rust_name,
                        g.value.marshal.describe(),
                        set,
                        s.rust_name,
                        s.value.marshal.describe(),
                    ));
                    return None;
                }
                get
            }
            (Some(only), None) | (None, Some(only)) => only.value.property_type(),
            (None, None) => return None,
        };

        Some(PropertyInfo {
            name,
            property_type,
            getter: getter.map(|slot| slot.method),
            setter: setter.map(|slot| slot.method),
        })
    }
}

/// Route each of a type's lowered methods either into the property it backs or
/// into the plain method list, in one pass.
///
/// Ordering is by property name because the accessors of one property can be
/// declared far apart in Rust; the plain methods keep declaration order.
pub(super) fn route_members<'ctx>(
    lowered: Vec<(Option<AccessorInfo>, MethodInfo<'ctx>)>,
    errors: &ErrorStore<'_, String>,
) -> (Vec<MethodInfo<'ctx>>, Vec<PropertyInfo<'ctx>>) {
    let mut methods = Vec::new();
    let mut drafts = BTreeMap::<String, PropertyDraft<'ctx>>::new();

    for (accessor, method) in lowered {
        match accessor {
            Some(accessor) => {
                let name = accessor.name.clone();
                drafts
                    .entry(name.clone())
                    .or_insert_with(PropertyDraft::new)
                    .fill(&name, accessor, method, errors);
            }
            None => methods.push(method),
        }
    }

    let properties = drafts
        .into_iter()
        .filter_map(|(name, draft)| draft.finish(name, errors))
        .collect();
    (methods, properties)
}

/// Refuse anything C# would not compile: a property sharing its name with
/// another member (CS0102), or with the type that contains it (CS0542).
///
/// The generated type is not only what Diplomat was asked for. Structs always
/// get `AsFFI` and `FromFFI`; opaques always get `Cleanup` and `Lease`, plus
/// `Dispose` when they opt into `IDisposable`; and a struct's fields are
/// members too — so a property named after any of those, or after the type
/// itself, compiles to nothing.
pub(super) fn reject_member_collisions(
    ty: &str,
    properties: &[PropertyInfo<'_>],
    methods: &[MethodInfo<'_>],
    field_names: &[&str],
    is_opaque: bool,
    has_generated_dispose: bool,
    errors: &ErrorStore<'_, String>,
) {
    /// The label for the enclosing type, which C# rejects on its own terms.
    const ENCLOSING_TYPE: &str = "the type that contains it";

    let mut seen = BTreeMap::<&str, &str>::new();
    seen.insert(ty, ENCLOSING_TYPE);
    let mut generated_members = BTreeMap::<&str, &str>::new();
    if is_opaque {
        for member in ["Cleanup", "Lease"] {
            generated_members.insert(member, "a member Diplomat always generates for opaques");
        }
        if has_generated_dispose {
            generated_members.insert(
                "Dispose",
                "a member Diplomat generates for manually_disposable opaques",
            );
        }
    } else {
        for member in ["AsFFI", "FromFFI"] {
            generated_members.insert(member, "a member Diplomat always generates for structs");
        }
    }
    for (member, description) in &generated_members {
        seen.entry(member).or_insert(description);
    }
    for field in field_names {
        seen.entry(field).or_insert("a struct field");
    }
    for method in methods {
        if let Some(generated) = generated_members.get(method.name.as_str()) {
            errors.push_error(format!(
                "[.NET backend] `{ty}` would have two members named `{}`: a method and \
                 {generated}. Rename the method with `#[diplomat::rename]`.",
                method.name
            ));
        }
        seen.entry(&method.name).or_insert("a method");
    }
    for property in properties {
        // `insert` so a second property of the same name still reports, and the
        // first description wins because everything above used `or_insert`.
        let Some(existing) = seen.insert(&property.name, "a property") else {
            continue;
        };
        if existing == ENCLOSING_TYPE {
            errors.push_error(format!(
                "[.NET backend] property `{}` has the same name as `{ty}`, the type that \
                 contains it, which C# does not allow. Name the accessor explicitly with \
                 `#[diplomat::attr(auto, getter = \"…\")]`, or rename it with \
                 `#[diplomat::rename]`.",
                property.name
            ));
        } else {
            errors.push_error(format!(
                "[.NET backend] `{ty}` would have two members named `{}`: a property and \
                 {existing}. Rename one with `#[diplomat::rename]`, or name the accessor \
                 explicitly with `#[diplomat::attr(auto, getter = \"…\")]`.",
                property.name
            ));
        }
    }
}
