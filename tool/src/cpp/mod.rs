use core::panic;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast::{self, PrimitiveType};
use indenter::indented;

pub fn gen_bindings(
    env: &HashMap<String, ast::CustomType>,
    outs: &mut HashMap<&str, String>,
) -> fmt::Result {
    super::c::gen_bindings(env, outs)?;

    let out = outs.entry("api.hpp").or_insert_with(String::new);
    writeln!(out, "#include <stdint.h>")?;
    writeln!(out, "#include <stddef.h>")?;
    writeln!(out, "#include <stdbool.h>")?;
    writeln!(out, "#include <memory>")?;
    writeln!(out)?;
    writeln!(out, "namespace capi {{")?;
    writeln!(out, "#include \"api.h\"")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    let mut all_types: Vec<&ast::CustomType> = env.values().collect();
    all_types.sort_by_key(|t| t.name());
    for custom_type in &all_types {
        writeln!(out)?;
        gen_struct(custom_type, env, out)?;
    }

    Ok(())
}

fn gen_struct<W: fmt::Write>(
    custom_type: &ast::CustomType,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    writeln!(out, "struct {}Deleter {{", custom_type.name())?;
    let mut deleter_body = indented(out).with_str("  ");
    writeln!(
        &mut deleter_body,
        "void operator()(capi::{}* l) const noexcept {{",
        custom_type.name()
    )?;
    let mut deleter_operator_body = indented(&mut deleter_body).with_str("  ");
    writeln!(
        &mut deleter_operator_body,
        "capi::{}_destroy(l);",
        custom_type.name()
    )?;
    writeln!(&mut deleter_body, "}}")?;
    writeln!(out, "}};")?;

    match custom_type {
        ast::CustomType::Opaque(opaque) => {
            writeln!(out, "class {} {{", opaque.name)?;
            writeln!(out, " public:")?;

            let mut public_body = indented(out).with_str("  ");
            for method in &opaque.methods {
                gen_method(method, env, &mut public_body)?;
            }

            writeln!(
                &mut public_body,
                "inline const capi::{}* AsFFI() const {{ return this->inner.get(); }}",
                opaque.name
            )?;

            writeln!(out, " private:")?;
            let mut private_body = indented(out).with_str("  ");
            writeln!(
                &mut private_body,
                "{}(capi::{}* i) : inner(i) {{}}",
                opaque.name, opaque.name
            )?;
            writeln!(
                &mut private_body,
                "std::unique_ptr<capi::{}, {}Deleter> inner;",
                opaque.name, opaque.name
            )?;
            writeln!(out, "}};")?;
        }

        ast::CustomType::Struct(_strct) => {
            // TODO(shadaj): wrap non-opaque structs
        }
    }

    Ok(())
}

fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match &method.return_type {
        Some(ret_type) => {
            gen_type(ret_type, false, env, out)?;
        }

        None => {
            write!(out, "void")?;
        }
    }

    write!(out, " {}(", method.full_path_name)?;
    let mut params_to_gen = method.params.clone();
    if let Some(param) = &method.self_param {
        params_to_gen.insert(0, param.clone());
    }

    let mut all_params_invocation = vec![];
    for (i, param) in params_to_gen.iter().enumerate() {
        if i != 0 {
            write!(out, ", ")?;
        }

        if param.ty == ast::TypeName::StrReference {
            write!(
                out,
                "const char* {}_data, size_t {}_len",
                param.name, param.name
            )?;
            all_params_invocation.push(format!("{}_data", param.name));
            all_params_invocation.push(format!("{}_len", param.name));
        } else {
            gen_type(&param.ty, false, env, out)?;
            write!(out, " {}", param.name)?;
            all_params_invocation.push(gen_cpp_to_rust(
                &param.name,
                &param.ty,
                env,
                param.name == "self",
            ));
        }
    }

    writeln!(out, ") {{")?;

    let mut method_body = indented(out).with_str("  ");
    if let Some(ret_typ) = &method.return_type {
        write!(
            &mut method_body,
            "return {};",
            gen_rust_to_cpp(
                &format!(
                    "capi::{}({})",
                    method.full_path_name,
                    all_params_invocation.join(", ")
                ),
                ret_typ,
                env
            )
        )?;
    } else {
        writeln!(
            &mut method_body,
            "capi::{}({});",
            method.full_path_name,
            all_params_invocation.join(", ")
        )?;
    }

    writeln!(out, "}}")?;

    Ok(())
}

