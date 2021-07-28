use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast::{self, PrimitiveType};
use indenter::indented;

use crate::util;

static RUNTIME_H: &str = include_str!("runtime.h");

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum StructOrType<'a> {
    Struct(&'a ast::Struct),
    Type(&'a ast::TypeName),
}

pub fn gen_bindings(
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    outs: &mut HashMap<&str, String>,
) -> fmt::Result {
    let diplomat_runtime_out = outs.entry("diplomat_runtime.h").or_insert_with(String::new);
    write!(diplomat_runtime_out, "{}", RUNTIME_H)?;

    let out = outs.entry("api.h").or_insert_with(String::new);
    writeln!(out, "#include <stdio.h>")?;
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
                writeln!(out, "typedef enum {} {{", enm.name)?;
                let mut enum_body_out = indented(out).with_str("  ");
                for (name, discriminant, _) in enm.variants.iter() {
                    writeln!(
                        &mut enum_body_out,
                        "{}_{} = {},",
                        enm.name, name, discriminant
                    )?;
                }
                writeln!(out, "}} {};", enm.name)?;
            }

            ast::CustomType::Struct(_) => {}
        }
    }

    let mut structs_seen = HashSet::new();
    let mut structs_order = Vec::new();
    for (in_path, custom_type) in &all_types {
        if let ast::CustomType::Struct(strct) = custom_type {
            topological_sort_structs(
                StructOrType::Struct(strct),
                ast::Path::clone(in_path),
                &mut structs_seen,
                &mut structs_order,
                env,
            );
        }
    }

    let mut seen_additional_results: HashSet<(&ast::TypeName, &ast::Path)> = HashSet::new();

    for (in_path, strct) in &structs_order {
        writeln!(out)?;
        match strct {
            StructOrType::Struct(strct) => {
                gen_struct(
                    &ast::CustomType::Struct(ast::Struct::clone(strct)),
                    &in_path,
                    env,
                    out,
                )?;
            }

            StructOrType::Type(typ) => {
                gen_result(typ, &in_path, env, out)?;
                seen_additional_results.insert((typ, in_path));
            }
        }
    }

    let mut additional_results = Vec::new();
    for (in_path, custom_type) in &all_types {
        for method in custom_type.methods() {
            method.params.iter().for_each(|param| {
                collect_results(
                    &param.ty,
                    in_path,
                    env,
                    &mut seen_additional_results,
                    &mut additional_results,
                )
            });
            if let Some(return_type) = method.return_type.as_ref() {
                collect_results(
                    return_type,
                    in_path,
                    env,
                    &mut seen_additional_results,
                    &mut additional_results,
                );
            }
        }
    }

    for (typ, in_path) in additional_results.iter() {
        writeln!(out)?;
        gen_result(typ, in_path, env, out)?;
    }

    for (in_path, custom_type) in all_types {
        for method in custom_type.methods() {
            writeln!(out)?;
            gen_method(method, in_path, env, out)?;
        }

        if custom_type.methods().is_empty() {
            writeln!(out)?;
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

fn collect_results<'a, 'b>(
    typ: &'a ast::TypeName,
    in_path: &'b ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    seen: &mut HashSet<(&'a ast::TypeName, &'b ast::Path)>,
    results: &mut Vec<(&'a ast::TypeName, &'b ast::Path)>,
) {
    match typ {
        ast::TypeName::Named(_) => {}
        ast::TypeName::Box(underlying) => {
            collect_results(underlying, in_path, env, seen, results);
        }
        ast::TypeName::Reference(underlying, _) => {
            collect_results(underlying, in_path, env, seen, results);
        }
        ast::TypeName::Primitive(_) => {}
        ast::TypeName::Option(underlying) => {
            collect_results(underlying, in_path, env, seen, results);
        }
        ast::TypeName::Result(ok, err) => {
            if !seen.contains(&(typ, in_path)) {
                seen.insert((typ, in_path));
                collect_results(ok, in_path, env, seen, results);
                collect_results(err, in_path, env, seen, results);
                results.push((typ, in_path));
            }
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference => {}
        ast::TypeName::Void => {}
    }
}

fn gen_result<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    if let ast::TypeName::Result(ok, err) = typ {
        let result_name = format!("{}_{}", in_path.elements.join("_"), name_for_type(typ));
        writeln!(out, "typedef struct {} {{", result_name)?;
        let mut result_indent = indented(out).with_str("    ");
        writeln!(&mut result_indent, "union {{")?;
        let mut union_indent = indented(&mut result_indent).with_str("    ");

        if let ast::TypeName::Void = ok.as_ref() {
            writeln!(&mut union_indent, "uint8_t ok[0];")?;
        } else {
            gen_type(
                ok,
                in_path,
                env,
                &mut ((&mut union_indent) as &mut dyn fmt::Write),
            )?;
            writeln!(&mut union_indent, " ok;")?;
        }

        if let ast::TypeName::Void = err.as_ref() {
            writeln!(&mut union_indent, "uint8_t err[0];")?;
        } else {
            gen_type(
                err,
                in_path,
                env,
                &mut ((&mut union_indent) as &mut dyn fmt::Write),
            )?;
            writeln!(&mut union_indent, " err;")?;
        }
        writeln!(&mut result_indent, "}};")?;
        writeln!(&mut result_indent, "bool is_ok;")?;
        writeln!(out, "}} {};", result_name)?;

        Ok(())
    } else {
        panic!()
    }
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

pub fn gen_type<W: fmt::Write>(
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
                // repr(C) fieldless enums use the default platform representation: isize
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

        ast::TypeName::Result(_, _) => {
            write!(out, "{}_{}", in_path.elements.join("_"), name_for_type(typ))?;
        }

        ast::TypeName::Writeable => write!(out, "DiplomatWriteable")?,
        ast::TypeName::StrReference => panic!(),
        ast::TypeName::Void => write!(out, "void")?,
    }

    Ok(())
}

fn name_for_type(typ: &ast::TypeName) -> String {
    match typ {
        ast::TypeName::Named(name) => name.elements.join("_"),
        ast::TypeName::Box(underlying) => format!("box_{}", name_for_type(underlying)),
        ast::TypeName::Reference(underlying, mutable) => {
            if *mutable {
                return format!("ref_mut_{}", name_for_type(underlying));
            } else {
                format!("ref_{}", name_for_type(underlying))
            }
        }
        ast::TypeName::Primitive(prim) => c_type_for_prim(prim).to_string(),
        ast::TypeName::Option(underlying) => format!("opt_{}", name_for_type(underlying)),
        ast::TypeName::Result(ok, err) => {
            format!("result_{}_{}", name_for_type(ok), name_for_type(err))
        }
        ast::TypeName::Writeable => "writeable".to_string(),
        ast::TypeName::StrReference => "str_ref".to_string(),
        ast::TypeName::Void => "void".to_string(),
    }
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
    root: StructOrType<'a>,
    in_path: ast::Path,
    seen: &mut HashSet<(ast::Path, StructOrType<'a>)>,
    order: &mut Vec<(ast::Path, StructOrType<'a>)>,
    env: &'a HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) {
    if !seen.contains(&(in_path.clone(), root.clone())) {
        seen.insert((in_path.clone(), root.clone()));
        match &root {
            StructOrType::Struct(strct) => {
                for (_, typ, _) in &strct.fields {
                    topological_sort_structs(
                        StructOrType::Type(typ),
                        in_path.clone(),
                        seen,
                        order,
                        env,
                    );
                }

                order.push((in_path, root));
            }

            StructOrType::Type(typ) => match typ {
                ast::TypeName::Named(_) => match typ.resolve_with_path(&in_path, env) {
                    (path, ast::CustomType::Struct(strct)) => {
                        topological_sort_structs(
                            StructOrType::Struct(strct),
                            path,
                            seen,
                            order,
                            env,
                        );
                    }
                    (_, ast::CustomType::Opaque(_) | ast::CustomType::Enum(_)) => {}
                },

                ast::TypeName::Option(underlying) => {
                    topological_sort_structs(
                        StructOrType::Type(underlying.as_ref()),
                        in_path,
                        seen,
                        order,
                        env,
                    );
                }

                ast::TypeName::Result(ok, err) => {
                    topological_sort_structs(
                        StructOrType::Type(ok.as_ref()),
                        in_path.clone(),
                        seen,
                        order,
                        env,
                    );
                    topological_sort_structs(
                        StructOrType::Type(err.as_ref()),
                        in_path.clone(),
                        seen,
                        order,
                        env,
                    );
                    order.push((in_path, root));
                }

                _ => {}
            },
        }
    }
}
