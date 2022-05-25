use diplomat_core::Env;
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast;
use indenter::indented;

use super::types::c_type_for_prim;
use super::types::gen_type;

pub fn gen_struct<W: fmt::Write>(
    custom_type: &ast::CustomType,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match custom_type {
        ast::CustomType::Opaque(opaque) => {
            writeln!(out, "typedef struct {} {};", opaque.name, opaque.name)?;
        }

        ast::CustomType::Struct(strct) => {
            write!(out, "typedef struct {} {{", strct.name)?;
            let mut class_body_out = indented(out).with_str("    ");
            for (name, typ, _) in strct.fields.iter() {
                writeln!(&mut class_body_out)?;
                gen_field(name, typ, in_path, env, &mut class_body_out)?;
            }
            writeln!(out)?;
            writeln!(out, "}} {};", strct.name)?;
        }

        ast::CustomType::Enum(_) => {}
    }

    Ok(())
}

pub fn gen_field<W: fmt::Write>(
    name: &ast::Ident,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    gen_type(typ, in_path, env, out)?;
    write!(out, " {};", name)?;

    Ok(())
}

pub fn gen_method<W: fmt::Write>(
    method: &ast::Method,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    match &method.return_type {
        Some(ret_type) => {
            gen_type(ret_type, in_path, env, out)?;
        }

        None => {
            write!(out, "void")?;
        }
    }

    write!(out, " {}(", method.full_path_name)?;
    let mut params_to_gen = method.params.clone();
    if let Some(param) = &method.self_param {
        params_to_gen.insert(0, param.clone());
    }

    for (i, param) in params_to_gen.iter().enumerate() {
        if i != 0 {
            write!(out, ", ")?;
        }

        if let ast::TypeName::StrReference(mutability) = &param.ty {
            write!(
                out,
                "{0}char* {1}_data, size_t {1}_len",
                if mutability.is_immutable() {
                    "const "
                } else {
                    ""
                },
                param.name
            )?;
        } else if let ast::TypeName::PrimitiveSlice(prim, mutability) = &param.ty {
            write!(
                out,
                "{0}{1}* {2}_data, size_t {2}_len",
                if mutability.is_immutable() {
                    "const "
                } else {
                    ""
                },
                c_type_for_prim(prim),
                param.name,
            )?;
        } else {
            gen_type(&param.ty, in_path, env, out)?;
            write!(out, " {}", param.name)?;
        }
    }

    writeln!(out, ");")?;

    Ok(())
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
    fn test_method_taking_str() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    pub fn new_str(v: &str) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn set_str(&mut self, new_str: &str) {
                        unimplemented!()
                    }

                    pub fn make_uppercase(&mut self, some_str: &mut str) {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_method_taking_slice() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(UnknownType);

                impl MyStruct {
                    pub fn new_slice(v: &[f64]) -> Box<MyStruct> {
                        unimplemented!()
                    }

                    pub fn fill_slice(v: &mut [f64]) {
                        unimplemented!()
                    }

                    pub fn set_slice(&mut self, new_slice: &[f64]) {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
