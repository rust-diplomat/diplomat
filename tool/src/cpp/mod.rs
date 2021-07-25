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
    outs: &mut HashMap<&str, String>,
) -> fmt::Result {
    super::c::gen_bindings(env, outs)?;

    let diplomat_runtime_out = outs
        .entry("diplomat_runtime.hpp")
        .or_insert_with(String::new);
    write!(diplomat_runtime_out, "{}", RUNTIME_HPP)?;

    let out = outs.entry("api.hpp").or_insert_with(String::new);
    writeln!(out, "#include <stdint.h>")?;
    writeln!(out, "#include <stddef.h>")?;
    writeln!(out, "#include <stdbool.h>")?;
    writeln!(out, "#include <algorithm>")?;
    writeln!(out, "#include <memory>")?;
    writeln!(out, "#include <optional>")?;
    writeln!(out, "#include \"diplomat_runtime.hpp\"")?;
    writeln!(out)?;

    let mut all_types = util::get_all_custom_types(env);
    all_types.sort_by_key(|t| t.1.name());

    for (_, custom_type) in &all_types {
        writeln!(out)?;
        match custom_type {
            ast::CustomType::Opaque(_) => {
                writeln!(out, "class {};", custom_type.name())?;
            }

            ast::CustomType::Enum(enm) => {
                writeln!(out, "enum struct {} : ssize_t {{", enm.name)?;
                let mut enm_indent = indented(out).with_str("  ");
                for (name, discriminant, _) in enm.variants.iter() {
                    writeln!(&mut enm_indent, "{} = {},", name, discriminant)?;
                }
                writeln!(out, "}};")?;
            }

            ast::CustomType::Struct(_) => {
                writeln!(out, "struct {};", custom_type.name())?;
            }
        }
    }

    for (in_path, custom_type) in &all_types {
        if let ast::CustomType::Opaque(_) = custom_type {
            writeln!(out)?;
            gen_struct(custom_type, in_path, true, env, out)?;
        }
    }

    let mut structs_seen = HashSet::new();
    let mut structs_order = Vec::new();
    for custom_type in &all_types {
        if let (in_path, ast::CustomType::Struct(strct)) = custom_type {
            if !structs_seen.contains(&strct.name) {
                super::c::topological_sort_structs(
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
        gen_struct(
            &ast::CustomType::Struct(strct.clone()),
            &in_path,
            true,
            env,
            out,
        )?;
    }

    for (in_path, custom_type) in all_types {
        writeln!(out)?;
        gen_struct(custom_type, in_path, false, env, out)?;
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
        write!(out, "std::string")?;
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
        }

        if is_writeable_out {
            writeln!(&mut method_body, "std::string diplomat_writeable_string;")?;
            writeln!(&mut method_body, "capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);")?;
            writeln!(
                &mut method_body,
                "capi::{}({});",
                method.full_path_name,
                all_params_invocation.join(", ")
            )?;
            writeln!(&mut method_body, "return diplomat_writeable_string;")?;
        } else if let Some(ret_typ) = &method.return_type {
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

            writeln!(&mut method_body, "return {};", out_expr)?;
        } else {
            writeln!(
                &mut method_body,
                "capi::{}({});",
                method.full_path_name,
                all_params_invocation.join(", ")
            )?;
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
    match typ {
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Opaque(opaque) => {
                if let Some(owned) = behind_ref {
                    if owned {
                        write!(out, "{}", opaque.name)?;
                    } else {
                        write!(out, "{}&", opaque.name)?;
                    }
                } else {
                    panic!("Cannot pass opaque structs as values");
                }
            }

            ast::CustomType::Struct(strct) => {
                write!(out, "{}", strct.name)?;
                if let Some(owned) = behind_ref {
                    if owned {
                        write!(out, "*")?;
                    } else {
                        write!(out, "&")?;
                    }
                }
            }

            ast::CustomType::Enum(enm) => {
                write!(out, "{}", enm.name)?;
                if let Some(owned) = behind_ref {
                    if owned {
                        write!(out, "*")?;
                    } else {
                        write!(out, "&")?;
                    }
                }
            }
        },

        ast::TypeName::Box(underlying) => {
            gen_type(underlying.as_ref(), in_path, Some(true), env, out)?;
            if let Some(owned) = behind_ref {
                if owned {
                    write!(out, "*")?;
                } else {
                    write!(out, "&")?;
                }
            }
        }

        ast::TypeName::Reference(underlying, mutable) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type(underlying.as_ref(), in_path, Some(false), env, out)?;
            if let Some(owned) = behind_ref {
                if owned {
                    write!(out, "*")?;
                } else {
                    write!(out, "&")?;
                }
            }
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                write!(out, "std::optional<")?;
                gen_type(underlying.as_ref(), in_path, behind_ref, env, out)?;
                write!(out, ">")?;
            }

            _ => todo!(),
        },

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", super::c::c_type_for_prim(prim))?;
            if let Some(owned) = behind_ref {
                if owned {
                    write!(out, "*")?;
                } else {
                    write!(out, "&")?;
                }
            }
        }

        ast::TypeName::Writeable => {
            write!(out, "capi::DiplomatWriteable")?;
            if let Some(owned) = behind_ref {
                if owned {
                    write!(out, "*")?;
                } else {
                    write!(out, "&")?;
                }
            }
        }

        ast::TypeName::StrReference => {
            write!(out, "const std::string_view")?;
            if let Some(owned) = behind_ref {
                if owned {
                    write!(out, "*")?;
                } else {
                    write!(out, "&")?;
                }
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

        ast::TypeName::Primitive(_) => cpp.to_string(),
        o => todo!("{:?}", o),
    }
}

#[derive(Eq, PartialEq)]
struct ReferenceMeta {
    owned: bool,
    mutable: bool
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
                mutable: *mutability
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
                    } else {
                        if reference.mutable {
                            format!("{}.AsFFIMut()", cpp)
                        } else {
                            format!("{}.AsFFI()", cpp)
                        }
                    }
                } else {
                    panic!("Cannot handle opaque types by value");
                }
            }

            ast::CustomType::Struct(strct) => {
                if let Some(reference) = behind_ref {
                    if reference.owned {
                        format!(
                            "(capi::{}*) {}",
                            strct.name,
                            cpp
                        )
                    } else {
                        format!(
                            "(capi::{}*) &{}",
                            strct.name,
                            cpp
                        )
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

            ast::CustomType::Enum(_) => format!("static_cast<ssize_t>({})", cpp),
        },
        ast::TypeName::Writeable => {
            if behind_ref == Some(ReferenceMeta {
                owned: false,
                mutable: true
            }) {
                format!("&{}", cpp)
            } else {
                panic!("Cannot send Writeable to Rust as a value");
            }
        }
        ast::TypeName::Primitive(_) => cpp.to_string(),
        o => todo!("{:?}", o),
    }
}
