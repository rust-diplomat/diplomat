use diplomat_core::ast::{self, BorrowedParams};
use diplomat_core::Env;
use std::fmt::{self, Write as _};
use std::num::NonZeroUsize;

use super::display;
use super::types::{return_type_form, ReturnTypeForm};
use crate::layout;

/// TODO: docs
#[allow(clippy::ptr_arg)] // false positive, rust-clippy#8463, fixed in 1.61
pub fn gen_value_js_to_rust(
    param_name: &ast::Ident,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    pre_logic: &mut Vec<String>,
    invocation_params: &mut Vec<String>,
    post_logic: &mut Vec<String>,
) {
    match typ {
        ast::TypeName::StrReference(..) | ast::TypeName::PrimitiveSlice(..) => {
            // TODO(#61): consider extracting into runtime function
            if let ast::TypeName::StrReference(..) = typ {
                pre_logic.push(format!(
                    "let {}_diplomat_bytes = (new TextEncoder()).encode({});",
                    param_name, param_name
                ));
            } else {
                pre_logic.push(format!(
                    "let {}_diplomat_bytes = new Uint8Array({});",
                    param_name, param_name
                ));
            }
            let align = if let ast::TypeName::PrimitiveSlice(.., prim) = typ {
                layout::primitive_size_alignment(*prim).align()
            } else {
                1
            };
            pre_logic.push(format!(
                "let {}_diplomat_ptr = wasm.diplomat_alloc({}_diplomat_bytes.length, {});",
                param_name, param_name, align
            ));
            pre_logic.push(format!("let {}_diplomat_buf = new Uint8Array(wasm.memory.buffer, {}_diplomat_ptr, {}_diplomat_bytes.length);", param_name, param_name, param_name));
            pre_logic.push(format!(
                "{}_diplomat_buf.set({}_diplomat_bytes, 0);",
                param_name, param_name
            ));

            invocation_params.push(format!("{}_diplomat_ptr", param_name));
            invocation_params.push(format!("{}_diplomat_bytes.length", param_name));

            post_logic.push(format!(
                "wasm.diplomat_free({}_diplomat_ptr, {}_diplomat_bytes.length, {});",
                param_name, param_name, align
            ));
        }
        ast::TypeName::Primitive(ast::PrimitiveType::char) => {
            // we use the spread operator here to count codepoints
            // codePointAt() does not return surrogate pairs if there are multiple
            invocation_params.push(format!(
                "diplomatRuntime.extractCodePoint({p}, '{p}')",
                p = param_name
            ));
        }
        ast::TypeName::Box(_) => {
            invocation_params.push(format!("{}.underlying", param_name));
        }
        ast::TypeName::Reference(_, _mut, _lt) => {
            invocation_params.push(format!("{}.underlying", param_name));
        }
        ast::TypeName::Named(path_type) => match path_type.resolve(in_path, env) {
            ast::CustomType::Struct(struct_type) => {
                for (field_name, field_type, _) in struct_type.fields.iter() {
                    let field_extracted_name =
                        format!("diplomat_{}_extracted_{}", struct_type.name, field_name).into();
                    pre_logic.push(format!(
                        "const {} = {}[\"{}\"];",
                        field_extracted_name, param_name, field_name
                    ));

                    gen_value_js_to_rust(
                        &field_extracted_name,
                        field_type,
                        in_path,
                        env,
                        pre_logic,
                        invocation_params,
                        post_logic,
                    );
                }
            }

            ast::CustomType::Enum(enm) => {
                invocation_params.push(format!("{}_js_to_rust[{}]", enm.name, param_name));
            }

            ast::CustomType::Opaque(_) => {
                panic!("Opaque types cannot be sent as values");
            }
        },
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
    args: Vec<String>, // stringly typed for now...
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
        display::expr(move |f| {
            write!(
                f,
                "wasm.{}({})",
                self.full_path_name,
                display::expr(|f| {
                    if let Some((first, rest)) = self.args.split_first() {
                        write!(f, "{}", first)?;
                        for param in rest {
                            write!(f, ", {param}")?;
                        }
                    }
                    Ok(())
                })
            )
        })
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
                    write!(f, "{}", buf)?;
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

    /// A boolean determining if the value borrows from the caller.
    pub borrows_self: bool,

    /// The params that the value borrows.
    pub borrowed_params: &'base [&'base ast::Param],
}

