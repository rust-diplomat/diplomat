use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast::{self, PrimitiveType};
use indenter::indented;

use crate::util;

static RUNTIME_H: &str = include_str!("runtime.h");

pub fn gen_bindings(
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
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

    let mut all_types = util::get_all_custom_types(env);
    all_types.sort_by_key(|t| t.1.name());

    for (in_path, custom_type) in &all_types {
        match custom_type {
            ast::CustomType::Opaque(_) => {
                writeln!(out)?;
                gen_struct(custom_type, in_path, env, out)?;
            }

            ast::CustomType::Enum(enm) => {
                writeln!(out)?;
                writeln!(out, "enum {} {{", enm.name)?;
                let mut enum_body_out = indented(out).with_str("  ");
                for (name, discriminant, _) in enm.variants.iter() {
                    writeln!(
                        &mut enum_body_out,
                        "{}_{} = {},",
                        enm.name, name, discriminant
                    )?;
                }
                writeln!(out, "}};")?;
            }

            ast::CustomType::Struct(_) => {}
        }
    }

    let mut structs_seen = HashSet::new();
    let mut structs_order = Vec::new();
    for (in_path, custom_type) in &all_types {
        if let ast::CustomType::Struct(strct) = custom_type {
            if !structs_seen.contains(&strct.name) {
                topological_sort_structs(
                    strct,
                    (*in_path).clone(),
                    &mut structs_seen,
                    &mut structs_order,
                    env,
                );
            }
        }
    }

    for (in_path, strct) in structs_order {
        writeln!(out)?;
        gen_struct(&ast::CustomType::Struct(strct.clone()), &in_path, env, out)?;
    }

    for (in_path, custom_type) in all_types {
        for method in custom_type.methods() {
            writeln!(out)?;
            gen_method(method, in_path, env, out)?;
        }

        write!(out, "void {}_destroy(", custom_type.name())?;

        gen_type(
            &ast::TypeName::Box(Box::new(ast::TypeName::Named(
                ast::Path::empty().sub_path(custom_type.name().clone()),
            ))),
            in_path,
            env,
            out,
        )?;

        writeln!(out, " self);")?;
    }

    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "}}")?;
    writeln!(out, "#endif")?;

    Ok(())
}

fn gen_struct<W: fmt::Write>(
    custom_type: &ast::CustomType,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
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
                gen_field(name, typ, in_path, env, &mut class_body_out)?;
            }
            writeln!(out)?;
            writeln!(out, "}} {};", strct.name)?;
        }

        ast::CustomType::Enum(_) => {}
    }

    Ok(())
}

fn gen_field<W: fmt::Write>(
    name: &str,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    gen_type(typ, in_path, env, out)?;
    write!(out, " {};", name)?;

    Ok(())
}

fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    match &method.return_type {
        Some(ret_type) => {
            gen_type(ret_type, in_path, env, out)?;
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
            gen_type(&param.ty, in_path, env, out)?;
            write!(out, " {}", param.name)?;
        }
    }

    writeln!(out, ");")?;

    Ok(())
}

fn gen_type<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            r @ ast::CustomType::Struct(_) | r @ ast::CustomType::Opaque(_) => {
                write!(out, "{}", r.name())?;
            }

            ast::CustomType::Enum(_) => {
                write!(out, "ssize_t")?;
            }
        },

        ast::TypeName::Box(underlying) => {
            gen_type(underlying.as_ref(), in_path, env, out)?;
            write!(out, "*")?;
        }

        ast::TypeName::Reference(underlying, mutable) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type(underlying.as_ref(), in_path, env, out)?;
            write!(out, "*")?;
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", c_type_for_prim(prim))?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                gen_type(underlying.as_ref(), in_path, env, out)?;
            }

            _ => todo!(),
        },

        ast::TypeName::Writeable => write!(out, "DiplomatWriteable")?,
        ast::TypeName::StrReference => panic!(),
    }

    Ok(())
}

pub fn c_type_for_prim(prim: &PrimitiveType) -> &str {
    match prim {
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
    }
}

pub fn topological_sort_structs<'a>(
    root: &'a ast::Struct,
    in_path: ast::Path,
    seen: &mut HashSet<String>,
    order: &mut Vec<(ast::Path, &'a ast::Struct)>,
    env: &'a HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) {
    seen.insert(root.name.clone());
    for (_, typ, _) in &root.fields {
        if let ast::TypeName::Named(_) = typ {
            match typ.resolve_with_path(&in_path, env) {
                (path, ast::CustomType::Struct(strct)) => {
                    if !seen.contains(&strct.name) {
                        topological_sort_structs(strct, path, seen, order, env);
                    }
                }
                (_, ast::CustomType::Opaque(_) | ast::CustomType::Enum(_)) => {}
            }
        }
    }

    order.push((in_path, root));
}
