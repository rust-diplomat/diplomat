//! Utilities for generating JS code that convert between JS values and Rust values.
//!
//! When calling Rust from JS, there are two necessary conversions. First, the
//! JS parameters must be converted into values that the WASM ABI understands.
//! Then, the Rust function is invoked via WASM, which may return a Rust value,
//! which then must be converted into a value that JS understands.
//!
//! For converting JS parameter values into a form that the WASM ABI understands,
//! the [`gen_value_js_to_rust`] function is used. It generates the setup and
//! tear down JS code for calling a function. This function will probably be
//! rewritten into an [`fmt::Display`] type in the future.
//!
//! For converting the returned Rust value into a form that JS understand,
//! the [`InvocationIntoJs`] type used. It's an [`fmt::Display`] type that, when
//! `Display`ed, generates the WASM invocation and the conversion code that
//! turns the returned Rust value into something that JS understand.
//!
//! Return types like non-opaque structs and `Result`s with a non-unit value are
//! returned into a pre-allocated buffer, which [`InvocationIntoJs`] manages.
//! In order to get JS values out of this buffer, [`UnderlyingIntoJs`] is used.
use diplomat_core::{ast, Env};
use displaydoc::Display;
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::num::NonZeroUsize;

use super::display;
use super::types::{return_type_form, ReturnTypeForm};
use crate::layout;

/// An [`fmt::Display`] type representing a disambiguated binding to a native JS object.
///
/// Normally, this type just represents method parameters and `this`. However,
/// structs have to be unpacked before passing into WASM functions, and so
/// unpacked fields are represented through the recursive `UnpackedBinding::Field`
/// variant, which allows for unpacking fields of arbitrarily nested structs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Display, Debug)]
pub enum UnpackedBinding<'env> {
    /// A method parameter.
    #[displaydoc("arg_{0}")]
    MethodParam(&'env ast::Ident),

    /// A field extracted from a struct.
    #[displaydoc("field_{field}_{value}")]
    Field {
        field: &'env ast::Ident,
        value: Box<Self>,
    },

    /// The `this` binding.
    #[displaydoc("this")]
    This,
}

/// An [`fmt::Display`] type representing an argument for a constructor or
/// WASM function.
#[derive(Clone, Display, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Argument<'env> {
    /// An unpacked binding.
    #[displaydoc("{0}")]
    UnpackedBinding(UnpackedBinding<'env>),

    /// A new binding to a `diplomatRuntime.DiplomatBuf` that will hold the
    /// contents of a binding to a JS `string`.
    #[displaydoc("buf_{0}")]
    DiplomatBuf(UnpackedBinding<'env>),

    /// A binding to received edge arguments.
    #[displaydoc("{0}")]
    ReceivedEdges(ReceivedEdges<'env>),
}

/// An [`fmt::Display`] type for disambiguating names of lifetime edges in
/// constructor arguments.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReceivedEdges<'env>(pub &'env ast::NamedLifetime);

impl fmt::Display for ReceivedEdges<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "edges_{}", self.0.name())
    }
}

/// An [`fmt::Display`] type representing an [`Argument`] that is passed
/// into a constructor as an element of an array of lifetime edges.
///
/// When we pass lifetime edges to a created object, we pass them in the form of
/// an array. To keep the array flattened, we can use the spread operator on
/// edge arrays passed in from the caller.
pub struct ArgumentLifetimeEdge<'env>(&'env Argument<'env>);

impl fmt::Display for ArgumentLifetimeEdge<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Argument::ReceivedEdges(edges) => write!(f, "...{edges}"),
            binding => binding.fmt(f),
        }
    }
}

