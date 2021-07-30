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
    outs: &mut HashMap<String, String>,
) -> fmt::Result {
    let diplomat_runtime_out = outs
        .entry("diplomat_runtime.h".to_string())
        .or_insert_with(String::new);
    write!(diplomat_runtime_out, "{}", RUNTIME_H)?;

    let all_types = util::get_all_custom_types(env);
    let mut seen_results = HashSet::new();
    let mut all_results = Vec::new();
    // all_types.sort_by_key(|t| t.1.name());
    for (in_path, typ) in all_types {
        let out = outs
            .entry(format!("{}_{}.h", in_path.elements.join("_"), typ.name()))
            .or_insert_with(String::new);

        writeln!(
            out,
            "#ifndef {}_{}_H",
            in_path.elements.join("_"),
            typ.name()
        )?;
        writeln!(
            out,
            "#define {}_{}_H",
            in_path.elements.join("_"),
            typ.name()
        )?;

        writeln!(out, "#include <stdio.h>")?;
        writeln!(out, "#include <stdint.h>")?;
        writeln!(out, "#include <stddef.h>")?;
        writeln!(out, "#include <stdbool.h>")?;
        writeln!(out, "#include \"diplomat_runtime.h\"")?;
        writeln!(out)?;
        writeln!(out, "#ifdef __cplusplus")?;
        writeln!(out, "extern \"C\" {{")?;
        writeln!(out, "#endif")?;

        writeln!(out)?;

        let mut seen_includes = HashSet::new();
        seen_includes.insert(format!(
            "#include \"{}_{}.h\"",
            in_path.elements.join("_"),
            typ.name()
        ));

        match typ {
            ast::CustomType::Opaque(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, env, out)?;
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

            ast::CustomType::Struct(strct) => {
                for (_, typ, _) in &strct.fields {
                    gen_includes(typ, in_path, true, env, &mut seen_includes, out)?;
                    collect_results(typ, in_path, env, &mut seen_results, &mut all_results);
                }
                writeln!(out)?;
                gen_struct(typ, in_path, env, out)?;
            }
        }

        for method in typ.methods() {
            for param in &method.params {
                gen_includes(&param.ty, in_path, false, env, &mut seen_includes, out)?;
                collect_results(&param.ty, in_path, env, &mut seen_results, &mut all_results);
            }

            if let Some(return_type) = method.return_type.as_ref() {
                gen_includes(return_type, in_path, false, env, &mut seen_includes, out)?;
                collect_results(
                    return_type,
                    in_path,
                    env,
                    &mut seen_results,
                    &mut all_results,
                );
            }
        }

        for method in typ.methods() {
            writeln!(out)?;
            gen_method(method, in_path, env, out)?;
        }

        if typ.methods().is_empty() {
            writeln!(out)?;
        }

        write!(out, "void {}_destroy(", typ.name())?;

        gen_type(
            &ast::TypeName::Box(Box::new(ast::TypeName::Named(
                ast::Path::empty().sub_path(typ.name().clone()),
            ))),
            in_path,
            env,
            out,
        )?;

        writeln!(out, " self);")?;

        writeln!(out, "#ifdef __cplusplus")?;
        writeln!(out, "}}")?;
        writeln!(out, "#endif")?;
        writeln!(out, "#endif")?;
    }

    for (in_path, typ) in &all_results {
        if let ast::TypeName::Result(ok, err) = typ {
            let out = outs
                .entry(format!(
                    "{}_{}.h",
                    in_path.elements.join("_"),
                    name_for_type(typ)
                ))
                .or_insert_with(String::new);

            writeln!(
                out,
                "#ifndef {}_{}_H",
                in_path.elements.join("_"),
                name_for_type(typ)
            )?;
            writeln!(
                out,
                "#define {}_{}_H",
                in_path.elements.join("_"),
                name_for_type(typ)
            )?;
            writeln!(out, "#include <stdio.h>")?;
            writeln!(out, "#include <stdint.h>")?;
            writeln!(out, "#include <stddef.h>")?;
            writeln!(out, "#include <stdbool.h>")?;
            writeln!(out, "#include \"diplomat_runtime.h\"")?;
            writeln!(out)?;
            writeln!(out, "#ifdef __cplusplus")?;
            writeln!(out, "extern \"C\" {{")?;
            writeln!(out, "#endif")?;

            let mut seen_includes = HashSet::new();
            gen_includes(ok.as_ref(), in_path, true, env, &mut seen_includes, out)?;
            gen_includes(err.as_ref(), in_path, true, env, &mut seen_includes, out)?;

            gen_result(typ, in_path, env, out)?;

            writeln!(out, "#ifdef __cplusplus")?;
            writeln!(out, "}}")?;
            writeln!(out, "#endif")?;
            writeln!(out, "#endif")?;
        } else {
            panic!()
        }
    }

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

fn gen_includes<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    pre_struct: bool,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    seen_includes: &mut HashSet<String>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => {
            let (path, custom_typ) = typ.resolve_with_path(in_path, env);
            match custom_typ {
                ast::CustomType::Opaque(_) => {
                    if pre_struct {
                        let decl = format!(
                            "typedef struct {} {};",
                            custom_typ.name(),
                            custom_typ.name()
                        );
                        if !seen_includes.contains(&decl) {
                            writeln!(out, "{}", decl)?;
                            seen_includes.insert(decl);
                        }
                    } else {
                        let include = format!(
                            "#include \"{}_{}.h\"",
                            path.elements.join("_"),
                            custom_typ.name()
                        );
                        if !seen_includes.contains(&include) {
                            writeln!(out, "{}", include)?;
                            seen_includes.insert(include);
                        }
                    }
                }

                ast::CustomType::Struct(_) | ast::CustomType::Enum(_) => {
                    let include = format!(
                        "#include \"{}_{}.h\"",
                        path.elements.join("_"),
                        custom_typ.name()
                    );
                    if !seen_includes.contains(&include) {
                        writeln!(out, "{}", include)?;
                        seen_includes.insert(include);
                    }
                }
            }
        }
        ast::TypeName::Box(underlying) => {
            gen_includes(underlying, in_path, pre_struct, env, seen_includes, out)?;
        }
        ast::TypeName::Reference(underlying, _) => {
            gen_includes(underlying, in_path, pre_struct, env, seen_includes, out)?;
        }
        ast::TypeName::Primitive(_) => {}
        ast::TypeName::Option(underlying) => {
            gen_includes(underlying, in_path, pre_struct, env, seen_includes, out)?;
        }
        ast::TypeName::Result(_, _) => {
            let include = format!(
                "#include \"{}_{}.h\"",
                in_path.elements.join("_"),
                name_for_type(typ)
            );
            if !seen_includes.contains(&include) {
                writeln!(out, "{}", include)?;
                seen_includes.insert(include);
            }
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference => {}
        ast::TypeName::Unit => {}
    }

    Ok(())
}

fn collect_results<'a, 'b>(
    typ: &'a ast::TypeName,
    in_path: &'b ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    seen: &mut HashSet<(ast::Path, &'a ast::TypeName)>,
    results: &mut Vec<(ast::Path, &'a ast::TypeName)>,
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
            let seen_key = (in_path.clone(), typ);
            if !seen.contains(&seen_key) {
                seen.insert(seen_key.clone());
                collect_results(ok, in_path, env, seen, results);
                collect_results(err, in_path, env, seen, results);
                results.push(seen_key);
            }
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference => {}
        ast::TypeName::Unit => {}
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

        if let ast::TypeName::Unit = ok.as_ref() {
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

        if let ast::TypeName::Unit = err.as_ref() {
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

            ast::CustomType::Enum(enm) => {
                write!(out, "{}", enm.name)?;
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
        ast::TypeName::Unit => write!(out, "void")?,
    }

    Ok(())
}

/// Generates a struct name that uniquely identifies the given type.
///
/// This is primarily used for generating structs for result types,
/// which require one struct for each distinct instance.
pub fn name_for_type(typ: &ast::TypeName) -> String {
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
        ast::TypeName::Unit => "void".to_string(),
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
