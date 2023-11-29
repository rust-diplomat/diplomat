use std::fmt::Write;

use diplomat_core::ast;
use diplomat_core::Env;

use crate::cpp::config::LibraryConfig;

pub fn gen_rust_to_cpp<W: Write>(
    cpp: &str,
    path: &str,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    library_config: &LibraryConfig,
    out: &mut W,
) -> String {
    match typ {
        ast::TypeName::Box(underlying) => match underlying.as_ref() {
            ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
                match path_type.resolve(in_path, env) {
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
                    &_ => unreachable!("unknown AST/HIR variant"),
                }
            }
            _o => todo!(),
        },
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            match path_type.resolve_with_path(in_path, env) {
                (_, ast::CustomType::Opaque(_)) => {
                    panic!("Cannot handle opaque structs by value");
                }

                (in_path, ast::CustomType::Struct(strct)) => {
                    let raw_struct_id = format!("diplomat_raw_struct_{path}");
                    writeln!(out, "capi::{} {} = {};", strct.name, raw_struct_id, cpp).unwrap();
                    let mut all_fields_wrapped = vec![];
                    for (name, typ, _) in &strct.fields {
                        all_fields_wrapped.push(format!(
                            ".{} = std::move({})",
                            name,
                            gen_rust_to_cpp(
                                &format!("{raw_struct_id}.{name}"),
                                &format!("{path}_{name}"),
                                typ,
                                &in_path,
                                env,
                                library_config,
                                out
                            )
                        ));
                    }

                    format!("{}{{ {} }}", strct.name, all_fields_wrapped.join(", "))
                }

                (_, ast::CustomType::Enum(enm)) => {
                    format!("static_cast<{}>({})", enm.name, cpp)
                }
                (_, &_) => unreachable!("unknown AST/HIR variant"),
            }
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                let raw_value_id = format!("diplomat_optional_raw_{path}");
                writeln!(out, "auto {raw_value_id} = {cpp};").unwrap();

                let ty_name =
                    super::types::gen_type(typ, in_path, None, env, library_config, false).unwrap();

                let wrapped_value_id = format!("diplomat_optional_{path}");
                writeln!(out, "{ty_name} {wrapped_value_id};").unwrap();

                writeln!(out, "if ({raw_value_id} != nullptr) {{").unwrap();

                let some_expr = gen_rust_to_cpp(
                    &raw_value_id,
                    path,
                    underlying.as_ref(),
                    in_path,
                    env,
                    library_config,
                    out,
                );
                if library_config.someopt.is_call {
                    writeln!(
                        out,
                        "  {} = {}({});",
                        wrapped_value_id, library_config.someopt.expr, some_expr
                    )
                    .unwrap();
                } else {
                    writeln!(
                        out,
                        "  {} = {}{};",
                        wrapped_value_id, library_config.someopt.expr, some_expr
                    )
                    .unwrap();
                }

                writeln!(out, "}} else {{").unwrap();
                if library_config.nullopt.is_call {
                    writeln!(
                        out,
                        "  {} = {}();",
                        wrapped_value_id, library_config.nullopt.expr
                    )
                    .unwrap();
                } else {
                    writeln!(
                        out,
                        "  {} = {};",
                        wrapped_value_id, library_config.nullopt.expr
                    )
                    .unwrap();
                }
                writeln!(out, "}}").unwrap();

                wrapped_value_id
            }

            _ => todo!(),
        },

        ast::TypeName::Result(ok, err, _) => {
            let raw_value_id = format!("diplomat_result_raw_{path}");
            writeln!(out, "auto {raw_value_id} = {cpp};").unwrap();
            let wrapped_value_id = format!("diplomat_result_{path}");
            let result_ty =
                super::types::gen_type(typ, in_path, None, env, library_config, false).unwrap();
            writeln!(out, "{result_ty} {wrapped_value_id};").unwrap();

            writeln!(out, "if ({raw_value_id}.is_ok) {{").unwrap();
            if !ok.is_zst() {
                let ok_expr = gen_rust_to_cpp(
                    &format!("{raw_value_id}.ok"),
                    path,
                    ok,
                    in_path,
                    env,
                    library_config,
                    out,
                );
                let ok_type =
                    super::types::gen_type(ok, in_path, None, env, library_config, false).unwrap();
                writeln!(
                    out,
                    "  {wrapped_value_id} = diplomat::Ok<{ok_type}>({ok_expr});"
                )
                .unwrap();
            } else {
                writeln!(
                    out,
                    "  {wrapped_value_id} = diplomat::Ok(std::monostate());"
                )
                .unwrap();
            };
            writeln!(out, "}} else {{").unwrap();

            if !err.is_zst() {
                let err_expr = gen_rust_to_cpp(
                    &format!("{raw_value_id}.err"),
                    path,
                    err,
                    in_path,
                    env,
                    library_config,
                    out,
                );
                let err_type =
                    super::types::gen_type(err, in_path, None, env, library_config, false).unwrap();
                writeln!(
                    out,
                    "  {wrapped_value_id} = diplomat::Err<{err_type}>({err_expr});"
                )
                .unwrap();
            } else {
                writeln!(
                    out,
                    "  {wrapped_value_id} = diplomat::Err(std::monostate());"
                )
                .unwrap();
            };
            writeln!(out, "}}").unwrap();

            wrapped_value_id
        }

        ast::TypeName::Primitive(_) => cpp.to_string(),
        ast::TypeName::Reference(_, _, _) => {
            todo!("Returning references from Rust to C++ is not currently supported")
        }
        ast::TypeName::Writeable => panic!("Returning writeables is not supported"),
        ast::TypeName::StrReference(
            _,
            ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8,
        ) => {
            let raw_value_id = format!("diplomat_str_raw_{path}");
            writeln!(out, "capi::DiplomatStringView {raw_value_id} = {cpp};").unwrap();

            writeln!(
                out,
                "std::string_view str({raw_value_id}.data, {raw_value_id}.len);"
            )
            .unwrap();
            "str".into()
        }
        ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf16) => {
            let raw_value_id = format!("diplomat_slice_raw_{path}");
            writeln!(out, "capi::DiplomatU16StringView {raw_value_id} = {cpp};").unwrap();

            let span = &library_config.span.expr;
            writeln!(
                out,
                "{span}<const char16_t> slice({raw_value_id}.data, {raw_value_id}.len);"
            )
            .unwrap();
            "slice".into()
        }
        ast::TypeName::PrimitiveSlice(_lt, mutability, prim) => {
            assert!(mutability.is_immutable());
            let raw_value_id = format!("diplomat_slice_raw_{path}");
            let mut prim_caps = prim.to_string();
            prim_caps.get_mut(0..1).unwrap().make_ascii_uppercase();
            let span = &library_config.span.expr;
            let prim = crate::c::types::c_type_for_prim(prim);
            writeln!(out, "capi::Diplomat{prim_caps}View {raw_value_id} = {cpp};").unwrap();

            writeln!(
                out,
                "{span}<const {prim}> slice({raw_value_id}.data, {raw_value_id}.len);"
            )
            .unwrap();
            "slice".into()
        }
        ast::TypeName::Unit => cpp.to_string(),
        &_ => unreachable!("unknown AST/HIR variant"),
    }
}

