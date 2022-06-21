use diplomat_core::{ast, Env};
use std::fmt::{self, Write as _};

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

/// An [`fmt::Display`] type that writes the conversion from a Rust value into
/// a JavaScript value.
pub struct ValueIntoJs<'a> {
    /// A JS expression that yields a Rust value through wasm.
    pub value_expr: &'a str,

    /// The type of the Rust value.
    pub typ: &'a ast::TypeName,

    /// A boolean determining if the value borrows from the object that created it.
    pub borrows_self: bool,

    /// A sequence of fields that the value borrows from.
    pub borrowed_params: &'a [&'a ast::Param],

    pub in_path: &'a ast::Path,
    pub env: &'a Env,
}

impl<'a> fmt::Display for ValueIntoJs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.typ {
            ast::TypeName::Named(path_type) => match path_type.resolve(self.in_path, self.env) {
                ast::CustomType::Struct(strct) => {
                    match return_type_form(self.typ, self.in_path, self.env) {
                        ReturnTypeForm::Scalar => {
                            todo!("Recieving structs that don't need a buffer: {}", strct.name)
                        }
                        ReturnTypeForm::Complex => {
                            let strct_layout = layout::type_size_alignment(self.typ, self.in_path, self.env);
                            let (size, align) = (strct_layout.size(), strct_layout.align());
                            display::iife(|mut f| {
                                let buf_name = ast::Ident::from("diplomat_receive_buffer");
                                writeln!(f, "const {buf_name} = wasm.diplomat_alloc({size}, {align});")?;
                                writeln!(f, "{};", self.value_expr)?;
                                writeln!(f, "const out = new {}({buf_name});", strct.name)?;
                                writeln!(f, "wasm.diplomat_free({buf_name}, {size}, {align});")?;
                                writeln!(f, "return out;")
                            }).fmt(f)
                        }
                        ReturnTypeForm::Empty => panic!("How do we handle this case?"),
                    }

                }
                ast::CustomType::Opaque(opaque) => {
                    if !self.borrows_self && self.borrowed_params.is_empty() {
                        write!(f, "new {}({})", opaque.name, self.value_expr)
                    } else {
                        display::iife(|mut f| {
                            writeln!(f, "const out = new {}({});", opaque.name, self.value_expr)?;
                            if self.borrows_self {
                                writeln!(f, "out.__this_lifetime_guard = this;")?;
                            }
                            for param in self.borrowed_params {
                                writeln!(f, "out.__{0}_lifetime_guard = {0};", param.name)?;
                            }
                            writeln!(f, "return out;")
                        }).fmt(f)
                    }
                }
                ast::CustomType::Enum(enm) => {
                    write!(f, "{}_rust_to_js[{}]", enm.name, ValueIntoJs {
                        typ: &ast::TypeName::Primitive(ast::PrimitiveType::isize),
                        ..*self
                    })
                }
            }
            ast::TypeName::Box(typ) | ast::TypeName::Reference(.., typ) => {
                ValueIntoJs {
                    typ: typ.as_ref(),
                    ..*self
                }
                .fmt(f)
            }
            ast::TypeName::Option(typ) => match typ.as_ref() {
                ptr_type @ (ast::TypeName::Box(..) | ast::TypeName::Reference(..)) => {
                    display::iife(|mut f| {
                        writeln!(
                            f,
                            "const option_ptr = {};",
                            self.value_expr
                        )?;
                        writeln!(
                            f,
                            "return (option_ptr == 0) ? null : {};",
                            ValueIntoJs {
                                value_expr: "option_ptr",
                                typ: ptr_type,
                                ..*self
                            }
                        )
                    })
                    .fmt(f)
                }
                other @ (ast::TypeName::StrReference(..) | ast::TypeName::PrimitiveSlice(..)) => panic!("`{}` is a fat pointer (ptr, len), so it can't be stored behind an option", other),
                other => panic!("`{0}` doesn't have the same layout guarantees as a ptr, so `Option<{0}>` is unsupported", other),
            },
            ast::TypeName::Result(ok, err) => {
                let (ok_offset, result_layout) = layout::result_ok_offset_size_align(ok, err, self.in_path, self.env);
                let (size, align) = (result_layout.size(), result_layout.align());
                let needs_buffer = matches!(return_type_form(self.typ, self.in_path, self.env), ReturnTypeForm::Complex);
                display::iife(|mut f| {
                    if needs_buffer {
                        let buf_ident = ast::Ident::from("diplomat_receive_buffer");
                        writeln!(f, "const {buf_ident} = wasm.diplomat_alloc({size}, {align});")?;
                        writeln!(f, "{};", self.value_expr)?;
                        writeln!(f, "const is_ok = diplomatRuntime.resultFlag(wasm, {buf_ident}, {ok_offset});")?;
                        writeln!(f, "if (is_ok) {if_true} else {if_false}",
                            if_true = display::block(|mut f| {
                                writeln!(f, "const ok_value = {};", BufferedIntoJs {
                                    buf_ptr: buf_ident.as_str(),
                                    offset: 0,
                                    typ: ok.as_ref(),
                                    borrows_self: false,
                                    borrowed_params: &[],
                                    in_path: self.in_path,
                                    env: self.env,
                                })?;
                                writeln!(f, "wasm.diplomat_free({buf_ident}, {size}, {align})")?;
                                writeln!(f, "return ok_value;")
                            }),
                            if_false = display::block(|mut f| {
                                writeln!(f, "const throw_value = {};", BufferedIntoJs {
                                    buf_ptr: buf_ident.as_str(),
                                    offset: 0,
                                    typ: err.as_ref(),
                                    borrows_self: false,
                                    borrowed_params: &[],
                                    in_path: self.in_path,
                                    env: self.env,
                                })?;
                                writeln!(f, "wasm.diplomat_free({buf_ident}, {size}, {align})")?;
                                writeln!(f, "throw new diplomatRuntime.FFIError(throw_value);")
                            })
                        )
                    } else {
                        writeln!(f, "const is_ok = {} == 1;", self.value_expr)?;
                        writeln!(f, "if (!is_ok) {}", display::block(|mut f| {
                            writeln!(f, "throw new diplomatRuntime.FFIError({{}});")
                        }))
                    }
                }).fmt(f)
            }
            ast::TypeName::Writeable => todo!("ast::TypeName::Writeable"),
            ast::TypeName::StrReference(..) => todo!("ast::TypeName::StrReference(_)"),
            ast::TypeName::PrimitiveSlice(..) => todo!("ast::TypeName::PrimitiveSlice(..)"),
            ast::TypeName::Primitive(..) | ast::TypeName::Unit => self.value_expr.fmt(f),
        }
    }
}

