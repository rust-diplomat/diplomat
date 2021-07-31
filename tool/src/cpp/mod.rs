use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast;
use indenter::indented;

use crate::util;

static RUNTIME_HPP: &str = include_str!("runtime.hpp");

pub fn gen_bindings(
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    outs: &mut HashMap<String, String>,
) -> fmt::Result {
    super::c::gen_bindings(env, outs)?;

    let diplomat_runtime_out = outs
        .entry("diplomat_runtime.hpp".to_string())
        .or_insert_with(String::new);
    write!(diplomat_runtime_out, "{}", RUNTIME_HPP)?;

    let all_types = util::get_all_custom_types(env);

    for (in_path, typ) in &all_types {
        let out = outs
            .entry(format!("{}.hpp", typ.name()))
            .or_insert_with(String::new);

        writeln!(out, "#ifndef {}_HPP", typ.name())?;
        writeln!(out, "#define {}_HPP", typ.name())?;

        writeln!(out, "#include <stdint.h>")?;
        writeln!(out, "#include <stddef.h>")?;
        writeln!(out, "#include <stdbool.h>")?;
        writeln!(out, "#include <algorithm>")?;
        writeln!(out, "#include <memory>")?;
        writeln!(out, "#include <optional>")?;
        writeln!(out, "#include <variant>")?;
        writeln!(out, "#include \"diplomat_runtime.hpp\"")?;
        writeln!(out)?;
        writeln!(out, "namespace capi {{")?;
        writeln!(out, "#include \"{}.h\"", typ.name())?;
        writeln!(out, "}}")?;

        writeln!(out)?;

        let mut seen_includes = HashSet::new();
        seen_includes.insert(format!("#include \"{}.hpp\"", typ.name()));

        match typ {
            ast::CustomType::Opaque(_) => {}

            ast::CustomType::Enum(enm) => {
                writeln!(out)?;
                writeln!(out, "enum struct {} {{", enm.name)?;
                let mut enm_indent = indented(out).with_str("  ");
                for (name, discriminant, _) in enm.variants.iter() {
                    writeln!(&mut enm_indent, "{} = {},", name, discriminant)?;
                }
                writeln!(out, "}};")?;
            }

            ast::CustomType::Struct(strct) => {
                for (_, typ, _) in &strct.fields {
                    gen_includes(typ, in_path, true, true, env, &mut seen_includes, out)?;
                }
            }
        }

        for method in typ.methods() {
            for param in &method.params {
                gen_includes(
                    &param.ty,
                    in_path,
                    true,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }

            if let Some(return_type) = method.return_type.as_ref() {
                gen_includes(
                    return_type,
                    in_path,
                    true,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }
        }

        match typ {
            ast::CustomType::Opaque(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, true, env, out)?;
            }

            ast::CustomType::Enum(_) => {}

            ast::CustomType::Struct(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, true, env, out)?;
            }
        }

        writeln!(out)?;

        for method in typ.methods() {
            for param in &method.params {
                gen_includes(
                    &param.ty,
                    in_path,
                    false,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }

            if let Some(return_type) = method.return_type.as_ref() {
                gen_includes(
                    return_type,
                    in_path,
                    false,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }
        }

        match typ {
            ast::CustomType::Opaque(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, false, env, out)?;
            }

            ast::CustomType::Enum(_) => {}

            ast::CustomType::Struct(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, false, env, out)?;
            }
        }

        writeln!(out, "#endif")?
    }

    Ok(())
}

