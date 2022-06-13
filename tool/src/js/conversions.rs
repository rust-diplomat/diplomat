use diplomat_core::Env;
use std::fmt;
use std::fmt::Write;
use std::ops::ControlFlow;

use diplomat_core::ast;

use super::display;
use super::types::{return_type_form, ReturnTypeForm};
use crate::layout;

/// TODO: docs
#[allow(clippy::ptr_arg)] // false positive, rust-clippy#8463, fixed in 1.61
pub fn gen_value_js_to_rust(
    param_name: &ast::Ident,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    borrowed_lifetimes: &[&ast::NamedLifetime],
    pre_logic: &mut Vec<String>,
    invocation_params: &mut Vec<String>,
    post_logic: &mut Vec<String>,
) {
    match typ {
        ast::TypeName::StrReference(lifetime) | ast::TypeName::PrimitiveSlice(lifetime, ..) => {
            let js_param_name = if let ast::TypeName::PrimitiveSlice(.., prim) = typ {
                let js_param_name = format!("{}_diplomat_slice", param_name);
                pre_logic.push(format!(
                    "let {param_name} = diplomatRuntime.RcAlloc.slice({param_name}, {align});",
                    align = layout::primitive_size_alignment(*prim).align()
                ));
                js_param_name
            } else {
                let js_param_name = format!("{}_diplomat_str", param_name);
                pre_logic.push(format!(
                    "let {js_param_name} = diplomatRuntime.RcAlloc.str({param_name});"
                ));
                js_param_name
            };

            invocation_params.push(format!("{js_param_name}.ptr"));
            invocation_params.push(format!("{js_param_name}.size"));

            match lifetime {
                ast::Lifetime::Named(ref named) if borrowed_lifetimes.contains(&named) => {
                    // TODO: don't create an edge if this is a str/slice,
                    // since they actually copy and thus don't rely on the buffer.
                    post_logic.push(format!(
                        "diplomat_out.__{param_name}_lifetime_guard = {js_param_name};",
                    ));
                }
                _ => {
                    post_logic.push(format!("{js_param_name}.free();"));
                }
            }
        }
        ast::TypeName::Primitive(ast::PrimitiveType::char) => {
            // we use the spread operator here to count codepoints
            // codePointAt() does not return surrogate pairs if there are multiple
            invocation_params.push(format!(
                "diplomatRuntime.extractCodePoint({p}, '{p}')",
                p = param_name
            ));
        }
        ast::TypeName::Box(_) => {
            invocation_params.push(format!("{}.underlying", param_name));
        }
        ast::TypeName::Reference(lifetime, _mut, _typ) => {
            match lifetime {
                ast::Lifetime::Named(named) if borrowed_lifetimes.contains(&named) => {
                    // We want to hold the reference here
                }
                _ => {}
            }
            invocation_params.push(format!("{param_name}.underlying"));
        }
        ast::TypeName::Named(path_type) => {
            match path_type.resolve(in_path, env) {
                ast::CustomType::Struct(struct_type) => {
                    // == THE GAMEPLAN ==
                    // do `Struct::borrowed_lifetimes`, which takes
                    // a filter that tells which indices of lifetimes we need.
                    // Let that filter take `i`, an index. do `path_type.lifetimes[i]`
                    // to get what the lifetime is in the current `LifetimeEnv`
                    // (not the `Struct`'s env!), and then see if that lifetime
                    // in `borrowed_lifetimes`. If it is,
                    // have the filter return `true`, otherwise `false`.
                    // The `Struct::borrowed_lifetimes` method will then return
                    // a list of all the relevant lifetimes in the `Struct`'s
                    // `LifetimeEnv` scope, which we can pass down to further
                    // recursive calls.
                    // Once we get this all working, we can start thinking about indices.
                    // ==================

                    // Shadow the borrowed lifetime from this invocation so they're
                    // ready for the recursive invocation.
                    let borrowed_lifetimes = struct_type.borrowed_lifetimes(|i| {
                        match path_type.lifetimes.get(i) {
                            Some(ast::Lifetime::Static) => false,
                            Some(ast::Lifetime::Named(ref named)) => {
                                borrowed_lifetimes.contains(&named)
                            }
                            Some(ast::Lifetime::Anonymous) | None => {
                                panic!("lifetime elision is unsupported, {} missing lifetime at index {}", path_type, i)
                            }
                        }
                    });

                    for (field_name, field_type, _) in struct_type.fields.iter() {
                        let field_extracted_name =
                            format!("diplomat_{}_extracted_{}", struct_type.name, field_name)
                                .into();
                        pre_logic.push(format!(
                            "const {} = {}[\"{}\"];",
                            field_extracted_name, param_name, field_name
                        ));

                        gen_value_js_to_rust(
                            &field_extracted_name,
                            field_type,
                            in_path,
                            env,
                            &borrowed_lifetimes[..],
                            pre_logic,
                            invocation_params,
                            post_logic,
                        );
                    }
                }
                ast::CustomType::Enum(enm) => {
                    invocation_params.push(format!("{}_js_to_rust[{}]", enm.name, param_name));
                }

                ast::CustomType::Opaque(_) => {
                    panic!("Opaque types cannot be sent as values");
                }
            }
        }
        _ => invocation_params.push(param_name.to_string()),
    }
}