/// Generate the necessary setup and tear down JS code to convert the parameters
/// into a form that Rust/WASM can understand.
#[allow(clippy::ptr_arg, clippy::too_many_arguments)] // false positive, rust-clippy#8463, fixed in 1.61
pub fn gen_value_js_to_rust<'env>(
    param_name: UnpackedBinding<'env>,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &'env Env,
    pre_logic: &mut Vec<String>,
    invocation_params: &mut Vec<String>,
    post_logic: &mut Vec<String>,
    lifetime_env: &ast::LifetimeEnv,
    borrowed_current_to_root: &BTreeMap<&ast::NamedLifetime, &'env ast::NamedLifetime>,
    entries: &mut BTreeMap<&'env ast::NamedLifetime, Vec<Argument<'env>>>,
) {
    match typ {
        ast::TypeName::StrReference(lifetime, ..) | ast::TypeName::PrimitiveSlice(lifetime, ..) => {
            let param_name_buf = Argument::DiplomatBuf(param_name.clone());
            // TODO: turn `gen_value_js_to_rust` into a struct and add a
            // `display_slice` method so we can use the `SliceKind` type here to
            // clean this up.
            if let ast::TypeName::PrimitiveSlice(.., prim) = typ {
                pre_logic.push(format!(
                    "const {param_name_buf} = diplomatRuntime.DiplomatBuf.slice(wasm, {param_name}, {rust_type:?});",
                    rust_type = prim.to_string(),
                ));
            } else if let ast::TypeName::StrReference(_, encoding) = typ {
                pre_logic.push(format!(
                    "const {param_name_buf} = diplomatRuntime.DiplomatBuf.{}(wasm, {param_name});",
                    match encoding {
                        ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8 => "str8",
                        ast::StringEncoding::UnvalidatedUtf16 => "str16",
                        _ => unreachable!("unknown AST/HIR variant"),
                    }
                ));
            } else {
                unreachable!("unknown AST/HIR variant");
            }

            invocation_params.push(format!("{param_name_buf}.ptr"));
            invocation_params.push(format!("{param_name_buf}.size"));

            if let Some(named) = lifetime
                .as_named()
                .and_then(|current| borrowed_current_to_root.get(current))
            {
                post_logic.push(format!("{param_name_buf}.garbageCollect();"));
                entries.entry(named).or_default().push(param_name_buf);
            } else if lifetime == &ast::Lifetime::Static {
                post_logic.push(format!("{param_name_buf}.leak();"));
            } else {
                post_logic.push(format!("{param_name_buf}.free();"));
            }
        }
        ast::TypeName::Primitive(ast::PrimitiveType::char) => {
            // we use the spread operator here to count codepoints
            // codePointAt() does not return surrogate pairs if there are multiple
            invocation_params.push(format!(
                "diplomatRuntime.extractCodePoint({param_name}, '{param_name}')"
            ));
        }
        ast::TypeName::Box(..) | ast::TypeName::Reference(..) => {
            invocation_params.push(format!("{param_name}.underlying"));

            let binding = Argument::UnpackedBinding(param_name);

            for current in typ.shorter_lifetimes(lifetime_env) {
                if let Some(root) = borrowed_current_to_root.get(current) {
                    entries.entry(root).or_default().push(binding.clone());
                }
            }
        }
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            match path_type.resolve(in_path, env) {
                ast::CustomType::Struct(struct_type) => {
                    let borrowed_current_to_root = path_type
                        .lifetimes
                        .iter()
                        .zip(struct_type.lifetimes.names())
                        .filter_map(|(current, inner)| {
                            current
                                .as_named()
                                .and_then(|current| borrowed_current_to_root.get(current))
                                .map(|&root| (inner, root))
                        })
                        .collect();

                    for (field_name, field_type, _) in struct_type.fields.iter() {
                        let field_extracted_name = UnpackedBinding::Field {
                            field: field_name,
                            value: Box::new(param_name.clone()),
                        };

                        pre_logic.push(format!(
                            "const {field_extracted_name} = {param_name}[\"{field_name}\"];"
                        ));

                        gen_value_js_to_rust(
                            field_extracted_name,
                            field_type,
                            in_path,
                            env,
                            pre_logic,
                            invocation_params,
                            post_logic,
                            &struct_type.lifetimes,
                            &borrowed_current_to_root,
                            entries,
                        );
                    }
                }
                ast::CustomType::Enum(enm) => {
                    invocation_params.push(format!("{}_js_to_rust[{}]", enm.name, param_name));
                }
                ast::CustomType::Opaque(_) => {
                    panic!("Opaque types cannot be sent as values");
                }
                &_ => unreachable!("unknown AST/HIR variant"),
            }
        }
        _ => invocation_params.push(param_name.to_string()),
    }
}