fn gen_type<W: fmt::Write>(
    typ: &ast::TypeName,
    behind_ref: bool,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => {
            if behind_ref {
                write!(out, "{}", typ.resolve(env).name())?;
            } else {
                write!(out, "capi::{}", typ.resolve(env).name())?;
            }
        }

        ast::TypeName::Box(underlying) => {
            gen_type(underlying.as_ref(), true, env, out)?;
            if behind_ref {
                write!(out, "*")?;
            }
        }

        ast::TypeName::Reference(underlying, mutable) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type(underlying.as_ref(), true, env, out)?;
            write!(out, "&")?;
            if behind_ref {
                write!(out, "*")?;
            }
        }

        ast::TypeName::Primitive(prim) => {
            let prim_type = match prim {
                PrimitiveType::i8 => "int8_t",
                PrimitiveType::u8 => "uint8_t",
                PrimitiveType::i16 => "int16_t",
                PrimitiveType::u16 => "uint16_t",
                PrimitiveType::i32 => "int32_t",
                PrimitiveType::u32 => "uint32_t",
                PrimitiveType::i64 => "int64_t",
                PrimitiveType::u64 => "uint64_t",
                PrimitiveType::i128 => panic!("i128 not supported in C"),
                PrimitiveType::u128 => panic!("u128 not supported in C"),
                PrimitiveType::isize => "ssize_t",
                PrimitiveType::usize => "size_t",
                PrimitiveType::f32 => "float",
                PrimitiveType::f64 => "double",
                PrimitiveType::bool => "bool",
                PrimitiveType::char => "char",
            };

            write!(out, "{}", prim_type)?;

            if behind_ref {
                write!(out, "*")?;
            }
        }

        ast::TypeName::Writeable => {
            write!(out, "capi::DiplomatWriteable")?;
        }
        ast::TypeName::StrReference => panic!(),
    }

    Ok(())
}

fn gen_rust_to_cpp(
    cpp: &str,
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
) -> String {
    match typ {
        ast::TypeName::Box(underlying) => match underlying.as_ref() {
            ast::TypeName::Named(_name) => match underlying.resolve(env) {
                ast::CustomType::Opaque(opaque) => {
                    return format!("{}({})", opaque.name, cpp);
                }

                ast::CustomType::Struct(_strct) => {
                    todo!()
                }
            },
            _o => todo!(),
        },
        ast::TypeName::Named(_) => cpp.to_string(),
        ast::TypeName::Primitive(_) => cpp.to_string(),
        o => todo!("{:?}", o),
    }
}

fn gen_cpp_to_rust(
    cpp: &str,
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
    is_self: bool,
) -> String {
    match typ {
        ast::TypeName::Reference(underlying, _) => match underlying.as_ref() {
            ast::TypeName::Named(_name) => match underlying.resolve(env) {
                ast::CustomType::Opaque(_opaque) => {
                    if is_self {
                        "this->inner.get()".to_string()
                    } else {
                        return format!("{}.AsFFI()", cpp);
                    }
                }

                ast::CustomType::Struct(_strct) => {
                    todo!()
                }
            },

            ast::TypeName::Writeable => format!("&{}", cpp),

            o => todo!("{:?}", o),
        },
        ast::TypeName::Named(_) => cpp.to_string(),
        ast::TypeName::Primitive(_) => cpp.to_string(),
        o => todo!("{:?}", o),
    }
}
