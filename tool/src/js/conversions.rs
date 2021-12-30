use diplomat_core::Env;
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast::{self, PrimitiveType};
use indenter::indented;

use super::types::{return_type_form, ReturnTypeForm};
use crate::layout;

pub fn gen_value_js_to_rust(
    param_name: String,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    pre_logic: &mut Vec<String>,
    invocation_params: &mut Vec<String>,
    post_logic: &mut Vec<String>,
) {
    match typ {
        ast::TypeName::StrReference | ast::TypeName::PrimitiveSlice(_) => {
            // TODO(#61): consider extracting into runtime function
            if *typ == ast::TypeName::StrReference {
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
            let align = if let ast::TypeName::PrimitiveSlice(prim) = typ {
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
        ast::TypeName::Primitive(PrimitiveType::char) => {
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
        ast::TypeName::Reference(_, _) => {
            invocation_params.push(format!("{}.underlying", param_name));
        }
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Struct(struct_type) => {
                for (field_name, field_type, _) in struct_type.fields.iter() {
                    let field_extracted_name =
                        format!("diplomat_{}_extracted_{}", struct_type.name, field_name);
                    pre_logic.push(format!(
                        "const {} = {}[\"{}\"];",
                        field_extracted_name, param_name, field_name
                    ));

                    gen_value_js_to_rust(
                        field_extracted_name,
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
        _ => invocation_params.push(param_name),
    }
}

pub fn gen_value_rust_to_js<W: fmt::Write>(
    value_expr: &str,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => {
            let custom_type = typ.resolve(in_path, env);
            match custom_type {
                ast::CustomType::Struct(strct) => {
                    let strct_size_align = layout::type_size_alignment(typ, in_path, env);
                    let needs_buffer = return_type_form(typ, in_path, env);
                    if needs_buffer != ReturnTypeForm::Complex {
                        todo!("Receiving structs that don't need a buffer")
                    }

                    writeln!(out, "(() => {{")?;
                    let mut iife_indent = indented(out).with_str("  ");
                    writeln!(
                        &mut iife_indent,
                        "const diplomat_receive_buffer = wasm.diplomat_alloc({}, {});",
                        strct_size_align.size(),
                        strct_size_align.align(),
                    )?;
                    writeln!(&mut iife_indent, "{};", value_expr)?;
                    writeln!(
                        &mut iife_indent,
                        "const out = new {}(diplomat_receive_buffer);",
                        strct.name
                    )?;

                    for (name, typ, _) in strct.fields.iter() {
                        gen_box_destructor(name, typ, in_path, env, &mut iife_indent)?;
                    }

                    writeln!(
                        &mut iife_indent,
                        "diplomat_alloc_destroy_registry.register(out, {{"
                    )?;

                    let mut alloc_dict_indent = indented(&mut iife_indent).with_str("  ");
                    writeln!(&mut alloc_dict_indent, "ptr: out.underlying,")?;
                    writeln!(&mut alloc_dict_indent, "size: {},", strct_size_align.size())?;
                    writeln!(
                        &mut alloc_dict_indent,
                        "align: {},",
                        strct_size_align.align()
                    )?;
                    writeln!(&mut iife_indent, "}});")?;
                    writeln!(&mut iife_indent, "return out;")?;
                    write!(out, "}})()")?;
                }

                ast::CustomType::Enum(enm) => {
                    write!(out, "{}_rust_to_js[{}]", enm.name, value_expr)?;
                }

                ast::CustomType::Opaque(_) => {
                    panic!("Opaque types cannot be used in value position")
                }
            }
        }

        ast::TypeName::Box(underlying) => {
            writeln!(out, "(() => {{")?;
            let mut iife_indent = indented(out).with_str("  ");
            write!(&mut iife_indent, "const out = ")?;
            gen_rust_reference_to_js(
                underlying.as_ref(),
                in_path,
                value_expr,
                "null", // JS owns the box
                env,
                &mut iife_indent,
            )?;
            writeln!(&mut iife_indent, ";")?;

            if let ast::TypeName::Named(_) = underlying.as_ref() {
                writeln!(
                    &mut iife_indent,
                    "{}_box_destroy_registry.register(out, out.underlying)",
                    underlying.resolve(in_path, env).name()
                )?;
            }

            writeln!(&mut iife_indent, "return out;")?;
            write!(out, "}})()")?;
        }

        ast::TypeName::Option(underlying) => {
            assert!(
                underlying.is_pointer(),
                "Options must contain pointer types"
            );
            writeln!(out, "(() => {{")?;
            let mut iife_indent = indented(out).with_str("  ");
            writeln!(&mut iife_indent, "const option_value = {}", value_expr)?;

            writeln!(&mut iife_indent, "if (option_value !== 0) {{")?;

            let mut if_indent = indented(&mut iife_indent).with_str("  ");
            write!(&mut if_indent, "const inhabited_value = ")?;
            // TODO(#62): actually return `null` if the option is `None`
            gen_value_rust_to_js(
                "option_value",
                underlying.as_ref(),
                in_path,
                env,
                &mut (&mut if_indent as &mut dyn fmt::Write),
            )?;
            writeln!(&mut if_indent, ";")?;

            writeln!(&mut if_indent, "return inhabited_value;")?;

            writeln!(&mut iife_indent, "}} else {{")?;
            writeln!(&mut iife_indent, "  return null;")?;
            writeln!(&mut iife_indent, "}}")?;

            write!(out, "}})()")?;
        }

        ast::TypeName::Result(ok, err) => {
            let (ok_offset, result_size_align) =
                layout::result_ok_offset_size_align(ok, err, in_path, env);
            let needs_buffer = return_type_form(typ, in_path, env) == ReturnTypeForm::Complex;
            writeln!(out, "(() => {{")?;
            let mut iife_indent = indented(out).with_str("  ");
            if needs_buffer {
                writeln!(
                    &mut iife_indent,
                    "const diplomat_receive_buffer = wasm.diplomat_alloc({}, {});",
                    result_size_align.size(),
                    result_size_align.align()
                )?;
            }

            if needs_buffer {
                writeln!(&mut iife_indent, "const result_tag = {{}};")?;
                writeln!(
                    &mut iife_indent,
                    "diplomat_alloc_destroy_registry.register(result_tag, {{"
                )?;
                let mut alloc_dict_indent = indented(&mut iife_indent).with_str("  ");
                writeln!(&mut alloc_dict_indent, "ptr: diplomat_receive_buffer,")?;
                writeln!(
                    &mut alloc_dict_indent,
                    "size: {},",
                    result_size_align.size()
                )?;
                writeln!(
                    &mut alloc_dict_indent,
                    "align: {},",
                    result_size_align.align()
                )?;
                writeln!(&mut iife_indent, "}});")?;
            }

            if !needs_buffer {
                write!(&mut iife_indent, "const is_ok = ")?;
                write!(&mut iife_indent, "{}", value_expr)?;
                writeln!(&mut iife_indent, " == 1;")?;
            } else {
                write!(&mut iife_indent, "{}", value_expr)?;
                writeln!(&mut iife_indent, ";")?;
                write!(&mut iife_indent, "const is_ok = ")?;
                gen_rust_reference_to_js(
                    &ast::TypeName::Primitive(PrimitiveType::bool),
                    in_path,
                    &format!("diplomat_receive_buffer + {}", ok_offset),
                    "result_tag",
                    env,
                    &mut ((&mut iife_indent) as &mut dyn fmt::Write),
                )?;
                writeln!(&mut iife_indent, ";")?;
            }

            if needs_buffer {
                writeln!(&mut iife_indent, "if (is_ok) {{")?;

                let mut ok_indent = indented(&mut iife_indent).with_str("  ");

                write!(&mut ok_indent, "const ok_value = ")?;
                gen_rust_reference_to_js(
                    ok.as_ref(),
                    in_path,
                    "diplomat_receive_buffer",
                    "result_tag",
                    env,
                    &mut ((&mut ok_indent) as &mut dyn fmt::Write),
                )?;
                writeln!(&mut ok_indent, ";")?;

                writeln!(&mut ok_indent, "return ok_value;")?;

                writeln!(&mut iife_indent, "}} else {{")?;
                let mut err_indent = indented(&mut iife_indent).with_str("  ");

                write!(&mut err_indent, "const throw_value = ")?;
                gen_rust_reference_to_js(
                    err.as_ref(),
                    in_path,
                    "diplomat_receive_buffer",
                    "result_tag",
                    env,
                    &mut ((&mut err_indent) as &mut dyn fmt::Write),
                )?;
                writeln!(&mut err_indent, ";")?;

                writeln!(
                    &mut err_indent,
                    "throw new diplomatRuntime.FFIError(throw_value);"
                )?;
            } else {
                writeln!(&mut iife_indent, "if (!is_ok) {{")?;
                let mut err_indent = indented(&mut iife_indent).with_str("  ");
                writeln!(&mut err_indent, "throw new diplomatRuntime.FFIError({{}});")?;
            }

            writeln!(&mut iife_indent, "}}")?;

            write!(out, "}})()")?;
        }

        ast::TypeName::Primitive(_prim) => {
            // TODO(#63): wrap with appropriate types for large widths
            write!(out, "{}", value_expr)?;
        }

        ast::TypeName::Reference(underlying, _mutability) => {
            // TODO(#12): pass in lifetime of the reference
            gen_rust_reference_to_js(underlying.as_ref(), in_path, value_expr, "null", env, out)?;
        }
        ast::TypeName::Writeable => todo!(),
        ast::TypeName::StrReference => todo!(),
        ast::TypeName::PrimitiveSlice(_) => todo!(),
        ast::TypeName::Unit => write!(out, "{}", value_expr)?,
    }

    Ok(())
}

fn gen_box_destructor<W: fmt::Write>(
    name: &str,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> Result<(), fmt::Error> {
    match typ {
        ast::TypeName::Box(underlying) => {
            writeln!(out, "const out_{}_value = out.{};", name, name)?;
            // TODO(#12): delete back-references when we start generating them
            // since the out value getter returns a borrowed box
            if let ast::TypeName::Named(_) = underlying.as_ref() {
                writeln!(
                    out,
                    "{}_box_destroy_registry.register(out_{}_value, out_{}_value.underlying);",
                    underlying.resolve(in_path, env).name(),
                    name,
                    name
                )?;
            }
            writeln!(
                out,
                "Object.defineProperty(out, \"{}\", {{ value: out_{}_value }});",
                name, name
            )?;
        }

        ast::TypeName::Option(underlying) => {
            assert!(
                underlying.is_pointer(),
                "Options must contain pointer types"
            );

            writeln!(out, "if (out.{}.underlying !== 0) {{", name)?;

            let mut if_indent = indented(out).with_str("  ");

            gen_box_destructor(
                name,
                underlying.as_ref(),
                in_path,
                env,
                &mut (&mut if_indent as &mut dyn fmt::Write),
            )?;

            writeln!(out, "}} else {{")?;
            writeln!(
                out,
                "  Object.defineProperty(out, \"{}\", {{ value: null }});",
                name
            )?;
            writeln!(out, "}}")?;

            // TODO(#62): don't generate destructor if null
        }

        _ => {}
    }

    Ok(())
}

fn gen_rust_reference_to_js<W: fmt::Write>(
    underlying: &ast::TypeName,
    in_path: &ast::Path,
    value_expr: &str,
    owner: &str,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match underlying {
        ast::TypeName::Box(typ) => {
            gen_rust_reference_to_js(
                typ.as_ref(),
                in_path,
                &format!(
                    "(new Uint32Array(wasm.memory.buffer, {}, 1))[0]",
                    value_expr
                ),
                owner,
                env,
                out,
            )?;
        }

        ast::TypeName::Reference(typ, _) => {
            gen_rust_reference_to_js(
                typ.as_ref(),
                in_path,
                &format!(
                    "(new Uint32Array(wasm.memory.buffer, {}, 1))[0]",
                    value_expr
                ),
                "null", // TODO(#12): pass in lifetime of the reference
                env,
                out,
            )?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                // TODO(#62): return null if pointer is 0
                gen_rust_reference_to_js(
                    underlying.as_ref(),
                    in_path,
                    value_expr,
                    owner,
                    env,
                    out,
                )?;
            }
            _ => todo!(),
        },

        ast::TypeName::Result(_, _) => {
            todo!("Receiving references to results")
        }

        ast::TypeName::Named(_) => {
            let custom_type = underlying.resolve(in_path, env);

            if let ast::CustomType::Enum(enm) = custom_type {
                write!(out, "{}_rust_to_js[", enm.name)?;
                gen_rust_reference_to_js(
                    &ast::TypeName::Primitive(PrimitiveType::isize),
                    in_path,
                    value_expr,
                    owner,
                    env,
                    out,
                )?;
                write!(out, "]")?;
            } else {
                writeln!(out, "(() => {{")?;
                let mut iife_indent = indented(out).with_str("  ");
                writeln!(
                    &mut iife_indent,
                    "const out = new {}({});",
                    custom_type.name(),
                    value_expr
                )?;

                writeln!(&mut iife_indent, "out.owner = {};", owner)?;

                writeln!(&mut iife_indent, "return out;")?;
                write!(out, "}})()")?;
            }
        }

        ast::TypeName::Primitive(prim) => {
            if let PrimitiveType::bool = prim {
                write!(
                    out,
                    "(new Uint8Array(wasm.memory.buffer, {}, 1))[0] == 1",
                    value_expr
                )?;
            } else if let PrimitiveType::char = prim {
                write!(
                    out,
                    "String.fromCharCode((new Uint32Array(wasm.memory.buffer, {}, 1))[0])",
                    value_expr
                )?;
            } else {
                let prim_type = match prim {
                    PrimitiveType::i8 => "Int8Array",
                    PrimitiveType::u8 => "Uint8Array",
                    PrimitiveType::i16 => "Int16Array",
                    PrimitiveType::u16 => "Uint16Array",
                    PrimitiveType::i32 => "Int32Array",
                    PrimitiveType::u32 => "Uint32Array",
                    PrimitiveType::i64 => "BigInt64Array",
                    PrimitiveType::u64 => "BigUint64Array",
                    PrimitiveType::i128 => panic!("i128 not supported on JS"),
                    PrimitiveType::u128 => panic!("u128 not supported on JS"),
                    PrimitiveType::isize => "Int32Array",
                    PrimitiveType::usize => "Uint32Array",
                    PrimitiveType::f32 => "Float32Array",
                    PrimitiveType::f64 => "Float64Array",
                    PrimitiveType::bool => panic!(),
                    PrimitiveType::char => panic!(),
                };

                write!(
                    out,
                    "(new {}(wasm.memory.buffer, {}, 1))[0]",
                    prim_type, value_expr
                )?;
            }
        }

        ast::TypeName::Unit => {
            write!(out, "{{}}")?;
        }

        _ => todo!(),
    }

    Ok(())
}