impl<'base> Base<'base> {
    /// Create a [`Base`] for a field.
    pub fn new_field(in_path: &'base ast::Path, env: &'base Env, borrows_self: bool) -> Self {
        Base {
            in_path,
            env,
            borrows_self,
            borrowed_params: &[],
        }
    }

    /// Create a [`Base`] for a method.
    pub fn new_method(
        in_path: &'base ast::Path,
        env: &'base Env,
        borrowed_params: &'base BorrowedParams,
    ) -> Self {
        Base {
            in_path,
            env,
            borrows_self: borrowed_params.0.is_some(),
            borrowed_params: &borrowed_params.1[..],
        }
    }

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

    /// Returns `true` if there are any borrows, otherwise `false`.
    fn borrows(&self) -> bool {
        self.borrows_self || !self.borrowed_params.is_empty()
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

/// An [`fmt::Display`] type representing a JS value created from a WASM invocation.
pub struct InvocationIntoJs<'base> {
    /// The type of the created value.
    pub typ: &'base ast::TypeName,

    /// The invocation that yields the value.
    pub invocation: Invocation,

    /// Base data.
    pub base: Base<'base>,
}

impl fmt::Display for InvocationIntoJs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.typ {
            ast::TypeName::Primitive(..) => self.invocation.scalar().fmt(f),
            ast::TypeName::Named(path_type) => match self.base.resolve_type(path_type) {
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
                            writeln!(f, "const out = new {}({diplomat_receive_buffer});", strct.name)?;
                            writeln!(f, "wasm.diplomat_free({diplomat_receive_buffer}, {size}, {align});")?;
                            writeln!(f, "return out;")
                        })
                        .fmt(f),
                        ReturnTypeForm::Empty => unreachable!(),
                    }
                }
                ast::CustomType::Opaque(_opaque) => {
                    unreachable!("Cannot construct an opaque struct that's not borrowed")
                }
                ast::CustomType::Enum(enm) => {
                    write!(f, "{}_rust_to_js[{}]", enm.name, self.invocation.scalar())
                }
            },
            ast::TypeName::Reference(.., inner) | ast::TypeName::Box(inner) => Pointer {
                inner,
                underlying: Underlying::Invocation(&self.invocation),
                base: self.base,
            }
            .fmt(f),
            ast::TypeName::Option(inner) => match inner.as_ref() {
                ast::TypeName::Box(inner) | ast::TypeName::Reference(.., inner) => {
                    display::iife(|mut f| {
                        let option_ptr: ast::Ident = "option_ptr".into();
                        writeln!(f, "const {option_ptr} = {};", self.invocation.scalar())?;
                        writeln!(
                            f,
                            "return ({option_ptr} == 0) ? null : {};",
                            Pointer {
                                inner,
                                underlying: Underlying::Binding(&option_ptr, None),
                                base: self.base,
                            }
                        )
                    })
                    .fmt(f)
                }
                _ => unreachable!(),
            },
            ast::TypeName::Result(ok, err) => {
                match self.base.return_type_form(self.typ) {
                    ReturnTypeForm::Scalar => display::iife(|mut f| {
                        writeln!(f, "const is_ok = {} == 1;", self.invocation.scalar())?;
                        writeln!(f, "if (!is_ok) {}", display::block(|mut f| {
                            writeln!(f, "throw new diplomatRuntime.FFIError({{}});")
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
            ast::TypeName::Unit => self.invocation.scalar().fmt(f),
            ast::TypeName::Writeable => todo!(),
            ast::TypeName::StrReference(..) => todo!(),
            ast::TypeName::PrimitiveSlice(..) => todo!(),
        }
    }
}

/// An [`fmt::Display`] type representing a JS value behind a pointer.
struct Pointer<'base> {
    /// The type behind the pointer.
    inner: &'base ast::TypeName,

    /// An expression that evaluates into the pointer.
    underlying: Underlying<'base>,

    /// Base data.
    base: Base<'base>,
}

impl fmt::Display for Pointer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let ast::TypeName::Named(path_type) = self.inner {
            if let ast::CustomType::Opaque(opaque) = self.base.resolve_type(path_type) {
                if !self.base.borrows() {
                    write!(f, "new {}({})", opaque.name, self.underlying)?;
                } else {
                    display::iife(|mut f| {
                        writeln!(f, "const out = new {}({});", opaque.name, self.underlying)?;
                        if self.base.borrows_self {
                            writeln!(f, "out.__this_lifetime_guard = this;")?;
                        }
                        for param in self.base.borrowed_params {
                            writeln!(f, "out.__{0}_lifetime_guard = {0};", param.name)?;
                        }
                        writeln!(f, "return out;")
                    })
                    .fmt(f)?;
                }

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
            ast::TypeName::Primitive(prim) => {
                macro_rules! read_prim {
                    ($array:expr) => {
                        write!(
                            f,
                            concat!("(new ", $array, "(wasm.memory.buffer, {}, 1))[0]"),
                            self.underlying
                        )
                    };
                }
                match prim {
                    ast::PrimitiveType::i8 => read_prim!("Int8Array"),
                    ast::PrimitiveType::u8 => read_prim!("Uint8Array"),
                    ast::PrimitiveType::i16 => read_prim!("Int16Array"),
                    ast::PrimitiveType::u16 => read_prim!("Uint16Array"),
                    ast::PrimitiveType::i32 => read_prim!("Int32Array"),
                    ast::PrimitiveType::u32 => read_prim!("Uint32Array"),
                    ast::PrimitiveType::i64 => read_prim!("BigInt64Array"),
                    ast::PrimitiveType::u64 => read_prim!("BigUint64Array"),
                    ast::PrimitiveType::i128 => panic!("i128 not supported on JS"),
                    ast::PrimitiveType::u128 => panic!("u128 not supported on JS"),
                    ast::PrimitiveType::isize => read_prim!("Int32Array"),
                    ast::PrimitiveType::usize => read_prim!("Uint32Array"),
                    ast::PrimitiveType::f32 => read_prim!("Float32Array"),
                    ast::PrimitiveType::f64 => read_prim!("Float64Array"),
                    ast::PrimitiveType::bool => write!(
                        f,
                        "(new Uint8Array(wasm.memory.buffer, {}, 1))[0] == 1",
                        self.underlying
                    ),
                    ast::PrimitiveType::char => write!(
                        f,
                        "String.fromCharCode((new Uint32Array(wasm.memory.buffer, {}, 1))[0])",
                        self.underlying
                    ),
                }
            }
            ast::TypeName::Named(path_type) => match self.base.resolve_type(path_type) {
                ast::CustomType::Struct(strct) => {
                    // TODO: optimize because we already know it's a non-opaque struct
                    match self.base.return_type_form(self.inner) {
                        ReturnTypeForm::Scalar => {
                            todo!("#173: constructing a scalar struct from a buffer")
                        }
                        ReturnTypeForm::Complex => {
                            write!(f, "new {}({})", strct.name, self.underlying)
                        }
                        ReturnTypeForm::Empty => unreachable!(),
                    }
                }
                ast::CustomType::Opaque(_opaque) => {
                    unreachable!("Opaque not behind a pointer")
                }
                ast::CustomType::Enum(enm) => write!(
                    f,
                    "{}_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, {})]",
                    enm.name, self.underlying,
                ),
            },
            ast::TypeName::Reference(.., typ) | ast::TypeName::Box(typ) => Pointer {
                inner: typ,
                underlying: Underlying::PtrRead(&self.underlying),
                base: self.base,
            }
            .fmt(f),
            ast::TypeName::Option(inner) => match inner.as_ref() {
                ast::TypeName::Box(inner) | ast::TypeName::Reference(.., inner) => {
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
                                base: self.base,
                            }
                        )
                    })
                    .fmt(f)
                }
                _ => unreachable!(),
            },
            ast::TypeName::Result(..) => todo!("Result in a buffer"),
            ast::TypeName::Writeable => todo!("Writeable in a buffer"),
            ast::TypeName::StrReference(..) => todo!("StrReference in a buffer"),
            ast::TypeName::PrimitiveSlice(..) => todo!("PrimitiveSlice in a buffer"),
            ast::TypeName::Unit => "{}".fmt(f),
        }
    }
}
