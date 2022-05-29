use diplomat_core::Env;
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast;
use indenter::indented;

use super::conversions::{gen_value_js_to_rust, gen_value_rust_to_js};
use super::types::{return_type_form, ReturnTypeForm};
use crate::layout;

/// Generates a JS class declaration
///
/// # Examples
///
/// ```js
/// const MyStruct_box_destroy_registry = new FinalizationRegistry(underlying => {
///   wasm.MyStruct_destroy(underlying);
/// })
///
/// export class MyStruct {
///   constructor(underlying) {
///     this.underlying = underlying;
///   }
///
///   // snip
/// }
/// ```
pub fn gen_struct<W: fmt::Write>(
    out: &mut W,
    custom_type: &ast::CustomType,
    in_path: &ast::Path,
    env: &Env,
) -> fmt::Result {
    if let ast::CustomType::Enum(enm) = custom_type {
        writeln!(
            out,
            "const {}_js_to_rust = {};",
            enm.name,
            BlockDisplay(|mut f| {
                enm.variants.iter().try_for_each(|(name, discriminant, _)| {
                    writeln!(f, "\"{}\": {},", name, discriminant)
                })
            })
        )?;

        writeln!(
            out,
            "const {}_rust_to_js = {};",
            enm.name,
            BlockDisplay(|mut f| {
                enm.variants.iter().try_for_each(|(name, discriminant, _)| {
                    writeln!(f, "{}: \"{}\"", discriminant, name)
                })
            })
        )?;
    } else {
        writeln!(
            out,
            "const {}_box_destroy_registry = new FinalizationRegistry(underlying => {});",
            custom_type.name(),
            BlockDisplay(|mut f| {
                writeln!(f, "wasm.{}_destroy(underlying);", custom_type.name())
            })
        )?;

        writeln!(out)?;

        writeln!(
            out,
            "export class {} {}",
            custom_type.name(),
            BlockDisplay(|mut f| {
                writeln!(
                    &mut f,
                    "constructor(underlying) {}",
                    BlockDisplay(|mut f| { writeln!(f, "this.underlying = underlying;") })
                )?;

                for method in custom_type.methods().iter() {
                    writeln!(f)?;
                    gen_method(method, in_path, env, &mut f)?;
                }

                if let ast::CustomType::Struct(strct) = custom_type {
                    let (offsets, _) = layout::struct_offsets_size_max_align(
                        strct.fields.iter().map(|(_, typ, _)| typ),
                        in_path,
                        env,
                    );
                    for ((name, typ, _), offset) in strct.fields.iter().zip(offsets.iter()) {
                        writeln!(f)?;
                        gen_field(name, typ, in_path, *offset, env, &mut f)?;
                    }
                }
                Ok(())
            })
        )?;
    }

    Ok(())
}

/// An `fmt::Display` object that accepts a writer function, and wraps everything
/// that it writes in braces to form a block.
///
/// This allows for generating source code without having to manually insert
/// opening/closing braces for blocks or worry about indentation.
struct BlockDisplay<F>(F)
where
    F: Fn(indenter::Indented<fmt::Formatter>) -> fmt::Result;

impl<F> fmt::Display for BlockDisplay<F>
where
    F: Fn(indenter::Indented<fmt::Formatter>) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        self.0(indented(f).with_str("  "))?;
        write!(f, "}}")
    }
}

/// Generates a getter function for a field.
///
/// # Examples
///
/// ```js
/// get a() {
///   return (() => {
///     // snip
///   })();
/// }
/// ```
fn gen_field<W: fmt::Write>(
    name: &ast::Ident,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    offset: usize,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    writeln!(
        out,
        "get {}() {}",
        name,
        BlockDisplay(|mut f| {
            write!(f, "return ")?;
            gen_value_rust_to_js(
                &format!("this.underlying + {}", offset),
                &ast::TypeName::Reference(
                    ast::Lifetime::Anonymous,
                    ast::Mutability::Mutable,
                    Box::new(typ.clone()),
                ),
                in_path,
                env,
                &mut f,
            )?;
            writeln!(f, ";")
        })
    )
}