/// Type alias for readability.
type Offset = Option<NonZeroUsize>;

/// An [`fmt::Display`] type representing an invocation of a WASM function.
pub struct Invocation {
    /// Name of the method in the WASM namespace.
    full_path_name: ast::Ident,

    /// Arguments to invoke the function with.
    args: Vec<String>, // FIXME: use `Vec<Argument>`
}

impl Invocation {
    /// Create a new [`Invocation`].
    pub fn new(full_path_name: ast::Ident, args: Vec<String>) -> Self {
        Invocation {
            full_path_name,
            args,
        }
    }

    /// Invoke the function without passing in a return buffer.
    pub fn scalar(&self) -> impl fmt::Display + '_ {
        display::expr(move |f| write!(f, "wasm.{}({})", self.full_path_name, Csv(&self.args[..])))
    }

    /// Invoke the function with a provided return buffer.
    pub fn complex<'invoke>(
        &'invoke self,
        buf: &'invoke ast::Ident,
    ) -> impl fmt::Display + 'invoke {
        display::expr(move |f| {
            write!(
                f,
                "wasm.{}({})",
                self.full_path_name,
                display::expr(|f| {
                    write!(f, "{buf}")?;
                    for param in &self.args {
                        write!(f, ", {param}")?;
                    }
                    Ok(())
                })
            )
        })
    }
}

/// Base information shared across the different conversion types.
#[derive(Copy, Clone)]
pub struct Base<'base> {
    /// Scope of the Diplomat type environment.
    pub in_path: &'base ast::Path,

    /// Diplomat type environment.
    pub env: &'base Env,

    /// Bindings that the value being created borrow from.
    pub borrows: &'base [Argument<'base>],
}

impl<'base> Base<'base> {
    /// Returns the [`ast::CustomType`] associated with a given [`ast::PathType`].
    fn resolve_type<'custom>(&'custom self, path_type: &ast::PathType) -> &'custom ast::CustomType {
        path_type.resolve(self.in_path, self.env)
    }

    /// Returns the [`ReturnTypeForm`] of a given [`ast::TypeName`].
    fn return_type_form(&self, typ: &ast::TypeName) -> ReturnTypeForm {
        return_type_form(typ, self.in_path, self.env)
    }

    /// Returns the size and align of a given [`ast::TypeName`].
    fn size_align(&self, typ: &ast::TypeName) -> (usize, usize) {
        let layout = layout::type_size_alignment(typ, self.in_path, self.env);
        (layout.size(), layout.align())
    }

    /// Returns the size, align, and ok-offset of a `DiplomatResult` containing
    /// the two provided types.
    fn result_size_align(
        &self,
        ok: &ast::TypeName,
        err: &ast::TypeName,
    ) -> (usize, (usize, usize)) {
        let (ok_offset, layout) =
            layout::result_ok_offset_size_align(ok, err, self.in_path, self.env);
        (ok_offset, (layout.size(), layout.align()))
    }
}

/// An [`fmt::Display`] type representing a JS expression that evaluates to a
/// pointer.
#[derive(Copy, Clone)]
pub enum Underlying<'base> {
    /// A WASM invocation that evaluates to a pointer.
    Invocation(&'base Invocation),

    /// A binding that holds a pointer.
    Binding(&'base ast::Ident, Offset),

    /// Dereference a double pointer once, yielding the inside pointer.
    PtrRead(&'base Underlying<'base>),
}

impl fmt::Display for Underlying<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Underlying::Invocation(invoke) => invoke.scalar().fmt(f),
            Underlying::Binding(name, offset) => {
                if let Some(offset) = offset {
                    write!(f, "{name} + {offset}")
                } else {
                    name.fmt(f)
                }
            }
            Underlying::PtrRead(underlying) => {
                write!(f, "diplomatRuntime.ptrRead(wasm, {underlying})")
            }
        }
    }
}