/// An [`fmt::Display`] type that writes the conversion from a Rust value in
/// a buffer to a JavaScript value.
///
/// This method is useful for reading fields of a non-opaque struct from a buffer,
/// and reading the `Ok` and `Err` values from a `DiplomatResult`.
pub struct BufferedIntoJs<'a> {
    /// The name of the buffer.
    pub buf_ptr: &'a str,

    /// The offset within the buffer to read from.
    pub offset: usize,

    /// The type being read from the buffer.
    pub typ: &'a ast::TypeName,

    /// A boolean determining if the value borrows from the object that created it.
    pub borrows_self: bool,

    /// A sequence of fields that the value borrows from.
    pub borrowed_params: &'a [&'a ast::Param],

    pub in_path: &'a ast::Path,
    pub env: &'a Env,
}

impl<'a> BufferedIntoJs<'a> {
    fn as_value(&'a self, value_expr: &'a str) -> ValueIntoJs<'a> {
        ValueIntoJs {
            value_expr,
            typ: self.typ,
            borrows_self: self.borrows_self,
            borrowed_params: self.borrowed_params,
            in_path: self.in_path,
            env: self.env,
        }
    }
}

impl<'a> fmt::Display for BufferedIntoJs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.typ {
            ast::TypeName::Primitive(prim) => {
                macro_rules! read_prim {
                    ($array:expr) => {
                        self.as_value(&format!(
                            concat!("(new ", $array, "(wasm.memory.buffer, {} + {}, 1))[0]"),
                            self.buf_ptr, self.offset
                        ))
                        .fmt(f)
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
                    ast::PrimitiveType::bool => self
                        .as_value(&format!(
                            "(new Uint8Array(wasm.memory.buffer, {} + {}, 1))[0] == 1",
                            self.buf_ptr, self.offset
                        ))
                        .fmt(f),
                    ast::PrimitiveType::char => self
                        .as_value(&format!(
                            "String.fromCharCode((new Uint32Array(wasm.memory.buffer, {} + {}, 1))[0])",
                            self.buf_ptr, self.offset
                        ))
                        .fmt(f),
                }
            }
            ast::TypeName::Named(path_type) => match path_type.resolve(self.in_path, self.env) {
                ast::CustomType::Struct(strct) => {
                    write!(f, "new {}({})", strct.name, self.buf_ptr)
                }
                ast::CustomType::Opaque(opaque) => {
                    write!(
                        f,
                        "new {}(diplomatRuntime.ptrRead(wasm, {} + {}))",
                        opaque.name, self.buf_ptr, self.offset
                    )
                }
                ast::CustomType::Enum(enm) => {
                    write!(
                        f,
                        "{}_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, {} + {})]",
                        enm.name, self.buf_ptr, self.offset
                    )
                }
            },
            ast::TypeName::Box(..) | ast::TypeName::Reference(..) => self
                .as_value(&format!(
                    "diplomatRuntime.ptrRead(wasm, {} + {})",
                    self.buf_ptr, self.offset
                ))
                .fmt(f),
            ast::TypeName::Option(typ) => match typ.as_ref() {
                ast::TypeName::Box(..) | ast::TypeName::Reference(..) => self
                    .as_value(&format!(
                        "diplomatRuntime.ptrRead(wasm, {} + {})",
                        self.buf_ptr, self.offset
                    ))
                    .fmt(f),
                slice @ (ast::TypeName::StrReference(..) | ast::TypeName::PrimitiveSlice(..)) => {
                    panic!(
                        "`{}` is a fat pointer (ptr, len), and cannot be held in an option",
                        slice
                    )
                }
                other => panic!(
                    "`{}` isn't a pointer type, and cannot be held in an option",
                    other
                ),
            },
            ast::TypeName::Unit => write!(f, "{{}}"),
            other => todo!("Read `{other}` from a buffer"),
        }
    }
}
