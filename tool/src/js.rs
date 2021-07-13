use core::panic;
use std::{collections::HashMap, fmt};
use std::{fmt::Write, usize};

use diplomat_core::ast::{self, PrimitiveType};
use indenter::indented;

use crate::layout;

pub fn gen_bindings<W: fmt::Write>(
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    writeln!(out, "import wasm from \"./wasm.mjs\"")?;
    writeln!(
        out,
        "import * as diplomatRuntime from \"./diplomat-runtime.mjs\""
    )?;

    let mut all_types: Vec<&ast::CustomType> = env.values().collect();
    all_types.sort_by_key(|t| t.name());
    for custom_type in all_types {
        writeln!(out)?;
        gen_struct(out, custom_type, env)?;
    }

    Ok(())
}

fn gen_struct<W: fmt::Write>(
    out: &mut W,
    custom_type: &ast::CustomType,
    env: &HashMap<String, ast::CustomType>,
) -> fmt::Result {
    writeln!(
        out,
        "const {}_box_destroy_registry = new FinalizationRegistry(underlying => {{",
        custom_type.name()
    )?;
    writeln!(
        indented(out).with_str("  "),
        "wasm.{}_destroy(underlying);",
        custom_type.name()
    )?;
    writeln!(out, "}});")?;
    writeln!(out)?;

    writeln!(
        out,
        "const {}_alloc_destroy_registry = new FinalizationRegistry(obj => {{",
        custom_type.name()
    )?;
    writeln!(
        indented(out).with_str("  "),
        "wasm.{}_drop_ptr(obj[\"ptr\"]);",
        custom_type.name()
    )?;
    writeln!(
        indented(out).with_str("  "),
        "wasm.diplomat_free(obj[\"ptr\"], obj[\"size\"]);"
    )?;
    writeln!(out, "}});")?;
    writeln!(out)?;

    writeln!(out, "export class {} {{", custom_type.name())?;

    let mut class_body_out = indented(out).with_str("  ");

    writeln!(&mut class_body_out, "constructor(underlying) {{")?;
    writeln!(
        indented(&mut class_body_out).with_str("  "),
        "this.underlying = underlying;"
    )?;
    writeln!(&mut class_body_out, "}}")?;

    for method in custom_type.methods().iter() {
        writeln!(&mut class_body_out)?;
        gen_method(method, env, &mut class_body_out)?;
    }

    if let ast::CustomType::Struct(strct) = custom_type {
        let (_, offsets, _) = layout::struct_size_offsets_max_align(strct, env);
        for ((name, typ), offset) in strct.fields.iter().zip(offsets.iter()) {
            writeln!(&mut class_body_out)?;
            gen_field(name, typ, *offset, env, &mut class_body_out)?;
        }
    }

    writeln!(out, "}}")?;
    Ok(())
}