/// An [`fmt::Display`] type that writes a slice as a sequence of comma separated
/// values.
pub struct Csv<I>(pub I);

impl<I, T> fmt::Display for Csv<I>
where
    I: IntoIterator<Item = T> + Clone,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.0.clone().into_iter();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for item in iter {
                write!(f, ", {item}")?;
            }
        }
        Ok(())
    }
}

/// An [`fmt::Display`] type representing a JS value created from a WASM invocation.
pub struct InvocationIntoJs<'base> {
    /// The type of the created value.
    pub typ: &'base ast::TypeName,

    /// The invocation that yields the value.
    pub invocation: Invocation,

    /// A mapping from lifetimes to the inputs that must outlive them.
    ///
    /// `Some` if `typ` is a named type otherwise `None`.
    pub lifetimes: Option<&'base BTreeMap<&'base ast::NamedLifetime, Vec<Argument<'base>>>>,

    /// Base data.
    pub base: Base<'base>,
}

impl fmt::Display for InvocationIntoJs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.typ {
            ast::TypeName::Primitive(..) => self.invocation.scalar().fmt(f),
            ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
                match self.base.resolve_type(path_type) {
                    ast::CustomType::Struct(strct) => {
                        // TODO: optimize `return_type_form` because we already know we're a non-opaque struct
                        match self.base.return_type_form(self.typ) {
                        ReturnTypeForm::Scalar => {
                            todo!("#173: constructing a scalar struct")
                        }
                        ReturnTypeForm::Complex => display::iife(|mut f| {
                            let (size, align) = self.base.size_align(self.typ);
                            let diplomat_receive_buffer: ast::Ident = "diplomat_receive_buffer".into();
                            writeln!(f, "const {diplomat_receive_buffer} = wasm.diplomat_alloc({size}, {align});")?;
                            writeln!(f, "{};", self.invocation.complex(&diplomat_receive_buffer))?;
                            writeln!(
                                f,
                                "const out = new {}({});",
                                strct.name,
                                display::expr(|f| {
                                    diplomat_receive_buffer.fmt(f)?;
                                    for lifetime in strct.lifetimes.names() {
                                        if let Some(inputs) = self.lifetimes.expect("must be `Some` for named types").get(lifetime) {
                                            write!(f, ", [{}]", Csv(inputs.iter().map(ArgumentLifetimeEdge)))?;
                                        } else {
                                            unreachable!("if the struct has any lifetimes, then it has to borrow from something")
                                        }
                                    }
                                    Ok(())
                                }),
                            )?;
                            writeln!(f, "wasm.diplomat_free({diplomat_receive_buffer}, {size}, {align});")?;
                            writeln!(f, "return out;")
                        })
                        .fmt(f),
                        ReturnTypeForm::Empty => unreachable!(),
                    }
                    }
                    ast::CustomType::Opaque(_opaque) => {
                        // Codegen for opaque structs is in `Pointer`s `fmt::Display` impl
                        unreachable!("Cannot construct an opaque struct that's not borrowed")
                    }
                    ast::CustomType::Enum(enm) => {
                        write!(f, "{}_rust_to_js[{}]", enm.name, self.invocation.scalar())
                    },
                    &_ => unreachable!("unknown AST/HIR variant")
                }
            }
            ast::TypeName::Reference(.., inner) => Pointer {
                inner,
                underlying: Underlying::Invocation(&self.invocation),
                owned: false,
                base: self.base,
            }
            .fmt(f),
            ast::TypeName::Box(inner) => Pointer {
                inner,
                underlying: Underlying::Invocation(&self.invocation),
                owned: true,
                base: self.base,
            }
            .fmt(f),
            ast::TypeName::Option(inner) => {
                let (inner, owned) = match inner.as_ref() {
                    ast::TypeName::Reference(.., inner) => (inner, false),
                    ast::TypeName::Box(inner) => (inner, true),
                    _ => unreachable!("non-pointer type in an Option"),
                };

                display::iife(|mut f| {
                    let option_ptr: ast::Ident = "option_ptr".into();
                    writeln!(f, "const {option_ptr} = {};", self.invocation.scalar())?;
                    writeln!(
                        f,
                        "return ({option_ptr} == 0) ? null : {};",
                        Pointer {
                            inner,
                            underlying: Underlying::Binding(&option_ptr, None),
                            owned,
                            base: self.base,
                        }
                    )
                })
                .fmt(f)
            }
            ast::TypeName::Result(ok, err, _) => {
                match self.base.return_type_form(self.typ) {
                    ReturnTypeForm::Scalar => display::iife(|mut f| {
                        writeln!(f, "const is_ok = {} == 1;", self.invocation.scalar())?;
                        writeln!(f, "if (!is_ok) {}", display::block(|mut f| {
                            writeln!(f, "throw new diplomatRuntime.FFIError(undefined);")
                        }))
                    })
                    .fmt(f),
                    ReturnTypeForm::Complex => {
                        display::iife(|mut f| {
                            let (flag_offset, (size, align)) = self.base.result_size_align(ok, err);
                            let diplomat_receive_buffer: ast::Ident = "diplomat_receive_buffer".into();
                            writeln!(f, "const {diplomat_receive_buffer} = wasm.diplomat_alloc({size}, {align});")?;
                            writeln!(f, "{};", self.invocation.complex(&diplomat_receive_buffer))?;
                            writeln!(f, "const is_ok = diplomatRuntime.resultFlag(wasm, {diplomat_receive_buffer}, {flag_offset});")?;
                            writeln!(
                                f,
                                "if (is_ok) {if_true} else {if_false}",
                                if_true = display::block(|mut f| {
                                    writeln!(f, "const ok_value = {};", UnderlyingIntoJs {
                                        inner: ok.as_ref(),
                                        underlying: Underlying::Binding(&diplomat_receive_buffer, None),
                                        base: self.base,
                                    })?;
                                    writeln!(f, "wasm.diplomat_free({diplomat_receive_buffer}, {size}, {align});")?;
                                    writeln!(f, "return ok_value;")
                                }),
                                if_false = display::block(|mut f| {
                                    writeln!(f, "const throw_value = {};", UnderlyingIntoJs {
                                        inner: err.as_ref(),
                                        underlying: Underlying::Binding(&diplomat_receive_buffer, None),
                                        base: self.base,
                                    })?;
                                    writeln!(f, "wasm.diplomat_free({diplomat_receive_buffer}, {size}, {align});")?;
                                    writeln!(f, "throw new diplomatRuntime.FFIError(throw_value);")
                                })
                            )
                        })
                        .fmt(f)
                    }
                    ReturnTypeForm::Empty => unreachable!(),
                }
            }
            ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8) => self.display_slice(SliceKind::Str).fmt(f),
            ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf16) => self.display_slice(SliceKind::Str16).fmt(f),
            ast::TypeName::PrimitiveSlice(.., prim) => {
                self.display_slice(SliceKind::Primitive(prim.into())).fmt(f)
            }
            ast::TypeName::Writeable => todo!(),
            ast::TypeName::Unit => self.invocation.scalar().fmt(f),
            &_ => unreachable!("unknown AST/HIR variant"),
        }
    }
}

