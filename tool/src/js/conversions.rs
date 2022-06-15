use diplomat_core::Env;
use std::fmt;
use std::fmt::Write;

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
                let js_param_name = ast::Ident::from(format!("{}_diplomat_slice", param_name));
                pre_logic.push(format!(
                    "let {js_param_name} = diplomatRuntime.DiplomatBuf.slice(wasm, {param_name}, {align});",
                    align = layout::primitive_size_alignment(*prim).align()
                ));
                js_param_name
            } else {
                let js_param_name = ast::Ident::from(format!("{}_diplomat_str", param_name));
                pre_logic.push(format!(
                    "let {js_param_name} = diplomatRuntime.DiplomatBuf.str(wasm, {param_name});"
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
                    // recursively call on single field
                }
                write!(
                    out,
                    "{}",
                    display::iife(|mut f| {
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

                        writeln!(
                            f,
                            "wasm.diplomat_free(diplomat_receive_buffer, {size}, {align});",
                            size = strct_size_align.size(),
                            align = strct_size_align.align()
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
            gen_rust_reference_to_js(
                underlying.as_ref(),
                in_path,
                value_expr,
                borrowed_params,
                env,
                out,
            )?;
        }

        ast::TypeName::Option(_) => {
            gen_rust_reference_to_js(typ, in_path, value_expr, borrowed_params, env, out)?;
        }

        ast::TypeName::Result(ok, err) => {
            let (ok_offset, result_size_align) =
                layout::result_ok_offset_size_align(ok, err, in_path, env);
            let needs_buffer = return_type_form(typ, in_path, env) == ReturnTypeForm::Complex;
            write!(
                out,
                "{}",
                display::iife(|mut f| {
                    let (size, align) = (result_size_align.size(), result_size_align.align());
                    if needs_buffer {
                        writeln!(
                            f,
                            "const diplomat_receive_buffer = wasm.diplomat_alloc({size}, {align});"
                        )?;
                        writeln!(f, "{};", value_expr)?;
                        writeln!(
                            f,
                            "const is_ok = {};",
                            display::expr(|mut f| {
                                gen_rust_reference_to_js(
                                    &ast::TypeName::Primitive(ast::PrimitiveType::bool),
                                    in_path,
                                    &format!("diplomat_receive_buffer + {}", ok_offset),
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
                                            borrowed_params,
                                            env,
                                            &mut f,
                                        )
                                    })
                                )?;
                                writeln!(
                                    f,
                                    "wasm.diplomat_free(diplomat_receive_buffer, {size}, {align});"
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
                                            borrowed_params,
                                            env,
                                            &mut f,
                                        )
                                    })
                                )?;
                                writeln!(
                                    f,
                                    "wasm.diplomat_free(diplomat_receive_buffer, {size}, {align});"
                                )?;
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
            gen_rust_reference_to_js(
                underlying.as_ref(),
                in_path,
                value_expr,
                borrowed_params,
                env,
                out,
            )?;
        }
        ast::TypeName::Writeable => todo!(),
        ast::TypeName::StrReference(..) => {
            gen_rust_reference_to_js(typ, in_path, value_expr, borrowed_params, env, out)?;
        }
        ast::TypeName::PrimitiveSlice(..) => todo!(),
        ast::TypeName::Unit => write!(out, "{}", value_expr)?,
    }

    Ok(())
}

/// TODO: docs
pub fn gen_rust_reference_to_js<W: fmt::Write>(
    underlying: &ast::TypeName,
    in_path: &ast::Path,
    value_expr: &str,
    borrowed_params: &ast::BorrowedParams,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match underlying {
        ast::TypeName::Box(typ) | ast::TypeName::Reference(.., typ) => {
            gen_rust_reference_to_js(
                typ.as_ref(),
                in_path,
                &format!(
                    "(new Uint32Array(wasm.memory.buffer, {}, 1))[0]",
                    value_expr
                ),
                borrowed_params,
                env,
                out,
            )?;
        }
        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(..) | ast::TypeName::Reference(..) => write!(
                out,
                "{}",
                display::iife(|mut f| {
                    let ident = ast::Ident::from("option_ptr");
                    writeln!(
                        f,
                        "const {ident} = (new Uint32Array(wasm.memory.buffer, {}, 1))[0];",
                        value_expr
                    )?;
                    writeln!(
                        f,
                        "return ({ident} == 0) ? null : {};",
                        display::expr(|mut f| {
                            gen_value_rust_to_js(
                                ident.as_str(),
                                underlying.as_ref(),
                                in_path,
                                borrowed_params,
                                env,
                                &mut f,
                            )
                        })
                    )
                })
            )?,
            other @ (ast::TypeName::StrReference(..) | ast::TypeName::PrimitiveSlice(..)) => {
                panic!("`{}` is a fat pointer (ptr, length), and so it can't be stored behind an option.", other)
            }
            other => panic!(
                "`{0}` doesn't have the same alignment as ptr, so `Option<{0}>` is unsupported",
                other
            ),
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
                            borrowed_params,
                            env,
                            &mut f,
                        )
                    })
                )?;
            } else {
                let params_borrowing_non_strrefs = borrowed_params
                    .borrowed_params
                    .iter()
                    .filter(|param| {
                        !param.ty.all_lifetimes(|lifetime, origin| {
                            matches!(
                                (lifetime, origin),
                                (ast::Lifetime::Named(_), ast::LifetimeOrigin::StrReference)
                            )
                        })
                    })
                    .collect::<Vec<_>>();

                if !borrowed_params.borrows_self && params_borrowing_non_strrefs.is_empty() {
                    write!(out, "new {}({})", custom_type.name(), value_expr)?;
                } else {
                    write!(
                        out,
                        "{}",
                        display::iife(|mut f| {
                            writeln!(f, "const out = new {}({});", custom_type.name(), value_expr)?;
                            if borrowed_params.borrows_self {
                                writeln!(f, "out.__this_lifetime_guard = this;")?;
                            }
                            for param in params_borrowing_non_strrefs.iter() {
                                writeln!(f, "out.__{0}_lifetime_guard = {0};", param.name)?;
                            }
                            writeln!(f, "return out;")
                        })
                    )?;
                }
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
                "{}",
                display::iife(|mut f| {
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
