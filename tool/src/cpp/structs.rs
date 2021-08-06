use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;

use super::conversions::{gen_cpp_to_rust, gen_rust_to_cpp};
use super::types::gen_type;

pub fn gen_struct<W: fmt::Write>(
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
                    true,
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
                    "inline {}(capi::{}* i) : inner(i) {{}}",
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
                    true,
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
    // should it convert writeables to string as an additional method?
    writeable_to_string: bool,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    // This method should rearrange the writeable
    let rearranged_writeable = method.is_writeable_out() && writeable_to_string;

    // This method has some writeable param that is preserved
    let has_writeable_param = method.has_writeable_param() && !writeable_to_string;

    if rearranged_writeable {
        // generate the normal method too
        gen_method(enclosing_type, method, in_path, is_header, false, env, out)?;
    }

    if has_writeable_param {
        write!(out, "template<typename W> ")?;
    }

    if !is_header {
        write!(out, "inline ")?;
    }

    if method.self_param.is_none() && is_header {
        write!(out, "static ")?;
    }

    if rearranged_writeable {
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

    // TODO(#60): handle other keywords
    if method.name == "new" || method.name == "default" {
        write!(out, "{}_(", method.name)?;
    } else if has_writeable_param {
        write!(out, "{}_to_writeable(", method.name)?;
    } else {
        write!(out, "{}(", method.name)?;
    }

    let mut params_to_gen = method.params.clone();

    if rearranged_writeable {
        params_to_gen.remove(params_to_gen.len() - 1);
    }

    for (i, param) in params_to_gen.iter().enumerate() {
        if i != 0 {
            write!(out, ", ")?;
        }
        if param.is_writeable() && !writeable_to_string {
            write!(out, "W&")?;
        } else {
            gen_type(&param.ty, in_path, None, env, out)?;
        }
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

        if rearranged_writeable {
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

                if rearranged_writeable {
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

                if rearranged_writeable {
                    gen_writeable_out_value(&out_expr, ret_typ, in_path, env, &mut method_body)?;
                } else {
                    writeln!(&mut method_body, "return {};", out_expr)?;
                }
            }
        }

        writeln!(out, "}}")?;
    }

    Ok(())
}

fn gen_writeable_out_value<W: fmt::Write>(
    out_expr: &str,
    ret_typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    method_body: &mut W,
) -> fmt::Result {
    if let ast::TypeName::Result(_, err) = ret_typ {
        gen_type(ret_typ, in_path, None, env, method_body)?;
        writeln!(method_body, " out_value = {};", out_expr)?;

        writeln!(method_body, "if (out_value.is_ok) {{")?;

        write!(method_body, "  return diplomat::result<std::string, ")?;
        if err.as_ref() == &ast::TypeName::Unit {
            write!(method_body, "std::monostate")?;
        } else {
            gen_type(err, in_path, None, env, method_body)?;
        }
        writeln!(method_body, ">::new_ok(diplomat_writeable_string);")?;

        writeln!(method_body, "}} else {{")?;
        write!(method_body, "  return diplomat::result<std::string, ")?;
        if err.as_ref() == &ast::TypeName::Unit {
            writeln!(method_body, "std::monostate>::new_err_void();")?;
        } else {
            gen_type(err, in_path, None, env, method_body)?;
            writeln!(method_body, ">::new_err(out_value.err);")?;
        }

        writeln!(method_body, "}}")?;
    } else {
        panic!("Not in writeable out form")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_non_opaque_struct() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct {
                    a: u8,
                    b: u8,
                }

                impl MyStruct {
                    fn new(a: u8, b: u8) -> MyStruct {
                        unimplemented!()
                    }

                    fn get_a(&self) -> u8 {
                        unimplemented!()
                    }

                    fn set_b(&mut self, b: u8) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_simple_opaque_struct() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    fn new(a: u8, b: u8) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    fn get_a(&self) -> u8 {
                        unimplemented!()
                    }

                    fn set_b(&mut self, b: u8) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_method_taking_str() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    fn new_str(v: &str) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    fn set_str(&mut self, new_str: &str) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_method_writeable_out() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    fn write(&self, out: &mut DiplomatWriteable) {
                        unimplemented!()
                    }

                    fn write_unit(&self, out: &mut DiplomatWriteable) -> () {
                        unimplemented!()
                    }

                    fn write_result(&self, out: &mut DiplomatWriteable) -> DiplomatResult<(), u8> {
                        unimplemented!()
                    }

                    fn write_no_rearrange(&self, out: &mut DiplomatWriteable) -> u8 {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
