//! `Result` / `Option` / exception runtime helpers.
//!
//! These are emitted per concrete type rather than as a generic
//! `DiplomatResult<T, E>` / `DiplomatOption<T>` because C# forbids
//! `[StructLayout(Explicit)]` / `[FieldOffset]` on a type whose layout
//! depends on a generic parameter — and the overlapped-field layout is what
//! matches the Rust `repr(C)` result/option on the wire.
use std::fmt::Display;

use askama::Template;
use diplomat_core::hir::{OutputOnly, Type};

use crate::dotnet::r#gen::{method::DotnetReturnType, DotnetPrimitives, ItemGenContext};

#[derive(Template)]
#[template(path = "dotnet/result.raw.cs.jinja", escape = "none")]
pub(crate) struct DotnetResult {
    pub(crate) namespace: String,
    pub(crate) result_struct_name: DotnetResultName,
    pub(crate) exception_name: String,
    pub(crate) ok_result: DotnetReturnType,
    pub(crate) error: DotnetErrorType,
}

/// Exception class — generated once per unique error type encountered in
/// any `Result<T, E>` return. Catchable by name in user code
/// (`catch (ColorErrorException ex) { ... ex.Inner ... }`).
#[derive(Template)]
#[template(path = "dotnet/exception.cs.jinja", escape = "none")]
pub(crate) struct DotnetException {
    pub(crate) namespace: String,
    pub(crate) error: DotnetErrorType,
    pub(crate) exception_name: String,
    pub(crate) message_method: Option<String>,
}

/// Runtime helper for `Option<value-type>` — tagged struct on the wire,
/// parallel to `DotnetResult` but with unit error (i.e. `IsSome` instead
/// of `IsOk`, no `Err` payload). One emitted per unique inner type.
#[derive(Template)]
#[template(path = "dotnet/option.raw.cs.jinja", escape = "none")]
pub(crate) struct DotnetOption {
    pub(crate) namespace: String,
    pub(crate) option_struct_name: DotnetOptionName,
    pub(crate) inner: DotnetReturnType,
}

#[derive(Debug, Clone)]
pub(crate) struct DotnetOptionName(String);

impl Display for DotnetOptionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl DotnetOption {
    pub(crate) fn new(namespace: String, inner: DotnetReturnType) -> Self {
        Self {
            namespace,
            option_struct_name: DotnetOptionName(format!("DiplomatOption{}", inner.name_token())),
            inner,
        }
    }

    pub(crate) fn key(&self) -> String {
        format!("option:{}", self.inner)
    }

    pub(crate) fn option_info(&self) -> OptionInfo {
        OptionInfo {
            raw_option_type: Some(self.option_struct_name.clone()),
        }
    }
}

/// Carried on `MethodInfo` when the return is `Option<T>`. Templates branch:
/// - `raw_option_type == None` → pointer-nullable (`Option<Box<T>>`). The
///   inner opaque return carries null directly; idiomatic body is
///   `result == null ? null : new T(result)`.
/// - `raw_option_type == Some(name)` → tagged struct (`Option<value-type>`).
///   Idiomatic body is `result.IsSome ? result.Value : (T?)null`.
#[derive(Debug, Clone)]
pub(crate) struct OptionInfo {
    pub(crate) raw_option_type: Option<DotnetOptionName>,
}