/// A flag type used to simplify write implementations for `StrReference` and `PrimitiveType` variants,
/// where the implementations are largely the same.
enum SliceKind {
    Str,
    Str16,
    Primitive(JsPrimitive),
}

impl SliceKind {
    fn display<'a>(&'a self, ptr: &'a ast::Ident, size: &'a ast::Ident) -> impl fmt::Display + 'a {
        display::expr(move |f| match self {
            SliceKind::Str => write!(f, "diplomatRuntime.readString8(wasm, {ptr}, {size})"),
            SliceKind::Str16 => write!(f, "diplomatRuntime.readString16(wasm, {ptr}, {size})"),
            SliceKind::Primitive(prim) => match prim {
                JsPrimitive::Number(num) => {
                    // TODO(#383): can we borrow this?
                    write!(
                        f,
                        "{num}Array.from(new {num}Array(wasm.memory.buffer, ptr, size))"
                    )
                }
                JsPrimitive::Bool => todo!("Handle returning `&[bool]`."),
                JsPrimitive::Char => todo!("Handle returning `&[char]`."),
            },
        })
    }
}

impl InvocationIntoJs<'_> {
    fn display_slice(&self, kind: SliceKind) -> impl fmt::Display + '_ {
        display::iife(move |mut f| {
            let (size, align) = self.base.size_align(self.typ);
            let diplomat_receive_buffer: ast::Ident = "diplomat_receive_buffer".into();
            let ptr_ident: ast::Ident = "ptr".into();
            let size_ident: ast::Ident = "size".into();
            writeln!(
                f,
                "const {diplomat_receive_buffer} = wasm.diplomat_alloc({size}, {align});"
            )?;
            writeln!(f, "{};", self.invocation.complex(&diplomat_receive_buffer))?;
            writeln!(f, "const [{ptr_ident}, {size_ident}] = new Uint32Array(wasm.memory.buffer, {diplomat_receive_buffer}, 2);")?;
            writeln!(
                f,
                "wasm.diplomat_free({diplomat_receive_buffer}, {size}, {align});"
            )?;

            writeln!(f, "return {};", kind.display(&ptr_ident, &size_ident))?;
            Ok(())
        })
    }
}

