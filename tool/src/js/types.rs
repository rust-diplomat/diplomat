use diplomat_core::Env;

use diplomat_core::ast;

#[derive(PartialEq, Eq, Debug)]
pub enum ReturnTypeForm {
    /// A struct recursively containing no scalar fields.
    Empty,

    /// A single scalar or a struct recursively containing only a single scalar.
    Scalar,

    /// A struct recursively containing multiple scalar fields.
    Complex,
}

/// Determines what return form the given return type will be translated to
/// in the WASM ABI.
///
/// See https://github.com/WebAssembly/tool-conventions/blob/master/BasicCABI.md#function-signatures.
pub fn return_type_form(typ: &ast::TypeName, in_path: &ast::Path, env: &Env) -> ReturnTypeForm {
    match typ {
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            match path_type.resolve_with_path(in_path, env) {
                (struct_path, ast::CustomType::Struct(strct)) => {
                    let all_field_forms: Vec<ReturnTypeForm> = strct
                        .fields
                        .iter()
                        .map(|f| return_type_form(&f.1, &struct_path, env))
                        .collect();

                    let scalar_count = all_field_forms
                        .iter()
                        .filter(|v| v == &&ReturnTypeForm::Scalar)
                        .count();
                    let complex_count = all_field_forms
                        .iter()
                        .filter(|v| v == &&ReturnTypeForm::Complex)
                        .count();

                    if scalar_count == 0 && complex_count == 0 {
                        ReturnTypeForm::Empty
                    } else if scalar_count == 1 && complex_count == 0 {
                        ReturnTypeForm::Scalar
                    } else {
                        ReturnTypeForm::Complex
                    }
                }

                (_, ast::CustomType::Opaque(_)) | (_, ast::CustomType::Enum(_)) => {
                    ReturnTypeForm::Scalar
                }
                (_, &_) => unreachable!("unknown AST/HIR variant"),
            }
        }

        ast::TypeName::Result(ok, err, _) => {
            let ok_form = return_type_form(ok, in_path, env);
            let err_form = return_type_form(err, in_path, env);

            if ok_form == ReturnTypeForm::Empty && err_form == ReturnTypeForm::Empty {
                ReturnTypeForm::Scalar
            } else {
                ReturnTypeForm::Complex
            }
        }

        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(..) | ast::TypeName::Reference(..) => {
                return_type_form(underlying, in_path, env)
            }
            _ => {
                if return_type_form(underlying, in_path, env) == ReturnTypeForm::Empty {
                    ReturnTypeForm::Scalar
                } else {
                    ReturnTypeForm::Complex
                }
            }
        },

        ast::TypeName::Unit => ReturnTypeForm::Empty,

        ast::TypeName::Box(_) => ReturnTypeForm::Scalar,

        ast::TypeName::Reference(_, _mut, _lt) => ReturnTypeForm::Scalar,

        ast::TypeName::StrReference(..) => ReturnTypeForm::Complex,

        ast::TypeName::PrimitiveSlice(..) => ReturnTypeForm::Complex,

        ast::TypeName::Primitive(_) => ReturnTypeForm::Scalar,

        ast::TypeName::Write => panic!("Cannot return write"),
        &_ => unreachable!("unknown AST/HIR variant"),
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
                    b: u8,
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
                    a: u8,
                    b: u8,
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
                struct MyStruct {
                    a: u8,
                    b: u8,
                }

                impl MyStruct {
                    pub fn new(v: &DiplomatStr) -> MyStruct {
                        unimplemented!()
                    }
                }
            }
        }
    }

    #[test]
    fn test_write_out() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                struct MyStruct {
                    a: u8,
                    b: u8,
                }

                impl MyStruct {
                    pub fn write(self, to: &mut DiplomatWrite) {
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
                struct MyStruct {
                    a: u8,
                    b: u8,
                }

                impl MyStruct {
                    pub fn something(self) -> () {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