/// TODO: docs
pub fn gen_value_rust_to_js<W: fmt::Write>(
    value_expr: &str,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    borrowed_params: &ast::BorrowedParams,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(path_type) => match path_type.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                let strct_size_align = layout::type_size_alignment(typ, in_path, env);
                let needs_buffer = return_type_form(typ, in_path, env);
                if needs_buffer != ReturnTypeForm::Complex {
                    todo!("Receiving structs that don't need a buffer: {}", strct.name)
                }
                write!(
                    out,
                    "(() => {})()",
                    display::block(|mut f| {
                        writeln!(
                            f,
                            "const diplomat_receive_buffer = wasm.diplomat_alloc({}, {});",
                            strct_size_align.size(),
                            strct_size_align.align(),
                        )?;
                        writeln!(f, "{};", value_expr)?;
                        writeln!(
                            f,
                            "const out = new {}(diplomat_receive_buffer);",
                            strct.name
                        )?;

                        for (name, typ, _) in strct.fields.iter() {
                            gen_box_destructor(name, typ, in_path, env, &mut f)?;
                        }

                        writeln!(
                            f,
                            "diplomat_alloc_destroy_registry.register(out, {});",
                            display::block(|mut f| {
                                writeln!(f, "ptr: out.underlying,")?;
                                writeln!(f, "size: {},", strct_size_align.size())?;
                                writeln!(f, "align: {},", strct_size_align.align())
                            })
                        )?;

                        writeln!(f, "return out;")
                    })
                )?;
            }

            ast::CustomType::Enum(enm) => {
                write!(out, "{}_rust_to_js[{}]", enm.name, value_expr)?;
            }

            ast::CustomType::Opaque(_) => {
                panic!("Opaque types cannot be used in value position")
            }
        },

        ast::TypeName::Box(underlying) => {
            write!(
                out,
                "(() => {})()",
                display::block(|mut f| {
                    writeln!(
                        f,
                        "const out = {};",
                        display::expr(|mut f| {
                            gen_rust_reference_to_js(
                                underlying.as_ref(),
                                in_path,
                                value_expr,
                                "null", // JS owns the box
                                borrowed_params,
                                env,
                                &mut f,
                            )
                        })
                    )?;

                    if let ast::TypeName::Named(path_type) = underlying.as_ref() {
                        writeln!(
                            f,
                            "{}_box_destroy_registry.register(out, out.underlying)",
                            path_type.resolve(in_path, env).name()
                        )?;
                    }

                    writeln!(f, "return out;")
                })
            )?;
        }

        ast::TypeName::Option(underlying) => {
            assert!(
                underlying.is_pointer(),
                "Options must contain pointer types"
            );
            write!(
                out,
                "(() => {})()",
                display::block(|mut f| {
                    writeln!(f, "const option_value = {}", value_expr)?;
                    writeln!(
                        f,
                        "if (option_value !== 0) {if_true} else {if_false}",
                        if_true = display::block(|mut f| {
                            writeln!(
                                f,
                                "const inhabited_value = {};",
                                display::expr(|mut f| {
                                    // TODO(#62): actually return `null` if the option is `None`
                                    gen_value_rust_to_js(
                                        "option_value",
                                        underlying.as_ref(),
                                        in_path,
                                        borrowed_params,
                                        env,
                                        &mut f,
                                    )
                                })
                            )?;
                            writeln!(f, "return inhabited_value;")
                        }),
                        if_false = display::block(|mut f| writeln!(f, "return null;"))
                    )
                })
            )?;
        }

        ast::TypeName::Result(ok, err) => {
            let (ok_offset, result_size_align) =
                layout::result_ok_offset_size_align(ok, err, in_path, env);
            let needs_buffer = return_type_form(typ, in_path, env) == ReturnTypeForm::Complex;
            write!(
                out,
                "(() => {})()",
                display::block(|mut f| {
                    if needs_buffer {
                        writeln!(
                            f,
                            "const rc_alloc = diplomatRuntime.RcAlloc.alloc({}, {});",
                            result_size_align.size(),
                            result_size_align.align()
                        )?;
                        writeln!(f, "const diplomat_receive_buffer = rc_alloc.ptr;",)?;
                        writeln!(f, "{};", value_expr)?;
                        writeln!(
                            f,
                            "const is_ok = {};",
                            display::expr(|mut f| {
                                gen_rust_reference_to_js(
                                    &ast::TypeName::Primitive(ast::PrimitiveType::bool),
                                    in_path,
                                    &format!("diplomat_receive_buffer + {}", ok_offset),
                                    "rc_alloc",
                                    borrowed_params,
                                    env,
                                    &mut f,
                                )
                            })
                        )?;
                        writeln!(
                            f,
                            "if (is_ok) {is_true} else {is_false}",
                            is_true = display::block(|mut f| {
                                writeln!(
                                    f,
                                    "const ok_value = {};",
                                    display::expr(|mut f| {
                                        gen_rust_reference_to_js(
                                            ok.as_ref(),
                                            in_path,
                                            "diplomat_receive_buffer",
                                            "rc_alloc",
                                            borrowed_params,
                                            env,
                                            &mut f,
                                        )
                                    })
                                )?;
                                writeln!(f, "return ok_value;")
                            }),
                            is_false = display::block(|mut f| {
                                writeln!(
                                    f,
                                    "const throw_value = {};",
                                    display::expr(|mut f| {
                                        gen_rust_reference_to_js(
                                            err.as_ref(),
                                            in_path,
                                            "diplomat_receive_buffer",
                                            "rc_alloc",
                                            borrowed_params,
                                            env,
                                            &mut f,
                                        )
                                    })
                                )?;
                                let borrows = err.as_ref().visit_lifetimes(&mut |lt, _| match lt {
                                    ast::Lifetime::Static => ControlFlow::Continue(()),
                                    ast::Lifetime::Named(_) => ControlFlow::Break(()),
                                    ast::Lifetime::Anonymous => {
                                        unreachable!("Lifetime elision in return types isn't allowed yet")
                                    }
                                }).is_break();

                                if !borrows {
                                    writeln!(f, "rc_alloc.free();")?;
                                }
                                writeln!(f, "throw new diplomatRuntime.FFIError(throw_value);")
                            })
                        )
                    } else {
                        writeln!(f, "const is_ok = {} == 1;", value_expr)?;
                        writeln!(
                            f,
                            "if (!is_ok) {}",
                            display::block(|mut f| {
                                writeln!(f, "throw new diplomatRuntime.FFIError({{}});")
                            })
                        )
                    }
                })
            )?;
        }

        ast::TypeName::Primitive(_prim) => {
            // TODO(#63): wrap with appropriate types for large widths
            write!(out, "{}", value_expr)?;
        }

        ast::TypeName::Reference(.., underlying) => {
            // TODO(#12): pass in lifetime of the reference
            gen_rust_reference_to_js(
                underlying.as_ref(),
                in_path,
                value_expr,
                "null",
                borrowed_params,
                env,
                out,
            )?;
        }
        ast::TypeName::Writeable => todo!(),
        ast::TypeName::StrReference(..) => {
            // So I just copied this from `gen_rust_reference_js` to make it work...
            write!(
                out,
                "(() => {})()",
                display::block(|mut f| {
                    writeln!(
                        f,
                        "const [ptr, len] = new Uint32Array(wasm.memory.buffer, {}, 2);",
                        value_expr
                    )?;
                    writeln!(f, "return diplomatRuntime.readString(wasm, ptr, len);")
                })
            )?;
        }
        ast::TypeName::PrimitiveSlice(..) => todo!(),
        ast::TypeName::Unit => write!(out, "{}", value_expr)?,
    }

    Ok(())
}

