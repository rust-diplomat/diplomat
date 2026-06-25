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
    collections::BTreeMap,
    fmt::{self, Display},
};

use diplomat_core::hir::{self, borrowing_param::LifetimeEdgeKind, MaybeOwn, Method};

use crate::dotnet::r#gen::fillable::{
    DotnetErrorType, DotnetOption, DotnetResult, ErrorInfo, OptionInfo,
};

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
}

pub(super) struct MethodInputContext<'ctx> {
    method: StructMethodContext<'ctx>,
    param: &'ctx hir::Param,
    param_index: usize,
    arg_name: String,
}

impl<'ctx> MethodInputContext<'ctx> {
    fn new(
        method: StructMethodContext<'ctx>,
        param_index: usize,
        param: &'ctx hir::Param,
        arg_name: String,
    ) -> Self {
        Self {
            method,
            param,
            param_index,
            arg_name,
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

    pub(super) fn arg_name(&self) -> &str {
        &self.arg_name
    }

    pub(super) fn param_ident(&self) -> &str {
        self.param.name.as_str()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Return type
// ─────────────────────────────────────────────────────────────────────────────

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
    /// `DiplomatWrite` writer. Not yet emitted; preserves prior behavior.
    Write,
    Unit,
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
            // Both render as the C# keyword `void` in raw externs. The
            // idiomatic signature spells `Write` methods as `string`
            // instead (see `idiomatic_signature_return_type`) — the writer
            // is allocated and consumed internally, so `DiplomatWriteable`
            // never appears on the public API.
            Self::Write | Self::Unit => write!(f, "void"),
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
        if self.is_opaque() {
            format!("{self}*")
        } else {
            self.to_string()
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
            _ => self.to_string(),
        }
    }

    fn edge_arg_suffix(edges: &[String], owned: bool) -> String {
        // Owned with no edges → the 1-arg ctor (which sets `_owned = true`).
        // Anything else must pass `owned` explicitly, so a borrowed return
        // with no edges (e.g. `&'static T`) can't fall back to the owning
        // ctor and double-free.
        if edges.is_empty() && owned {
            String::new()
        } else {
            let array = if edges.is_empty() {
                "System.Array.Empty<object>()".to_string()
            } else {
                format!("new object[] {{ {} }}", edges.join(", "))
            };
            format!(", {array}, owned: {owned}")
        }
    }

    /// Convert a raw FFI value expression into the public C# value
    /// expression for this return type.
    fn idiomatic_value_expr(&self, raw_expr: RawExpr, edges: &[String], owned: bool) -> String {
        match self {
            Self::Opaque(name) => {
                format!("new {name}({raw_expr}{})", Self::edge_arg_suffix(edges, owned))
            }
            Self::Struct(name) => format!("{name}.FromFFI({raw_expr})"),
            Self::Unit | Self::Write => String::new(),
            Self::Primitive(_) | Self::Enum(_) => raw_expr.to_string(),
        }
    }

    fn option_none_expr(&self) -> String {
        match self {
            Self::Opaque(_) => "null".to_string(),
            Self::Unit | Self::Write => unreachable!("unit/write options are rejected earlier"),
            Self::Primitive(_) | Self::Struct(_) | Self::Enum(_) => format!("({self}?)null"),
        }
    }

    fn tagged_option_expr(&self, option_expr: RawExpr, edges: &[String], owned: bool) -> String {
        format!(
            "{} ? {} : {}",
            option_expr.is_some_expr(),
            self.idiomatic_value_expr(option_expr.option_value(), edges, owned),
            self.option_none_expr()
        )
    }

    fn nullable_pointer_option_expr(&self, raw_expr: RawExpr, edges: &[String], owned: bool) -> String {
        match self {
            Self::Opaque(name) => format!(
                "{raw_expr} == null ? null : new {name}({raw_expr}{})",
                Self::edge_arg_suffix(edges, owned)
            ),
            _ => unreachable!("nullable pointer options only lower from opaque returns"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Inputs
// ─────────────────────────────────────────────────────────────────────────────

/// One HIR input (param or self) lowered to the three rendered surfaces.
///
/// Self produces an empty `idiomatic_param` (since `this` is implicit) and
/// uses kind-specific call args (`"_inner"` for opaque, `"this.AsFFI()"` for
/// struct).
#[derive(Debug, Default)]
struct InputLowering {
    /// C# decl for the raw `[DllImport]` extern: `"Color* handle"`, `"byte v"`.
    raw_param: String,
    /// C# decl for the idiomatic wrapper signature: `"Color name"`, `"byte v"`.
    /// Empty for self — `this` is implicit.
    idiomatic_param: String,
    /// Expression passed to the raw call from the idiomatic body:
    /// `"_inner"` / `"this.AsFFI()"` for self, `"name._inner"` for an opaque
    /// param, `"v"` for a primitive.
    raw_call_arg: String,
    validation_statement: Option<String>,

    /// Statements that must run calling into the raw layer — e.g. the DiplomatStr
    fix_statement: Option<String>,

    /// For inputs that need to convert from string to pointer, basically the DiplomatStr only
    to_bytes_statement: Option<String>,
    idiomatic_param_type: Option<String>,

    /// Wrapper to `GC.KeepAlive` after the raw call — else the GC may free its
    /// pointer mid-call. `Some` for opaque self/params, `None` otherwise.
    keep_alive_target: Option<String>,
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
    /// Raw call args from the idiomatic body: `"_inner, value"`.
    pub(super) raw_call_args: String,
    pub(super) validation_statements: Vec<String>,
    pub(super) fix_statements: Vec<String>,
    pub(super) to_bytes_statements: Vec<String>,
    pub(super) first_param_type: Option<String>,
    pub(super) param_count: usize,
    /// Keep-alive targets (opaque self + params), in raw-call arg order.
    pub(super) keep_alive_targets: Vec<String>,
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
    pub(super) owned_return: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Method info
// ─────────────────────────────────────────────────────────────────────────────

/// Pre-processed view of a single HIR `Method`. Carries both raw-layer and
/// idiomatic-layer render data — templates pick the side they want.
///
/// TODO(dotnet): Model Rust lifetime relationships on the public C# surface.
/// Today borrowed inputs are lowered as call-scoped borrows (pinned arrays,
/// temporary slices, opaque handles), but the generated C# does not enforce
/// Rust lifetime edges. Borrowed opaque returns/errors are rejected until the
/// backend has a non-owning wrapper/lifetime strategy. Lifetime-carrying owned
/// returns that borrow from temporary slice/string params are rejected; those
/// that borrow from an opaque wrapper are documented so callers know they must
/// preserve backing storage.
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
    /// Rooted by the returned wrapper so the GC can't free a borrowed-from
    /// parent while the child lives. Cf. `keep_alive_targets` (per-call only).
    pub(super) keep_alive_edges: Vec<String>,
    /// `false` for a borrowed opaque return — the wrapper is built non-owning
    /// so it never frees a pointer Rust still owns.
    pub(super) owned_return: bool,
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
    pub(super) property_accessor: Option<PropertyAccessor>,
}

pub(super) struct PropertyInfo {
    pub(super) name: String,
    pub(super) property_type: String,
    pub(super) getter: Option<String>,
    pub(super) setter: Option<String>,
    pub(super) lifetime_warning: bool,
}

#[derive(Clone, Debug)]
pub(super) enum PropertyAccessorKind {
    Getter,
    Setter,
}

#[derive(Clone, Debug)]
pub(super) struct PropertyAccessor {
    pub(super) name: String,
    pub(super) kind: PropertyAccessorKind,
    pub(super) property_type: String,
}

impl MethodInfo<'_> {
    /// True for methods with a `self` receiver. Drives the disposed-check
    /// emission in `opaque.impl.cs.jinja` (every opaque instance method
    /// must validate `_inner` before calling into the raw layer).
    pub(super) fn is_instance(&self) -> bool {
        self.static_kw.is_empty()
    }

    /// Base indent for the method body — every body line in
    /// `method_body.cs.jinja` is prefixed with this. The canonical
    /// position inside `unsafe { ... }` inside a class method is
    /// 12 spaces; methods with `fixed (...) { ... }` wrapping
    /// (string / slice params) get an extra 4 so the body inside the
    /// fix block reads as nested rather than flush with the `fixed`
    /// declaration line.
    pub(super) fn body_indent(&self) -> &'static str {
        if self.inputs.fix_statements.is_empty() {
            "            "
        } else {
            "                "
        }
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
    /// low-level `DiplomatWriteable` is never exposed on the public API.
    /// Everything else defers to [`Self::idiomatic_return_type`].
    pub(super) fn idiomatic_signature_return_type(&self) -> String {
        if self.return_type.is_write() {
            "string".to_string()
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
            "DiplomatWriteable* writeable".to_string()
        } else {
            format!("{}, DiplomatWriteable* writeable", self.inputs.raw_params)
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

    pub(super) fn can_return_raw_call_directly(&self) -> bool {
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
        if uses_tagged_option || self.can_return_raw_call_directly() {
            format!("var result = {raw_call};")
        } else {
            format!("Raw.{} result = {raw_call};", self.return_type.raw())
        }
    }

    /// Full public return statement for a raw success expression. This keeps
    /// nullable option, opaque wrapping, and struct bridging out of the C#
    /// control-flow template.
    pub(super) fn success_return_statement<R>(&self, raw_expr: R) -> String
    where
        R: TryInto<RawExpr>,
        R::Error: Display,
    {
        let raw_expr = raw_expr.try_into().unwrap_or_else(|err| panic!("{err}"));
        let edges = self.keep_alive_edges.as_slice();
        let owned = self.owned_return;

        if let Some(option_info) = &self.option_info {
            let expr = if option_info.raw_option_type.is_some() {
                self.return_type.tagged_option_expr(raw_expr, edges, owned)
            } else {
                self.return_type
                    .nullable_pointer_option_expr(raw_expr, edges, owned)
            };
            return format!("return {expr};");
        }

        if self.return_type.is_void() {
            "return;".to_string()
        } else {
            format!(
                "return {};",
                self.return_type.idiomatic_value_expr(raw_expr, edges, owned)
            )
        }
    }
}

pub(super) fn collect_properties(methods: &[MethodInfo<'_>]) -> Vec<PropertyInfo> {
    let mut properties = BTreeMap::<String, PropertyInfo>::new();

    for method in methods {
        let Some(accessor) = &method.property_accessor else {
            continue;
        };
        let property = properties
            .entry(accessor.name.clone())
            .or_insert_with(|| PropertyInfo {
                name: accessor.name.clone(),
                property_type: accessor.property_type.clone(),
                getter: None,
                setter: None,
                lifetime_warning: false,
            });

        match accessor.kind {
            PropertyAccessorKind::Getter => {
                property.property_type = accessor.property_type.clone();
                property.getter = Some(method.name.clone());
                property.lifetime_warning |= method.lifetime_warning;
            }
            PropertyAccessorKind::Setter => {
                property.setter = Some(method.name.clone());
            }
        }
    }

    properties
        .into_values()
        .filter(|property| property.getter.is_some())
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-method builders
// ─────────────────────────────────────────────────────────────────────────────

impl<'ctx, 'tcx> ItemGenContext<'ctx, 'tcx> {
    /// Build a method's render view, or `None` if the method uses an HIR
    /// shape the backend can't lower yet (the diagnostic is recorded during
    /// lowering). Callers `filter_map` over this to skip unsupported methods.
    pub(super) fn build_method_info(
        &self,
        method_context: StructMethodContext<'tcx>,
    ) -> Option<MethodInfo<'tcx>> {
        let method = method_context.method();
        // Refine the diagnostic context from `Type` to `Type::method` for
        // anything pushed while lowering this method. Restored on scope exit.
        let method_name = self.formatter.fmt_method_name(method).into_owned();
        let _guard = self.errors.set_context_method(method_name.clone().into());
        let static_kw = if method.param_self.is_some() {
            ""
        } else {
            "static "
        };
        let ReturnLowering {
            return_type,
            error_info,
            option_info,
            owned_return,
        } = self.lower_return(&method.output)?;

        let inputs = self.lower_inputs(method_context)?;
        let return_type_name = if option_info.is_some() {
            format!("{return_type}?")
        } else {
            return_type.to_string()
        };
        let property_accessor =
            self.property_accessor(method, &return_type, &return_type_name, &inputs);
        let keep_alive_edges = self.borrowed_output_keep_alive_edges(method)?;
        let lifetime_warning = !keep_alive_edges.is_empty();

        // A non-opaque return drops edges silently in `idiomatic_value_expr`
        // (no struct edge-plumbing yet) — a use-after-free; reject instead.
        if !keep_alive_edges.is_empty() && !matches!(return_type, DotnetReturnType::Opaque(_)) {
            self.errors.push_error(format!(
                "[.NET backend] return value of type `{return_type}` borrows from the receiver \
                 or an opaque parameter; keep-alive edges are only supported for opaque (`Box<T>`) \
                 returns today. Return an opaque, or disable this API for .NET."
            ));
            return None;
        }

        // Edges aren't threaded into the error/exception arm, so a borrowing
        // error would dangle; reject fallible borrowing returns.
        if !keep_alive_edges.is_empty() && error_info.is_some() {
            self.errors.push_error(
                "[.NET backend] a fallible method returning a borrowing value is not yet \
                 supported — keep-alive edges are not threaded into the error/exception arm. \
                 Make it infallible, return an owned (non-borrowing) value, or disable this \
                 API for .NET."
                    .to_string(),
            );
            return None;
        }

        Some(MethodInfo {
            abi_name: method.abi_name.as_str(),
            name: method_name,
            static_kw,
            inputs,
            return_type,
            lifetime_warning,
            keep_alive_edges,
            owned_return,
            error_info,
            option_info,
            property_accessor,
        })
    }

    /// Opaque type names that some generated method returns *borrowed*
    /// (`&T` / `Option<&T>`). Such a type needs the non-owning constructor
    /// even when it carries no lifetime of its own (the borrow lives on the
    /// `&`, not the type) — so it can't be gated on lifetime params alone.
    pub(crate) fn borrowed_return_targets(&self) -> std::collections::HashSet<String> {
        let mut targets = std::collections::HashSet::new();
        for (_, ty) in self.tcx.all_types() {
            if ty.attrs().disable {
                continue;
            }
            for method in ty.methods() {
                let success = match &method.output {
                    hir::ReturnType::Infallible(s) | hir::ReturnType::Nullable(s) => s,
                    hir::ReturnType::Fallible(s, _) => s,
                };
                if let hir::SuccessType::OutType(hir::Type::Opaque(p)) = success {
                    if !p.is_owned() {
                        targets.insert(self.opaque_name(p));
                    }
                }
            }
        }
        targets
    }

    /// Sometimes a method hands back a value that's really just pointing into
    /// another object instead of owning its own. If the garbage collector frees
    /// that other object too early, the returned value is left pointing at freed
    /// memory. So this figures out which objects we need to keep alive: `"this"`
    /// if it borrows from the receiver, or the parameter's name if it borrows
    /// from a parameter.
    ///
    /// If it's a kind of borrow we can't safely handle yet — from a string or
    /// slice (we only pin those while the call runs), or through a struct — it
    /// gives up and returns `None` with an error, instead of generating code
    /// that could crash.
    fn borrowed_output_keep_alive_edges(&self, method: &'tcx Method) -> Option<Vec<String>> {
        let mut visitor = method.borrowing_param_visitor(self.tcx, false);

        if let Some(param_self) = method.param_self.as_ref() {
            visitor.visit_param(&param_self.ty.clone().into(), "this");
        }

        for param in &method.params {
            // Format here so a Rust param named `this` becomes `@this` and
            // can't collide with the receiver sentinel "this".
            let arg_name = self
                .formatter
                .fmt_param_name(param.name.as_str())
                .into_owned();
            visitor.visit_param(&param.ty, &arg_name);
        }

        let mut edges = Vec::new();

        for edge in visitor
            .borrow_map()
            .into_values()
            .flat_map(|borrow_info| borrow_info.incoming_edges)
        {
            match edge.kind {
                LifetimeEdgeKind::OpaqueParam => {
                    let expr = edge.param_name;
                    if !edges.contains(&expr) {
                        edges.push(expr);
                    }
                }
                // Slice/string params are only pinned for the call, so a
                // borrowing return would dangle.
                LifetimeEdgeKind::SliceParam => {
                    self.errors.push_error(format!(
                        "[.NET backend] return value borrows from slice/string parameter `{}`; \
                         this is not supported because generated C# only pins or converts those \
                         inputs for the duration of the call",
                        edge.param_name
                    ));
                    return None;
                }
                // No struct edge-plumbing yet.
                LifetimeEdgeKind::StructLifetime(..) => {
                    self.errors.push_error(
                        "[.NET backend] return value borrows through a struct lifetime; \
                         keep-alive edges for struct-borrowed returns are not yet supported"
                            .to_string(),
                    );
                    return None;
                }
                other => {
                    self.errors.push_error(format!(
                        "[.NET backend] return value borrow kind not yet supported: {other:?}"
                    ));
                    return None;
                }
            }
        }

        Some(edges)
    }

    fn property_accessor(
        &self,
        method: &'tcx Method,
        return_type: &DotnetReturnType,
        return_type_name: &str,
        inputs: &DotnetInputs,
    ) -> Option<PropertyAccessor> {
        method.param_self.as_ref()?;

        if let Some(prefix) = self.getters_prefix {
            if inputs.param_count == 0 {
                if let Some(name) = method.name.as_str().strip_prefix(prefix) {
                    // Write-returning getters surface as `string PropName`
                    // properties (using the convenience overload), not
                    // `void` — `void` is illegal as a property type and
                    // wouldn't match what callers expect anyway.
                    let property_type = if return_type.is_write() {
                        "string".to_string()
                    } else {
                        return_type_name.to_string()
                    };
                    return Some(PropertyAccessor {
                        name: self.formatter.fmt_property_name(name).into_owned(),
                        kind: PropertyAccessorKind::Getter,
                        property_type,
                    });
                }
            }
        }

        if let Some(prefix) = self.setters_prefix {
            if inputs.param_count == 1 && return_type.is_void() {
                if let (Some(name), Some(param_type)) = (
                    method.name.as_str().strip_prefix(prefix),
                    inputs.first_param_type.clone(),
                ) {
                    return Some(PropertyAccessor {
                        name: self.formatter.fmt_property_name(name).into_owned(),
                        kind: PropertyAccessorKind::Setter,
                        property_type: param_type,
                    });
                }
            }
        }

        None
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
        let mut owned_return = true;
        let return_type = match success {
            hir::SuccessType::Unit => DotnetReturnType::Unit,
            hir::SuccessType::Write => DotnetReturnType::Write,
            hir::SuccessType::OutType(hir::Type::Primitive(p)) => {
                DotnetReturnType::Primitive(self.lower_primitive(p)?)
            }
            hir::SuccessType::OutType(hir::Type::Opaque(p)) => {
                // A borrowed return (`!is_owned`) is constructed non-owning
                // (`owned: false`) so Dispose/finalizer skip Destroy — Rust
                // still owns the pointer; the keep-alive edges hold the
                // borrowed-from owner alive.
                owned_return = p.is_owned();
                if p.is_optional() {
                    pointer_nullable = true;
                }
                DotnetReturnType::Opaque(self.opaque_name(p))
            }
            hir::SuccessType::OutType(hir::Type::Struct(p)) => {
                DotnetReturnType::Struct(self.returnable_struct_name(p)?)
            }
            hir::SuccessType::OutType(hir::Type::Enum(p)) => {
                DotnetReturnType::Enum(self.enum_name(p))
            }
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
            owned_return,
        })
    }

    /// Lower `param_self` + user `params` into the joined-string surfaces
    /// templates consume. `None` (with a recorded diagnostic) if any input
    /// uses an unsupported shape.
    pub(super) fn lower_inputs(
        &self,
        method_context: StructMethodContext<'tcx>,
    ) -> Option<DotnetInputs> {
        let method = method_context.method();
        let self_lowering = match method.param_self.as_ref() {
            Some(s) => Some(self.lower_self(s)?),
            None => None,
        };
        let param_lowerings: Vec<InputLowering> = method
            .params
            .iter()
            .enumerate()
            .map(|(index, p)| {
                let arg_name = self.formatter.fmt_param_name(p.name.as_str()).into_owned();
                self.lower_input(MethodInputContext::new(method_context, index, p, arg_name))
            })
            .collect::<Option<Vec<_>>>()?;

        let mut raw_params = Vec::new();
        let mut idiomatic_params = Vec::new();
        let mut call_args = Vec::new();
        let mut validation_statements = Vec::new();
        let mut fix_statements = Vec::new();
        let mut to_bytes_statements = Vec::new();
        let mut first_param_type = None;
        let mut param_count = 0;
        let mut keep_alive_targets = Vec::new();

        if let Some(s) = &self_lowering {
            raw_params.push(s.raw_param.as_str());
            call_args.push(s.raw_call_arg.as_str());
            if let Some(target) = &s.keep_alive_target {
                keep_alive_targets.push(target.clone());
            }
            // self contributes nothing to the idiomatic decl — `this` is implicit.
        }

        for p in &param_lowerings {
            raw_params.push(p.raw_param.as_str());
            idiomatic_params.push(p.idiomatic_param.as_str());
            call_args.push(p.raw_call_arg.as_str());
            param_count += 1;
            if first_param_type.is_none() {
                first_param_type = p.idiomatic_param_type.clone();
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
            if let Some(to_bytes) = &p.to_bytes_statement {
                to_bytes_statements.push(to_bytes.clone());
            }
            if let Some(target) = &p.keep_alive_target {
                keep_alive_targets.push(target.clone());
            }
        }

        Some(DotnetInputs {
            raw_params: raw_params.join(", "),
            idiomatic_params: idiomatic_params.join(", "),
            raw_call_args: call_args.join(", "),
            validation_statements,
            fix_statements,
            to_bytes_statements,
            first_param_type,
            param_count,
            keep_alive_targets,
        })
    }

    fn lower_self(&self, this: &hir::ParamSelf) -> Option<InputLowering> {
        Some(match &this.ty {
            hir::SelfType::Opaque(p) => {
                let name = self.opaque_name_borrowed(p);
                InputLowering {
                    raw_param: format!("{name}* handle"),
                    idiomatic_param: String::new(),
                    // `GC.KeepAlive(this)` after the call keeps the pointer
                    // alive across it.
                    raw_call_arg: "AsFFI()".into(),
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

    fn lower_input(&self, input_context: MethodInputContext<'tcx>) -> Option<InputLowering> {
        let arg_name = input_context.arg_name();
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
                    raw_param: format!("{raw_ty} {arg_name}"),
                    idiomatic_param: format!("{ty} {arg_name}"),
                    raw_call_arg: arg_name.to_string(),
                    idiomatic_param_type: Some(ty),
                    ..Default::default()
                }
            }
            hir::Type::Opaque(p) => {
                let ty = self.opaque_name_borrowed(p);
                let optional = p.is_optional();
                let idiomatic_ty = if optional {
                    format!("{ty}?")
                } else {
                    ty.clone()
                };
                // Cache `AsFFI()` to a local — calling it once for the
                // null check and again at the call site is wasted. The
                // disposed check then guards against a use-after-Dispose
                // without invoking `AsFFI()` twice.
                // The opaque is non-nullable in the C# signature (`Locale
                // locale`), but the warning is suppressible — a caller
                // compiled without `#nullable enable` can still hand us
                // null. Surface `ArgumentNullException` (vs. the
                // `NullReferenceException` a bare `.AsFFI()` would throw)
                // so the failure mode names the bad argument. Matches
                // the callback-input validation at `lower_callback_input`.
                let raw_var = format!("{arg_name}Raw");
                let validation_statement = if optional {
                    Some(format!(
                        "Raw.{ty}* {raw_var} = {arg_name} == null ? null : {arg_name}.AsFFI();\n\
                         if ({arg_name} != null && {raw_var} == null) throw new ObjectDisposedException(nameof({ty}));"
                    ))
                } else {
                    Some(format!(
                        "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));\n\
                         Raw.{ty}* {raw_var} = {arg_name}.AsFFI();\n\
                         if ({raw_var} == null) throw new ObjectDisposedException(nameof({ty}));"
                    ))
                };
                let raw_call_arg = raw_var;
                InputLowering {
                    raw_param: format!("{ty}* {arg_name}"),
                    idiomatic_param: format!("{idiomatic_ty} {arg_name}"),
                    raw_call_arg,
                    validation_statement,
                    idiomatic_param_type: Some(idiomatic_ty),
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
                        hir::MaybeStatic::NonStatic(_) => {
                            // The marshaller below is hard-coded to UTF-8.
                            // `attr_support.utf16_strings = false` keeps the
                            // HIR validator from sending UTF-16 here, but be
                            // explicit so any future encoding (e.g.
                            // `UnvalidatedUtf8`) doesn't silently slip
                            // through and get encoded incorrectly.
                            match string_encoding {
                                hir::StringEncoding::Utf8
                                | hir::StringEncoding::UnvalidatedUtf8 => {}
                                other => {
                                    self.errors.push_error(format!(
                                        "[.NET backend] string encoding not yet supported: {other:?}"
                                    ));
                                    return None;
                                }
                            }
                            let base = input_context.param_ident();
                            let ptr = self.slice_local_name(base, "Ptr");
                            let bytes = self.slice_local_name(base, "Bytes");
                            InputLowering {
                                raw_param: format!("DiplomatSliceU8 {arg_name}"),
                                idiomatic_param: format!("string {arg_name}"),
                                raw_call_arg: format!(
                                    "new DiplomatSliceU8 {{ Ptr = {ptr}, Len = (nuint){bytes}.Length }}"
                                ),
                                // `&DiplomatStr` is non-optional on the Rust
                                // side, so a null string is a contract
                                // violation. Surface `ArgumentNullException`
                                // naming the actual parameter — without this,
                                // `Encoding.UTF8.GetBytes(null)` throws with
                                // its own internal param name (`"s"`). The
                                // template emits validation before to-bytes.
                                validation_statement: Some(format!(
                                    "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
                                )),
                                to_bytes_statement: Some(format!(
                                    "byte[] {bytes} = System.Text.Encoding.UTF8.GetBytes({arg_name});"
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
                                idiomatic_param_type: Some("string".to_string()),
                                keep_alive_target: None,
                            }
                        }
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
                        let ptr = self.slice_local_name(input_context.param_ident(), "Ptr");
                        let MaybeOwn::Borrow(borrow) = maybe_own else {
                            self.errors.push_error(format!(
                                "[.NET backend] owned primitive slice not yet supported: \
                                 {primitive_type:?} : {maybe_own:?}"
                            ));
                            return None;
                        };

                        let (element_type, ptr_type, immutable_class, mutable_class) =
                            match primitive_type {
                                hir::PrimitiveType::Byte
                                | hir::PrimitiveType::Int(hir::IntType::U8) => {
                                    ("byte", "byte", "DiplomatSliceU8", "DiplomatSliceMutU8")
                                }
                                hir::PrimitiveType::Int(hir::IntType::U32) => {
                                    ("uint", "uint", "DiplomatSliceU32", "DiplomatSliceMutU32")
                                }
                                _ => unreachable!(),
                            };

                        let slice_class = match borrow.mutability {
                            hir::Mutability::Mutable => mutable_class,
                            hir::Mutability::Immutable => immutable_class,
                        };

                        InputLowering {
                            raw_param: format!("{slice_class} {arg_name}"),
                            idiomatic_param: format!("{element_type}[] {arg_name}"),
                            raw_call_arg: format!(
                                "new {slice_class} {{ Ptr = {ptr}, Len = (nuint){arg_name}.Length }}"
                            ),
                            // Non-optional slice param — null array is a
                            // contract violation. Without this check the
                            // `{arg_name}.Length` in the call arg throws a
                            // bare `NullReferenceException` with no
                            // parameter name.
                            validation_statement: Some(format!(
                                "if ({arg_name} == null) throw new ArgumentNullException(nameof({arg_name}));"
                            )),
                            fix_statement: Some(format!("fixed ({ptr_type}* {ptr} = {arg_name})")),
                            to_bytes_statement: None,
                            idiomatic_param_type: Some(format!("{element_type}[]")),
                            keep_alive_target: None,
                        }
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
                    raw_param: format!("{ty} {arg_name}"),
                    idiomatic_param: format!("{ty} {arg_name}"),
                    raw_call_arg: arg_name.to_string(),
                    idiomatic_param_type: Some(ty),
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
                    raw_param: format!("{ty} {arg_name}"),
                    idiomatic_param: format!("{ty} {arg_name}"),
                    raw_call_arg: format!("{arg_name}.AsFFI()"),
                    idiomatic_param_type: Some(ty),
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
        let arg_name = input_context.arg_name().to_string();
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
            idiomatic_param_type: Some(idiomatic_type),
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
