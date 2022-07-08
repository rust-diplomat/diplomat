use diplomat_core::{ast, Env};
use std::collections::BTreeMap;
use std::fmt::{self, Display as _, Write as _};
use std::num::NonZeroUsize;

use super::conversions::{
    gen_value_js_to_rust, Argument, Base, Csv, Invocation, InvocationIntoJs, ReceivedEdges,
    Underlying, UnderlyingIntoJs, UnpackedBinding,
};
use super::display;
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
                    let underlying: ast::Ident = "underlying".into();
                    writeln!(
                        f,
                        "constructor({params}) {body}",
                        params = display::expr(|f| {
                            underlying.fmt(f)?;
                            for lifetime in strct.lifetimes.names() {
                                write!(f, ", {}", ReceivedEdges(lifetime))?;
                            }
                            Ok(())
                        }),
                        body = display::block(|mut f| {
                            let (offsets, _) = layout::struct_offsets_size_max_align(
                                strct.fields.iter().map(|(_, typ, _)| typ),
                                in_path,
                                env,
                            );

                            for ((name, inner, _), &offset) in
                                strct.fields.iter().zip(offsets.iter())
                            {
                                let borrows: Vec<Argument> = inner
                                    .longer_lifetimes(&strct.lifetimes)
                                    .iter()
                                    .map(|lt| Argument::ReceivedEdges(ReceivedEdges(lt)))
                                    .collect();

                                writeln!(
                                    f,
                                    "this.{} = {};",
                                    name,
                                    UnderlyingIntoJs {
                                        inner,
                                        underlying: Underlying::Binding(
                                            &underlying,
                                            NonZeroUsize::new(offset),
                                        ),
                                        base: Base {
                                            in_path,
                                            env,
                                            borrows: &borrows[..],
                                        },
                                    },
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
                    writeln!(f, "#lifetimeEdges = [];")?;
                    writeln!(
                        f,
                        "constructor(underlying, owned, edges) {}",
                        display::block(|mut f| {
                            writeln!(f, "this.underlying = underlying;")?;
                            writeln!(f, "this.#lifetimeEdges.push(...edges);")?;
                            writeln!(
                                f,
                                "if (owned) {}",
                                display::block(|mut f| {
                                    writeln!(
                                        f,
                                        "{}_box_destroy_registry.register(this, underlying);",
                                        opaque.name
                                    )
                                })
                            )
                        }),
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

    let mut entries: BTreeMap<&ast::NamedLifetime, Vec<Argument>> = BTreeMap::new();

    let borrowed_current_to_root = method
        .return_type
        .as_ref()
        .map(|return_type| {
            return_type
                .longer_lifetimes(&method.lifetime_env)
                // TODO: optimize this so we don't have to double alloc
                .into_iter()
                .map(|name| (name, name))
                .collect()
        })
        .unwrap_or_default();

    if let Some(ref self_param) = method.self_param {
        let self_type = self_param.to_typename();
        gen_value_js_to_rust(
            UnpackedBinding::This,
            &self_type,
            in_path,
            env,
            &mut pre_stmts,
            &mut all_param_exprs,
            &mut post_stmts,
            &method.lifetime_env,
            &borrowed_current_to_root,
            &mut entries,
        );
    }

    for p in method.params.iter() {
        gen_value_js_to_rust(
            UnpackedBinding::MethodParam(&p.name),
            &p.ty,
            in_path,
            env,
            &mut pre_stmts,
            &mut all_param_exprs,
            &mut post_stmts,
            &method.lifetime_env,
            &borrowed_current_to_root,
            &mut entries,
        );
    }

    // Rebuild the mapping from lifetimes to `Arguments`, but using the names of
    // lifetimes as declared in the returned struct, if the return type is a struct.
    // Otherwise return `None`.
    let arguments_that_use_lifetimes: Option<BTreeMap<&ast::NamedLifetime, Vec<Argument>>> =
        if let Some(ast::TypeName::Named(path_type)) = &method.return_type {
            path_type
                .resolve(in_path, env)
                .lifetimes()
                .map(|lifetime_env| {
                    // change names in entries from path_type names to lifetime_env names
                    assert_eq!(
                        path_type.lifetimes.len(),
                        lifetime_env.len(),
                        "{} doesn't have the same number of lifetimes as declared with",
                        path_type.path
                    );

                    path_type
                        .lifetimes
                        .iter()
                        .zip(lifetime_env.names())
                        .filter_map(|(path_lt, decl_lt)| {
                            if let ast::Lifetime::Named(path_lt) = path_lt {
                                return Some((decl_lt, entries[path_lt].clone()));
                            }
                            None
                        })
                        .collect()
                })
        } else {
            None
        };

    let mut all_params: Vec<UnpackedBinding> = method
        .params
        .iter()
        .map(|p| UnpackedBinding::MethodParam(&p.name))
        .collect();

    if is_writeable {
        *all_param_exprs.last_mut().unwrap() = "writeable".to_string();

        all_params.pop();
    }

    if method.self_param.is_none() {
        out.write_str("static ")?;
    }

    writeln!(
        out,
        "{}({}) {}",
        method.name,
        Csv(&all_params[..]),
        display::block(|mut f| {
            for s in pre_stmts.iter() {
                writeln!(f, "{}", s)?;
            }

            let diplomat_out = display::expr(|f| {
                let display_return_type = display::expr(|f| {
                    let invocation =
                        Invocation::new(method.full_path_name.clone(), all_param_exprs.clone());

                    if let Some(ref typ) = method.return_type {
                        let mut borrows: Vec<Argument> = entries
                            .values()
                            .flat_map(|bindings| bindings.iter().cloned())
                            .collect();

                        borrows.sort_unstable();
                        borrows.dedup();

                        InvocationIntoJs {
                            invocation,
                            typ,
                            lifetimes: arguments_that_use_lifetimes.as_ref(),
                            base: Base {
                                in_path,
                                env,
                                borrows: &borrows[..],
                            },
                        }
                        .fmt(f)
                    } else {
                        invocation.scalar().fmt(f)
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
            });

            let do_return = method.return_type.is_some() || is_writeable;

            if post_stmts.is_empty() && do_return {
                writeln!(f, "return {diplomat_out};")
            } else {
                if do_return {
                    writeln!(f, "const diplomat_out = {diplomat_out};")?;
                } else {
                    writeln!(f, "{diplomat_out};")?;
                }

                for s in post_stmts.iter() {
                    writeln!(f, "{}", s)?;
                }

                if do_return {
                    writeln!(f, "return diplomat_out;")?;
                }

                Ok(())
            }
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