/// Meta information about a [ast::TypeName::Reference].
#[derive(Eq, PartialEq)]
pub struct ReferenceMeta {
    /// Whether or not the reference is owned.
    owned: bool,
    /// Whether or not the reference is mutable.
    mutable: bool,
    /// Whether the reference is nullable.
    is_nullable: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn gen_cpp_to_rust<W: Write>(
    cpp: &str,
    path: &str,
    behind_ref: Option<ReferenceMeta>,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    is_self: bool,
    out: &mut W,
) -> String {
    match typ {
        ast::TypeName::Reference(_, mutability, underlying) => {
            let behind_ref = match behind_ref {
                Some(mut br) => {
                    br.mutable = mutability.is_mutable();
                    br
                }
                None => ReferenceMeta {
                    owned: false,
                    mutable: mutability.is_mutable(),
                    is_nullable: false,
                },
            };
            gen_cpp_to_rust(
                cpp,
                path,
                Some(behind_ref),
                underlying.as_ref(),
                in_path,
                env,
                is_self,
                out,
            )
        }
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            match path_type.resolve(in_path, env) {
                ast::CustomType::Opaque(_opaque) => {
                    if let Some(reference) = behind_ref {
                        if is_self {
                            format!("{cpp}->inner.get()")
                        } else if reference.is_nullable {
                            format!("({cpp}) ? {cpp}->AsFFI() : nullptr")
                        } else if reference.mutable {
                            format!("{cpp}.AsFFIMut()")
                        } else {
                            format!("{cpp}.AsFFI()")
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
                        let wrapped_struct_id = format!("diplomat_wrapped_struct_{path}");
                        writeln!(out, "{} {} = {};", strct.name, wrapped_struct_id, cpp).unwrap();
                        let mut all_fields_wrapped = vec![];
                        for (name, typ, _) in &strct.fields {
                            all_fields_wrapped.push(format!(
                                ".{} = {}",
                                name,
                                gen_cpp_to_rust(
                                    &format!("{wrapped_struct_id}.{name}"),
                                    &format!("{path}_{name}"),
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
                &_ => unreachable!("unknown AST/HIR variant"),
            }
        }
        ast::TypeName::Writeable => {
            if behind_ref
                == Some(ReferenceMeta {
                    owned: false,
                    mutable: true,
                    is_nullable: false,
                })
            {
                writeln!(out, "capi::DiplomatWriteable {cpp}_writer = diplomat::WriteableTrait<W>::Construct({cpp});").unwrap();
                format!("&{cpp}_writer")
            } else {
                panic!("Cannot send Writeable to Rust as a value");
            }
        }
        ast::TypeName::Primitive(_) => cpp.to_string(),
        ast::TypeName::StrReference(..) => {
            format!("{{ {cpp}.data(), {cpp}.size() }}")
        }
        ast::TypeName::PrimitiveSlice(..) => {
            format!("{{ {cpp}.data(), {cpp}.size() }}")
        }
        ast::TypeName::Option(boxed) => match &**boxed {
            ast::TypeName::Reference(_, mutability, _) => {
                let behind_ref = match behind_ref {
                    Some(mut br) => {
                        br.mutable = mutability.is_mutable();
                        br.is_nullable = true;
                        br
                    }
                    None => ReferenceMeta {
                        owned: false,
                        mutable: mutability.is_mutable(),
                        is_nullable: true,
                    },
                };
                gen_cpp_to_rust(
                    cpp,
                    path,
                    Some(behind_ref),
                    boxed,
                    in_path,
                    env,
                    is_self,
                    out,
                )
            }
            o => todo!("{:?}", o),
        },
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

    #[test]
    fn test_option_conversion() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct {
                    a: u8,
                }

                impl MyStruct {
                    pub fn create(&self) -> Option<Box<MyStruct>> {
                        unimplemented!();
                    }
                }
            }
        }
    }

    #[test]
    fn test_option_conversion_using_library_config() {
        test_file_using_library_config! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct {
                    a: u8,
                }

                impl MyStruct {
                    pub fn create(&self) -> Option<Box<MyStruct>> {
                        unimplemented!();
                    }
                }
            }
        }
    }
}