/// An [`fmt::Display`] type representing a JS value behind a pointer.
struct Pointer<'base> {
    /// The type behind the pointer.
    inner: &'base ast::TypeName,

    /// An expression that evaluates into the pointer.
    underlying: Underlying<'base>,

    /// Whether or not the pointer is owned.
    owned: bool,

    /// Base data.
    base: Base<'base>,
}

impl fmt::Display for Pointer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) = self.inner {
            if let ast::CustomType::Opaque(opaque) = self.base.resolve_type(path_type) {
                write!(
                    f,
                    "new {name}({underlying}, {owned}, [{edges}])",
                    name = opaque.name,
                    underlying = self.underlying,
                    owned = self.owned,
                    edges = Csv(self.base.borrows.iter().map(ArgumentLifetimeEdge)),
                )?;

                return Ok(());
            }
        }

        UnderlyingIntoJs {
            inner: self.inner,
            underlying: self.underlying,
            base: self.base,
        }
        .fmt(f)
    }
}

/// Primitive numeric types available in JS.
#[derive(Display)]
enum JsPrimitiveNumber {
    #[displaydoc("Int8")]
    Int8,
    #[displaydoc("Uint8")]
    Uint8,
    #[displaydoc("Int16")]
    Int16,
    #[displaydoc("Uint16")]
    Uint16,
    #[displaydoc("Int32")]
    Int32,
    #[displaydoc("Uint32")]
    Uint32,
    #[displaydoc("BigInt64")]
    BigInt64,
    #[displaydoc("BigUint64")]
    BigUint64,
    #[displaydoc("Float32")]
    Float32,
    #[displaydoc("Float64")]
    Float64,
}

/// Primitive scalar types available in JS.
enum JsPrimitive {
    Number(JsPrimitiveNumber),
    Bool,
    Char,
}

