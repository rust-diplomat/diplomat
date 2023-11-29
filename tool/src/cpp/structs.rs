use diplomat_core::Env;
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast;
use indenter::indented;

use crate::cpp::config::LibraryConfig;
use crate::cpp::util::{gen_comment_block, transform_keyword_ident};

use super::conversions::{gen_cpp_to_rust, gen_rust_to_cpp};
use super::types::gen_type;

pub fn gen_struct<W: fmt::Write>(
    custom_type: &ast::CustomType,
    in_path: &ast::Path,
    is_header: bool,
    env: &Env,
    library_config: &LibraryConfig,
    docs_url_gen: &ast::DocsUrlGenerator,
    out: &mut W,
) -> fmt::Result {
    match custom_type {
        ast::CustomType::Opaque(opaque) => {
            if is_header {
                writeln!(
                    out,
                    "/**\n * A destruction policy for using {} with {}.\n */",
                    custom_type.name(),
                    library_config.unique_ptr.name,
                )?;
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

                gen_comment_block(
                    out,
                    &opaque
                        .docs
                        .to_markdown(docs_url_gen, ast::MarkdownStyle::Normal),
                )?;
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
                    library_config,
                    docs_url_gen,
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
                writeln!(
                    &mut public_body,
                    "{0}() = default;\n{0}({0}&&) noexcept = default;\n{0}& operator=({0}&& other) noexcept = default;",
                    opaque.name
                )?;
                writeln!(out, " private:")?;
                let mut private_body = indented(out).with_str("  ");
                writeln!(
                    &mut private_body,
                    "{}<capi::{}, {}Deleter> inner;",
                    library_config.unique_ptr.expr, opaque.name, opaque.name
                )?;
                writeln!(out, "}};")?;
            }
        }

        ast::CustomType::Struct(strct) => {
            if is_header {
                gen_comment_block(
                    out,
                    &strct
                        .docs
                        .to_markdown(docs_url_gen, ast::MarkdownStyle::Normal),
                )?;
                writeln!(out, "struct {} {{", strct.name)?;
                writeln!(out, " public:")?;
            }

            let mut public_body = if is_header {
                indented(out).with_str("  ")
            } else {
                indented(out).with_str("")
            };

            if is_header {
                for (name, typ, docs) in &strct.fields {
                    let ty_name = gen_type(typ, in_path, None, env, library_config, true)?;
                    gen_comment_block(
                        &mut public_body,
                        &docs.to_markdown(docs_url_gen, ast::MarkdownStyle::Normal),
                    )?;
                    writeln!(&mut public_body, "{ty_name} {name};")?;
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
                    library_config,
                    docs_url_gen,
                    &mut public_body,
                )?;
            }

            if is_header {
                writeln!(out, "}};")?;
            }
        }

        ast::CustomType::Enum(_) => {}
        &_ => unreachable!("unknown AST/HIR variant"),
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn gen_method<W: fmt::Write>(
    enclosing_type: &ast::CustomType,
    method: &ast::Method,
    in_path: &ast::Path,
    is_header: bool,
    // should it convert writeables to string as an additional method?
    writeable_to_string: bool,
    env: &Env,
    library_config: &LibraryConfig,
    docs_url_gen: &ast::DocsUrlGenerator,
    out: &mut W,
) -> fmt::Result {
    // This method should rearrange the writeable
    let rearranged_writeable = method.is_writeable_out() && writeable_to_string;

    // This method has some writeable param that is preserved
    let has_writeable_param = method.has_writeable_param() && !writeable_to_string;

    if rearranged_writeable {
        // generate the normal method too
        gen_method(
            enclosing_type,
            method,
            in_path,
            is_header,
            false,
            env,
            library_config,
            docs_url_gen,
            out,
        )?;
    }

    if is_header {
        let mut comment = super::docs::gen_docs_and_lifetime_notes_markdown(
            method,
            docs_url_gen,
            ast::MarkdownStyle::Normal,
        );

        if method.params.iter().any(|p| {
            matches!(
                p,
                ast::Param {
                    ty: ast::TypeName::StrReference(_, ast::StringEncoding::Utf8),
                    ..
                }
            )
        }) {
            if !comment.is_empty() {
                writeln!(comment).unwrap();
                writeln!(comment).unwrap();
            }
            writeln!(comment, "Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).").unwrap();
        }

        gen_comment_block(out, &comment)?;
    }
    let params_to_gen = gen_method_interface(
        method,
        enclosing_type,
        in_path,
        is_header,
        has_writeable_param,
        rearranged_writeable,
        env,
        library_config,
        out,
        writeable_to_string,
    )?;

    if is_header {
        writeln!(out, ";")?;
    } else {
        writeln!(out, " {{")?;

        let mut method_body = indented(out).with_str("  ");

        let mut all_params_invocation = vec![];

        if let Some(self_param) = &method.self_param {
            // non opaque structs are handled by-move, however
            // their `this` will still be a reference!
            let cpp_expr = if self_param.reference.is_some() {
                "this"
            } else {
                "std::move(*this)"
            };
            let invocation_expr = gen_cpp_to_rust(
                cpp_expr,
                "this",
                None,
                &self_param.to_typename(),
                in_path,
                env,
                true,
                &mut method_body,
            );
            all_params_invocation.push(invocation_expr);
        }

        for param in params_to_gen.iter() {
            match param.ty {
                ast::TypeName::StrReference(..) | ast::TypeName::PrimitiveSlice(..) => {
                    all_params_invocation.push(format!("{}.data()", param.name));
                    all_params_invocation.push(format!("{}.size()", param.name));
                }
                _ => {
                    let invocation_expr = gen_cpp_to_rust(
                        param.name.as_str(),
                        param.name.as_str(),
                        None,
                        &param.ty,
                        in_path,
                        env,
                        false,
                        &mut method_body,
                    );
                    all_params_invocation.push(invocation_expr);
                }
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
                    library_config,
                    &mut method_body,
                );

                if rearranged_writeable {
                    gen_writeable_out_value(&out_expr, ret_typ, &mut method_body)?;
                } else {
                    writeln!(&mut method_body, "return {out_expr};")?;
                }
            }
        }

        writeln!(out, "}}")?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn gen_method_interface<W: fmt::Write>(
    method: &ast::Method,
    enclosing_type: &ast::CustomType,
    in_path: &ast::Path,
    is_header: bool,
    has_writeable_param: bool,
    rearranged_writeable: bool,
    env: &Env,
    library_config: &LibraryConfig,
    out: &mut W,
    writeable_to_string: bool,
) -> Result<Vec<ast::Param>, fmt::Error> {
    if has_writeable_param {
        write!(out, "template<typename W> ")?;
    }

    if !is_header {
        write!(out, "inline ")?;
    }

    let mut is_const = false;
    if let Some(ref self_param) = method.self_param {
        // If it's pass-by-value, mutability doesn't matter.
        if let Some((_, ref mutability)) = self_param.reference {
            is_const = mutability.is_immutable();
        }
    } else if is_header {
        write!(out, "static ")?;
    }

    if rearranged_writeable {
        if let Some(ast::TypeName::Result(_, err, _)) = &method.return_type {
            let err_ty = if err.is_zst() {
                "std::monostate".into()
            } else {
                gen_type(err, in_path, None, env, library_config, false)?
            };
            write!(out, "diplomat::result<std::string, {err_ty}>")?;
        } else {
            write!(out, "std::string")?;
        }
    } else {
        let ty_name = match &method.return_type {
            Some(ret_type) => gen_type(ret_type, in_path, None, env, library_config, false)?,

            None => "void".into(),
        };
        write!(out, "{ty_name}")?;
    }

    if is_header {
        write!(out, " ")?;
    } else {
        write!(out, " {}::", enclosing_type.name())?;
    }

    if has_writeable_param {
        write!(out, "{}_to_writeable(", method.name)?;
    } else {
        write!(out, "{}(", transform_keyword_ident(&method.name))?;
    }

    let mut params_to_gen = method.params.clone();
    if rearranged_writeable {
        params_to_gen.remove(params_to_gen.len() - 1);
    }

    for (i, param) in params_to_gen.iter().enumerate() {
        if i != 0 {
            write!(out, ", ")?;
        }
        let ty_name = if param.is_writeable() && !writeable_to_string {
            "W&".into()
        } else {
            gen_type(&param.ty, in_path, None, env, library_config, false)?
        };
        write!(out, "{} {}", ty_name, param.name)?;
    }

    write!(out, ")")?;

    if is_const {
        write!(out, " const")?
    }
    Ok(params_to_gen)
}

fn gen_writeable_out_value<W: fmt::Write>(
    out_expr: &str,
    ret_typ: &ast::TypeName,
    method_body: &mut W,
) -> fmt::Result {
    if let ast::TypeName::Result(_, _, _) = ret_typ {
        writeln!(
            method_body,
            "return {out_expr}.replace_ok(std::move(diplomat_writeable_string));"
        )?;
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
                    pub fn new(a: u8, b: u8) -> MyStruct {
                        unimplemented!()
                    }

                    pub fn get_a(&self) -> u8 {
                        unimplemented!()
                    }

                    pub fn set_b(&mut self, b: u8) {
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
                    pub fn new(a: u8, b: u8) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn get_a(&self) -> u8 {
                        unimplemented!()
                    }

                    pub fn set_b(&mut self, b: u8) {
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
                    pub fn new_str(v: &DiplomatStr) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn set_str(&mut self, new_str: &DiplomatStr) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_method_taking_slice() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    pub fn new_slice(v: &[f64]) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn set_slice(&mut self, new_slice: &[f64]) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_method_taking_slice_using_library_config() {
        test_file_using_library_config! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    pub fn new_slice(v: &[f64]) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn set_slice(&mut self, new_slice: &[f64]) {
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
                    pub fn write(&self, out: &mut DiplomatWriteable) {
                        unimplemented!()
                    }

                    pub fn write_unit(&self, out: &mut DiplomatWriteable) -> () {
                        unimplemented!()
                    }

                    pub fn write_result(&self, out: &mut DiplomatWriteable) -> Result<(), u8> {
                        unimplemented!()
                    }

                    pub fn write_no_rearrange(&self, out: &mut DiplomatWriteable) -> u8 {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_struct_documentation() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                /// Documentation for Foo.
                /// Second line.
                #[diplomat::rust_link(foo::Bar, Struct)]
                struct Foo {
                    /// Documentation for x.
                    x: u8,
                }

                impl Foo {
                    /// Documentation for get_x.
                    #[diplomat::rust_link(foo::Bar::get, FnInStruct)]
                    pub fn get_x(&self) -> u8 {
                        x
                    }
                }
            }
        }
    }

    #[test]
    fn test_lifetimes() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct Foo<'a>;

                impl Foo {
                    pub fn foo<'a>(a: &'a [u8]) -> Foo<'a> {
                        todo!()
                    }
                    pub fn bar<'a>(a: &'static [u8]) -> u32 {
                        todo!()
                    }
                }
            }
        }
    }
}