/// TODO: docs
fn gen_box_destructor<W: fmt::Write>(
    name: &ast::Ident,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> Result<(), fmt::Error> {
    match typ {
        ast::TypeName::Box(underlying) => {
            writeln!(out, "const out_{}_value = out.{};", name, name)?;
            // TODO(#12): delete back-references when we start generating them
            // since the out value getter returns a borrowed box
            if let ast::TypeName::Named(path_type) = underlying.as_ref() {
                writeln!(
                    out,
                    "{}_box_destroy_registry.register(out_{}_value, out_{}_value.underlying);",
                    path_type.resolve(in_path, env).name(),
                    name,
                    name
                )?;
            }
            writeln!(
                out,
                "Object.defineProperty(out, \"{}\", {{ value: out_{}_value }});",
                name, name
            )?;
        }

        ast::TypeName::Option(underlying) => {
            assert!(
                underlying.is_pointer(),
                "Options must contain pointer types"
            );

            writeln!(
                out,
                "if (out.{}.underlying !== 0) {if_true} else {if_false}",
                name,
                if_true = display::block(|mut f| {
                    gen_box_destructor(name, underlying.as_ref(), in_path, env, &mut f)
                }),
                if_false = display::block(|mut f| {
                    writeln!(
                        f,
                        "Object.defineProperty(out, \"{}\", {{ value: null }});",
                        name
                    )
                })
            )?;
            // TODO(#62): don't generate destructor if null
        }

        _ => {}
    }

    Ok(())
}

/// TODO: docs
fn gen_rust_reference_to_js<W: fmt::Write>(
    underlying: &ast::TypeName,
    in_path: &ast::Path,
    value_expr: &str,
    owner: &str,
    borrowed_params: &ast::BorrowedParams,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match underlying {
        ast::TypeName::Box(typ) => {
            gen_rust_reference_to_js(
                typ.as_ref(),
                in_path,
                &format!(
                    "(new Uint32Array(wasm.memory.buffer, {}, 1))[0]",
                    value_expr
                ),
                owner,
                borrowed_params,
                env,
                out,
            )?;
        }

        ast::TypeName::Reference(.., typ) => {
            gen_rust_reference_to_js(
                typ.as_ref(),
                in_path,
                &format!(
                    "(new Uint32Array(wasm.memory.buffer, {}, 1))[0]",
                    value_expr
                ),
                "null", // TODO(#12): pass in lifetime of the reference
                borrowed_params,
                env,
                out,
            )?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                // TODO(#62): return null if pointer is 0
                gen_rust_reference_to_js(
                    underlying.as_ref(),
                    in_path,
                    value_expr,
                    owner,
                    borrowed_params,
                    env,
                    out,
                )?;
            }
            _ => todo!(),
        },

        ast::TypeName::Result(_, _) => {
            todo!("Receiving references to results")
        }

        ast::TypeName::Named(path_type) => {
            let custom_type = path_type.resolve(in_path, env);

            if let ast::CustomType::Enum(enm) = custom_type {
                write!(
                    out,
                    "{}_rust_to_js[{}]",
                    enm.name,
                    display::expr(|mut f| {
                        gen_rust_reference_to_js(
                            &ast::TypeName::Primitive(ast::PrimitiveType::isize),
                            in_path,
                            value_expr,
                            owner,
                            borrowed_params,
                            env,
                            &mut f,
                        )
                    })
                )?;
            } else {
                write!(
                    out,
                    "(() => {})()",
                    display::block(|mut f| {
                        writeln!(f, "const out = new {}({});", custom_type.name(), value_expr)?;
                        writeln!(f, "out.owner = {};", owner)?;
                        if borrowed_params.borrows_self {
                            writeln!(f, "out.__this_lifetime_guard = this;")?;
                        }
                        for param in borrowed_params.borrowed_params.iter() {
                            let str_base = param
                                .ty
                                .visit_lifetimes(&mut |_, origin| match origin {
                                    ast::LifetimeOrigin::StrReference => ControlFlow::Break(()),
                                    _ => ControlFlow::Continue(()),
                                })
                                .is_break();

                            if !str_base {
                                writeln!(f, "out.__{0}_lifetime_guard = {0};", param.name)?;
                            }
                        }
                        writeln!(f, "return out;")
                    })
                )?;
            }
        }

        ast::TypeName::Primitive(prim) => {
            let prim_type = match prim {
                ast::PrimitiveType::i8 => "Int8Array",
                ast::PrimitiveType::u8 => "Uint8Array",
                ast::PrimitiveType::i16 => "Int16Array",
                ast::PrimitiveType::u16 => "Uint16Array",
                ast::PrimitiveType::i32 => "Int32Array",
                ast::PrimitiveType::u32 => "Uint32Array",
                ast::PrimitiveType::i64 => "BigInt64Array",
                ast::PrimitiveType::u64 => "BigUint64Array",
                ast::PrimitiveType::i128 => panic!("i128 not supported on JS"),
                ast::PrimitiveType::u128 => panic!("u128 not supported on JS"),
                ast::PrimitiveType::isize => "Int32Array",
                ast::PrimitiveType::usize => "Uint32Array",
                ast::PrimitiveType::f32 => "Float32Array",
                ast::PrimitiveType::f64 => "Float64Array",
                ast::PrimitiveType::bool => {
                    return write!(
                        out,
                        "(new Uint8Array(wasm.memory.buffer, {}, 1))[0] == 1",
                        value_expr
                    )
                }
                ast::PrimitiveType::char => {
                    return write!(
                        out,
                        "String.fromCharCode((new Uint32Array(wasm.memory.buffer, {}, 1))[0])",
                        value_expr
                    )
                }
            };

            write!(
                out,
                "(new {}(wasm.memory.buffer, {}, 1))[0]",
                prim_type, value_expr
            )?;
        }

        ast::TypeName::Unit => {
            write!(out, "{{}}")?;
        }

        ast::TypeName::StrReference(..) => {
            write!(
                out,
                "(() => {})()",
                display::block(|mut f| {
                    writeln!(
                        f,
                        "const [ptr, len] = new Uint32Array(wasm.memory.buffer, {}, 2);",
                        value_expr
                    )?;
                    writeln!(f, "return diplomatRuntime.readString(wasm, ptr, len);")
                })
            )?;
        }
        _ => todo!(),
    }

    Ok(())
}