fn gen_field<W: fmt::Write>(
    name: &str,
    typ: &ast::TypeName,
    offset: usize,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    writeln!(out, "{}() {{", name)?;
    let mut method_body_out = indented(out).with_str("  ");
    write!(&mut method_body_out, "return ")?;
    gen_value_rust_to_js(
        &|out| write!(out, "this.underlying + {}", offset),
        &ast::TypeName::Reference(Box::new(typ.clone()), true),
        env,
        &mut method_body_out,
    )?;
    writeln!(&mut method_body_out, ";")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    let is_writeable = method.is_writeable_out();

    let mut pre_stmts = vec![];
    let mut all_param_exprs = vec![];
    let mut post_stmts = vec![];

    method.params.iter().for_each(|p| {
        gen_value_js_to_rust(
            p.name.clone(),
            &p.ty,
            env,
            &mut pre_stmts,
            &mut all_param_exprs,
            &mut post_stmts,
        )
    });

    let mut all_params = method
        .params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>();

    if is_writeable {
        let last_index_exprs = all_param_exprs.len() - 1;
        all_param_exprs[last_index_exprs] = "writeable".to_string();

        all_params.remove(all_params.len() - 1);
    }

    let all_params_invocation = {
        if let Some(ast::TypeName::Named(_)) = &method.return_type {
            if let ast::CustomType::Struct(_) = method.return_type.as_ref().unwrap().resolve(env) {
                all_param_exprs.insert(0, "diplomat_receive_buffer".to_string());
            }
        }

        if method.self_param.is_some() {
            all_param_exprs.insert(0, "this.underlying".to_string());
        }

        all_param_exprs.join(", ")
    };

    if method.self_param.is_some() {
        writeln!(out, "{}({}) {{", method.name, all_params.join(", "))?;
    } else {
        writeln!(out, "static {}({}) {{", method.name, all_params.join(", "))?;
    }

    let mut method_body_out = indented(out).with_str("  ");

    for s in pre_stmts.iter() {
        writeln!(&mut method_body_out, "{}", s)?
    }

    let invocation_expr = format!("wasm.{}({})", method.full_path_name, all_params_invocation);

    match &method.return_type {
        Some(ret_type) => {
            write!(&mut method_body_out, "const diplomat_out = ")?;
            gen_value_rust_to_js(
                &|out| write!(out, "{}", invocation_expr),
                ret_type,
                env,
                &mut method_body_out,
            )?;
            writeln!(&mut method_body_out, ";")?;
        }

        None => {
            if is_writeable {
                writeln!(
                    &mut method_body_out,
                    "const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {});",
                    invocation_expr
                )?;
            } else {
                writeln!(&mut method_body_out, "{}", invocation_expr)?;
            };
        }
    }

    for s in post_stmts.iter() {
        writeln!(&mut method_body_out, "{}", s)?
    }

    if method.return_type.is_some() || is_writeable {
        writeln!(&mut method_body_out, "return diplomat_out;")?;
    }

    writeln!(out, "}}")?;

    Ok(())
}

fn gen_value_js_to_rust(
    param_name: String,
    typ: &ast::TypeName,
    _env: &HashMap<String, ast::CustomType>,
    pre_logic: &mut Vec<String>,
    invocation_params: &mut Vec<String>,
    post_logic: &mut Vec<String>,
) {
    match typ {
        ast::TypeName::StrReference => {
            // TODO(shadaj): consider extracting into runtime function
            pre_logic.push(format!(
                "let {}_diplomat_bytes = (new TextEncoder()).encode({});",
                param_name, param_name
            ));
            pre_logic.push(format!(
                "let {}_diplomat_ptr = wasm.diplomat_alloc({}_diplomat_bytes.length);",
                param_name, param_name
            ));
            pre_logic.push(format!("let {}_diplomat_buf = new Uint8Array(wasm.memory.buffer, {}_diplomat_ptr, {}_diplomat_bytes.length);", param_name, param_name, param_name));
            pre_logic.push(format!(
                "{}_diplomat_buf.set({}_diplomat_bytes, 0);",
                param_name, param_name
            ));

            invocation_params.push(format!("{}_diplomat_ptr", param_name));
            invocation_params.push(format!("{}_diplomat_bytes.length", param_name));

            post_logic.push(format!(
                "wasm.diplomat_free({}_diplomat_ptr, {}_diplomat_bytes.length);",
                param_name, param_name
            ));
        }
        ast::TypeName::Box(_) => {
            invocation_params.push(format!("{}.underlying", param_name));
        }
        ast::TypeName::Reference(_, _) => {
            invocation_params.push(format!("{}.underlying", param_name));
        }
        _ => invocation_params.push(param_name),
    }
}

