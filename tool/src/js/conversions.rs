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
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::num::NonZeroUsize;

use super::display;
use super::types::{return_type_form, ReturnTypeForm};
use crate::layout;

/// Generate the necessary setup and tear down JS code to convert the parameters
/// into a form that Rust/WASM can understand.
#[allow(clippy::ptr_arg, clippy::too_many_arguments)] // false positive, rust-clippy#8463, fixed in 1.61
pub fn gen_value_js_to_rust(
    param_name: &ast::Ident,
    typ: &ast::TypeName,
    borrowed_params: &ast::BorrowedParams,
    in_path: &ast::Path,
    env: &Env,
    pre_logic: &mut Vec<String>,
    invocation_params: &mut Vec<String>,
    post_logic: &mut Vec<String>,
) {
    match typ {
        ast::TypeName::StrReference(..) | ast::TypeName::PrimitiveSlice(..) => {
            if let ast::TypeName::PrimitiveSlice(.., prim) = typ {
                pre_logic.push(format!(
                    "{param_name} = diplomatRuntime.DiplomatBuf.slice(wasm, {param_name}, {align});",
                    align = layout::primitive_size_alignment(*prim).align()
                ));
            } else {
                pre_logic.push(format!(
                    "{param_name} = diplomatRuntime.DiplomatBuf.str(wasm, {param_name});"
                ));
            }

            invocation_params.push(format!("{param_name}.ptr"));
            invocation_params.push(format!("{param_name}.size"));

            if !borrowed_params.contains(param_name) {
                post_logic.push(format!("{param_name}.free();"));
            }
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
                    let field_extracted_name = format!("f_{param_name}_{field_name}").into();
                    pre_logic.push(format!(
                        "const {} = {}[\"{}\"];",
                        field_extracted_name, param_name, field_name
                    ));

                    gen_value_js_to_rust(
                        &field_extracted_name,
                        field_type,
                        borrowed_params,
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

    pub borrows: &'base [ast::Ident],
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
struct Csv<'a, T>(&'a [T]);

impl<T: fmt::Display> fmt::Display for Csv<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, rest)) = self.0.split_first() {
            first.fmt(f)?;
            for item in rest {
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
    pub lifetimes: &'base BTreeMap<&'base ast::NamedLifetime, Vec<ast::Ident>>,

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
                            writeln!(
                                f,
                                "const out = new {}({});",
                                strct.name,
                                display::expr(|f| {
                                    diplomat_receive_buffer.fmt(f)?;
                                    for inputs in strct.lifetimes.names().map(|name| &self.lifetimes[name][..]) {
                                        write!(f, ", [{}]", Csv(inputs))?;
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
                    unreachable!("Cannot construct an opaque struct that's not borrowed")
                }
                ast::CustomType::Enum(enm) => {
                    write!(f, "{}_rust_to_js[{}]", enm.name, self.invocation.scalar())
                }
            },
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

    /// Whether or not the pointer is owned.
    owned: bool,

    /// Base data.
    base: Base<'base>,
}

impl fmt::Display for Pointer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let ast::TypeName::Named(path_type) = self.inner {
            if let ast::CustomType::Opaque(opaque) = self.base.resolve_type(path_type) {
                if self.base.borrows.is_empty() && !self.owned {
                    write!(f, "new {}({})", opaque.name, self.underlying)?;
                } else {
                    display::iife(|mut f| {
                        if self.owned {
                            writeln!(f, "const underlying = {};", self.underlying)?;
                            writeln!(f, "const out = new {}(underlying);", opaque.name)?;
                            writeln!(
                                f,
                                "{}_box_destroy_registry.register(out, underlying);",
                                opaque.name
                            )?;
                        } else {
                            writeln!(f, "const out = new {}({});", opaque.name, self.underlying)?;
                        }
                        for param in self.base.borrows {
                            writeln!(f, "out.__{param}_lifetime_guard = {param};")?;
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
                        ReturnTypeForm::Complex => write!(
                            f,
                            "new {}({})",
                            strct.name,
                            display::expr(|f| {
                                self.underlying.fmt(f)?;
                                for inputs in self.base.borrows.iter() {
                                    write!(f, ", {inputs}")?;
                                }
                                Ok(())
                            }),
                        ),
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
            ast::TypeName::Result(..) => todo!("Result in a buffer"),
            ast::TypeName::Writeable => todo!("Writeable in a buffer"),
            ast::TypeName::StrReference(..) => todo!("StrReference in a buffer"),
            ast::TypeName::PrimitiveSlice(..) => todo!("PrimitiveSlice in a buffer"),
            ast::TypeName::Unit => "{}".fmt(f),
        }
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

                pub struct PointReflection<'u, 'v> {
                    point: Point<'u, 'v>,
                    reflection: Point<'v, 'u>,
                }

                impl<'u, 'v> PointReflection<'u, 'v> {
                    pub fn new(u: &'u Opaque, v: &'v Opaque) -> Self {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
