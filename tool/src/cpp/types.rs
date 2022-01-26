use core::fmt;
use diplomat_core::Env;

use diplomat_core::ast;

use crate::cpp::config::LibraryConfig;

pub fn gen_type(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    behind_ref: Option<bool>, // owned?
    env: &Env,
    library_config: &LibraryConfig,
) -> Result<String, fmt::Error> {
    let mut s = String::new();
    gen_type_inner(typ, in_path, behind_ref, env, library_config, &mut s)?;
    Ok(s)
}

fn gen_type_inner<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    behind_ref: Option<bool>, // owned?
    env: &Env,
    library_config: &LibraryConfig,
    out: &mut W,
) -> fmt::Result {
    let mut handled_ref = false;
    match typ {
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Opaque(opaque) => {
                if let Some(owned) = behind_ref {
                    if owned {
                        write!(out, "{}", opaque.name)?;
                    } else {
                        write!(out, "{}&", opaque.name)?;
                    }

                    handled_ref = true;
                } else {
                    panic!("Cannot pass opaque structs as values");
                }
            }

            ast::CustomType::Struct(strct) => {
                write!(out, "{}", strct.name)?;
            }

            ast::CustomType::Enum(enm) => {
                write!(out, "{}", enm.name)?;
            }
        },

        ast::TypeName::Box(underlying) => {
            gen_type_inner(
                underlying.as_ref(),
                in_path,
                Some(true),
                env,
                library_config,
                out,
            )?;
        }

        ast::TypeName::Reference(underlying, mutable, _lt) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type_inner(
                underlying.as_ref(),
                in_path,
                Some(false),
                env,
                library_config,
                out,
            )?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                write!(out, "{}<", library_config.optional.expr)?;
                gen_type_inner(
                    underlying.as_ref(),
                    in_path,
                    behind_ref,
                    env,
                    library_config,
                    out,
                )?;
                write!(out, ">")?;
            }

            _ => todo!(),
        },

        ast::TypeName::Result(ok, err) => {
            write!(out, "diplomat::result<")?;
            if ok.is_zst() {
                write!(out, "std::monostate")?;
            } else {
                gen_type_inner(ok, in_path, behind_ref, env, library_config, out)?;
            }

            write!(out, ", ")?;
            if err.is_zst() {
                write!(out, "std::monostate")?;
            } else {
                gen_type_inner(err, in_path, behind_ref, env, library_config, out)?;
            }
            write!(out, ">")?;
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", crate::c::types::c_type_for_prim(prim))?;
        }

        ast::TypeName::Writeable => {
            write!(out, "capi::DiplomatWriteable")?;
        }

        ast::TypeName::StrReference(true) => {
            write!(out, "{}", library_config.string_view.expr)?;
        }

        ast::TypeName::StrReference(false) => {
            write!(out, "const {}", library_config.string_view.expr)?;
        }

        ast::TypeName::PrimitiveSlice(prim, true) => {
            write!(
                out,
                "{}<{}>",
                library_config.span.expr,
                crate::c::types::c_type_for_prim(prim)
            )?;
        }

        ast::TypeName::PrimitiveSlice(prim, false) => {
            write!(
                out,
                "const {}<{}>",
                library_config.span.expr,
                crate::c::types::c_type_for_prim(prim)
            )?;
        }

        ast::TypeName::Unit => {
            write!(out, "void")?;
        }
    }

    if !handled_ref {
        if let Some(owned) = behind_ref {
            if owned {
                write!(out, "*")?;
            } else {
                write!(out, "&")?;
            }
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
                    pub fn new(foo: &MyStruct, bar: &mut MyStruct) -> MyStruct {
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
                    pub fn new() -> DiplomatResult<MyStruct, u8> {
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
                    pub fn new(v: &str) -> MyStruct {
                        unimplemented!()
                    }

                    pub fn make_uppercase(v: &mut str) {
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