/// Generates the contents of a JS method.
///
/// # Examples
///
/// It could generate something like this
/// ```js
/// static node(data) {
///   const diplomat_out = (() => {
///     // snip
///   })
/// }
/// ```
fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    let is_writeable = method.is_writeable_out();

    let mut pre_stmts = vec![];
    let mut all_param_exprs = vec![];
    let mut post_stmts = vec![];

    method.params.iter().for_each(|p| {
        gen_value_js_to_rust(
            p.name.clone(),
            &p.ty,
            in_path,
            env,
            &mut pre_stmts,
            &mut all_param_exprs,
            &mut post_stmts,
        )
    });

    let mut all_params = method
        .params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>();

    if is_writeable {
        let last_index_exprs = all_param_exprs.len() - 1;
        all_param_exprs[last_index_exprs] = "writeable".to_string();

        all_params.remove(all_params.len() - 1);
    }

    let all_params_invocation = {
        if method.self_param.is_some() {
            all_param_exprs.insert(0, "this.underlying".to_string());
        }

        if method.return_type.is_some()
            && return_type_form(method.return_type.as_ref().unwrap(), in_path, env)
                == ReturnTypeForm::Complex
        {
            all_param_exprs.insert(0, "diplomat_receive_buffer".to_string());
        }

        all_param_exprs.join(", ")
    };

    if method.self_param.is_none() {
        out.write_str("static ")?;
    }

    writeln!(
        out,
        "{}({}) {}",
        method.name,
        all_params.join(", "),
        BlockDisplay(|mut f| {
            for s in pre_stmts.iter() {
                writeln!(f, "{}", s)?;
            }

            let invocation_expr =
                format!("wasm.{}({})", method.full_path_name, all_params_invocation);

            write!(f, "const diplomat_out = ")?;

            if is_writeable {
                writeln!(
                    f,
                    "diplomatRuntime.withWriteable(wasm, (writeable) => {});",
                    BlockDisplay(|mut f| {
                        write!(f, "return ")?;

                        match &method.return_type {
                            None | Some(ast::TypeName::Unit) => {
                                write!(f, "{}", invocation_expr)?;
                            }
                            Some(ret_type) => {
                                gen_value_rust_to_js(
                                    &invocation_expr,
                                    ret_type,
                                    in_path,
                                    env,
                                    &mut f,
                                )?;
                            }
                        }

                        writeln!(f, ";")
                    })
                )?;
            } else {
                match &method.return_type {
                    None | Some(ast::TypeName::Unit) => {
                        write!(f, "{}", invocation_expr)?;
                    }

                    Some(ret_type) => {
                        gen_value_rust_to_js(
                            &invocation_expr,
                            ret_type,
                            in_path,
                            env,
                            &mut f,
                        )?;
                    }
                }

                writeln!(f, ";")?;
            }

            for s in post_stmts.iter() {
                writeln!(f, "{}", s)?;
            }

            if method.return_type.is_some() || is_writeable {
                writeln!(f, "return diplomat_out;")?;
            }
            Ok(())
        })
    )
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
    fn test_method_returning_struct() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                struct NonOpaqueStruct {
                    a: u16,
                    b: u8,
                    c: u32,
                }

                impl MyStruct {
                    pub fn get_non_opaque(&self) -> NonOpaqueStruct {
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
                #[diplomat::rust_link(foo::bar::Batz, Struct)]
                /// Use this.
                struct MyStruct(UnknownType);

                impl MyStruct {
                    pub fn new_str(v: &str) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn set_str(&mut self, new_str: &str) {
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

                    pub fn write_result(&self, out: &mut DiplomatWriteable) -> DiplomatResult<(), u8> {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