fn gen_struct<W: fmt::Write>(
    custom_type: &ast::CustomType,
    in_path: &ast::Path,
    is_header: bool,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    if is_header {
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
    }

    match custom_type {
        ast::CustomType::Opaque(opaque) => {
            if is_header {
                writeln!(out, "class {} {{", opaque.name)?;
                writeln!(out, " public:")?;
            }

            let mut public_body = if is_header {
                indented(out).with_str("  ")
            } else {
                indented(out).with_str("")
            };

            for method in &opaque.methods {
                gen_method(
                    custom_type,
                    method,
                    in_path,
                    is_header,
                    env,
                    &mut public_body,
                )?;
            }

            if is_header {
                writeln!(
                    &mut public_body,
                    "inline const capi::{}* AsFFI() const {{ return this->inner.get(); }}",
                    opaque.name
                )?;

                writeln!(
                    &mut public_body,
                    "inline capi::{}* AsFFIMut() {{ return this->inner.get(); }}",
                    opaque.name
                )?;

                writeln!(
                    &mut public_body,
                    "{}(capi::{}* i) : inner(i) {{}}",
                    opaque.name, opaque.name
                )?;

                writeln!(out, " private:")?;
                let mut private_body = indented(out).with_str("  ");
                writeln!(
                    &mut private_body,
                    "std::unique_ptr<capi::{}, {}Deleter> inner;",
                    opaque.name, opaque.name
                )?;
                writeln!(out, "}};")?;
            }
        }

        ast::CustomType::Struct(strct) => {
            if is_header {
                writeln!(out, "struct {} {{", strct.name)?;
                writeln!(out, " public:")?;
            }

            let mut public_body = if is_header {
                indented(out).with_str("  ")
            } else {
                indented(out).with_str("")
            };

            if is_header {
                for (name, typ, _) in &strct.fields {
                    gen_type(typ, in_path, None, env, &mut public_body)?;
                    writeln!(&mut public_body, " {};", name)?;
                }
            }

            for method in &strct.methods {
                gen_method(
                    custom_type,
                    method,
                    in_path,
                    is_header,
                    env,
                    &mut public_body,
                )?;
            }

            if is_header {
                writeln!(out, "}};")?;
            }
        }

        ast::CustomType::Enum(_) => {}
    }

    Ok(())
}

