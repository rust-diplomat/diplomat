use diplomat_core::Env;
use std::fmt::{self, Display as _, Write as _};

use diplomat_core::ast;

use super::conversions::{gen_value_js_to_rust, BufferedIntoJs, ValueIntoJs};
use super::display;
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
    match custom_type {
        ast::CustomType::Enum(enm) => {
            writeln!(
                out,
                "const {}_js_to_rust = {};",
                enm.name,
                display::block(|mut f| {
                    enm.variants.iter().try_for_each(|(name, discriminant, _)| {
                        writeln!(f, "\"{}\": {},", name, discriminant)
                    })
                })
            )?;

            writeln!(
                out,
                "const {}_rust_to_js = {};",
                enm.name,
                display::block(|mut f| {
                    enm.variants.iter().try_for_each(|(name, discriminant, _)| {
                        writeln!(f, "{}: \"{}\",", discriminant, name)
                    })
                })
            )
        }
        ast::CustomType::Struct(strct) => {
            writeln!(
                out,
                "export class {} {}",
                strct.name,
                display::block(|mut f| {
                    writeln!(
                        f,
                        "constructor(underlying) {}",
                        display::block(|mut f| {
                            let (offsets, _) = layout::struct_offsets_size_max_align(
                                strct.fields.iter().map(|(_, typ, _)| typ),
                                in_path,
                                env,
                            );

                            for ((name, typ, _), &offset) in strct.fields.iter().zip(offsets.iter())
                            {
                                // If the type of a field has any named lifetimes
                                // (elision is impossible in fields), then it
                                // borrows from self because the lifetime guard
                                // is attached to self. In the future, we may
                                // want to be more intelligent about this and
                                // only attach lifetime guards to the exact object
                                // that holds it, instead of the outermost struct.
                                let borrows_self = typ.any_lifetime(|lifetime, _| {
                                    matches!(lifetime, ast::Lifetime::Named(_))
                                });

                                writeln!(
                                    f,
                                    "this.{} = {};",
                                    name,
                                    BufferedIntoJs {
                                        buf_ptr: "underlying",
                                        offset,
                                        typ,
                                        in_path,
                                        borrows_self,
                                        borrowed_params: &[],
                                        env,
                                    }
                                )?;
                            }

                            Ok(())
                        })
                    )?;

                    for method in strct.methods.iter() {
                        writeln!(f)?;
                        gen_method(method, in_path, env, &mut f)?;
                    }
                    Ok(())
                })
            )
        }
        ast::CustomType::Opaque(opaque) => {
            writeln!(
                out,
                "const {}_box_destroy_registry = new FinalizationRegistry(underlying => {});",
                opaque.name,
                display::block(|mut f| {
                    writeln!(f, "wasm.{}_destroy(underlying);", opaque.name)
                })
            )?;
            writeln!(out)?;
            writeln!(
                out,
                "export class {} {}",
                opaque.name,
                display::block(|mut f| {
                    writeln!(
                        f,
                        "constructor(underlying) {}",
                        display::block(|mut f| {
                            writeln!(f, "this.underlying = underlying;")?;
                            writeln!(
                                f,
                                "{}_box_destroy_registry.register(this, underlying);",
                                opaque.name
                            )
                        })
                    )?;

                    for method in opaque.methods.iter() {
                        writeln!(f)?;
                        gen_method(method, in_path, env, &mut f)?;
                    }

                    Ok(())
                })
            )
        }
    }
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

    if let Some(ref self_param) = method.self_param {
        gen_value_js_to_rust(
            &ast::Ident::from("this"),
            &self_param.to_typename(),
            in_path,
            env,
            &mut pre_stmts,
            &mut all_param_exprs,
            &mut post_stmts,
        );
    }

    for p in method.params.iter() {
        gen_value_js_to_rust(
            &p.name,
            &p.ty,
            in_path,
            env,
            &mut pre_stmts,
            &mut all_param_exprs,
            &mut post_stmts,
        );
    }

    let mut all_params = method
        .params
        .iter()
        .map(|p| p.name.as_str())
        .collect::<Vec<_>>();

    if is_writeable {
        *all_param_exprs.last_mut().unwrap() = "writeable".to_string();

        all_params.pop();
    }

    let all_params_invocation = {
        if let Some(ref return_type) = method.return_type {
            if let ReturnTypeForm::Complex = return_type_form(return_type, in_path, env) {
                all_param_exprs.insert(0, "diplomat_receive_buffer".to_string());
            }
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
        display::block(|mut f| {
            for s in pre_stmts.iter() {
                writeln!(f, "{}", s)?;
            }

            let invocation_expr =
                format!("wasm.{}({})", method.full_path_name, all_params_invocation);

            writeln!(
                f,
                "const diplomat_out = {};",
                display::expr(|f| {
                    let display_return_type = display::expr(|f| match &method.return_type {
                        None | Some(ast::TypeName::Unit) => invocation_expr.fmt(f),
                        Some(typ) => {
                            let borrowed_params = method.borrowed_params();
                            ValueIntoJs {
                                value_expr: &invocation_expr,
                                typ,
                                borrows_self: borrowed_params.borrows_self(),
                                borrowed_params: &borrowed_params.1[..],
                                in_path,
                                env,
                            }
                            .fmt(f)
                        }
                    });

                    if is_writeable {
                        write!(
                            f,
                            "diplomatRuntime.withWriteable(wasm, (writeable) => {})",
                            display::block(|mut f| writeln!(f, "return {};", display_return_type))
                        )
                    } else {
                        write!(f, "{}", display_return_type)
                    }
                })
            )?;

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
