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
    if method.return_type.is_none()
        && !method.params.is_empty()
        && method.params[method.params.len() - 1].ty
            == ast::TypeName::Reference(Box::new(ast::TypeName::Writeable), true)
    {
        gen_writeable_method(method, out)?;
    } else {
        let all_params = method
            .params
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        if method.self_param.is_some() {
            writeln!(out, "{}({}) {{", method.name, &all_params)?;
            let mut method_body_out = indented(out).with_str("  ");
            match &method.return_type {
                Some(ret_type) => {
                    let value = gen_value_rust_to_js(
                        format!(
                            "wasm.{}(this.underlying, {})",
                            method.full_path_name, all_params
                        ),
                        ret_type,
                        env,
                    );
                    writeln!(&mut method_body_out, "return {};", value)?;
                }

                None => {
                    writeln!(
                        &mut method_body_out,
                        "wasm.{}(this.underlying, {});",
                        method.full_path_name, all_params
                    )?;
                }
            }
        } else {
            writeln!(out, "static {}({}) {{", method.name, &all_params)?;
            let mut method_body_out = indented(out).with_str("  ");
            match &method.return_type {
                Some(ret_type) => {
                    let value = gen_value_rust_to_js(
                        format!("wasm.{}({})", method.full_path_name, all_params),
                        ret_type,
                        env,
                    );
                    writeln!(&mut method_body_out, "return {};", value)?;
                }

                None => {
                    writeln!(
                        &mut method_body_out,
                        "wasm.{}({});",
                        method.full_path_name, all_params
                    )?;
                }
            }
        }

        writeln!(out, "}}")?;
    }

    Ok(())
}

fn gen_writeable_method<W: fmt::Write>(
    method: &ast::Method,
    out: &mut W,
) -> Result<(), fmt::Error> {
    let all_params_except_last = method.params[..method.params.len() - 1]
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    writeln!(out, "{}({}) {{", method.name, &all_params_except_last)?;
    let mut method_body_out = indented(out).with_str("  ");
    if method.self_param.is_some() {
        writeln!(
            &mut method_body_out,
            "return diplomatRuntime.withWriteable(wasm, (writeable) => wasm.{}(this.underlying, {}{}writeable));",
            method.full_path_name, if method.params.len() > 1 {
                ", "
            } else {
                ""
            }, all_params_except_last
        )?;
    } else {
        writeln!(
            &mut method_body_out,
            "return diplomatRuntime.withWriteable(wasm, (writeable) => wasm.{}({}{}writeable));",
            method.full_path_name,
            if method.params.len() > 1 { ", " } else { "" },
            all_params_except_last
        )?;
    }
    writeln!(out, "}}")?;
    Ok(())
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
    }
}
