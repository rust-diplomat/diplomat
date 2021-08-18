use core::fmt;
use std::collections::HashMap;

use diplomat_core::ast;

pub fn gen_type<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    behind_ref: Option<bool>, // owned?
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
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
            gen_type(underlying.as_ref(), in_path, Some(true), env, out)?;
        }

        ast::TypeName::Reference(underlying, mutable) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type(underlying.as_ref(), in_path, Some(false), env, out)?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => {
                write!(out, "std::optional<")?;
                gen_type(underlying.as_ref(), in_path, behind_ref, env, out)?;
                write!(out, ">")?;
            }

            _ => todo!(),
        },

        ast::TypeName::Result(ok, err) => {
            write!(out, "diplomat::result<")?;
            if ok.is_zst() {
                write!(out, "std::monostate")?;
            } else {
                gen_type(ok, in_path, behind_ref, env, out)?;
            }

            write!(out, ", ")?;
            if err.is_zst() {
                write!(out, "std::monostate")?;
            } else {
                gen_type(err, in_path, behind_ref, env, out)?;
            }
            write!(out, ">")?;
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", crate::c::types::c_type_for_prim(prim))?;
        }

        ast::TypeName::Writeable => {
            write!(out, "capi::DiplomatWriteable")?;
        }

        ast::TypeName::StrReference => {
            write!(out, "const std::string_view")?;
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