impl OptionInfo {
    /// Pointer-nullable Option — no runtime helper struct needed.
    pub(crate) fn nullable_pointer() -> Self {
        Self {
            raw_option_type: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DotnetResultName(String);

impl Display for DotnetResultName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl DotnetResult {
    pub(crate) fn new(
        namespace: String,
        ok_result: DotnetReturnType,
        error: DotnetErrorType,
        exception_name: String,
    ) -> Self {
        Self {
            namespace,
            result_struct_name: DotnetResultName(format!(
                "DiplomatResult{}{}",
                ok_result.name_token(),
                error
            )),
            exception_name,
            ok_result,
            error,
        }
    }

    pub(crate) fn key(&self) -> String {
        // Variant-tagged, mirroring the exception dedup path (see
        // `DotnetErrorType::dedup_key`). Bare `Display` collapses
        // Opaque/Struct/Enum to their name and renders both `Unit` and
        // `Write` as `void`, so two genuinely distinct (Ok, Err) pairs
        // could hash to the same registry slot and silently overwrite one
        // another. `name_token()` (PascalCase, distinguishes `Void`) plus
        // the error's `dedup_key()` keeps distinct pairs distinct.
        //
        // The ok side uses `name_token()` rather than a variant-tagged key on
        // purpose: two distinct ok types sharing one C# name (an opaque `Foo`
        // and a struct `Foo`) is already unreachable — they'd emit a clashing
        // `class Foo` / `struct Foo` and fail to compile upstream — so there's
        // no ok-side collision left for the key to guard against.
        format!("{}|{}", self.ok_result.name_token(), self.error.dedup_key())
    }

    pub(crate) fn error_info(&self) -> ErrorInfo {
        ErrorInfo {
            error: self.error.clone(),
            exception_name: self.exception_name.clone(),
            raw_return_type: self.result_struct_name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DotnetErrorType {
    // Indicates Optional values
    Primitive(DotnetPrimitives),
    Opaque(String),
    Enum(String),
    Struct(String),
    /// `Result<T, ()>` — unit error type. No payload on the wire; the
    /// idiomatic body throws a built-in `InvalidOperationException` on
    /// the failure arm.
    Unit,
}

#[derive(Clone)]
pub(crate) struct ErrorInfo {
    pub(crate) error: DotnetErrorType,
    pub(crate) exception_name: String,
    pub(crate) raw_return_type: DotnetResultName,
}

impl DotnetErrorType {
    pub(crate) fn new(value: &Type<OutputOnly>, ctx: &ItemGenContext) -> Option<Self> {
        Some(match value {
            Type::Primitive(primitive_type) => {
                DotnetErrorType::Primitive(ctx.lower_primitive(primitive_type)?)
            }
            Type::Opaque(opaque_path) => {
                if !opaque_path.is_owned() {
                    ctx.errors.push_error(
                        "[.NET backend] borrowed opaque error (`Result<_, &E>` / \
                         `Result<_, &mut E>` / `Result<_, Option<&E>>`) is not \
                         yet supported — the generated exception wrapper would \
                         double-free the Rust-owned pointer. Return `Box<E>` \
                         instead."
                            .to_string(),
                    );
                    return None;
                }
                let opaque_name = ctx.opaque_name(opaque_path);
                DotnetErrorType::Opaque(opaque_name)
            }
            Type::Enum(enum_path) => {
                let enum_name = ctx.enum_name(enum_path);
                DotnetErrorType::Enum(enum_name)
            }
            Type::Struct(struct_path) => {
                let struct_name = ctx.returnable_struct_name(struct_path)?;
                DotnetErrorType::Struct(struct_name)
            }
            other => {
                ctx.errors
                    .push_error(format!("[.NET backend] unsupported error type: {other:?}"));
                return None;
            }
        })
    }

    pub(crate) fn raw(&self) -> String {
        match self {
            DotnetErrorType::Opaque(name) => format!("{name}*"),
            // Unit err has no payload on the wire; templates gate emission
            // on `is_unit()` so this string is never inserted.
            DotnetErrorType::Unit => String::new(),
            _ => self.to_string(),
        }
    }

    /// C# type stored for this error arm inside a result
    /// `[StructLayout(LayoutKind.Explicit)]` union — `byte` for `bool`,
    /// the raw spelling otherwise. See [`DotnetReturnType::union_field_type`]
    /// for why `bool` can't sit in the union as a `[MarshalAs(U1)] bool`.
    pub(crate) fn union_field_type(&self) -> String {
        if self.is_bool() {
            "byte".to_string()
        } else {
            self.raw()
        }
    }

    /// Read this error arm back out of union field `expr` (`!= 0` for the
    /// `byte`-stored `bool`, the field unchanged otherwise). Pairs with
    /// [`Self::union_field_type`].
    pub(crate) fn read_union_field(&self, expr: &str) -> String {
        if self.is_bool() {
            format!("{expr} != 0")
        } else {
            expr.to_string()
        }
    }

    pub(crate) fn is_opaque(&self) -> bool {
        matches!(self, DotnetErrorType::Opaque(_))
    }

    pub(crate) fn is_struct(&self) -> bool {
        matches!(self, DotnetErrorType::Struct(_))
    }

    /// Stable, variant-aware key for deduplicating exception emission.
    /// `Display` collapses Opaque/Enum/Struct/Primitive to bare names, so
    /// two distinct error types that happen to share a name (e.g. an
    /// opaque `Foo` plus a struct `Foo` reached via `#[diplomat::rename]`)
    /// would dedup incorrectly — this key prepends the variant tag.
    pub(crate) fn dedup_key(&self) -> String {
        match self {
            DotnetErrorType::Primitive(p) => format!("primitive:{p}"),
            DotnetErrorType::Opaque(name) => format!("opaque:{name}"),
            DotnetErrorType::Enum(name) => format!("enum:{name}"),
            DotnetErrorType::Struct(name) => format!("struct:{name}"),
            DotnetErrorType::Unit => "unit".to_string(),
        }
    }

    pub(crate) fn is_bool(&self) -> bool {
        matches!(
            self,
            DotnetErrorType::Primitive(crate::dotnet::gen::DotnetPrimitives::Bool)
        )
    }

    pub(crate) fn is_unit(&self) -> bool {
        matches!(self, DotnetErrorType::Unit)
    }

    pub(crate) fn exception_name(&self, trim_suffix: Option<&str>) -> String {
        // Unit errors map to the built-in BCL exception — no per-method
        // exception class to emit.
        if matches!(self, DotnetErrorType::Unit) {
            return "InvalidOperationException".to_string();
        }
        let mut name = self.to_string();
        if let Some(trim_suffix) = trim_suffix {
            if let Some(trimmed) = name.strip_suffix(trim_suffix) {
                name = trimmed.to_string();
            }
        }
        format!("{name}Exception")
    }
}

impl Display for DotnetErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotnetErrorType::Primitive(p) => write!(f, "{}", p),
            DotnetErrorType::Opaque(name)
            | DotnetErrorType::Enum(name)
            | DotnetErrorType::Struct(name) => write!(f, "{}", name),
            // Unit's name appears in the `{Ok}{Err}` result-struct key
            // (so two methods returning `Result<T1, ()>` and `Result<T2, ()>`
            // map to distinct struct names) but never reaches a generated
            // template, since both result.raw and method_body gate on
            // `is_unit()`.
            DotnetErrorType::Unit => write!(f, "Unit"),
        }
    }
}
