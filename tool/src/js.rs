use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;
use indoc::formatdoc;

pub fn gen_bindings<W: fmt::Write>(
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    writeln!(out, "import wasm from \"./wasm.mjs\"")?;
    writeln!(
        out,
        "import * as diplomatRuntime from \"./diplomat-runtime.mjs\""
    )?;

    for custom_type in env.values() {
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
        "const {}_destroy_registry = new FinalizationRegistry(underlying => {{",
        custom_type.name()
    )?;
    writeln!(
        indented(out).with_str("  "),
        "wasm.{}_destroy(underlying);",
        custom_type.name()
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

    writeln!(out, "}}")?;
    Ok(())
}

fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    // TODO(shadaj): support results with empty success value
    // TODO(shadaj): reconsider if we should auto-detect writeables
    let is_writeable = method.return_type.is_none()
        && !method.params.is_empty()
        && method.params[method.params.len() - 1].ty
            == ast::TypeName::Reference(Box::new(ast::TypeName::Writeable), true);

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

    if is_writeable {
        let last_index = all_param_exprs.len() - 1;
        all_param_exprs[last_index] = "writeable".to_string();
    }

    let all_params = method
        .params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let all_params_invocation = {
        if method.self_param.is_some() {
            all_param_exprs.insert(0, "this.underlying".to_string());
        }

        all_param_exprs.join(", ")
    };

    if method.self_param.is_some() {
        writeln!(out, "{}({}) {{", method.name, &all_params)?;
    } else {
        writeln!(out, "static {}({}) {{", method.name, &all_params)?;
    }

    let mut method_body_out = indented(out).with_str("  ");

    for s in pre_stmts.iter() {
        writeln!(&mut method_body_out, "{}", s)?
    }

    let invocation_expr = format!("wasm.{}({})", method.full_path_name, all_params_invocation);

    match &method.return_type {
        Some(ret_type) => {
            let value = gen_value_rust_to_js(invocation_expr, ret_type, env);

            writeln!(&mut method_body_out, "const diplomat_out = {};", value)?;
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
        _ => invocation_params.push(param_name),
    }
}

fn gen_value_rust_to_js(
    value_expr: String,
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
) -> String {
    match typ {
        ast::TypeName::Named(name) => {
            todo!("TODO: implement custom type as value {}", name);
        }
        ast::TypeName::Box(underlying) => match underlying.resolve(env) {
            ast::CustomType::Opaque(name) => {
                formatdoc! {"
                    (() => {{
                      const out = new {}({});
                      {}_destroy_registry.register(out, out.underlying);
                      return out;
                    }})()",
                    name.name, value_expr, name.name
                }
            }
            ast::CustomType::Struct(_strct) => {
                todo!()
            }
        },
        ast::TypeName::Primitive(_prim) => {
            // TODO(shadaj): wrap with appropriate types for large widths
            value_expr
        }
        ast::TypeName::Reference(_underlying, _mutability) => {
            todo!()
        }
        ast::TypeName::Writeable => todo!(),
        ast::TypeName::StrReference => todo!(),
    }
}