fn gen_value_rust_to_js<W: fmt::Write>(
    value_expr: &dyn Fn(&mut dyn fmt::Write) -> fmt::Result,
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => {
            let custom_type = typ.resolve(env);
            match custom_type {
                ast::CustomType::Struct(strct) => {
                    let (strct_size, _, _) = layout::struct_size_offsets_max_align(strct, env);
                    writeln!(out, "(() => {{")?;
                    let mut iife_indent = indented(out).with_str("  ");
                    writeln!(
                        &mut iife_indent,
                        "const diplomat_receive_buffer = wasm.diplomat_alloc({});",
                        strct_size
                    )?;
                    value_expr(&mut iife_indent)?;
                    writeln!(&mut iife_indent, ";")?;
                    writeln!(
                        &mut iife_indent,
                        "const out = new {}(diplomat_receive_buffer);",
                        strct.name
                    )?;
                    writeln!(
                        &mut iife_indent,
                        "{}_alloc_destroy_registry.register(out, {{",
                        strct.name
                    )?;

                    let mut alloc_dict_indent = indented(&mut iife_indent).with_str("  ");
                    writeln!(&mut alloc_dict_indent, "ptr: out.underlying,")?;
                    writeln!(&mut alloc_dict_indent, "size: {}", strct_size)?;
                    writeln!(&mut iife_indent, "}});")?;
                    writeln!(&mut iife_indent, "return out;")?;
                    write!(out, "}})()")?;
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
            gen_rust_reference_to_js(underlying.as_ref(), value_expr, env, &mut iife_indent)?;
            writeln!(&mut iife_indent, ";")?;

            if let ast::TypeName::Named(_) = underlying.as_ref() {
                writeln!(
                    &mut iife_indent,
                    "{}_box_destroy_registry.register(out, out.underlying)",
                    underlying.resolve(env).name()
                )?;
            }

            writeln!(&mut iife_indent, "return out;")?;
            write!(out, "}})()")?;
        }

        ast::TypeName::Primitive(_prim) => {
            // TODO(shadaj): wrap with appropriate types for large widths
            value_expr(out)?;
        }

        ast::TypeName::Reference(underlying, _mutability) => {
            gen_rust_reference_to_js(underlying.as_ref(), value_expr, env, out)?;
        }
        ast::TypeName::Writeable => todo!(),
        ast::TypeName::StrReference => todo!(),
    }

    Ok(())
}

fn gen_rust_reference_to_js<W: fmt::Write>(
    underlying: &ast::TypeName,
    value_expr: &dyn Fn(&mut dyn fmt::Write) -> fmt::Result,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match underlying {
        ast::TypeName::Box(typ) | ast::TypeName::Reference(typ, _) => {
            gen_rust_reference_to_js(
                typ.as_ref(),
                &|out| {
                    write!(out, "(new Uint32Array(wasm.memory.buffer, ")?;
                    value_expr(out)?;
                    write!(out, ", 1))[0]")?;
                    Ok(())
                },
                env,
                out,
            )?;
        }

        ast::TypeName::Named(_) => {
            let custom_type = underlying.resolve(env);
            writeln!(out, "(() => {{")?;
            let mut iife_indent = indented(out).with_str("  ");
            write!(&mut iife_indent, "const out = new {}(", custom_type.name())?;
            value_expr(&mut iife_indent)?;
            writeln!(&mut iife_indent, ");")?;

            // TODO(shadaj): add lifetime references

            writeln!(&mut iife_indent, "return out;")?;
            write!(out, "}})()")?;
        }

        ast::TypeName::Primitive(prim) => {
            if let PrimitiveType::bool = prim {
                write!(out, "(new Uint8Array(wasm.memory.buffer, ")?;
                value_expr(out)?;
                write!(out, ", 1))[0] == 1")?;
            } else if let PrimitiveType::char = prim {
                write!(
                    out,
                    "String.fromCharCode((new Uint8Array(wasm.memory.buffer, "
                )?;
                value_expr(out)?;
                write!(out, ", 1))[0])")?;
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

                write!(out, "new {}(wasm.memory.buffer, ", prim_type)?;
                value_expr(out)?;
                write!(out, ", 1))[0]")?;
            }
        }

        _ => todo!(),
    }

    Ok(())
}
