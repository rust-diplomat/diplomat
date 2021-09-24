use std::{collections::HashMap, fmt::Write};

use diplomat_core::ast;

pub fn gen_rust_to_cpp<W: Write>(
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
                    // TODO(#59): should emit a unique_ptr
                    todo!("Receiving boxes of structs is not yet supported")
                }

                ast::CustomType::Enum(_) => {
                    // TODO(#59): should emit a unique_ptr
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
                format!("static_cast<{}>({})", enm.name, cpp)
            }
        },

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                let raw_value_id = format!("diplomat_optional_raw_{}", path);
                writeln!(out, "auto {} = {};", raw_value_id, cpp).unwrap();

                let wrapped_value_id = format!("diplomat_optional_{}", path);
                super::types::gen_type(typ, in_path, None, env, out).unwrap();
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
            super::types::gen_type(typ, in_path, None, env, out).unwrap();
            writeln!(out, " {}({}.is_ok);", wrapped_value_id, raw_value_id).unwrap();

            if !ok.is_zst() || !err.is_zst() {
                writeln!(out, "if ({}.is_ok) {{", raw_value_id).unwrap();
                if !ok.is_zst() {
                    let ok_expr = gen_rust_to_cpp(
                        &format!("{}.ok", raw_value_id),
                        path,
                        ok,
                        in_path,
                        env,
                        out,
                    );
                    writeln!(
                        out,
                        "  {}.set_ok((std::move({})));",
                        wrapped_value_id, ok_expr
                    )
                    .unwrap();
                }
                writeln!(out, "}} else {{").unwrap();
                if !err.is_zst() {
                    let err_expr = gen_rust_to_cpp(
                        &format!("{}.err", raw_value_id),
                        path,
                        err,
                        in_path,
                        env,
                        out,
                    );
                    writeln!(
                        out,
                        "  {}.set_err((std::move({})));",
                        wrapped_value_id, err_expr
                    )
                    .unwrap();
                }
                writeln!(out, "}}").unwrap();
            }

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
        ast::TypeName::PrimitiveSlice(_) => {
            todo!("Returning &[T] from Rust to C++ is not currently supported")
        }
        ast::TypeName::Unit => cpp.to_string(),
    }
}

/// Meta information about a [ast::TypeName::Reference].
#[derive(Eq, PartialEq)]
pub struct ReferenceMeta {
    /// Whether or not the reference is owned.
    owned: bool,
    /// Whether or not the reference is mutable.
    mutable: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn gen_cpp_to_rust<W: Write>(
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
                writeln!(out, "capi::DiplomatWriteable {cpp}_writer = diplomat::WriteableTrait<W>::Construct({cpp});", cpp=cpp).unwrap();
                format!("&{}_writer", cpp)
            } else {
                panic!("Cannot send Writeable to Rust as a value");
            }
        }
        ast::TypeName::Primitive(_) => cpp.to_string(),
        o => todo!("{:?}", o),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_enum_conversion() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                enum MyEnum {
                    A, B, C
                }
                struct MyStruct {
                    a: u8,
                    b: MyEnum,
                }

                #[diplomat::opaque]
                struct Foo(Box<u8>);

                impl Foo {
                    pub fn get_struct(&self) -> MyStruct {
                        MyStruct { a: 1, b: MyEnum::A }
                    }
                }
            }
        }
    }
}