impl From<&ast::PrimitiveType> for JsPrimitive {
    fn from(prim: &ast::PrimitiveType) -> Self {
        match prim {
            ast::PrimitiveType::i8 => JsPrimitive::Number(JsPrimitiveNumber::Int8),
            ast::PrimitiveType::u8 => JsPrimitive::Number(JsPrimitiveNumber::Uint8),
            ast::PrimitiveType::i16 => JsPrimitive::Number(JsPrimitiveNumber::Int16),
            ast::PrimitiveType::u16 => JsPrimitive::Number(JsPrimitiveNumber::Uint16),
            ast::PrimitiveType::i32 => JsPrimitive::Number(JsPrimitiveNumber::Int32),
            ast::PrimitiveType::u32 => JsPrimitive::Number(JsPrimitiveNumber::Uint32),
            ast::PrimitiveType::i64 => JsPrimitive::Number(JsPrimitiveNumber::BigInt64),
            ast::PrimitiveType::u64 => JsPrimitive::Number(JsPrimitiveNumber::BigUint64),
            ast::PrimitiveType::i128 => panic!("128-bit integers are unsupported"),
            ast::PrimitiveType::u128 => panic!("128-bit unsigned integers are unsupported"),
            ast::PrimitiveType::isize => JsPrimitive::Number(JsPrimitiveNumber::Int32),
            ast::PrimitiveType::usize => JsPrimitive::Number(JsPrimitiveNumber::Uint32),
            ast::PrimitiveType::f32 => JsPrimitive::Number(JsPrimitiveNumber::Float32),
            ast::PrimitiveType::f64 => JsPrimitive::Number(JsPrimitiveNumber::Float64),
            ast::PrimitiveType::bool => JsPrimitive::Bool,
            ast::PrimitiveType::char => JsPrimitive::Char,
        }
    }
}

/// An [`fmt::Display`] type representing a JS expression that builds a value
/// from an underlying buffer.
///
/// WASM writes the contents of returned non-opaque structs and `Result`s into
/// a buffer, and [`UnderlyingIntoJs`] allows for reading the contents of that
/// buffer.
pub struct UnderlyingIntoJs<'base> {
    /// The type inside the underlying buffer.
    pub inner: &'base ast::TypeName,

    /// The underlying buffer to read the value from.
    pub underlying: Underlying<'base>,

    /// Base data.
    pub base: Base<'base>,
}

impl fmt::Display for UnderlyingIntoJs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ast::TypeName::Primitive(prim) => match prim.into() {
                JsPrimitive::Number(num) => write!(
                    f,
                    "(new {num}Array(wasm.memory.buffer, {}, 1))[0]",
                    self.underlying
                ),
                JsPrimitive::Bool => write!(
                    f,
                    "(new Uint8Array(wasm.memory.buffer, {}, 1))[0] == 1",
                    self.underlying
                ),
                JsPrimitive::Char => write!(
                    f,
                    "String.fromCharCode((new Uint32Array(wasm.memory.buffer, {}, 1))[0])",
                    self.underlying
                ),
            },
            ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
                match self.base.resolve_type(path_type) {
                    ast::CustomType::Struct(strct) => {
                        // TODO: optimize because we already know it's a non-opaque struct
                        match self.base.return_type_form(self.inner) {
                            ReturnTypeForm::Scalar => {
                                todo!("#173: constructing a scalar struct from a buffer")
                            }
                            ReturnTypeForm::Complex => write!(
                                f,
                                "new {}({})",
                                strct.name,
                                display::expr(|f| {
                                    self.underlying.fmt(f)?;
                                    for inputs in self.base.borrows {
                                        write!(f, ", {inputs}")?;
                                    }
                                    Ok(())
                                }),
                            ),
                            ReturnTypeForm::Empty => unreachable!(),
                        }
                    }
                    ast::CustomType::Opaque(_opaque) => {
                        // Codegen for opaque structs is in `Pointer`s `fmt::Display` impl
                        if let ast::TypeName::SelfType(_) = self.inner {
                            unreachable!("Self Opaque not behind a pointer: {}", self.inner);
                        }
                        unreachable!("Opaque not behind a pointer")
                    }
                    ast::CustomType::Enum(enm) => write!(
                        f,
                        "{}_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, {})]",
                        enm.name, self.underlying,
                    ),
                    &_ => unreachable!("unknown AST/HIR variant"),
                }
            }
            ast::TypeName::Reference(.., typ) => Pointer {
                inner: typ,
                underlying: Underlying::PtrRead(&self.underlying),
                owned: false,
                base: self.base,
            }
            .fmt(f),
            ast::TypeName::Box(typ) => Pointer {
                inner: typ,
                underlying: Underlying::PtrRead(&self.underlying),
                owned: true,
                base: self.base,
            }
            .fmt(f),
            ast::TypeName::Option(inner) => {
                let (inner, owned) = match inner.as_ref() {
                    ast::TypeName::Reference(.., inner) => (inner, false),
                    ast::TypeName::Box(inner) => (inner, true),
                    _ => unreachable!("non-pointer in an Option"),
                };

                display::iife(|mut f| {
                    let option_ptr: ast::Ident = "option_ptr".into();
                    writeln!(
                        f,
                        "const {option_ptr} = {};",
                        Underlying::PtrRead(&self.underlying)
                    )?;
                    writeln!(
                        f,
                        "return ({option_ptr} == 0) ? null : {};",
                        Pointer {
                            inner,
                            underlying: Underlying::Binding(&option_ptr, None),
                            owned,
                            base: self.base,
                        }
                    )
                })
                .fmt(f)
            }
            ast::TypeName::Result(..) => {
                todo!("Result in a buffer")
            }
            ast::TypeName::Writeable => todo!("Writeable in a buffer"),
            ast::TypeName::StrReference(
                _,
                ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8,
            ) => self.display_slice(SliceKind::Str).fmt(f),
            ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf16) => {
                self.display_slice(SliceKind::Str16).fmt(f)
            }
            ast::TypeName::PrimitiveSlice(.., prim) => {
                self.display_slice(SliceKind::Primitive(prim.into())).fmt(f)
            }
            ast::TypeName::Unit => "{}".fmt(f),
            &_ => unreachable!("unknown AST/HIR variant"),
        }
    }
}

