use core::fmt;
use diplomat_core::Env;

use diplomat_core::ast;

use crate::cpp::config::LibraryConfig;

#[derive(Debug, Clone, Copy)]
pub enum RefType {
    Owned,
    BorrowedNullable,
    BorrowedNonNull,
}

pub fn gen_type(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    ref_type: Option<RefType>,
    env: &Env,
    library_config: &LibraryConfig,
    // whether we are generating a struct field.
    // structs shouldn't have `const` fields, otherwise we lose
    // our assignment operators
    in_struct: bool,
) -> Result<String, fmt::Error> {
    let mut s = String::new();
    gen_type_inner(
        typ,
        in_path,
        ref_type,
        env,
        library_config,
        in_struct,
        &mut s,
    )?;
    Ok(s)
}

fn gen_type_inner<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    ref_type: Option<RefType>,
    env: &Env,
    library_config: &LibraryConfig,
    in_struct: bool,
    out: &mut W,
) -> fmt::Result {
    let mut handled_ref = false;
    match typ {
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            match path_type.resolve(in_path, env) {
                ast::CustomType::Opaque(opaque) => {
                    match ref_type {
                        None => panic!("Cannot pass opaque structs as values"),
                        Some(RefType::Owned) => write!(out, "{}", opaque.name)?,
                        Some(RefType::BorrowedNullable) => write!(out, "{}*", opaque.name)?,
                        Some(RefType::BorrowedNonNull) => write!(out, "{}&", opaque.name)?,
                    }
                    handled_ref = true;
                }

                ast::CustomType::Struct(strct) => {
                    write!(out, "{}", strct.name)?;
                }

                ast::CustomType::Enum(enm) => {
                    write!(out, "{}", enm.name)?;
                }
                &_ => unreachable!("unknown AST/HIR variant"),
            }
        }

        ast::TypeName::Box(underlying) => {
            gen_type_inner(
                underlying.as_ref(),
                in_path,
                Some(RefType::Owned),
                env,
                library_config,
                in_struct,
                out,
            )?;
        }

        ast::TypeName::Reference(_, mutability, underlying) => {
            if mutability.is_immutable() && !in_struct {
                write!(out, "const ")?;
            }
            gen_type_inner(
                underlying.as_ref(),
                in_path,
                Some(RefType::BorrowedNonNull),
                env,
                library_config,
                in_struct,
                out,
            )?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                write!(out, "{}<", library_config.optional.expr)?;
                gen_type_inner(
                    underlying.as_ref(),
                    in_path,
                    ref_type,
                    env,
                    library_config,
                    in_struct,
                    out,
                )?;
                write!(out, ">")?;
            }

            ast::TypeName::Reference(_, mutability, underlying) => {
                if mutability.is_immutable() && !in_struct {
                    write!(out, "const ")?;
                }
                gen_type_inner(
                    underlying.as_ref(),
                    in_path,
                    Some(RefType::BorrowedNullable),
                    env,
                    library_config,
                    in_struct,
                    out,
                )?;
            }

            _ => unreachable!("unknown AST/HIR variant"),
        },

        ast::TypeName::Result(ok, err, _) => {
            write!(out, "diplomat::result<")?;
            if ok.is_zst() {
                write!(out, "std::monostate")?;
            } else {
                gen_type_inner(ok, in_path, ref_type, env, library_config, in_struct, out)?;
            }

            write!(out, ", ")?;
            if err.is_zst() {
                write!(out, "std::monostate")?;
            } else {
                gen_type_inner(err, in_path, ref_type, env, library_config, in_struct, out)?;
            }
            write!(out, ">")?;
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", crate::c::types::c_type_for_prim(prim))?;
        }

        ast::TypeName::Writeable => {
            write!(out, "capi::DiplomatWriteable")?;
        }

        ast::TypeName::StrReference(
            _,
            ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8,
        ) => {
            let maybe_const = if in_struct { "" } else { "const " };
            write!(out, "{maybe_const}{}", library_config.string_view.expr)?;
        }

        ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf16) => {
            let maybe_const = if in_struct { "" } else { "const " };
            write!(out, "{maybe_const}{}", library_config.string_view16.expr)?;
        }

        ast::TypeName::PrimitiveSlice(_, ast::Mutability::Mutable, prim) => {
            write!(
                out,
                "{}<const {}>",
                library_config.span.expr,
                crate::c::types::c_type_for_prim(prim)
            )?;
        }

        ast::TypeName::PrimitiveSlice(_, ast::Mutability::Immutable, prim) => {
            write!(
                out,
                "const {}<const {}>",
                library_config.span.expr,
                crate::c::types::c_type_for_prim(prim)
            )?;
        }

        ast::TypeName::Unit => {
            write!(out, "void")?;
        }
        &_ => unreachable!("unknown AST/HIR variant"),
    }

    if !handled_ref {
        match ref_type {
            None => (),
            Some(RefType::Owned) | Some(RefType::BorrowedNullable) => write!(out, "*")?,
            Some(RefType::BorrowedNonNull) => write!(out, "&")?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pointer_types() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct(UnknownType);

                struct MyStruct {
                    a: Box<MyOpaqueStruct>,
                }

                impl MyStruct {
                    pub fn new(foo: &MyOpaqueStruct, bar: &mut MyOpaqueStruct) -> MyStruct {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_option_types() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct(UnknownType);

                struct MyStruct {
                    a: Option<Box<MyOpaqueStruct>>,
                }
            }
        }
    }

    #[test]
    fn test_option_types_using_library_config() {
        test_file_using_library_config! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct(UnknownType);

                struct MyStruct {
                    a: Option<Box<MyOpaqueStruct>>,
                }
            }
        }
    }

    #[test]
    fn test_result_types() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct(UnknownType);

                struct MyStruct {
                    a: DiplomatResult<Box<MyOpaqueStruct>, u8>,
                }

                impl MyStruct {
                    pub fn new() -> Result<MyStruct, u8> {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_string_reference() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct;

                impl MyStruct {
                    pub fn new(v: &DiplomatStr) -> MyStruct {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_writeable_out() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct;

                impl MyStruct {
                    pub fn write(&self, to: &mut DiplomatWriteable) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_unit_type() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct;

                impl MyStruct {
                    pub fn something(&self) -> () {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
