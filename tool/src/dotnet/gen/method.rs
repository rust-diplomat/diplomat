//! Method composition vocabulary — how a single HIR `Method` turns into
//! the C# fragments that templates consume.
//!
//! Both inputs and outputs are layer-agnostic at the data level — variants
//! name the *kind* of type ("Opaque", "Struct", "Primitive", …) and the raw
//! vs idiomatic C# spellings come from view methods. Body-shape decisions
//! (the disposed check, the `new T(...)` wrap, the `FromFFI` bridge) live
//! in the templates, where the C# code naturally lives.
//!
//! ## What lives here
//!
//! * [`DotnetReturnType`] — return-side vocabulary. Predicates +
//!   `as_raw` / `as_idiomatic` let templates pick the right spelling and
//!   render the right body shape.
//! * [`DotnetInputs`] — input-side vocabulary. Three precomputed,
//!   comma-joined strings: one for the raw extern decl, one for the
//!   idiomatic method decl (no self), one for the raw call args.
//! * [`MethodInfo`] — one method's render data; consumed by every template.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display},
};

use diplomat_core::hir::{
    self,
    borrowing_param::{BorrowedLifetimeInfo, LifetimeEdgeKind, ParamBorrowInfo},
    MaybeOwn, Method, SpecialMethod,
};

use crate::dotnet::r#gen::fillable::{
    DotnetErrorType, DotnetOption, DotnetResult, ErrorInfo, OptionInfo,
};

use super::accessor::{AccessorInfo, AccessorKind, AccessorMarshal, AccessorValue};
use super::disposal::{BorrowMode, BorrowSource, ReturnArm};
use super::{callback::DotnetCallback, DotnetPrimitives, ItemGenContext};

#[derive(Debug, Clone)]
pub(crate) struct RawExprParseError {
    value: String,
}

impl Display for RawExprParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported .NET raw expression: {}", self.value)
    }
}

pub(crate) fn dependencies_array_expr(sources: &[String]) -> String {
    if sources.is_empty() {
        "System.Array.Empty<object>()".to_string()
    } else {
        format!("new object[] {{ {} }}", sources.join(", "))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawExpr {
    Result,
    ResultOk,
    ResultErr,
    ResultValue,
    ResultOkValue,
}

impl RawExpr {
    fn option_value(self) -> Self {
        match self {
            Self::Result => Self::ResultValue,
            Self::ResultOk => Self::ResultOkValue,
            other => panic!("{other} cannot be read as an option value"),
        }
    }

    fn is_some_expr(self) -> String {
        format!("{self}.IsSome")
    }
}

impl TryFrom<&str> for RawExpr {
    type Error = RawExprParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "result" => Ok(Self::Result),
            "result.Ok" => Ok(Self::ResultOk),
            "result.Err" => Ok(Self::ResultErr),
            "result.Value" => Ok(Self::ResultValue),
            "result.Ok.Value" => Ok(Self::ResultOkValue),
            _ => Err(RawExprParseError {
                value: value.to_string(),
            }),
        }
    }
}

impl Display for RawExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Result => write!(f, "result"),
            Self::ResultOk => write!(f, "result.Ok"),
            Self::ResultErr => write!(f, "result.Err"),
            Self::ResultValue => write!(f, "result.Value"),
            Self::ResultOkValue => write!(f, "result.Ok.Value"),
        }
    }
}

/// Context from the type-level codegen call site that a bare HIR method
/// does not carry by itself.
///
/// This matters for callback helper naming: `hir::Type::Callback` describes
/// the callback signature, but not the owner type whose method contains it.
#[derive(Clone, Copy)]
pub(super) struct StructMethodContext<'ctx> {
    method: &'ctx Method,
}

impl<'ctx> StructMethodContext<'ctx> {
    pub(super) fn new(method: &'ctx Method) -> Self {
        Self { method }
    }

    pub(super) fn method(&self) -> &'ctx Method {
        self.method
    }

    pub(super) fn method_abi_name(&self) -> &str {
        self.method.abi_name.as_str()
    }

    /// A setter renders as a property accessor, whose incoming value is always
    /// the implicit `value` — the Rust parameter name is not in scope there, and
    /// a parameter inside a property has to marshal as the property's type.
    pub(super) fn is_setter(&self) -> bool {
        matches!(
            self.method.attrs.special_method,
            Some(SpecialMethod::Setter(_))
        )
    }
}

pub(super) struct MethodInputContext<'ctx> {
    method: StructMethodContext<'ctx>,
    param: &'ctx hir::Param,
    param_index: usize,
    raw_name: String,
    local_name: String,
}

impl<'ctx> MethodInputContext<'ctx> {
    fn new(
        method: StructMethodContext<'ctx>,
        param_index: usize,
        param: &'ctx hir::Param,
        raw_name: String,
        local_name: String,
    ) -> Self {
        Self {
            method,
            param,
            param_index,
            raw_name,
            local_name,
        }
    }

    pub(super) fn method(&self) -> StructMethodContext<'ctx> {
        self.method
    }

    fn param(&self) -> &'ctx hir::Param {
        self.param
    }

    pub(super) fn param_index(&self) -> usize {
        self.param_index
    }

    /// The parameter's name in the P/Invoke declaration: always the Rust one,
    /// escaped. The raw layer is a plain method however the idiomatic layer
    /// presents it, so a setter's `value` alias must not reach it.
    pub(super) fn raw_name(&self) -> &str {
        &self.raw_name
    }

    /// The value's name inside the idiomatic body. A C# property setter receives
    /// its argument as the implicit `value`, and the Rust parameter name is not
    /// in scope there, so a setter aliases to `value` and everything the body
    /// emits — the parameter list, the call argument, the checks — follows it.
    pub(super) fn local_name(&self) -> &str {
        &self.local_name
    }

    /// Base identifier for body-local names (`{base}Ptr`, `{base}Bytes`), which
    /// tracks `local_name` for the same reason, minus the escaping: a local
    /// derived from a param named `this` is `thisPtr`, not `@thisPtr`.
    pub(super) fn local_base(&self) -> &str {
        if self.method.is_setter() {
            "value"
        } else {
            self.rust_ident()
        }
    }

    /// The parameter's Rust identifier, unescaped and never aliased — for names
    /// that must stay stable no matter how the accessor is presented, like the
    /// generated callback helper type.
    pub(super) fn rust_ident(&self) -> &str {
        self.param.name.as_str()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Return type
// ─────────────────────────────────────────────────────────────────────────────

/// Who frees the pointer behind a returned opaque wrapper. An owned return
/// (`Box<T>`) is the C# side's to free; a borrowed return (`&T`) belongs to
/// Rust, so the wrapper must never free it. We carry this as a named pair
/// rather than a bare `bool` so the construction site reads as what it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ownership {
    Owned,
    Borrowed(hir::Mutability),
}

impl Ownership {
    fn is_owned(self) -> bool {
        matches!(self, Self::Owned)
    }

    fn is_borrowed_shared(self) -> bool {
        matches!(self, Self::Borrowed(hir::Mutability::Immutable))
    }

    fn is_borrowed_mutable(self) -> bool {
        matches!(self, Self::Borrowed(hir::Mutability::Mutable))
    }
}

/// Element type carried by a borrowed slice/string return
/// (`DiplomatBorrowedSpan<T>`). Distinct from [`DotnetPrimitives`] because
/// `char` (a UTF-16 code unit) isn't a Rust primitive type being lowered —
/// it's the C# wire element type a string encoding picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BorrowedSpanElement {
    Byte,
    UInt32,
    Char,
}

impl BorrowedSpanElement {
    pub(super) fn element_type(self) -> &'static str {
        match self {
            Self::Byte => "byte",
            Self::UInt32 => "uint",
            Self::Char => "char",
        }
    }

    /// The raw wire struct this element type is pinned onto — same structs
    /// used for input slices (`DiplomatSliceU8`/`DiplomatSliceU32`/`DiplomatSliceU16`).
    pub(super) fn wire_struct(self) -> &'static str {
        match self {
            Self::Byte => "DiplomatSliceU8",
            Self::UInt32 => "DiplomatSliceU32",
            Self::Char => "DiplomatSliceU16",
        }
    }

    /// PascalCase token for embedding in generated *type names*.
    pub(super) fn name_token(self) -> &'static str {
        match self {
            Self::Byte => "Byte",
            Self::UInt32 => "UInt32",
            Self::Char => "Char",
        }
    }
}

/// The return type of a method, expressed once. The variant names the
/// *kind*; [`Display`] writes the bare C# name; templates branch on kind
/// via `is_*` predicates and supply the kind-specific bits (the `*` for
/// opaque in raw externs, the `new T(...)` wrap, the `T.FromFFI(...)` bridge).
#[derive(Debug, Clone)]
pub(crate) enum DotnetReturnType {
    Primitive(DotnetPrimitives),
    /// `Box<T>` / `&T` / `&mut T` opaque return. Carries the bare name
    /// (`"Color"`). Raw externs append `*`; idiomatic wrappers don't.
    Opaque(String),
    /// Returnable struct (by-value). Carries the bare name (`"Point2D"`).
    Struct(String),
    /// Enum return by value. Carries the bare name (`"ShaVariant"`). Same
    /// shape as a primitive at the C ABI (an integer discriminant) —
    /// neither raw extern nor idiomatic surface needs marshalling glue.
    Enum(String),
    /// A borrowed slice/string return (`&'a str` / `&'a DiplomatStr` /
    /// `&'a DiplomatStr16` / `&'a [u8]` / `&'a [u32]`) — a zero-copy view
    /// over Rust-owned memory, wrapped in `DiplomatBorrowedSpan<T>` and
    /// rooted with keep-alive edges the same way a borrowed opaque return is.
    BorrowedSpan(BorrowedSpanElement),
    /// `DiplomatWrite` writer. Not yet emitted; preserves prior behavior.
    Write,
    Unit,
    /// Owned `Box<[u8]>` return. Crosses the raw FFI boundary as the
    /// `DiplomatOwnedSliceU8` `(ptr, len)` struct (returned by value, not
    /// behind a pointer); the idiomatic surface wraps it in `RustVec` for
    /// scoped zero-copy access or an explicit managed clone.
    OwnedByteSlice,
}

impl Display for DotnetReturnType {
    /// Writes the bare C# type name (e.g. `byte`, `Color`, `Point2D`, `void`).
    /// Used directly in idiomatic method signatures and as the prefix in
    /// raw externs (which append `*` for opaque).
    ///
    /// Struct returns inside the `Raw.<Name>` partial-struct resolve to the
    /// enclosing type via C# name lookup — no explicit `Raw.` prefix needed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(p) => write!(f, "{p}"),
            Self::Opaque(name) | Self::Struct(name) | Self::Enum(name) => write!(f, "{name}"),
            Self::BorrowedSpan(elem) => {
                write!(f, "DiplomatBorrowedSpan<{}>", elem.element_type())
            }
            // Both render as the C# keyword `void` in raw externs. The
            // idiomatic signature spells `Write` methods as `string`
            // instead (see `idiomatic_signature_return_type`) — the writer
            // is allocated and consumed internally, so `DiplomatWrite`
            // never appears on the public API.
            Self::Write | Self::Unit => write!(f, "void"),
            // The raw shape (a by-value struct, not a pointer) — same
            // spelling for both `raw()` and the union-field type. The
            // idiomatic signature spells this as `RustVec` instead (see
            // `idiomatic_signature_return_type`).
            Self::OwnedByteSlice => write!(f, "DiplomatOwnedSliceU8"),
        }
    }
}

impl DotnetReturnType {
    /// Bare name + `*` for opaque, bare name otherwise. The "raw FFI
    /// surface" form — what the type looks like at the C ABI boundary.
    ///
    /// Use this where the C# extern declares the return type, where a
    /// union arm declares an opaque-payload field, or where the idiomatic
    /// body binds the raw result (`Raw.{{ x.raw() }} result = ...`).
    /// Templates avoid carrying the inline `{% if is_opaque() %}*{% endif %}`
    /// micro-conditional.
    pub(super) fn raw(&self) -> String {
        match self {
            Self::Opaque(_) => format!("{self}*"),
            // A borrowed span crosses the wire as the same struct-by-value
            // shape used for input slices — not `DiplomatBorrowedSpan<T>`,
            // which only exists on the idiomatic side.
            Self::BorrowedSpan(elem) => elem.wire_struct().to_string(),
            _ => self.to_string(),
        }
    }