fn gen_includes<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    pre_struct: bool,
    for_field: bool,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    seen_includes: &mut HashSet<String>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => {
            let (_, custom_typ) = typ.resolve_with_path(in_path, env);
            match custom_typ {
                ast::CustomType::Opaque(_) => {
                    if pre_struct {
                        let decl = format!("class {};", custom_typ.name());
                        if !seen_includes.contains(&decl) {
                            writeln!(out, "{}", decl)?;
                            seen_includes.insert(decl);
                        }
                    } else {
                        let include = format!("#include \"{}.hpp\"", custom_typ.name());
                        if !seen_includes.contains(&include) {
                            writeln!(out, "{}", include)?;
                            seen_includes.insert(include);
                        }
                    }
                }

                ast::CustomType::Struct(_) => {
                    if pre_struct && !for_field {
                        let decl = format!("struct {};", custom_typ.name());
                        if !seen_includes.contains(&decl) {
                            writeln!(out, "{}", decl)?;
                            seen_includes.insert(decl);
                        }
                    } else {
                        let include = format!("#include \"{}.hpp\"", custom_typ.name());
                        if !seen_includes.contains(&include) {
                            writeln!(out, "{}", include)?;
                            seen_includes.insert(include);
                        }
                    }
                }

                ast::CustomType::Enum(_) => {
                    if pre_struct && !for_field {
                        let decl = format!("enum struct {};", custom_typ.name());
                        if !seen_includes.contains(&decl) {
                            writeln!(out, "{}", decl)?;
                            seen_includes.insert(decl);
                        }
                    } else {
                        let include = format!("#include \"{}.hpp\"", custom_typ.name());
                        if !seen_includes.contains(&include) {
                            writeln!(out, "{}", include)?;
                            seen_includes.insert(include);
                        }
                    }
                }
            }
        }
        ast::TypeName::Box(underlying) => {
            gen_includes(
                underlying,
                in_path,
                pre_struct,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Reference(underlying, _) => {
            gen_includes(
                underlying,
                in_path,
                pre_struct,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Primitive(_) => {}
        ast::TypeName::Option(underlying) => {
            gen_includes(
                underlying,
                in_path,
                pre_struct,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Result(ok, err) => {
            gen_includes(
                ok.as_ref(),
                in_path,
                pre_struct,
                for_field,
                env,
                seen_includes,
                out,
            )?;

            gen_includes(
                err.as_ref(),
                in_path,
                pre_struct,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference => {}
        ast::TypeName::Unit => {}
    }

    Ok(())
}

fn gen_method<W: fmt::Write>(
    enclosing_type: &ast::CustomType,
    method: &ast::Method,
    in_path: &ast::Path,
    is_header: bool,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    if method.self_param.is_none() && is_header {
        write!(out, "static ")?;
    }

    let is_writeable_out = method.is_writeable_out();

    if is_writeable_out {
        if let Some(ast::TypeName::Result(_, err)) = &method.return_type {
            write!(out, "diplomat::result<std::string, ")?;
            if err.as_ref() == &ast::TypeName::Unit {
                write!(out, "std::monostate")?;
            } else {
                gen_type(err, in_path, None, env, out)?;
            }
            write!(out, ">")?;
        } else {
            write!(out, "std::string")?;
        }
    } else {
        match &method.return_type {
            Some(ret_type) => {
                gen_type(ret_type, in_path, None, env, out)?;
            }

            None => {
                write!(out, "void")?;
            }
        }
    }

    if !is_header {
        write!(out, " {}::", enclosing_type.name())?;
    } else {
        write!(out, " ")?;
    }

    // TODO(shadaj): handle other keywords
    if method.name == "new" || method.name == "default" {
        write!(out, "{}_(", method.name)?;
    } else {
        write!(out, "{}(", method.name)?;
    }

    let mut params_to_gen = method.params.clone();

    if is_writeable_out {
        params_to_gen.remove(params_to_gen.len() - 1);
    }

    for (i, param) in params_to_gen.iter().enumerate() {
        if i != 0 {
            write!(out, ", ")?;
        }

        gen_type(&param.ty, in_path, None, env, out)?;
        write!(out, " {}", param.name)?;
    }

    if is_header {
        writeln!(out, ");")?;
    } else {
        writeln!(out, ") {{")?;

        let mut method_body = indented(out).with_str("  ");

        let mut all_params_invocation = vec![];

        if let Some(param) = &method.self_param {
            let invocation_expr = gen_cpp_to_rust(
                "this",
                "this",
                None,
                &param.ty,
                in_path,
                env,
                true,
                &mut method_body,
            );
            all_params_invocation.push(invocation_expr);
        }

        for param in params_to_gen.iter() {
            if param.ty == ast::TypeName::StrReference {
                all_params_invocation.push(format!("{}.data()", param.name));
                all_params_invocation.push(format!("{}.length()", param.name));
            } else {
                let invocation_expr = gen_cpp_to_rust(
                    &param.name,
                    &param.name,
                    None,
                    &param.ty,
                    in_path,
                    env,
                    param.name == "self",
                    &mut method_body,
                );
                all_params_invocation.push(invocation_expr);
            }
        }

        if is_writeable_out {
            all_params_invocation.push("&diplomat_writeable_out".to_string());
            writeln!(&mut method_body, "std::string diplomat_writeable_string;")?;
            writeln!(&mut method_body, "capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);")?;
        }

        match &method.return_type {
            None | Some(ast::TypeName::Unit) => {
                writeln!(
                    &mut method_body,
                    "capi::{}({});",
                    method.full_path_name,
                    all_params_invocation.join(", ")
                )?;

                if is_writeable_out {
                    writeln!(&mut method_body, "return diplomat_writeable_string;")?;
                }
            }

            Some(ret_typ) => {
                let out_expr = gen_rust_to_cpp(
                    &format!(
                        "capi::{}({})",
                        method.full_path_name,
                        all_params_invocation.join(", ")
                    ),
                    "out_value",
                    ret_typ,
                    in_path,
                    env,
                    &mut method_body,
                );

                if is_writeable_out {
                    if let ast::TypeName::Result(_, err) = ret_typ {
                        // TODO(shadaj): do something if not okay
                        gen_type(ret_typ, in_path, None, env, &mut method_body)?;
                        writeln!(&mut method_body, " out_value = {};", out_expr)?;

                        writeln!(&mut method_body, "if (out_value.is_ok) {{")?;

                        write!(&mut method_body, "  return diplomat::result<std::string, ")?;
                        if err.as_ref() == &ast::TypeName::Unit {
                            write!(&mut method_body, "std::monostate")?;
                        } else {
                            gen_type(err, in_path, None, env, &mut method_body)?;
                        }
                        writeln!(&mut method_body, ">::new_ok(diplomat_writeable_string);")?;

                        writeln!(&mut method_body, "}} else {{")?;
                        write!(&mut method_body, "  return diplomat::result<std::string, ")?;
                        if err.as_ref() == &ast::TypeName::Unit {
                            writeln!(&mut method_body, "std::monostate>::new_err_void();")?;
                        } else {
                            gen_type(err, in_path, None, env, &mut method_body)?;
                            writeln!(&mut method_body, ">::new_err(out_value.err);")?;
                        }

                        writeln!(&mut method_body, "}}")?;
                    } else {
                        panic!("Not in writeable out form")
                    }
                } else {
                    writeln!(&mut method_body, "return {};", out_expr)?;
                }
            }
        }

        writeln!(out, "}}")?;
    }

    Ok(())
}

fn gen_type<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    behind_ref: Option<bool>, // owned?
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    let mut handled_ref = false;
    match typ {
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Opaque(opaque) => {
                if let Some(owned) = behind_ref {
                    if owned {
                        write!(out, "{}", opaque.name)?;
                    } else {
                        write!(out, "{}&", opaque.name)?;
                    }

                    handled_ref = true;
                } else {
                    panic!("Cannot pass opaque structs as values");
                }
            }

            ast::CustomType::Struct(strct) => {
                write!(out, "{}", strct.name)?;
            }

            ast::CustomType::Enum(enm) => {
                write!(out, "{}", enm.name)?;
            }
        },

        ast::TypeName::Box(underlying) => {
            gen_type(underlying.as_ref(), in_path, Some(true), env, out)?;
        }

        ast::TypeName::Reference(underlying, mutable) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type(underlying.as_ref(), in_path, Some(false), env, out)?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                write!(out, "std::optional<")?;
                gen_type(underlying.as_ref(), in_path, behind_ref, env, out)?;
                write!(out, ">")?;
            }

            _ => todo!(),
        },

        ast::TypeName::Result(ok, err) => {
            write!(out, "diplomat::result<")?;
            if let ast::TypeName::Unit = ok.as_ref() {
                write!(out, "std::monostate")?;
            } else {
                gen_type(ok, in_path, behind_ref, env, out)?;
            }

            write!(out, ", ")?;
            if let ast::TypeName::Unit = err.as_ref() {
                write!(out, "std::monostate")?;
            } else {
                gen_type(err, in_path, behind_ref, env, out)?;
            }
            write!(out, ">")?;
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", super::c::c_type_for_prim(prim))?;
        }

        ast::TypeName::Writeable => {
            write!(out, "capi::DiplomatWriteable")?;
        }

        ast::TypeName::StrReference => {
            write!(out, "const std::string_view")?;
        }

        ast::TypeName::Unit => {
            write!(out, "void")?;
        }
    }

    if !handled_ref {
        if let Some(owned) = behind_ref {
            if owned {
                write!(out, "*")?;
            } else {
                write!(out, "&")?;
            }
        }
    }

    Ok(())
}

fn gen_rust_to_cpp<W: Write>(
    cpp: &str,
    path: &str,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> String {
    match typ {
        ast::TypeName::Box(underlying) => match underlying.as_ref() {
            ast::TypeName::Named(_name) => match underlying.resolve(in_path, env) {
                ast::CustomType::Opaque(opaque) => {
                    format!("{}({})", opaque.name, cpp)
                }

                ast::CustomType::Struct(_strct) => {
                    // TODO(shadaj): should emit a unique_ptr
                    todo!("Receiving boxes of structs is not yet supported")
                }

                ast::CustomType::Enum(_) => {
                    // TODO(shadaj): should emit a unique_ptr
                    todo!("Receiving boxes of enums is not yet supported")
                }
            },
            _o => todo!(),
        },
        ast::TypeName::Named(_) => match typ.resolve_with_path(in_path, env) {
            (_, ast::CustomType::Opaque(_)) => {
                panic!("Cannot handle opaque structs by value");
            }

            (in_path, ast::CustomType::Struct(strct)) => {
                let raw_struct_id = format!("diplomat_raw_struct_{}", path);
                writeln!(out, "capi::{} {} = {};", strct.name, raw_struct_id, cpp).unwrap();
                let mut all_fields_wrapped = vec![];
                for (name, typ, _) in &strct.fields {
                    all_fields_wrapped.push(format!(
                        ".{} = std::move({})",
                        name,
                        gen_rust_to_cpp(
                            &format!("{}.{}", raw_struct_id, name),
                            &format!("{}_{}", path, name),
                            typ,
                            &in_path,
                            env,
                            out
                        )
                    ));
                }

                format!("{}{{ {} }}", strct.name, all_fields_wrapped.join(", "))
            }

            (_, ast::CustomType::Enum(enm)) => {
                format!("{}{{ {} }}", enm.name, cpp)
            }
        },

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                let raw_value_id = format!("diplomat_optional_raw_{}", path);
                writeln!(out, "auto {} = {};", raw_value_id, cpp).unwrap();

                let wrapped_value_id = format!("diplomat_optional_{}", path);
                gen_type(typ, in_path, None, env, out).unwrap();
                writeln!(out, " {};", wrapped_value_id).unwrap();

                writeln!(out, "if ({} != nullptr) {{", raw_value_id).unwrap();

                let some_expr =
                    gen_rust_to_cpp(&raw_value_id, path, underlying.as_ref(), in_path, env, out);
                writeln!(out, "  {} = {};", wrapped_value_id, some_expr).unwrap();

                writeln!(out, "}} else {{").unwrap();
                writeln!(out, "  {} = std::nullopt;", wrapped_value_id).unwrap();
                writeln!(out, "}}").unwrap();

                wrapped_value_id
            }

            _ => todo!(),
        },

        ast::TypeName::Result(ok, err) => {
            let raw_value_id = format!("diplomat_result_raw_{}", path);
            writeln!(out, "auto {} = {};", raw_value_id, cpp).unwrap();

            let wrapped_value_id = format!("diplomat_result_{}", path);
            gen_type(typ, in_path, None, env, out).unwrap();
            writeln!(out, " {};", wrapped_value_id).unwrap();

            writeln!(out, "{}.is_ok = {}.is_ok;", wrapped_value_id, raw_value_id).unwrap();
            writeln!(out, "if ({}.is_ok) {{", raw_value_id).unwrap();
            if ok.as_ref() != &ast::TypeName::Unit {
                let ok_expr =
                    gen_rust_to_cpp(&format!("{}.ok", raw_value_id), path, ok, in_path, env, out);
                writeln!(out, "  {}.ok = {};", wrapped_value_id, ok_expr).unwrap();
            }
            writeln!(out, "}} else {{").unwrap();
            if err.as_ref() != &ast::TypeName::Unit {
                let err_expr = gen_rust_to_cpp(
                    &format!("{}.err", raw_value_id),
                    path,
                    err,
                    in_path,
                    env,
                    out,
                );
                writeln!(out, "  {}.err = {};", wrapped_value_id, err_expr).unwrap();
            }
            writeln!(out, "}}").unwrap();

            wrapped_value_id
        }

        ast::TypeName::Primitive(_) => cpp.to_string(),
        ast::TypeName::Reference(_, _) => {
            todo!("Returning references from Rust to C++ is not currently supported")
        }
        ast::TypeName::Writeable => panic!("Returning writeables is not supported"),
        ast::TypeName::StrReference => {
            todo!("Returning &str from Rust to C++ is not currently supported")
        }
        ast::TypeName::Unit => cpp.to_string(),
    }
}

#[derive(Eq, PartialEq)]
struct ReferenceMeta {
    owned: bool,
    mutable: bool,
}

#[allow(clippy::too_many_arguments)]
fn gen_cpp_to_rust<W: Write>(
    cpp: &str,
    path: &str,
    behind_ref: Option<ReferenceMeta>,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    is_self: bool,
    out: &mut W,
) -> String {
    match typ {
        ast::TypeName::Reference(underlying, mutability) => gen_cpp_to_rust(
            cpp,
            path,
            Some(ReferenceMeta {
                owned: false,
                mutable: *mutability,
            }),
            underlying.as_ref(),
            in_path,
            env,
            is_self,
            out,
        ),
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Opaque(_opaque) => {
                if let Some(reference) = behind_ref {
                    if is_self {
                        format!("{}->inner.get()", cpp)
                    } else if reference.mutable {
                        format!("{}.AsFFIMut()", cpp)
                    } else {
                        format!("{}.AsFFI()", cpp)
                    }
                } else {
                    panic!("Cannot handle opaque types by value");
                }
            }

            ast::CustomType::Struct(strct) => {
                if let Some(reference) = behind_ref {
                    if reference.owned {
                        format!("(capi::{}*) {}", strct.name, cpp)
                    } else {
                        format!("(capi::{}*) &{}", strct.name, cpp)
                    }
                } else {
                    let wrapped_struct_id = format!("diplomat_wrapped_struct_{}", path);
                    writeln!(out, "{} {} = {};", strct.name, wrapped_struct_id, cpp).unwrap();
                    let mut all_fields_wrapped = vec![];
                    for (name, typ, _) in &strct.fields {
                        all_fields_wrapped.push(format!(
                            ".{} = {}",
                            name,
                            gen_cpp_to_rust(
                                &format!("{}.{}", wrapped_struct_id, name),
                                &format!("{}_{}", path, name),
                                None,
                                typ,
                                in_path,
                                env,
                                false,
                                out
                            )
                        ));
                    }

                    format!(
                        "capi::{}{{ {} }}",
                        strct.name,
                        all_fields_wrapped.join(", ")
                    )
                }
            }

            ast::CustomType::Enum(enm) => format!("static_cast<capi::{}>({})", enm.name, cpp),
        },
        ast::TypeName::Writeable => {
            if behind_ref
                == Some(ReferenceMeta {
                    owned: false,
                    mutable: true,
                })
            {
                format!("&{}", cpp)
            } else {
                panic!("Cannot send Writeable to Rust as a value");
            }
        }
        ast::TypeName::Primitive(_) => cpp.to_string(),
        o => todo!("{:?}", o),
    }
}