impl UnderlyingIntoJs<'_> {
    /// Returns an [`fmt::Display`] object that writes code to generate a slice
    /// from an underlying buffer.
    fn display_slice(&self, kind: SliceKind) -> impl fmt::Display + '_ {
        display::iife(move |mut f| {
            let ptr_ident: ast::Ident = "ptr".into();
            let size_ident: ast::Ident = "size".into();
            writeln!(
                f,
                "const [{ptr_ident}, {size_ident}] = new Uint32Array(wasm.memory.buffer, {}, 2);",
                self.underlying
            )?;
            writeln!(f, "return {};", kind.display(&ptr_ident, &size_ident))?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_unambiguous_names() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                pub struct Point {
                    x: i32,
                    y: i32,
                }

                pub struct Line {
                    start: Point,
                    end: Point,
                }

                impl Line {
                    pub fn do_stuff(self) {}
                }
            }
        }
    }

    #[test]
    fn test_struct_borrowing() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Scalar;

                pub struct Point<'x, 'y> {
                    x: &'x Scalar,
                    y: &'y Scalar,
                }

                pub struct PointTranspose<'u, 'v> {
                    point: Point<'u, 'v>,
                    transpose: Point<'v, 'u>,
                }

                impl<'x, 'y> Point<'x, 'y> {
                    pub fn get_x(self) -> &'x Scalar {
                        self.x
                    }
                }

                impl<'u, 'v> PointTranspose<'u, 'v> {
                    pub fn new(u: &'u Scalar, v: &'v Scalar) -> Self {
                        unimplemented!()
                    }

                    pub fn transpose(self) -> PointTranspose<'v, 'u> {
                        unimplemented!()
                    }

                    pub fn point(self) -> Point<'u, 'v> {
                        self.point
                    }
                }
            }
        }
    }

    #[test]
    fn test_str_borrowing() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                pub struct MyStruct<'a> {
                    s: &'a DiplomatStr,
                }

                impl<'a> MyStruct<'a> {
                    pub fn new(s: &'a DiplomatStr) -> Self {
                        Self { s }
                    }

                    pub fn get(self) -> &'a DiplomatStr {
                        self.s
                    }
                }
            }
        }
    }

    #[test]
    fn test_borrowing_opaque_owned_by_struct() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct BorrowingOpaque<'a>(&'a ());

                pub struct BorrowingStruct<'a> {
                    opaque: Box<BorrowingOpaque<'a>>,
                    x: u8,
                }

                impl<'a> BorrowingStruct<'a> {
                    pub fn new(opaque: Box<BorrowingOpaque<'a>>) -> Self {
                        Self { opaque, x: 8 }
                    }
                }
            }
        }
    }
}