    pub(super) fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque(_))
    }

    pub(super) fn is_void(&self) -> bool {
        // Both `Unit` (no return) and `Write` (writer-out-param) have no
        // value in the success arm of a Result struct — neither belongs
        // in the ok union slot. Display is also `void` for both.
        matches!(self, Self::Unit | Self::Write)
    }

    /// True for `DiplomatWrite` returns. The idiomatic wrapper renders
    /// these as `string`-returning methods (auto-allocates writer);
    /// the raw extern declares them as `void` + writer-pointer param.
    pub(super) fn is_write(&self) -> bool {
        matches!(self, Self::Write)
    }

    pub(super) fn is_bool(&self) -> bool {
        matches!(self, Self::Primitive(DotnetPrimitives::Bool))
    }

    /// True for an owned `Box<[u8]>` return. The idiomatic wrapper renders
    /// these as `RustVec`-returning methods; the raw extern returns the
    /// `DiplomatOwnedSliceU8` `(ptr, len)` struct by value.
    pub(super) fn is_owned_byte_slice(&self) -> bool {
        matches!(self, Self::OwnedByteSlice)
    }

    /// C# type stored for this arm inside a result/option
    /// `[StructLayout(LayoutKind.Explicit)]` union. A `bool` is stored as a
    /// blittable `byte`: a `[MarshalAs(U1)] bool` overlapping a pointer arm
    /// at the same `FieldOffset` makes the union non-blittable and can throw
    /// `TypeLoadException` at first use. Everything else uses the raw
    /// spelling. Pairs with [`Self::read_union_field`].
    pub(super) fn union_field_type(&self) -> String {
        if self.is_bool() {
            "byte".to_string()
        } else {
            self.raw()
        }
    }

    /// Read this arm back out of the union field expression `expr` as its
    /// idiomatic value: `bool` arms are stored as `byte` (see
    /// [`Self::union_field_type`]) and converted with `!= 0`; everything else
    /// is the field unchanged.
    pub(super) fn read_union_field(&self, expr: &str) -> String {
        if self.is_bool() {
            format!("{expr} != 0")
        } else {
            expr.to_string()
        }
    }

    /// PascalCase token for embedding in generated *type names* (result /
    /// option helper structs). Distinct from [`Display`], which renders the
    /// C# type and spells `Unit` / `Write` as the lowercase keyword `void` —
    /// fine in a signature, but it yields awkwardly-cased names like
    /// `DiplomatResultvoidUnit` when concatenated into an identifier. Likewise
    /// primitives render lowercase (`int`, `double`) via `Display`, so they go
    /// through [`DotnetPrimitives::name_token`] for a PascalCase spelling.
    pub(super) fn name_token(&self) -> String {
        match self {
            Self::Unit | Self::Write => "Void".to_string(),
            Self::Primitive(p) => p.name_token().to_string(),
            Self::BorrowedSpan(elem) => format!("BorrowedSpan{}", elem.name_token()),
            _ => self.to_string(),
        }
    }

    fn opaque_edges_args(dependencies: &[String], pins: &[String]) -> String {
        let edges: Vec<String> = dependencies
            .iter()
            .cloned()
            .chain(pins.iter().cloned())
            .collect();
        if edges.is_empty() {
            String::new()
        } else {
            format!(", {}", edges.join(", "))
        }
    }

    /// Build the C# expression that wraps a raw opaque pointer. An owned
    /// return uses the owning constructor; a borrowed return uses the
    /// non-owning constructor so the wrapper never frees Rust's pointer. The
    /// caller decides which via [`Ownership`] — no `owned` flag leaks into the
    /// generated arguments.
    fn opaque_construction(
        name: &str,
        raw_expr: &RawExpr,
        dependencies: &[String],
        pins: &[String],
        ownership: Ownership,
    ) -> String {
        let edges = Self::opaque_edges_args(dependencies, pins);
        match ownership {
            Ownership::Owned => format!("new {name}({raw_expr}{edges})"),
            // `new {name}(...)` (not `{name}.Borrowed(...)`) so the type
            // always resolves even when the wrapper has a same-named method.
            Ownership::Borrowed(hir::Mutability::Immutable) => {
                format!("new {name}({raw_expr}, Ownership.SharedView{edges})")
            }
            Ownership::Borrowed(hir::Mutability::Mutable) => {
                format!("new {name}({raw_expr}, Ownership.ExclusiveView{edges})")
            }
        }
    }

    /// Convert a raw FFI value expression into the public C# value
    /// expression for this return type.
    fn idiomatic_value_expr(
        &self,
        raw_expr: RawExpr,
        dependencies: &[String],
        pins: &[String],
        ownership: Ownership,
    ) -> String {
        match self {
            Self::Opaque(name) => {
                Self::opaque_construction(name, &raw_expr, dependencies, pins, ownership)
            }
            Self::Struct(name) => format!("{name}.FromFFI({raw_expr})"),
            // Shared borrowed spans version the source like shared opaque views.
            // Mutation is allowed after the call returns; WithSpan/Clone re-check.
            Self::BorrowedSpan(elem) => {
                debug_assert!(
                    pins.is_empty(),
                    "a borrowed-span return never has pins of its own"
                );
                format!(
                    "new DiplomatBorrowedSpan<{}>({raw_expr}.Ptr, {raw_expr}.Len, {})",
                    elem.element_type(),
                    dependencies_array_expr(dependencies)
                )
            }
            Self::Unit | Self::Write => String::new(),
            Self::Primitive(_) | Self::Enum(_) => raw_expr.to_string(),
            // The raw struct's fields are named to match `DiplomatSliceU8`
            // (`Ptr` / `Len`) — see the constructor in `RustVec.cs.jinja`.
            Self::OwnedByteSlice => format!("new RustVec({raw_expr}.Ptr, {raw_expr}.Len)"),
        }
    }

    fn option_none_expr(&self) -> String {
        match self {
            Self::Opaque(_) => "null".to_string(),
            Self::Unit | Self::Write => unreachable!("unit/write options are rejected earlier"),
            Self::OwnedByteSlice => {
                unreachable!("`Option<Box<[u8]>>` returns are rejected earlier")
            }
            Self::BorrowedSpan(_) => {
                unreachable!("Option-wrapped borrowed-span returns are rejected earlier")
            }
            Self::Primitive(_) | Self::Struct(_) | Self::Enum(_) => format!("({self}?)null"),
        }
    }

    fn tagged_option_expr(
        &self,
        option_expr: RawExpr,
        dependencies: &[String],
        pins: &[String],
        ownership: Ownership,
    ) -> String {
        format!(
            "{} ? {} : {}",
            option_expr.is_some_expr(),
            self.idiomatic_value_expr(option_expr.option_value(), dependencies, pins, ownership),
            self.option_none_expr()
        )
    }

    fn nullable_pointer_option_expr(
        &self,
        raw_expr: RawExpr,
        dependencies: &[String],
        pins: &[String],
        ownership: Ownership,
    ) -> String {
        match self {
            Self::Opaque(name) => {
                let none = self.option_none_expr();
                format!(
                    "{raw_expr} == null ? {none} : {}",
                    Self::opaque_construction(name, &raw_expr, dependencies, pins, ownership)
                )
            }
            _ => unreachable!("nullable pointer options only lower from opaque returns"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Inputs
// ─────────────────────────────────────────────────────────────────────────────

/// A parameter's two C# names. The raw P/Invoke always uses the Rust name; the
/// idiomatic body uses `value` when the method is a property setter.
pub(super) struct ParamNames {
    pub(super) raw: String,
    pub(super) local: String,
}

/// One HIR input (param or self) lowered to the three rendered surfaces.
///
/// Self produces an empty `idiomatic_param` (since `this` is implicit) and
/// uses kind-specific call args (`"selfLease.Ptr"` for opaque, `"this.AsFFI()"` for
/// struct).
#[derive(Debug, Default)]
struct InputLowering {
    /// C# decl for the raw `[DllImport]` extern: `"Color* handle"`, `"byte v"`.
    raw_param: String,
    /// C# decl for the idiomatic wrapper signature: `"Color name"`, `"byte v"`.
    /// Empty for self — `this` is implicit.
    idiomatic_param: String,
    /// Expression passed to the raw call from the idiomatic body:
    /// `"selfLease.Ptr"` / `"this.AsFFI()"` for self, `"nameLease.Ptr"` for an opaque
    /// param, `"v"` for a primitive.
    raw_call_arg: String,
    validation_statement: Option<String>,
    borrow_statement: Option<String>,
    borrow_lease: Option<OpaqueBorrowLease>,

    /// Statements that must run calling into the raw layer — e.g. the DiplomatStr
    fix_statement: Option<String>,

    /// For inputs that need to convert from string to pointer, basically the DiplomatStr only
    to_bytes_statement: Option<String>,

    /// What this param would present as if the method is a property setter:
    /// which marshal was chosen, and whether the value can be null. `None` for a
    /// shape that cannot be a property at all (a callback).
    accessor_value: Option<AccessorValue>,

    /// Wrapper to `GC.KeepAlive` after the raw call — else the GC may free its
    /// pointer mid-call. `Some` for opaque self/params, `None` otherwise.
    keep_alive_target: Option<String>,

    /// `Some` for a `ReadOnlyMemory` param the output borrows — pin statements
    /// and the keep-alive edge both derive from it.
    borrowed_slice_pin: Option<SlicePin>,
}

#[derive(Debug, Clone)]
struct OpaqueBorrowLease {
    source: String,
    lease_expr: String,
}

/// A borrowed slice param pinned for the returned wrapper's lifetime: the
/// C# argument name and the local holding its `DiplomatPinnedMemory`.
#[derive(Debug, Clone)]
pub(super) struct SlicePin {
    arg_name: String,
    pin_local: String,
}

/// Which output arm keep-alive edges are computed for. Only the Ok arm may
/// root pins: a thrown exception has no owner to ever unpin the input buffer.
enum OutputArm<'a> {
    Ok(&'a [SlicePin]),
    Err,
}

impl OutputArm<'_> {
    fn what(&self) -> &'static str {
        match self {
            Self::Ok(_) => "return value",
            Self::Err => "error return",
        }
    }

    fn pin_for(&self, param_name: &str) -> Option<&SlicePin> {
        match self {
            Self::Ok(pins) => pins.iter().find(|pin| pin.arg_name == param_name),
            Self::Err => None,
        }
    }
}

/// All of a method's inputs, joined for template substitution.
///
/// Self's quirks (empty idiomatic decl, kind-specific call arg) are absorbed
/// by the builder — the finished aggregate has no "self is special" surface.
#[derive(Debug, Default, Clone)]
pub(super) struct DotnetInputs {
    /// Raw `[DllImport]` decl: `"Color* handle, byte value"`.
    pub(super) raw_params: String,
    /// Idiomatic wrapper decl (no self): `"byte value"`.
    pub(super) idiomatic_params: String,
    /// Raw call args from the idiomatic body: `"selfLease.Ptr, value"`.
    pub(super) raw_call_args: String,
    pub(super) validation_statements: Vec<String>,
    pub(super) borrow_statements: Vec<String>,
    borrow_leases: Vec<OpaqueBorrowLease>,
    pub(super) fix_statements: Vec<String>,
    pub(super) to_bytes_statements: Vec<String>,
    /// The value a setter assigns, i.e. what its property exposes. `None` for
    /// every method that is not a setter.
    pub(super) setter_value: Option<AccessorValue>,
    /// Keep-alive targets (opaque self + params), in raw-call arg order.
    pub(super) keep_alive_targets: Vec<String>,
    /// Pinned borrowed-slice params, in declaration order.
    pub(super) borrowed_slice_pins: Vec<SlicePin>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Return lowering — one struct out, two fields, no special-case predicates
// ─────────────────────────────────────────────────────────────────────────────

/// One method's lowered return — `return_type` is always present;
/// `error_info` is `Some` iff the Rust side returns `Result<T, E>` with a
/// concrete error type; `option_info` is `Some` iff the success value is
/// wrapped in `Option<T>`. `error_info` and `option_info` are NOT mutually
/// exclusive: `Result<Option<Box<T>>, E>` populates both (see `MethodInfo`).
/// Consumers pattern-match these Option fields directly; no separate
/// is_fallible / is_optional predicates exist.
pub(super) struct ReturnLowering {
    pub(super) return_type: DotnetReturnType,
    pub(super) error_info: Option<ErrorInfo>,
    pub(super) option_info: Option<OptionInfo>,
    pub(super) ownership: Ownership,
    pub(super) opaque_id: Option<hir::OpaqueId>,
    pub(super) error_opaque_id: Option<hir::OpaqueId>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Method info
// ─────────────────────────────────────────────────────────────────────────────

/// Pre-processed view of a single HIR `Method`. Carries both raw-layer and
/// idiomatic-layer render data — templates pick the side they want.
///
/// TODO(dotnet): Model Rust lifetime relationships on the public C# surface.
/// Borrowed inputs are call-scoped. Borrowed opaque returns use non-owning
/// handles and keep-alive edges. An owned opaque success return borrowing a
/// `&[u8]`/`&[u32]` param pins that param via `ReadOnlyMemory` +
/// `DiplomatPinnedMemory` and roots the pin as an edge; borrowed errors and
/// other slice/string borrow positions are still rejected.
#[derive(Clone)]
pub(super) struct MethodInfo<'ctx> {
    /// `extern "C"` symbol name (e.g. `Color_brightness`).
    pub(super) abi_name: &'ctx str,
    /// C# method name (PascalCase, e.g. `Brightness`).
    pub(super) name: String,
    /// `"static "` for static methods, `""` for instance. Renders directly
    /// before the return type in the idiomatic method declaration.
    pub(super) static_kw: &'static str,
    pub(super) inputs: DotnetInputs,
    pub(super) return_type: DotnetReturnType,
    pub(super) lifetime_warning: bool,
    pub(super) required_disposal_borrow: bool,
    /// Direct opaque-param/`this` borrow edges the returned wrapper retains
    /// by taking over the input's `BorrowLease<T>`: the source's
    /// physical Rust destructor is deferred until this dependent (and every
    /// other holder) has released its reference, regardless of which
    /// wrapper's managed lifetime ends first.
    pub(super) keep_alive_sources: Vec<String>,
    /// This return's own pinned input buffers (`&[u8]`/`&[u32]`/
    /// `&DiplomatStr`/`&DiplomatStr16` params rooted as `DiplomatPinnedMemory`)
    /// — unrelated to `keep_alive_sources`: these are threaded straight
    /// into this return's own handle (see `RustHandle.cs.jinja`) and unpinned
    /// right after its own Rust destructor actually runs, never shared with
    /// another wrapper.
    pub(super) keep_alive_pins: Vec<String>,
    /// Same idea as `keep_alive_sources` but for the thrown exception
    /// when the error type carries non-static lifetimes — routed through the
    /// inner error opaque's own RC state (see `DotnetErrorType::exception_inner_expr`)
    /// rather than a separate array on the exception class itself. Pins never
    /// apply to the error arm (an exception has no unpin path), so this is
    /// always a pure dependency list.
    pub(super) error_keep_alive_sources: Vec<String>,
    /// `Borrowed` for a borrowed opaque return — the wrapper is built
    /// non-owning so it never frees a pointer Rust still owns.
    pub(super) ownership: Ownership,
    /// `Some` iff this method returns `Result<T, E>` with a concrete `E`.
    /// Templates branch on `{% if let Some(info) = method.error_info %}` —
    /// no separate `is_fallible()` predicate needed.
    pub(super) error_info: Option<ErrorInfo>,
    /// `Some` iff this method's return is wrapped in `Option<T>`. Templates
    /// branch on `{% if let Some(opt) = method.option_info %}` to render the
    /// nullable C# return type + null/IsSome check.
    ///
    /// NOT mutually exclusive with `error_info`: `Result<Option<Box<T>>, E>`
    /// populates both — the error arm throws, and the success arm is a
    /// nullable pointer. `method_body.cs.jinja` handles that combination
    /// (the `error_info` branch checks `option_info` after `result.IsOk`).
    pub(super) option_info: Option<OptionInfo>,
}

/// The doc comments a member carries. Assembled here rather than in the
/// templates because a property is two methods in one member: it has to union
/// what both accessors would have documented, and say each thing once.
#[derive(Debug, Default, Clone)]
pub(super) struct MemberDocs {
    pub(super) exception_names: Vec<String>,
    /// Set when the member hands back a Rust-allocated opaque.
    pub(super) returns_opaque: Option<String>,
    pub(super) lifetime: Option<LifetimeNote>,
}

/// The lifetime remark, with the pin note nested inside it the same way the
/// template renders them — a pinned input is only ever worth mentioning as part
/// of the borrow story.
#[derive(Debug, Clone)]
pub(super) struct LifetimeNote {
    pub(super) exclusive_return: bool,
    pub(super) versioned_return: bool,
    pub(super) required_disposal_borrow: bool,
    /// Always false for an accessor: pinning needs the *return* to borrow a
    /// slice parameter, and a setter returns unit while a getter takes none.
    pub(super) pinned_inputs: bool,
}

impl MemberDocs {
    pub(super) fn for_method(method: &MethodInfo<'_>) -> Self {
        Self {
            exception_names: method
                .error_info
                .iter()
                .map(|info| info.exception_name.clone())
                .collect(),
            returns_opaque: match &method.return_type {
                DotnetReturnType::Opaque(name) => Some(name.clone()),
                _ => None,
            },
            lifetime: method.lifetime_warning.then(|| LifetimeNote {
                exclusive_return: matches!(method.return_type, DotnetReturnType::Opaque(_))
                    && method.ownership.is_borrowed_mutable(),
                versioned_return: (matches!(method.return_type, DotnetReturnType::Opaque(_))
                    && method.ownership.is_borrowed_shared())
                    || matches!(method.return_type, DotnetReturnType::BorrowedSpan(_)),
                required_disposal_borrow: method.required_disposal_borrow,
                pinned_inputs: method.has_pinned_inputs(),
            }),
        }
    }

    /// Union of both accessors' docs. Two accessors sharing an error type would
    /// otherwise document the same exception twice on one member.
    pub(super) fn for_accessors(
        getter: Option<&MethodInfo<'_>>,
        setter: Option<&MethodInfo<'_>>,
    ) -> Self {
        let mut docs = Self::default();
        for accessor in getter.into_iter().chain(setter) {
            let one = Self::for_method(accessor);
            for name in one.exception_names {
                if !docs.exception_names.contains(&name) {
                    docs.exception_names.push(name);
                }
            }
            docs.returns_opaque = docs.returns_opaque.or(one.returns_opaque);
            if let Some(note) = one.lifetime {
                let lifetime = docs.lifetime.get_or_insert(LifetimeNote {
                    exclusive_return: false,
                    versioned_return: false,
                    required_disposal_borrow: false,
                    pinned_inputs: false,
                });
                lifetime.exclusive_return |= note.exclusive_return;
                lifetime.versioned_return |= note.versioned_return;
                lifetime.required_disposal_borrow |= note.required_disposal_borrow;
                lifetime.pinned_inputs |= note.pinned_inputs;
            }
        }
        docs
    }
}

impl MethodInfo<'_> {
    pub(super) fn docs(&self) -> MemberDocs {
        MemberDocs::for_method(self)
    }

    /// True for methods with a `self` receiver. Drives the disposed-check
    /// emission in `opaque.impl.cs.jinja` (every opaque instance method
    /// must validate `_inner` before calling into the raw layer).
    pub(super) fn is_instance(&self) -> bool {
        self.static_kw.is_empty()
    }

    /// Base indent for the method body — every body line in
    /// `method_body.cs.jinja` is prefixed with this. The canonical
    /// position inside `unsafe { ... }` inside a class method is
    /// 12 spaces; the pin `try { ... }` wrap (pinned slice params) and the
    /// `fixed (...) { ... }` wrap (string / temporary slice params) each
    /// add 4 so the body reads as nested rather than flush with the
    /// wrapping line.
    ///
    /// `extra` is the caller's own nesting, as literal spaces: empty inside a
    /// method, one level inside a property accessor, which sits a block deeper.
    pub(super) fn body_indent(&self, extra: &str) -> String {
        let mut base = match (
            self.inputs.borrowed_slice_pins.is_empty(),
            self.inputs.fix_statements.is_empty(),
        ) {
            (true, true) => 12,
            (false, false) => 20,
            _ => 16,
        };
        if !self.inputs.borrow_statements.is_empty() {
            base += 4;
        }
        format!("{extra}{}", " ".repeat(base))
    }

    /// Indent for the `fixed (...)` lines — one level deeper when they sit
    /// inside the pin `try { ... }` block.
    pub(super) fn fix_indent(&self, extra: &str) -> String {
        let mut base = if self.inputs.borrowed_slice_pins.is_empty() {
            12
        } else {
            16
        };
        if !self.inputs.borrow_statements.is_empty() {
            base += 4;
        }
        format!("{extra}{}", " ".repeat(base))
    }

    pub(super) fn borrow_body_indent(&self, extra: &str) -> String {
        let base = if self.inputs.borrow_statements.is_empty() {
            12
        } else {
            16
        };
        format!("{extra}{}", " ".repeat(base))
    }

    /// True when the convenience write overload would emit a
    /// `public string ToString()` that exactly matches the signature of
    /// `object.ToString()`. The C# compiler warns (CS0114) on
    /// signature-matching hides that aren't explicitly `override` or
    /// `new`; treating it as `override` matches author intent (Rust's
    /// `to_string` is meant to be THE stringifier) and silences the
    /// warning. Treating it as `override` matches author intent (Rust's
    /// `to_string` is meant to be THE stringifier) and silences the warning.
    pub(super) fn is_to_string_override(&self) -> bool {
        self.is_instance() && self.name == "ToString" && self.inputs.idiomatic_params.is_empty()
    }

    /// C# fragment for the idiomatic method signature's return type —
    /// `Color` for plain returns, `Color?` for `Option<T>` returns. The
    /// `?` suffix tells C# 8+ "this might be null" and triggers
    /// `Nullable<T>` for value types.
    pub(super) fn idiomatic_return_type(&self) -> String {
        if self.option_info.is_some() {
            format!("{}?", self.return_type)
        } else {
            self.return_type.to_string()
        }
    }

    /// Idiomatic return type as it appears in the generated method
    /// signature. `Write` methods surface as `string`: the writer is
    /// allocated, filled, and disposed inside the generated body, so the
    /// low-level `DiplomatWrite` is never exposed on the public API.
    /// Everything else defers to [`Self::idiomatic_return_type`].
    pub(super) fn idiomatic_signature_return_type(&self) -> String {
        if self.return_type.is_write() {
            "string".to_string()
        } else if self.return_type.is_owned_byte_slice() {
            "RustVec".to_string()
        } else {
            self.idiomatic_return_type()
        }
    }

    /// Raw `[DllImport]` extern param list. For `DiplomatWrite` returns,
    /// the writer pointer is an implicit trailing parameter not present
    /// in the Rust signature's user-facing params — appended here so the
    /// raw template doesn't have to know about it.
    pub(super) fn raw_params_with_writer(&self) -> String {
        if !self.return_type.is_write() {
            return self.inputs.raw_params.clone();
        }
        if self.inputs.raw_params.is_empty() {
            "DiplomatWrite* writeable".to_string()
        } else {
            format!("{}, DiplomatWrite* writeable", self.inputs.raw_params)
        }
    }

    /// Raw call arg list. Mirror of `raw_params_with_writer`: appends
    /// `&writeable` for `DiplomatWrite` returns so the idiomatic body's
    /// raw-call line doesn't have to special-case it.
    pub(super) fn raw_call_args_with_writer(&self) -> String {
        if !self.return_type.is_write() {
            return self.inputs.raw_call_args.clone();
        }
        if self.inputs.raw_call_args.is_empty() {
            "&writeable".to_string()
        } else {
            format!("{}, &writeable", self.inputs.raw_call_args)
        }
    }

    pub(super) fn raw_call_expr(&self, owner_name: &str) -> String {
        format!(
            "Raw.{owner_name}.{}({})",
            self.name, self.inputs.raw_call_args
        )
    }

    pub(super) fn raw_call_statement(&self, owner_name: &str) -> String {
        format!("{};", self.raw_call_expr(owner_name))
    }

    pub(super) fn direct_raw_return_statement(&self, owner_name: &str) -> String {
        format!("return {};", self.raw_call_expr(owner_name))
    }

    /// True if any opaque pointer crosses the raw call — then a direct
    /// `return Raw...(...)` must capture-then-return so the keep-alive runs.
    pub(super) fn has_keep_alive(&self) -> bool {
        !self.inputs.keep_alive_targets.is_empty()
    }

    /// `GC.KeepAlive(x);` lines to emit after the raw call, one per opaque
    /// wrapper. Empty when none cross the boundary.
    pub(super) fn keep_alive_statements(&self) -> Vec<String> {
        self.inputs
            .keep_alive_targets
            .iter()
            .map(|target| format!("GC.KeepAlive({target});"))
            .collect()
    }

    /// Pin locals are declared nullable before the `try` so the `catch` can
    /// dispose whatever was successfully pinned.
    pub(super) fn pin_declaration_statements(&self) -> Vec<String> {
        self.inputs
            .borrowed_slice_pins
            .iter()
            .map(|pin| format!("DiplomatPinnedMemory? {} = null;", pin.pin_local))
            .collect()
    }

    /// Pinning happens inside the `try` — anything thrown after the first
    /// successful `Pin` funnels through the catch's dispose-and-rethrow.
    pub(super) fn pin_assignment_statements(&self) -> Vec<String> {
        self.inputs
            .borrowed_slice_pins
            .iter()
            .map(|pin| {
                format!(
                    "{} = DiplomatPinnedMemory.Pin({});",
                    pin.pin_local, pin.arg_name
                )
            })
            .collect()
    }

    /// `{pin}?.Dispose();` per pinned borrowed slice, for the `catch` arm —
    /// no wrapper exists yet to own (and later unpin) the holder there.
    pub(super) fn pin_dispose_statements(&self) -> Vec<String> {
        self.inputs
            .borrowed_slice_pins
            .iter()
            .map(|pin| format!("{}?.Dispose();", pin.pin_local))
            .collect()
    }

    pub(super) fn has_pinned_inputs(&self) -> bool {
        !self.inputs.borrowed_slice_pins.is_empty()
    }

    /// True if this method returns a `DiplomatBorrowedSpan<T>` — used to
    /// gate emitting that helper type only when a run actually needs it
    /// (mirrors `has_pinned_inputs` gating `DiplomatPinnedMemory`).
    pub(super) fn returns_borrowed_span(&self) -> bool {
        matches!(self.return_type, DotnetReturnType::BorrowedSpan(_))
    }

    pub(super) fn can_return_raw_call_directly(&self) -> bool {
        // Only shapes whose raw value already IS the public value. A
        // `BorrowedSpan` return does not qualify even though its wire struct
        // needs no `Raw.` prefix: the raw call yields `DiplomatSliceU8`, and
        // the public type is `DiplomatBorrowedSpan<T>` — returning the raw
        // call directly is uncompilable C# (CS0029).
        self.option_info.is_none()
            && matches!(
                self.return_type,
                DotnetReturnType::Primitive(_) | DotnetReturnType::Enum(_)
            )
    }

    pub(super) fn success_result_declaration(&self, owner_name: &str) -> String {
        let raw_call = self.raw_call_expr(owner_name);
        let uses_tagged_option = self
            .option_info
            .as_ref()
            .and_then(|option| option.raw_option_type.as_ref())
            .is_some();
        // `var`, not `Raw.<T>`: primitive/enum returns have no `Raw.` mirror,
        // so capturing the result for the keep-alive case stays valid C#.
        // `DiplomatOwnedSliceU8` and the `BorrowedSpan` wire structs
        // (`DiplomatSliceU8`/U16/U32) are runtime helper types in
        // `{namespace}.Diplomat`, not per-type `Raw.` mirrors, so they take
        // the same `var` path.
        if uses_tagged_option
            || self.can_return_raw_call_directly()
            || self.return_type.is_owned_byte_slice()
            || matches!(self.return_type, DotnetReturnType::BorrowedSpan(_))
        {
            format!("var result = {raw_call};")
        } else {
            format!("Raw.{} result = {raw_call};", self.return_type.raw())
        }
    }

    /// Full public return statement for a raw success expression. This keeps
    /// nullable option, opaque wrapping, and struct bridging out of the C#
    /// control-flow template. `dependencies` are retained directly, right at
    /// this construction site — never through a pre-call transaction.
    pub(super) fn success_return_statement<R>(&self, raw_expr: R) -> String
    where
        R: TryInto<RawExpr>,
        R::Error: Display,
    {
        let raw_expr = raw_expr.try_into().unwrap_or_else(|err| panic!("{err}"));
        let dependencies = self.keep_alive_sources.as_slice();
        let pins = self.keep_alive_pins.as_slice();
        let ownership = self.ownership;

        if let Some(option_info) = &self.option_info {
            let expr = if option_info.raw_option_type.is_some() {
                self.return_type
                    .tagged_option_expr(raw_expr, dependencies, pins, ownership)
            } else {
                self.return_type.nullable_pointer_option_expr(
                    raw_expr,
                    dependencies,
                    pins,
                    ownership,
                )
            };
            return format!("return {expr};");
        }

        if self.return_type.is_void() {
            "return;".to_string()
        } else {
            format!(
                "return {};",
                self.return_type
                    .idiomatic_value_expr(raw_expr, dependencies, pins, ownership)
            )
        }
    }

    /// The `throw new …(result.Err, …);` statement for the error arm, with
    /// `error_keep_alive_sources` retained directly, right at the inner error
    /// opaque's own construction, so its source(s) stay alive for at least as
    /// long as the error opaque itself does.
    pub(super) fn error_throw_statement(&self) -> String {
        let info = self
            .error_info
            .as_ref()
            .expect("error_throw_statement called on a non-fallible method");
        info.throw_statement_with_edges("result.Err", &self.error_keep_alive_sources)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-method builders
// ─────────────────────────────────────────────────────────────────────────────

/// The bits that differ between element types lowered by
/// [`ItemGenContext::lower_immutable_element_slice`] — bundled into one
/// struct so that function stays under clippy's argument-count limit.
/// `mutable_class` is only consulted for the plain (non-borrowed-by-return)
/// case — the borrowed-by-return case always rejects mutable slices before
/// it matters.
struct ImmutableElementShape<'a> {
    element: BorrowedSpanElement,
    ptr_type: &'a str,
    immutable_class: &'a str,
    mutable_class: &'a str,
}

/// `borrowed_output_keep_alive_edges`'s full result: the success return
/// wrapper's two keep-alive groups, and the thrown exception's dependency
/// list.
struct KeepAliveResult {
    ok_dependencies: Vec<String>,
    ok_pins: Vec<SlicePin>,
    err_dependencies: Vec<String>,
}

struct OutputKeepAliveEdges {
    dependencies: Vec<String>,
    pins: Vec<SlicePin>,
}

impl<'ctx, 'tcx> ItemGenContext<'ctx, 'tcx> {
    /// Build a method's render view, plus its accessor role if it has one, or
    /// `None` if the method uses an HIR shape the backend can't lower yet (the
    /// diagnostic is recorded during lowering). Callers `filter_map` over this to
    /// skip unsupported methods.
    ///
    /// The accessor comes back beside the `MethodInfo` rather than inside it: it
    /// says where the method belongs, which is the caller's business, and no
    /// template ever needs it.
    pub(super) fn build_method_info(
        &self,
        method_context: StructMethodContext<'tcx>,
        owner_rust_name: &str,
    ) -> Option<(Option<AccessorInfo>, MethodInfo<'tcx>)> {
        let method = method_context.method();
        // Refine the diagnostic context from `Type` to `Type::method` for
        // anything pushed while lowering this method. Restored on scope exit.
        let method_name = self.formatter.fmt_method_name(method).into_owned();
        let _guard = self.errors.set_context_method((&method.name).into());
        let static_kw = if method.param_self.is_some() {
            ""
        } else {
            "static "
        };
        let ReturnLowering {
            return_type,
            error_info,
            option_info,
            ownership,
            opaque_id,
            error_opaque_id,
        } = self.lower_return(&method.output)?;

        // One visitor pass classifies every param AND yields the borrow map,
        // so the pin<->edge name correlation holds by construction.
        let mut visitor = method.borrowing_param_visitor(self.tcx, false);
        let mut opaque_params: BTreeMap<String, (hir::OpaqueId, BorrowSource, BorrowMode)> =
            BTreeMap::new();
        if let Some(param_self) = method.param_self.as_ref() {
            visitor.visit_param(&param_self.ty.clone().into(), "this");
            if let hir::SelfType::Opaque(path) = &param_self.ty {
                opaque_params.insert(
                    "this".to_string(),
                    (
                        path.tcx_id,
                        BorrowSource::Receiver,
                        param_self.get_mutability().into(),
                    ),
                );
            }
        }
        let param_borrows: Vec<(ParamNames, ParamBorrowInfo<'tcx>)> = method
            .params
            .iter()
            .map(|param| {
                // Format here so a Rust param named `this` becomes `@this` and
                // can't collide with the receiver sentinel "this".
                let raw_name = self
                    .formatter
                    .fmt_param_name(param.name.as_str())
                    .into_owned();
                // Only the idiomatic side is renamed; the raw P/Invoke keeps the
                // Rust name.
                let local_name = if method_context.is_setter() {
                    "value".to_string()
                } else {
                    raw_name.clone()
                };
                // Borrow edges are emitted in the idiomatic body, so they follow
                // the name that body uses.
                let borrow_info = visitor.visit_param(&param.ty, &local_name);
                if let hir::Type::Opaque(path) = &param.ty {
                    opaque_params.insert(
                        local_name.clone(),
                        (
                            path.tcx_id,
                            BorrowSource::Parameter(param.name.as_str().to_string()),
                            path.owner.mutability.into(),
                        ),
                    );
                }
                (
                    ParamNames {
                        raw: raw_name,
                        local: local_name,
                    },
                    borrow_info,
                )
            })
            .collect();
        let borrow_map = visitor.borrow_map();

        let inputs = self.lower_inputs(method_context, param_borrows)?;

        // A null result would return before any wrapper is built, leaving the
        // pinned buffer with no owner to unpin it.
        if option_info.is_some() && !inputs.borrowed_slice_pins.is_empty() {
            self.errors.push_error(
                "[.NET backend] Option-wrapped return borrowing from a slice parameter is not \
                 supported: a null result would have no owner to unpin the input buffer."
                    .to_string(),
            );
            return None;
        }

        let accessor = self.accessor_info(method, &return_type, option_info.is_some(), &inputs);
        let KeepAliveResult {
            ok_dependencies,
            ok_pins,
            err_dependencies,
        } = self.borrowed_output_keep_alive_edges(method, &inputs, &borrow_map, ownership)?;
        let keep_alive_pins: Vec<String> =
            ok_pins.iter().map(|pin| pin.pin_local.clone()).collect();
        let resolve_dependencies =
            |dependencies: &[String]| -> Vec<(BorrowSource, hir::OpaqueId, BorrowMode)> {
                dependencies
                    .iter()
                    .map(|name| {
                        let (id, source, borrow_mode) = opaque_params.get(name).expect(
                            "an opaque borrow edge always names the receiver or an opaque parameter registered above",
                        );
                        (source.clone(), *id, *borrow_mode)
                    })
                    .collect()
            };
        let ok_dependency_ids = resolve_dependencies(&ok_dependencies);
        let err_dependency_ids = resolve_dependencies(&err_dependencies);
        let method_path = format!("{owner_rust_name}::{}", method.name.as_str());
        {
            let mut disposal = self.disposal.borrow_mut();
            if let Some(returned) = opaque_id {
                if ownership.is_borrowed_mutable() {
                    disposal.record_mutable_borrow(returned, &method_path, ReturnArm::Success);
                }
                for (source, source_id, borrow_mode) in &ok_dependency_ids {
                    if ownership.is_owned() {
                        disposal.record_owned_borrow(
                            returned,
                            source.clone(),
                            *source_id,
                            *borrow_mode,
                            &method_path,
                            ReturnArm::Success,
                        );
                    }
                    disposal.record_retain(returned, *source_id, &method_path, ReturnArm::Success);
                }
                if ownership.is_owned() {
                    for pin in &ok_pins {
                        disposal.record_pin(
                            returned,
                            &pin.arg_name,
                            &method_path,
                            ReturnArm::Success,
                        );
                    }
                }
            }
            if let Some(returned) = error_opaque_id {
                for (source, source_id, borrow_mode) in &err_dependency_ids {
                    disposal.record_owned_borrow(
                        returned,
                        source.clone(),
                        *source_id,
                        *borrow_mode,
                        &method_path,
                        ReturnArm::Error,
                    );
                    disposal.record_retain(returned, *source_id, &method_path, ReturnArm::Error);
                }
            }
        }
        let required_disposal_borrow = opaque_id.is_some()
            && (ownership.is_borrowed_mutable()
                || (ownership.is_owned() && !ok_dependency_ids.is_empty())
                || ok_dependency_ids.iter().any(|(_, source, _)| {
                    self.tcx.resolve_opaque(*source).attrs.manually_disposable
                }));
        let keep_alive_sources = self.borrow_dependencies(&inputs, ok_dependencies)?;
        let error_keep_alive_sources = self.borrow_dependencies(&inputs, err_dependencies)?;
        let lifetime_warning = !keep_alive_sources.is_empty() || !keep_alive_pins.is_empty();

        // A non-opaque, non-borrowed-span success return drops edges silently
        // in `idiomatic_value_expr` (no struct edge-plumbing yet) — would be
        // a use-after-free.
        if (!keep_alive_sources.is_empty() || !keep_alive_pins.is_empty())
            && !matches!(
                return_type,
                DotnetReturnType::Opaque(_) | DotnetReturnType::BorrowedSpan(_)
            )
        {
            self.errors.push_error(format!(
                "[.NET backend] return value of type `{return_type}` borrows from the receiver \
                 or an opaque parameter; keep-alive edges are only supported for opaque (`Box<T>`) \
                 and borrowed-span (`&str`/`&[T]`) returns today. Return an opaque or borrowed \
                 span, or disable this API for .NET."
            ));
            return None;
        }

        // `DiplomatBorrowedSpan<T>` carries its edges in its own constructor
        // the same way an opaque wrapper does, but wrapping it in
        // `Result`/`Option` hasn't been exercised end-to-end (the result/
        // option helper structs' union-field and none-value plumbing was
        // only ever built for opaque/primitive/struct/enum arms) — reject
        // rather than risk generating broken bridging code.
        if matches!(return_type, DotnetReturnType::BorrowedSpan(_))
            && (option_info.is_some() || error_info.is_some())
        {
            self.errors.push_error(
                "[.NET backend] wrapping a borrowed span return (`&str` / `&[T]`) in \
                 `Result`/`Option` is not yet supported — return it bare, or disable this \
                 API for .NET."
                    .to_string(),
            );
            return None;
        }

        if !error_keep_alive_sources.is_empty() {
            if let Some(error_info) = &error_info {
                if !error_info.error.can_carry_borrow_edges() {
                    self.errors.push_error(format!(
                        "[.NET backend] error value of type `{}` borrows from the receiver or an \
                         opaque parameter; keep-alive edges are only supported for opaque (`Box<E>`) \
                         errors today. Return an opaque error, make the error non-borrowing, or \
                         disable this API for .NET.",
                        error_info.error
                    ));
                    return None;
                }
            }
        }

        Some((
            accessor,
            MethodInfo {
                abi_name: method.abi_name.as_str(),
                name: method_name,
                static_kw,
                inputs,
                return_type,
                lifetime_warning,
                required_disposal_borrow,
                keep_alive_sources,
                keep_alive_pins,
                error_keep_alive_sources,
                ownership,
                error_info,
                option_info,
            },
        ))
    }

    fn borrow_dependencies(
        &self,
        inputs: &DotnetInputs,
        sources: Vec<String>,
    ) -> Option<Vec<String>> {
        sources
            .into_iter()
            .map(|source| {
                inputs
                    .borrow_leases
                    .iter()
                    .find(|lease| lease.source == source)
                    .map(|lease| lease.lease_expr.clone())
                    .or_else(|| {
                        self.errors.push_error(format!(
                            "[.NET backend] no borrow lease was generated for opaque source `{source}`"
                        ));
                        None
                    })
            })
            .collect()
    }

    /// Sometimes a method hands back a value that's really just pointing into
    /// another object instead of owning its own. If the garbage collector frees
    /// that other object too early, the returned value is left pointing at freed
    /// memory. So this figures out which objects we need to keep alive: `"this"`
    /// if it borrows from the receiver, or the parameter's name if it borrows
    /// from a parameter.
    ///
    /// An `OpaqueParam` edge is a real cross-wrapper native dependency —
    /// retained by the returned wrapper's handle
    /// so the source's physical destruction is deferred correctly. A
    /// `&[u8]`/`&[u32]` param the success value borrows contributes its own
    /// pin holder instead — an unrelated, this-wrapper-only concern, never
    /// shared with another wrapper. For borrows we
    /// can't safely handle yet — strings (pinned only while the call runs),
    /// struct lifetimes, or anything the error arm borrows from a slice — it
    /// gives up and returns `None` with an error, instead of generating code
    /// that could crash.
    ///
    /// Returns the success return wrapper's two keep-alive groups, and the
    /// thrown exception's dependency list (pins are structurally impossible
    /// on the error arm — `OutputArm::pin_for` always returns `None` there).
    fn borrowed_output_keep_alive_edges(
        &self,
        method: &'tcx Method,
        inputs: &DotnetInputs,
        borrow_map: &BTreeMap<hir::Lifetime, BorrowedLifetimeInfo<'tcx>>,
        ownership: Ownership,
    ) -> Option<KeepAliveResult> {
        // The Ok value's keep-alive edges ride on the returned wrapper, the Err
        // value's on the thrown exception — so compute each from where that
        // output type borrows, rather than pre-splitting the method's lifetimes.
        let (ok_ty, err_ty) = match &method.output {
            hir::ReturnType::Infallible(s) | hir::ReturnType::Nullable(s) => (s.as_type(), None),
            hir::ReturnType::Fallible(s, e) => (s.as_type(), e.as_ref()),
        };

        // A borrowed return's Dispose never runs Rust's destructor, so
        // unpinning there would free the buffer while Rust still holds it.
        let ok_pins: &[SlicePin] = if ownership.is_owned() {
            &inputs.borrowed_slice_pins
        } else {
            &[]
        };
        let ok_edges = match ok_ty {
            Some(ty) => self.output_keep_alive_edges(ty, borrow_map, OutputArm::Ok(ok_pins))?,
            None => OutputKeepAliveEdges {
                dependencies: Vec::new(),
                pins: Vec::new(),
            },
        };
        let err_dependencies = match err_ty {
            Some(ty) => {
                let err_edges = self.output_keep_alive_edges(ty, borrow_map, OutputArm::Err)?;
                debug_assert!(
                    err_edges.pins.is_empty(),
                    "the error arm can never pin a slice param — OutputArm::pin_for(Err) is \
                     always None"
                );
                err_edges.dependencies
            }
            None => Vec::new(),
        };

        Some(KeepAliveResult {
            ok_dependencies: ok_edges.dependencies,
            ok_pins: ok_edges.pins,
            err_dependencies,
        })
    }

    /// Keep-alive edges contributed by one output type: the receiver / opaque
    /// parameters its non-static lifetimes borrow from, looked up in the
    /// method's borrow map. This mirrors how the other backends derive lifetime
    /// edges while handling an output type (e.g. Kotlin's return conversions),
    /// instead of threading a separate per-arm lifetime set. `arm` names the
    /// arm for diagnostics and says whether pins may be rooted; returns `None`
    /// (diagnostic pushed) for borrow kinds the backend can't keep alive yet.
    ///
    /// Returns `(dependencies, pins)`: `OpaqueParam` edges (real cross-wrapper
    /// native dependencies, destined for the RC mechanism) are kept separate
    /// from `SliceParam` pin holders (this wrapper's own pinned buffer,
    /// unrelated to any other wrapper) because the two have entirely
    /// different release paths on the C# side. Each dependency is the bare
    /// call-site expression naming the source (`"this"` or a parameter's
    /// local name) — no separate id lookup is needed since role
    /// classification no longer exists.
    fn output_keep_alive_edges(
        &self,
        out_ty: &hir::OutType,
        borrow_map: &BTreeMap<hir::Lifetime, BorrowedLifetimeInfo<'tcx>>,
        arm: OutputArm<'_>,
    ) -> Option<OutputKeepAliveEdges> {
        let what = arm.what();
        let lifetimes: BTreeSet<hir::Lifetime> = out_ty
            .lifetimes()
            .filter_map(|lt| match lt {
                hir::MaybeStatic::NonStatic(lt) => Some(lt),
                hir::MaybeStatic::Static => None,
            })
            .collect();

        let mut dependencies: Vec<String> = Vec::new();
        let mut pins: Vec<SlicePin> = Vec::new();
        for (lt, borrow_info) in borrow_map {
            if !lifetimes.contains(lt) {
                continue;
            }
            for edge in &borrow_info.incoming_edges {
                match &edge.kind {
                    LifetimeEdgeKind::OpaqueParam => {
                        if !dependencies.contains(&edge.param_name) {
                            dependencies.push(edge.param_name.clone());
                        }
                    }
                    // A pinned param roots its holder; any other slice/string
                    // param is call-scoped, so a borrowing return would dangle.
                    // Borrowed-span pin ownership is not implemented.
                    LifetimeEdgeKind::SliceParam => match arm.pin_for(&edge.param_name) {
                        Some(pin) => {
                            if !pins.iter().any(|known| known.pin_local == pin.pin_local) {
                                pins.push(pin.clone());
                            }
                        }
                        None => {
                            self.errors.push_error(format!(
                                "[.NET backend] {what} borrows from slice/string parameter \
                                     `{}`; only owned opaque success returns borrowing from \
                                     `&[u8]`/`&[u32]`/`&DiplomatStr`/`&DiplomatStr16` parameters \
                                     are supported — borrowed-span pin ownership is not implemented, \
                                     and other positions still pin only for the duration of the call",
                                edge.param_name
                            ));
                            return None;
                        }
                    },
                    // No struct edge-plumbing yet.
                    LifetimeEdgeKind::StructLifetime(..) => {
                        self.errors.push_error(format!(
                            "[.NET backend] {what} borrows through a struct lifetime; keep-alive \
                             edges for struct-borrowed returns are not yet supported"
                        ));
                        return None;
                    }
                    other => {
                        self.errors.push_error(format!(
                            "[.NET backend] {what} borrow kind not yet supported: {other:?}"
                        ));
                        return None;
                    }
                }
            }
        }
        Some(OutputKeepAliveEdges { dependencies, pins })
    }

    /// The accessor role of a method, if HIR gave it one: which half of which
    /// property it is, and what that half puts on the wire.
    ///
    /// Only *instance* accessors reach here — `static_accessors` is off, so HIR
    /// never marks a static method as one.
    fn accessor_info(
        &self,
        method: &'tcx Method,
        return_type: &DotnetReturnType,
        nullable: bool,
        inputs: &DotnetInputs,
    ) -> Option<AccessorInfo> {
        let param_self = method.param_self.as_ref()?;

        let (kind, name) = match &method.attrs.special_method {
            Some(SpecialMethod::Getter(name)) => (AccessorKind::Getter, name),
            Some(SpecialMethod::Setter(name)) => (AccessorKind::Setter, name),
            _ => return None,
        };
        // A `&mut self` getter is free to change the value it reports, and a C#
        // property gets read more than once — by a debugger watch, a serializer,
        // or just twice in a row. A one-shot `self.x.take()` behind a property
        // drains to null on the second read, so refuse it. Setters keep
        // `&mut self`; assigning is the whole point.
        if kind == AccessorKind::Getter {
            if let hir::SelfType::Opaque(p) = &param_self.ty {
                if matches!(p.owner.mutability, hir::Mutability::Mutable) {
                    self.errors.push_error(format!(
                        "[.NET backend] getter `{}` takes `&mut self`, so reading the property \
                         could change the value — a C# property has to look idempotent. Drop the \
                         `getter` attribute, or take `&self`.",
                        method.name.as_str()
                    ));
                    return None;
                }
            }
        }
        let value = match kind {
            AccessorKind::Getter => {
                AccessorValue::nullable_if(nullable, self.return_marshal(return_type)?)
            }
            // HIR guarantees a setter takes exactly one parameter, so the first
            // one is the value being assigned.
            //
            // A callback is the only parameter shape with no property type, and
            // HIR rejects callbacks for this backend before one reaches us — so
            // this arm has no test: it is the guard for when `callbacks` lands.
            AccessorKind::Setter => inputs.setter_value.clone().or_else(|| {
                self.errors.push_error(
                    "[.NET backend] this setter's parameter cannot be a property type (a \
                     callback can't be assigned through one); drop the `setter` attribute or \
                     disable the method for .NET."
                        .to_string(),
                );
                None
            })?,
        };
        Some(AccessorInfo {
            name: self.formatter.fmt_accessor_name(name.as_deref(), method),
            kind,
            rust_name: method.name.as_str().to_string(),
            value,
        })
    }

    /// How a getter's return value presents as a property.
    fn return_marshal(&self, return_type: &DotnetReturnType) -> Option<AccessorMarshal> {
        Some(match return_type {
            DotnetReturnType::Primitive(p) => AccessorMarshal::Primitive(*p),
            DotnetReturnType::Opaque(name) => AccessorMarshal::Opaque(name.clone()),
            DotnetReturnType::Struct(name) => AccessorMarshal::Struct(name.clone()),
            DotnetReturnType::Enum(name) => AccessorMarshal::Enum(name.clone()),
            DotnetReturnType::BorrowedSpan(elem) => AccessorMarshal::BorrowedSpanReturn(*elem),
            // A `DiplomatWrite` getter is `void` on the wire but `string` to the
            // caller — and `void` is not a legal property type anyway.
            DotnetReturnType::Write => AccessorMarshal::WrittenUtf8,
            DotnetReturnType::OwnedByteSlice => AccessorMarshal::OwnedBytesReturn,
            DotnetReturnType::Unit => {
                self.errors.push_error(
                    "[.NET backend] a getter has to return something, and this one returns \
                     `()`. Drop the `getter` attribute, or return a value."
                        .to_string(),
                );
                return None;
            }
        })
    }

    /// Lower a method's [`hir::ReturnType`] to a [`ReturnLowering`].
    ///
    /// Three sequential bindings, top to bottom — read the whole story without
    /// jumping. No nested cartesian-product matching between Fallible /
    /// Infallible / Nullable and the success type; each HIR variant appears
    /// in exactly one arm.
    pub(super) fn lower_return(&self, output: &hir::ReturnType) -> Option<ReturnLowering> {
        // 1. Decompose the HIR shape into the three orthogonal axes:
        //    success type, optional error, optional "wrap in Option".
        //
        //    Note: Option<Box<T>> hits the Infallible arm too — HIR encodes
        //    nullability on the opaque path itself via `is_optional()`,
        //    not in the ReturnType variant, because the pointer carries
        //    null natively (no wrapper struct needed on the wire).
        // `failed` is `Some(Some(ty))` when the error is a concrete type,
        // `Some(None)` for the unit error `()`, and `None` for infallible.
        // The double-Option distinguishes "no failure path" from "failure
        // path with no payload" — the latter is the `Result<T, ()>` shape.
        let (success, failed, is_nullable_path) = match output {
            hir::ReturnType::Infallible(s) => (s, None, false),
            hir::ReturnType::Fallible(s, Some(err)) => (s, Some(Some(err)), false),
            hir::ReturnType::Fallible(s, None) => (s, Some(None), false),
            // `Option<()>` would lower into a `DiplomatOptionvoid` helper
            // struct with a `void` field and an idiomatic signature of
            // `void?` — both invalid C#. Reject up-front rather than
            // emitting broken code. Same treatment if it ever shows up as
            // a `DiplomatWrite` nullable (`Option<&mut DiplomatWrite>`).
            hir::ReturnType::Nullable(hir::SuccessType::Unit) => {
                self.errors.push_error(
                    "[.NET backend] `Option<()>` return is not supported — \
                     a nullable unit has no representation in C#. Return \
                     `bool` instead, or use `Result<(), E>` if you need a \
                     failure-arm with no payload."
                        .to_string(),
                );
                return None;
            }
            hir::ReturnType::Nullable(hir::SuccessType::Write) => {
                self.errors.push_error(
                    "[.NET backend] `Option<&mut DiplomatWrite>` return is not supported."
                        .to_string(),
                );
                return None;
            }
            hir::ReturnType::Nullable(s) => (s, None, true),
        };

        // 2. Lower the success side. Detect "this opaque return was
        //    originally Option<Box<T>>" via the Optional marker on the
        //    opaque path — this is Path A (pointer-nullable) for Option.
        let mut pointer_nullable = false;
        let mut ownership = Ownership::Owned;
        let mut opaque_id = None;
        let return_type = match success {
            hir::SuccessType::Unit => DotnetReturnType::Unit,
            hir::SuccessType::Write => DotnetReturnType::Write,
            hir::SuccessType::OutType(hir::Type::Primitive(p)) => {
                DotnetReturnType::Primitive(self.lower_primitive(p)?)
            }
            hir::SuccessType::OutType(hir::Type::Opaque(p)) => {
                // Borrowed opaque returns use a non-owning handle — Dispose/finalizer
                // skip Destroy because Rust still owns the pointer.
                ownership = if p.is_owned() {
                    Ownership::Owned
                } else {
                    Ownership::Borrowed(p.owner.mutability())
                };
                if p.is_optional() {
                    pointer_nullable = true;
                }
                opaque_id = Some(p.tcx_id);
                DotnetReturnType::Opaque(self.opaque_name(p))
            }
            hir::SuccessType::OutType(hir::Type::Struct(p)) => {
                DotnetReturnType::Struct(self.returnable_struct_name(p)?)
            }
            hir::SuccessType::OutType(hir::Type::Enum(p)) => {
                DotnetReturnType::Enum(self.enum_name(p))
            }
            hir::SuccessType::OutType(hir::Type::Slice(slice)) => match slice {
                hir::Slice::Str(Some(lifetime), encoding) => {
                    // `'static` string returns have no managed owner to root.
                    // Reject until there's an explicit static-slice design.
                    if matches!(lifetime, hir::MaybeStatic::Static) {
                        self.errors.push_error(
                            "[.NET backend] `'static` string returns are not supported \
                             today — return via `DiplomatWrite`, borrow from an opaque \
                             owner, or disable this API for .NET."
                                .to_string(),
                        );
                        return None;
                    }
                    let elem = match encoding {
                        hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => {
                            BorrowedSpanElement::Byte
                        }
                        hir::StringEncoding::UnvalidatedUtf16 => BorrowedSpanElement::Char,
                        other => {
                            self.errors.push_error(format!(
                                "[.NET backend] borrowed string return encoding not yet \
                                 supported: {other:?}"
                            ));
                            return None;
                        }
                    };
                    // Pin holders must not ride on a borrowed span: unpinning
                    // a caller buffer has to be deterministic, and a versioned
                    // reference is not a pin. Slice-param borrow edges are rejected
                    // later via ownership == Borrowed.
                    ownership = Ownership::Borrowed(hir::Mutability::Immutable);
                    DotnetReturnType::BorrowedSpan(elem)
                }
                // Owned string returns (`Box<str>`) need the separately-decided
                // owned-slice-return design (see DECISIONS.md) — not
                // implemented here, don't re-litigate it.
                hir::Slice::Str(None, _) => {
                    self.errors.push_error(
                        "[.NET backend] owned string return (`Box<str>`) is not yet \
                         supported."
                            .to_string(),
                    );
                    return None;
                }
                hir::Slice::Primitive(MaybeOwn::Borrow(reference), primitive_type) => {
                    if matches!(reference.lifetime, hir::MaybeStatic::Static) {
                        self.errors.push_error(
                            "[.NET backend] `'static` slice returns are not supported \
                             today — borrow from an opaque owner, or disable this API \
                             for .NET."
                                .to_string(),
                        );
                        return None;
                    }
                    let elem = match primitive_type {
                        hir::PrimitiveType::Byte | hir::PrimitiveType::Int(hir::IntType::U8) => {
                            BorrowedSpanElement::Byte
                        }
                        hir::PrimitiveType::Int(hir::IntType::U32) => BorrowedSpanElement::UInt32,
                        other => {
                            self.errors.push_error(format!(
                                "[.NET backend] borrowed slice return element type not yet \
                                 supported: {other:?}"
                            ));
                            return None;
                        }
                    };
                    ownership = Ownership::Borrowed(reference.mutability);
                    DotnetReturnType::BorrowedSpan(elem)
                }
                hir::Slice::Primitive(MaybeOwn::Own, primitive_type) => {
                    if !matches!(
                        primitive_type,
                        hir::PrimitiveType::Byte | hir::PrimitiveType::Int(hir::IntType::U8)
                    ) {
                        self.errors.push_error(format!(
                            "[.NET backend] owned slice return not yet supported for element \
                             type {primitive_type:?}; only owned `u8`/`DiplomatByte` slices \
                             (`Box<[u8]>`) are supported today"
                        ));
                        return None;
                    }
                    // Both rejections are defense in depth — HIR lowering only
                    // accepts an owned byte slice as a plain top-level return
                    // (`!in_result_option`), because the macro leaves a `Result`'s
                    // ok arm as a raw `Box<[u8]>` fat pointer inside
                    // `DiplomatResult` instead of converting it to the repr(C)
                    // `DiplomatOwnedSlice<u8>`, so the union layout would not be
                    // FFI-stable.
                    if is_nullable_path {
                        self.errors.push_error(
                            "[.NET backend] `Option<Box<[u8]>>` return is not supported."
                                .to_string(),
                        );
                        return None;
                    }
                    if failed.is_some() {
                        self.errors.push_error(
                            "[.NET backend] `Result<Box<[u8]>, E>` return is not supported."
                                .to_string(),
                        );
                        return None;
                    }
                    DotnetReturnType::OwnedByteSlice
                }
                other => {
                    self.errors.push_error(format!(
                        "[.NET backend] slice return type not yet supported: {other:?}"
                    ));
                    return None;
                }
            },
            other => {
                self.errors.push_error(format!(
                    "[.NET backend] success return type not yet supported: {other:?}"
                ));
                return None;
            }
        };

        // 3. If there's an error: register the (Ok, Err) pair and build
        //    the ErrorInfo for the throw site. `Result<T, ()>` (unit err)
        //    takes the `Unit` variant — same wire shape (DiplomatResult
        //    struct with a tag byte) but no err payload field, and the
        //    failure arm throws a built-in `InvalidOperationException`
        //    without per-method exception class generation.
        // A borrowed error arm never reaches codegen: `DotnetErrorType::new`
        // below rejects it, so this id only ever names an owned `Box<E>`.
        let error_opaque_id = match failed.as_ref() {
            Some(Some(hir::Type::Opaque(path))) => Some(path.tcx_id),
            _ => None,
        };
        let error_info = match failed {
            Some(err) => {
                let error_type = match err {
                    Some(ty) => DotnetErrorType::new(ty, self)?,
                    None => DotnetErrorType::Unit,
                };
                let exception_name = error_type.exception_name(self.exception_trim_suffix);
                let result = DotnetResult::new(
                    self.namespace.to_string(),
                    return_type.clone(),
                    error_type,
                    exception_name,
                );
                let info = result.error_info();
                self.result_struct_registry
                    .borrow_mut()
                    .insert(result.key(), result);
                Some(info)
            }
            None => None,
        };

        // 4. If the return is wrapped in Option: either Path A (pointer
        //    null carries the None) or Path B (DiplomatOption<T> tagged
        //    struct on the wire). Path B registers a runtime helper struct.
        let option_info = if pointer_nullable {
            Some(OptionInfo::nullable_pointer())
        } else if is_nullable_path {
            let option = DotnetOption::new(self.namespace.to_string(), return_type.clone());
            let info = option.option_info();
            self.option_struct_registry
                .borrow_mut()
                .insert(option.key(), option);
            Some(info)
        } else {
            None
        };

        Some(ReturnLowering {
            return_type,
            error_info,
            option_info,
            ownership,
            opaque_id,
            error_opaque_id,
        })
    }

    /// Lower `param_self` + user `params` into the joined-string surfaces
    /// templates consume. `param_borrows` pairs each param's names with its
    /// borrow classification, both from the caller's single visitor pass. `None`
    /// (with a recorded diagnostic) if any input uses an unsupported shape.
    pub(super) fn lower_inputs(
        &self,
        method_context: StructMethodContext<'tcx>,
        param_borrows: Vec<(ParamNames, ParamBorrowInfo<'tcx>)>,
    ) -> Option<DotnetInputs> {
        let method = method_context.method();
        let mut used_local_names = param_borrows
            .iter()
            .map(|(names, _)| names.local.clone())
            .collect();
        let self_lowering = match method.param_self.as_ref() {
            Some(s) => Some(self.lower_self(s, &mut used_local_names)?),
            None => None,
        };
        let mut param_lowerings: Vec<InputLowering> = Vec::with_capacity(method.params.len());
        for ((index, p), (names, borrow_info)) in
            method.params.iter().enumerate().zip(param_borrows)
        {
            param_lowerings.push(self.lower_input(
                MethodInputContext::new(method_context, index, p, names.raw, names.local),
                borrow_info,
                &mut used_local_names,
            )?);
        }

        let mut raw_params = Vec::new();
        let mut idiomatic_params = Vec::new();
        let mut call_args = Vec::new();
        let mut validation_statements = Vec::new();
        let mut borrow_statements = Vec::new();
        let mut borrow_leases = Vec::new();
        let mut fix_statements = Vec::new();
        let mut to_bytes_statements = Vec::new();
        let mut setter_value = None;
        let mut keep_alive_targets = Vec::new();
        let mut borrowed_slice_pins = Vec::new();

        if let Some(s) = &self_lowering {
            raw_params.push(s.raw_param.as_str());
            call_args.push(s.raw_call_arg.as_str());
            if let Some(target) = &s.keep_alive_target {
                keep_alive_targets.push(target.clone());
            }
            if let Some(statement) = &s.borrow_statement {
                borrow_statements.push(statement.clone());
            }
            if let Some(lease) = &s.borrow_lease {
                borrow_leases.push(lease.clone());
            }
            // self contributes nothing to the idiomatic decl — `this` is implicit.
        }

        for p in &param_lowerings {
            raw_params.push(p.raw_param.as_str());
            idiomatic_params.push(p.idiomatic_param.as_str());
            call_args.push(p.raw_call_arg.as_str());
            // HIR guarantees a setter takes exactly one parameter, so the first
            // one is the value being assigned.
            if method_context.is_setter() && setter_value.is_none() {
                setter_value = p.accessor_value.clone();
            }
            if let Some(validation) = &p.validation_statement {
                // A single validation entry may encode multiple statements
                // separated by `\n` (e.g. a null check + a cached-AsFFI
                // declaration + a disposed check). Split them out so each
                // becomes its own template iteration, picking up the
                // surrounding indent uniformly.
                for line in validation.split('\n') {
                    let line = line.trim();
                    if !line.is_empty() {
                        validation_statements.push(line.to_string());
                    }
                }
            }
            if let Some(fix) = &p.fix_statement {
                fix_statements.push(fix.clone());
            }
            if let Some(statement) = &p.borrow_statement {
                borrow_statements.push(statement.clone());
            }
            if let Some(lease) = &p.borrow_lease {
                borrow_leases.push(lease.clone());
            }
            if let Some(to_bytes) = &p.to_bytes_statement {
                to_bytes_statements.push(to_bytes.clone());
            }
            if let Some(target) = &p.keep_alive_target {
                keep_alive_targets.push(target.clone());
            }
            if let Some(pin) = &p.borrowed_slice_pin {
                borrowed_slice_pins.push(pin.clone());
            }
        }

        Some(DotnetInputs {
            raw_params: raw_params.join(", "),
            idiomatic_params: idiomatic_params.join(", "),
            raw_call_args: call_args.join(", "),
            validation_statements,
            borrow_statements,
            borrow_leases,
            fix_statements,
            to_bytes_statements,
            setter_value,
            keep_alive_targets,
            borrowed_slice_pins,
        })
    }

    fn lower_self(
        &self,
        this: &hir::ParamSelf,
        used_local_names: &mut BTreeSet<String>,
    ) -> Option<InputLowering> {
        Some(match &this.ty {
            hir::SelfType::Opaque(p) => {
                let name = self.opaque_name_borrowed(p);
                let kind = match this.get_mutability() {
                    hir::Mutability::Immutable => "BorrowKind.Shared",
                    hir::Mutability::Mutable => "BorrowKind.Exclusive",
                };
                let lease_var = Self::unique_local_name(used_local_names, "selfLease".to_string());
                InputLowering {
                    raw_param: format!("{name}* handle"),
                    idiomatic_param: String::new(),
                    raw_call_arg: format!("{lease_var}.Ptr"),
                    borrow_statement: Some(format!(
                        "using (BorrowLease<Raw.{name}> {lease_var} = Lease({kind}))"
                    )),
                    borrow_lease: Some(OpaqueBorrowLease {
                        source: "this".into(),
                        lease_expr: lease_var,
                    }),
                    keep_alive_target: Some("this".into()),
                    ..Default::default()
                }
            }
            hir::SelfType::Struct(p) => {
                let name = self.struct_name(p);
                InputLowering {
                    raw_param: format!("{name} self"),
                    idiomatic_param: String::new(),
                    raw_call_arg: "this.AsFFI()".into(),
                    ..Default::default()
                }
            }
            hir::SelfType::Enum(_) => {
                self.errors.push_error(
                    "[.NET backend] enum receiver (`&self` / `&mut self` on an enum) \
                     is not yet supported"
                        .to_string(),
                );
                return None;
            }
            other => {
                self.errors.push_error(format!(
                    "[.NET backend] self type not yet supported: {other:?}"
                ));
                return None;
            }
        })
    }

    /// Derive a C# local-variable name for a slice parameter's pointer / byte
    /// buffer. Built from the *un-escaped* base identifier and then escaped
    /// once, so a keyword param such as `class` yields a valid `classPtr`
    /// rather than `@classPtr` — where the `@` would only have escaped the
    /// original token, leaving the suffixed name parsed as `classPtr` anyway.
    fn slice_local_name(&self, base: &str, suffix: &str) -> String {
        self.formatter
            .fmt_param_name(&format!("{base}{suffix}"))
            .into_owned()
    }

    fn unique_local_name(used: &mut BTreeSet<String>, preferred: String) -> String {
        if used.insert(preferred.clone()) {
            return preferred;
        }

        for suffix in 2.. {
            let candidate = format!("{preferred}{suffix}");
            if used.insert(candidate.clone()) {
                return candidate;
            }
        }

        unreachable!()
    }

    /// Lowers a parameter that's always immutable and always borrowed, and
    /// whose C# idiomatic surface is an array/`ReadOnlyMemory` of some
    /// blittable element type pinned directly onto the wire struct — shared
    /// by `&[u8]`/`&[u32]` primitive slices and by `&DiplomatStr`
    /// (`StringEncoding::UnvalidatedUtf8`), which carries no caller-side
    /// validity contract and so is really just `&[u8]` tagged as
    /// string-like.
    fn lower_immutable_element_slice(
        &self,
        input_context: &MethodInputContext<'tcx>,
        borrow_info: ParamBorrowInfo<'tcx>,
        shape: ImmutableElementShape<'_>,
        mutability: hir::Mutability,
    ) -> Option<InputLowering> {
        let ImmutableElementShape {
            element,
            ptr_type,
            immutable_class,
            mutable_class,
        } = shape;
        let element_type = element.element_type();
        let arg_name = input_context.local_name();
        let raw_name = input_context.raw_name();
        let ptr = self.slice_local_name(input_context.local_base(), "Ptr");

        Some(if matches!(borrow_info, ParamBorrowInfo::BorrowedSlice) {
            if matches!(mutability, hir::Mutability::Mutable) {
                self.errors.push_error(format!(
                    "[.NET backend] mutable slice parameter `{arg_name}` borrowed \
                     by the output is not yet supported; ReadOnlyMemory cannot hand \
                     Rust a mutable view"
                ));
                return None;
            }
            let pin = self.slice_local_name(input_context.local_base(), "Pin");
            InputLowering {
                raw_param: format!("{immutable_class} {raw_name}"),
                idiomatic_param: format!("ReadOnlyMemory<{element_type}> {arg_name}"),
                raw_call_arg: format!(
                    "new {immutable_class} {{ Ptr = ({ptr_type}*){pin}.Pointer, Len = (nuint){arg_name}.Length }}"
                ),
                borrowed_slice_pin: Some(SlicePin {
                    arg_name: arg_name.to_string(),
                    pin_local: pin,
                }),
                accessor_value: Some(AccessorValue::plain(AccessorMarshal::PinnedMemoryParam(
                    element,
                ))),
                ..Default::default()
            }
        } else {
            let slice_class = match mutability {
                hir::Mutability::Mutable => mutable_class,
                hir::Mutability::Immutable => immutable_class,
            };

            InputLowering {
                raw_param: format!("{slice_class} {raw_name}"),
                idiomatic_param: format!("{element_type}[] {arg_name}"),
                raw_call_arg: format!(
                    "new {slice_class} {{ Ptr = {ptr}, Len = (nuint){arg_name}.Length }}"
                ),
                // Non-optional slice param — null array is a contract
                // violation. Without this check the `{arg_name}.Length` in
                // the call arg throws a bare `NullReferenceException` with
                // no parameter name.
                validation_statement: Some(format!(
                    "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
                )),
                fix_statement: Some(format!("fixed ({ptr_type}* {ptr} = {arg_name})")),
                accessor_value: Some(AccessorValue::plain(AccessorMarshal::ManagedArrayParam(
                    element,
                ))),
                ..Default::default()
            }
        })
    }

    fn lower_input(
        &self,
        input_context: MethodInputContext<'tcx>,
        borrow_info: ParamBorrowInfo<'tcx>,
        used_local_names: &mut BTreeSet<String>,
    ) -> Option<InputLowering> {
        let arg_name = input_context.local_name();
        let raw_name = input_context.raw_name();
        // Every text marshal presents as `PropertyType::Text` — `string` — so a
        // `&DiplomatStr` parameter inside a property marshals like `&str` instead
        // of taking its zero-copy `byte[]` shape. See `gen::accessor`.
        let in_accessor = input_context.method().is_setter();
        Some(match &input_context.param().ty {
            hir::Type::Primitive(p) => {
                let primitive = self.lower_primitive(p)?;
                let ty = primitive.to_string();
                let raw_ty = if matches!(primitive, DotnetPrimitives::Bool) {
                    "[MarshalAs(UnmanagedType.U1)] bool".to_string()
                } else {
                    ty.clone()
                };
                InputLowering {
                    raw_param: format!("{raw_ty} {raw_name}"),
                    idiomatic_param: format!("{ty} {arg_name}"),
                    raw_call_arg: arg_name.to_string(),
                    accessor_value: Some(AccessorValue::plain(AccessorMarshal::Primitive(
                        primitive,
                    ))),
                    ..Default::default()
                }
            }
            hir::Type::Opaque(p) => {
                let ty = self.opaque_name_borrowed(p);
                let optional = p.is_optional();
                let kind = match p.owner.mutability {
                    hir::Mutability::Immutable => "BorrowKind.Shared",
                    hir::Mutability::Mutable => "BorrowKind.Exclusive",
                };
                let idiomatic_ty = if optional {
                    format!("{ty}?")
                } else {
                    ty.clone()
                };
                let lease_var = Self::unique_local_name(
                    used_local_names,
                    self.slice_local_name(input_context.local_base(), "Lease"),
                );
                let validation_statement = if optional {
                    None
                } else {
                    Some(format!(
                        "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
                    ))
                };
                let borrow_statement = if optional {
                    format!(
                        "using (BorrowLease<Raw.{ty}>? {lease_var} = {arg_name} == null ? null : {arg_name}.Lease({kind}))"
                    )
                } else {
                    format!("using (BorrowLease<Raw.{ty}> {lease_var} = {arg_name}.Lease({kind}))")
                };
                InputLowering {
                    raw_param: format!("{ty}* {raw_name}"),
                    idiomatic_param: format!("{idiomatic_ty} {arg_name}"),
                    raw_call_arg: if optional {
                        format!("{lease_var} == null ? null : {lease_var}.Ptr")
                    } else {
                        format!("{lease_var}.Ptr")
                    },
                    validation_statement,
                    borrow_statement: Some(borrow_statement),
                    borrow_lease: Some(OpaqueBorrowLease {
                        source: arg_name.to_string(),
                        lease_expr: if optional {
                            format!("{lease_var}!")
                        } else {
                            lease_var
                        },
                    }),
                    accessor_value: Some(AccessorValue::nullable_if(
                        optional,
                        AccessorMarshal::Opaque(ty),
                    )),
                    // Keep the param's wrapper alive across the call.
                    keep_alive_target: Some(arg_name.to_string()),
                    ..Default::default()
                }
            }
            hir::Type::Slice(slice) => match slice {
                hir::Slice::Str(maybe_static, string_encoding) => match maybe_static {
                    Some(lifetime) => match lifetime {
                        hir::MaybeStatic::Static => {
                            self.errors.push_error(
                                "[.NET backend] `&'static str` parameters not yet supported"
                                    .to_string(),
                            );
                            return None;
                        }
                        hir::MaybeStatic::NonStatic(_) => match string_encoding {
                            // `&str` requires the caller to *guarantee*
                            // valid UTF-8 (UB on the Rust side otherwise —
                            // see the doc comment on `StringEncoding::Utf8`),
                            // so this can't be reshaped to a raw `byte[]`
                            // like `UnvalidatedUtf8` below: a careless
                            // caller could then hand Rust invalid UTF-8.
                            // `Encoding.UTF8.GetBytes` on a real C# `string`
                            // is the only thing that can make that
                            // guarantee, so the transcode-copy stays —
                            // routed through the explicitly-named
                            // `Diplomat.Utf8.Clone` instead of inlining the
                            // BCL call, so the allocation is visible.
                            hir::StringEncoding::UnvalidatedUtf8 if !in_accessor => self
                                .lower_immutable_element_slice(
                                    &input_context,
                                    borrow_info,
                                    ImmutableElementShape {
                                        element: BorrowedSpanElement::Byte,
                                        ptr_type: "byte",
                                        immutable_class: "DiplomatSliceU8",
                                        mutable_class: "DiplomatSliceU8",
                                    },
                                    hir::Mutability::Immutable,
                                )?,
                            hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => {
                                let base = input_context.local_base();
                                let ptr = self.slice_local_name(base, "Ptr");
                                let bytes = self.slice_local_name(base, "Bytes");
                                InputLowering {
                                    raw_param: format!("DiplomatSliceU8 {raw_name}"),
                                    idiomatic_param: format!("string {arg_name}"),
                                    raw_call_arg: format!(
                                        "new DiplomatSliceU8 {{ Ptr = {ptr}, Len = (nuint){bytes}.Length }}"
                                    ),
                                    // `&str` is non-optional on the Rust
                                    // side, so a null string is a contract
                                    // violation. Surface `ArgumentNullException`
                                    // naming the actual parameter — without this,
                                    // `Utf8.Clone(null)` throws with its own
                                    // internal param name (`"value"`). The
                                    // template emits validation before to-bytes.
                                    validation_statement: Some(format!(
                                        "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
                                    )),
                                    borrow_statement: None,
                                    borrow_lease: None,
                                    to_bytes_statement: Some(format!(
                                        "byte[] {bytes} = Diplomat.Utf8.Clone({arg_name});"
                                    )),
                                    // FIXME: an empty string yields a zero-length
                                    // `byte[]`, and `fixed` on an empty array binds
                                    // a null pointer — so Rust receives
                                    // `{ Ptr = null, Len = 0 }`. Diplomat's C ABI
                                    // tolerates `(null, 0)` today (it only reads the
                                    // pointer when `Len > 0`), but a strictly-correct
                                    // binding would hand over a non-null dangling
                                    // pointer for the empty case.
                                    fix_statement: Some(format!(
                                        "fixed (byte* {ptr} = {bytes})"
                                    )),
                                    accessor_value: Some(AccessorValue::plain(
                                        match string_encoding {
                                            hir::StringEncoding::Utf8 => {
                                                AccessorMarshal::ValidatedUtf8Param
                                            }
                                            _ => AccessorMarshal::UnvalidatedUtf8Param,
                                        },
                                    )),
                                    keep_alive_target: None,
                                    borrowed_slice_pin: None,
                                }
                            }
                            hir::StringEncoding::UnvalidatedUtf16 => {
                                // A C# `string` is already a flat UTF-16
                                // buffer — `fixed` pins it directly with no
                                // allocation, unlike the UTF-8 arm above
                                // which must transcode first. Bonus: `fixed`
                                // on a C# string (even `""`) always yields a
                                // valid pointer to its null terminator, so
                                // the empty-string dangling-pointer FIXME
                                // above doesn't apply here.
                                let base = input_context.local_base();
                                let ptr = self.slice_local_name(base, "Ptr");

                                if matches!(borrow_info, ParamBorrowInfo::BorrowedSlice) {
                                    let pin = self.slice_local_name(base, "Pin");
                                    InputLowering {
                                        raw_param: format!("DiplomatSliceU16 {raw_name}"),
                                        idiomatic_param: format!(
                                            "ReadOnlyMemory<char> {arg_name}"
                                        ),
                                        raw_call_arg: format!(
                                            "new DiplomatSliceU16 {{ Ptr = (char*){pin}.Pointer, Len = (nuint){arg_name}.Length }}"
                                        ),
                                        borrowed_slice_pin: Some(SlicePin {
                                            arg_name: arg_name.to_string(),
                                            pin_local: pin,
                                        }),
                                        accessor_value: Some(AccessorValue::plain(
                                            AccessorMarshal::PinnedMemoryParam(
                                                BorrowedSpanElement::Char,
                                            ),
                                        )),
                                        ..Default::default()
                                    }
                                } else {
                                    InputLowering {
                                        raw_param: format!("DiplomatSliceU16 {raw_name}"),
                                        idiomatic_param: format!("string {arg_name}"),
                                        raw_call_arg: format!(
                                            "new DiplomatSliceU16 {{ Ptr = {ptr}, Len = (nuint){arg_name}.Length }}"
                                        ),
                                        validation_statement: Some(format!(
                                            "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
                                        )),
                                        fix_statement: Some(format!(
                                            "fixed (char* {ptr} = {arg_name})"
                                        )),
                                        accessor_value: Some(AccessorValue::plain(
                                            AccessorMarshal::Utf16Param,
                                        )),
                                        ..Default::default()
                                    }
                                }
                            }
                            other => {
                                self.errors.push_error(format!(
                                    "[.NET backend] string encoding not yet supported: {other:?}"
                                ));
                                return None;
                            }
                        },
                    },
                    None => {
                        self.errors.push_error(
                            "[.NET backend] `&str` parameter without a tracked lifetime is \
                             not yet supported"
                                .to_string(),
                        );
                        return None;
                    }
                },
                hir::Slice::Primitive(maybe_own, primitive_type) => match primitive_type {
                    hir::PrimitiveType::Byte
                    | hir::PrimitiveType::Int(hir::IntType::U8 | hir::IntType::U32) => {
                        let MaybeOwn::Borrow(borrow) = maybe_own else {
                            self.errors.push_error(format!(
                                "[.NET backend] owned primitive slice not yet supported: \
                                 {primitive_type:?} : {maybe_own:?}"
                            ));
                            return None;
                        };

                        let (element, ptr_type, immutable_class, mutable_class) =
                            match primitive_type {
                                hir::PrimitiveType::Byte
                                | hir::PrimitiveType::Int(hir::IntType::U8) => (
                                    BorrowedSpanElement::Byte,
                                    "byte",
                                    "DiplomatSliceU8",
                                    "DiplomatSliceMutU8",
                                ),
                                hir::PrimitiveType::Int(hir::IntType::U32) => (
                                    BorrowedSpanElement::UInt32,
                                    "uint",
                                    "DiplomatSliceU32",
                                    "DiplomatSliceMutU32",
                                ),
                                _ => unreachable!(),
                            };

                        self.lower_immutable_element_slice(
                            &input_context,
                            borrow_info,
                            ImmutableElementShape {
                                element,
                                ptr_type,
                                immutable_class,
                                mutable_class,
                            },
                            borrow.mutability,
                        )?
                    }
                    hir::PrimitiveType::Int(int_type) => {
                        self.errors.push_error(format!(
                            "[.NET backend] primitive slice not yet supported: \
                             {int_type:?} : {maybe_own:?}"
                        ));
                        return None;
                    }
                    other => {
                        self.errors.push_error(format!(
                            "[.NET backend] primitive slice element type not yet supported: \
                             {other:?} : {maybe_own:?}"
                        ));
                        return None;
                    }
                },
                hir::Slice::Strs(enc) => {
                    self.errors.push_error(format!(
                        "[.NET backend] string-slice parameter (`&[&str]`) not yet supported: \
                         encoding {enc:?}"
                    ));
                    return None;
                }
                hir::Slice::Struct(maybe_own, _) => {
                    self.errors.push_error(format!(
                        "[.NET backend] struct-slice parameter not yet supported: \
                         ownership {maybe_own:?}"
                    ));
                    return None;
                }
                other => {
                    self.errors.push_error(format!(
                        "[.NET backend] slice parameter shape not yet supported: {other:?}"
                    ));
                    return None;
                }
            },
            hir::Type::Callback(callback) => self.lower_callback_input(input_context, callback)?,
            hir::Type::Enum(enum_path) => {
                // Enums cross the FFI boundary by value as their underlying
                // integer discriminant. The raw extern and the idiomatic
                // surface both take the enum type directly; no marshalling
                // glue needed.
                let ty = self.enum_name(enum_path);
                InputLowering {
                    raw_param: format!("{ty} {raw_name}"),
                    idiomatic_param: format!("{ty} {arg_name}"),
                    raw_call_arg: arg_name.to_string(),
                    accessor_value: Some(AccessorValue::plain(AccessorMarshal::Enum(ty))),
                    ..Default::default()
                }
            }
            hir::Type::Struct(struct_path) => {
                // Struct-by-value param: the raw extern takes the
                // `[StructLayout(Sequential)]` mirror (bare name — the
                // extern itself is declared inside `{namespace}.Raw`, so
                // there's no `Raw.` prefix). The idiomatic surface takes
                // the wrapper struct; bridge via `.AsFFI()`, which the
                // struct codegen always emits.
                let ty = self.struct_name(struct_path);
                InputLowering {
                    raw_param: format!("{ty} {raw_name}"),
                    idiomatic_param: format!("{ty} {arg_name}"),
                    raw_call_arg: format!("{arg_name}.AsFFI()"),
                    accessor_value: Some(AccessorValue::plain(AccessorMarshal::Struct(ty))),
                    ..Default::default()
                }
            }
            other => {
                self.errors.push_error(format!(
                    "[.NET backend] method input type not yet supported: {other:?}"
                ));
                return None;
            }
        })
    }

    // -------------------------------------------------------------------
    // Callback lowering — INCOMPLETE WIP
    //
    // The DiplomatCallback wire and the C# delegate plumbing are sketched
    // out (see callback.rs, callback.cs.jinja) but not finished. The
    // backend's `attr_support.callbacks` is set to `false`, so the HIR
    // validator rejects any bridge that uses `impl Fn`/`impl FnMut`
    // callbacks before they reach this code path. The functions below
    // exist for the day we flip that flag — until then they're dead code
    // that compiles but does nothing useful.
    //
    // Tracked in: see `attr_support.callbacks` in `dotnet/mod.rs`.
    // -------------------------------------------------------------------
    fn lower_callback_input(
        &self,
        input_context: MethodInputContext<'tcx>,
        callback: &hir::Callback,
    ) -> Option<InputLowering> {
        let arg_name = input_context.local_name().to_string();
        let return_type = self.lower_callback_return_type(&callback.output)?;
        let mut callback_param_types = Vec::new();
        let mut callback_param_decls = Vec::new();
        let mut callback_param_names = Vec::new();

        for (index, param) in callback.params.iter().enumerate() {
            let param_type = self.lower_callback_param_type(&param.ty)?;
            let param_name = param
                .name
                .as_ref()
                .map(|name| self.formatter.fmt_param_name(name.as_str()).into_owned())
                .unwrap_or_else(|| format!("arg{index}"));

            callback_param_decls.push(format!("{param_type} {param_name}"));
            callback_param_types.push(param_type);
            callback_param_names.push(param_name);
        }

        let mut delegate_args = vec!["IntPtr callbackHandle".to_string()];
        delegate_args.extend(callback_param_decls.iter().cloned());
        let idiomatic_type = callback_idiomatic_type(&callback_param_types, &return_type);
        let callback = DotnetCallback::new(
            self.namespace.to_string(),
            &input_context,
            return_type,
            delegate_args.join(", "),
            callback_param_decls.join(", "),
            callback_param_names.join(", "),
            idiomatic_type.clone(),
        );
        let callback_name = callback.name.clone();
        self.callback_struct_registry
            .borrow_mut()
            .insert(callback_name.clone(), callback);

        Some(InputLowering {
            raw_param: format!("{callback_name} {arg_name}"),
            idiomatic_param: format!("{idiomatic_type} {arg_name}"),
            raw_call_arg: format!("{callback_name}.FromDelegate({arg_name})"),
            validation_statement: Some(format!(
                "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
            )),
            // No `accessor_value`: a delegate has no property marshal, so a
            // setter taking a callback is refused rather than given one.
            ..Default::default()
        })
    }

    fn lower_callback_return_type(
        &self,
        output: &hir::ReturnType<hir::InputOnly>,
    ) -> Option<DotnetReturnType> {
        Some(match output {
            hir::ReturnType::Infallible(hir::SuccessType::Unit) => DotnetReturnType::Unit,
            hir::ReturnType::Infallible(hir::SuccessType::OutType(hir::Type::Primitive(p))) => {
                DotnetReturnType::Primitive(self.lower_primitive(p)?)
            }
            other => {
                self.errors.push_error(format!(
                    "[.NET backend] callback return type not yet supported (WIP): {other:?}"
                ));
                return None;
            }
        })
    }

    fn lower_callback_param_type(&self, ty: &hir::Type<hir::OutputOnly>) -> Option<String> {
        Some(match ty {
            hir::Type::Primitive(p) => self.lower_primitive(p)?.to_string(),
            other => {
                self.errors.push_error(format!(
                    "[.NET backend] callback parameter type not yet supported (WIP): {other:?}"
                ));
                return None;
            }
        })
    }
}

fn callback_idiomatic_type(param_types: &[String], return_type: &DotnetReturnType) -> String {
    if return_type.is_void() {
        if param_types.is_empty() {
            "Action".to_string()
        } else {
            format!("Action<{}>", param_types.join(", "))
        }
    } else {
        let mut types = param_types.to_vec();
        types.push(return_type.to_string());
        format!("Func<{}>", types.join(", "))
    }
}
