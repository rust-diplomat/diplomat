use diplomat_core::Env;
use std::fmt;

use diplomat_core::ast::{self, PrimitiveType};

pub fn gen_type<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            r @ ast::CustomType::Struct(_) | r @ ast::CustomType::Opaque(_) => {
                write!(out, "{}", r.name())?;
            }

            ast::CustomType::Enum(enm) => {
                write!(out, "{}", enm.name)?;
            }
        },

        ast::TypeName::Box(underlying) => {
            gen_type(underlying.as_ref(), in_path, env, out)?;
            write!(out, "*")?;
        }

        ast::TypeName::Reference(underlying, mutable, _lt) => {
            if !mutable {
                write!(out, "const ")?;
            }
            gen_type(underlying.as_ref(), in_path, env, out)?;
            write!(out, "*")?;
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", c_type_for_prim(prim))?;
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) | ast::TypeName::Reference(..) => {
                gen_type(underlying.as_ref(), in_path, env, out)?;
            }

            _ => unreachable!("Cannot have non-pointer types inside Option"),
        },

        ast::TypeName::Result(_, _) => {
            write!(out, "{}_{}", in_path.elements.join("_"), name_for_type(typ))?;
        }

        ast::TypeName::Writeable => write!(out, "DiplomatWriteable")?,
        ast::TypeName::StrReference(..) => unreachable!("Strings handled in structs.rs"),
        ast::TypeName::PrimitiveSlice(..) => unreachable!("Slices handled in structs.rs"),
        ast::TypeName::Unit => write!(out, "void")?,
    }

    Ok(())
}

/// Generates a struct name that uniquely identifies the given type.
///
/// This is primarily used for generating structs for result types,
/// which require one struct for each distinct instance.
pub fn name_for_type(typ: &ast::TypeName) -> String {
    match typ {
        ast::TypeName::Named(name) => name.path.elements.last().unwrap().clone(),
        ast::TypeName::Box(underlying) => format!("box_{}", name_for_type(underlying)),
        ast::TypeName::Reference(underlying, mutable, _lt) => {
            if *mutable {
                return format!("ref_mut_{}", name_for_type(underlying));
            } else {
                format!("ref_{}", name_for_type(underlying))
            }
        }
        ast::TypeName::Primitive(prim) => c_type_for_prim(prim).to_string(),
        ast::TypeName::Option(underlying) => format!("opt_{}", name_for_type(underlying)),
        ast::TypeName::Result(ok, err) => {
            format!("result_{}_{}", name_for_type(ok), name_for_type(err))
        }
        ast::TypeName::Writeable => "writeable".to_string(),
        ast::TypeName::StrReference(true) => "str_ref_mut".to_string(),
        ast::TypeName::StrReference(false) => "str_ref".to_string(),
        ast::TypeName::PrimitiveSlice(prim, true) => {
            format!("ref_mut_prim_slice_{}", c_type_for_prim(prim))
        }
        ast::TypeName::PrimitiveSlice(prim, false) => {
            format!("ref_prim_slice_{}", c_type_for_prim(prim))
        }
        ast::TypeName::Unit => "void".to_string(),
    }
}

pub fn c_type_for_prim(prim: &PrimitiveType) -> &str {
    match prim {
        PrimitiveType::i8 => "int8_t",
        PrimitiveType::u8 => "uint8_t",
        PrimitiveType::i16 => "int16_t",
        PrimitiveType::u16 => "uint16_t",
        PrimitiveType::i32 => "int32_t",
        PrimitiveType::u32 => "uint32_t",
        PrimitiveType::i64 => "int64_t",
        PrimitiveType::u64 => "uint64_t",
        PrimitiveType::i128 => panic!("i128 not supported in C"),
        PrimitiveType::u128 => panic!("u128 not supported in C"),
        PrimitiveType::isize => "ssize_t",
        PrimitiveType::usize => "size_t",
        PrimitiveType::f32 => "float",
        PrimitiveType::f64 => "double",
        PrimitiveType::bool => "bool",
        PrimitiveType::char => "char32_t",
    }
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

                struct MyStruct<'a> {
                    a: &'a MyOpaqueStruct,
                }

                impl<'a> MyStruct<'a> {
                    pub fn new(foo: &'a MyOpaqueStruct, bar: &'a mut MyOpaqueStruct) -> MyStruct<'a> {
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
                struct MyStruct {
                    a: Option<Box<MyStruct>>,
                }
            }
        }
    }

    #[test]
    fn test_result_types() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct {
                    a: DiplomatResult<Box<MyStruct>, u8>,
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
    fn test_empty_result_types() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct {
                    a: DiplomatResult<(), u8>,
                    b: DiplomatResult<(), ()>,
                    c: DiplomatResult<u8, ()>,
                }

                impl MyStruct {
                    pub fn new() -> MyStruct {
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
