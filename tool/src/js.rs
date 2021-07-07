use std::collections::HashMap;

use diplomat_core::ast;

pub fn gen_bindings(env: &HashMap<String, ast::CustomType>) -> Vec<String> {
    let mut out = vec!["import wasm from \"./wasm.mjs\"".to_string()];

    for custom_type in env.values() {
        gen_struct(&mut out, custom_type, env);
    }

    out
}

fn gen_struct(
    out: &mut Vec<String>,
    custom_type: &ast::CustomType,
    env: &HashMap<String, ast::CustomType>,
) {
    out.push(format!(
        "const {}_destroy_registry = new FinalizationRegistry(underlying => {{",
        custom_type.name()
    ));
    out.push(format!("wasm.{}_destroy(underlying);", custom_type.name()));
    out.push("});".to_string());

    out.push(format!("export class {} {{", custom_type.name()));

    out.push("constructor(underlying) {".to_string());
    out.push("this.underlying = underlying;".to_string());
    out.push("}".to_string());

    for method in custom_type.methods().iter() {
        gen_method(method, &env, out);
    }

    out.push("}".to_string());
}

fn gen_method(method: &ast::Method, env: &HashMap<String, ast::CustomType>, out: &mut Vec<String>) {
    let all_params = method
        .params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    if method.self_param.is_some() {
        out.push(format!("{}({}) {{", method.name, &all_params));
        match &method.return_type {
            Some(ret_type) => {
                let value = gen_value(
                    format!(
                        "wasm.{}(this.underlying, {})",
                        method.full_path_name, all_params
                    ),
                    ret_type,
                    env,
                );
                out.push(format!("return {};", value));
            }

            None => {
                out.push(format!(
                    "wasm.{}(this.underlying, {});",
                    method.full_path_name, all_params
                ));
            }
        }
    } else {
        out.push(format!("static {}({}) {{", method.name, &all_params));
        match &method.return_type {
            Some(ret_type) => {
                let value = gen_value(
                    format!("wasm.{}({})", method.full_path_name, all_params),
                    ret_type,
                    env,
                );
                out.push(format!("return {};", value));
            }

            None => {
                out.push(format!("wasm.{}({});", method.full_path_name, all_params));
            }
        }
    }

    out.push("}".to_string());
}

fn gen_value(
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
                format!(
                    "(() => {{\n\
                      const out = new {}({});\n\
                      {}_destroy_registry.register(out, out.underlying);\n\
                      return out;\n\
                    }})()",
                    name.name, value_expr, name.name
                )
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
    }
}
