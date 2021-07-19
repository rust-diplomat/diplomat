use core::panic;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast::{self, PrimitiveType};
use indenter::indented;

static RUNTIME_H: &str = include_str!("runtime.h");

pub fn gen_bindings(
    env: &HashMap<String, ast::CustomType>,
    outs: &mut HashMap<&str, String>,
) -> fmt::Result {
    let diplomat_runtime_out = outs.entry("diplomat_runtime.h").or_insert_with(String::new);
    write!(diplomat_runtime_out, "{}", RUNTIME_H)?;

    let out = outs.entry("api.h").or_insert_with(String::new);
    writeln!(out, "#include <stdint.h>")?;
    writeln!(out, "#include <stddef.h>")?;
    writeln!(out, "#include <stdbool.h>")?;
    writeln!(out, "#include \"diplomat_runtime.h\"")?;
    writeln!(out)?;
    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "extern \"C\" {{")?;
    writeln!(out, "#endif")?;

    let mut all_types: Vec<&ast::CustomType> = env.values().collect();
    all_types.sort_by_key(|t| t.name());
    for custom_type in &all_types {
        writeln!(out)?;
        gen_struct(custom_type, env, out)?;
    }

    for custom_type in all_types {
        for method in custom_type.methods() {
            writeln!(out)?;
            gen_method(method, env, out)?;
        }
    }

    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "}}")?;
    writeln!(out, "#endif")?;

    Ok(())
}

fn gen_struct<W: fmt::Write>(
    custom_type: &ast::CustomType,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match custom_type {
        ast::CustomType::Opaque(opaque) => {
            writeln!(out, "typedef struct {} {};", opaque.name, opaque.name)?;
        }

        ast::CustomType::Struct(strct) => {
            write!(out, "typedef struct {} {{", strct.name)?;
            let mut class_body_out = indented(out).with_str("    ");
            for (name, typ, _) in strct.fields.iter() {
                writeln!(&mut class_body_out)?;
                gen_field(name, typ, env, &mut class_body_out)?;
            }
            writeln!(out)?;
            writeln!(out, "}} {};", strct.name)?;
        }
    }

    Ok(())
}

fn gen_field<W: fmt::Write>(
    name: &str,
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    gen_type(typ, env, out)?;
    write!(out, " {};", name)?;

    Ok(())
}

fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match &method.return_type {
        Some(ret_type) => {
            gen_type(ret_type, env, out)?;
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
        } else {
            gen_type(&param.ty, env, out)?;
            write!(out, " {}", param.name)?;
        }
    }

    writeln!(out, ");")?;

    Ok(())
}

fn gen_type<W: fmt::Write>(
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => {
            write!(out, "{}", typ.resolve(env).name())?;
        }

        ast::TypeName::Box(underlying) | ast::TypeName::Reference(underlying, _) => {
            gen_type(underlying.as_ref(), env, out)?;
            write!(out, "*")?;
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
        }

        ast::TypeName::Writeable => write!(out, "DiplomatWriteable")?,
        ast::TypeName::StrReference => panic!(),
    }

    Ok(())
}
