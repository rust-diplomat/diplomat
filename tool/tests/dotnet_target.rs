use diplomat_tool::dotnet::*;

macro_rules! test_file {
    ($($file:tt)*) => {
        let parsed: syn::File = syn::parse_quote! { $($file)* };
        let custom_types = diplomat_core::ast::File::from(&parsed);
        let env = custom_types.all_types();

        let mut out_texts = std::collections::HashMap::new();

        gen_bindings(&env, None, &Default::default(), &mut out_texts).unwrap();
        out_texts.remove("DiplomatRuntime.cs");

        for out in out_texts.keys() {
            insta::with_settings!({ snapshot_suffix => out.clone() }, {
                insta::assert_snapshot!(out_texts.get(out).unwrap())
            });
        }
    }
}

macro_rules! test_file_using_library_config {
    ($($file:tt)*) => {
        let parsed: syn::File = syn::parse_quote! { $($file)* };
        let custom_types = diplomat_core::ast::File::from(&parsed);
        let env = custom_types.all_types();

        let mut out_texts = std::collections::HashMap::new();

        use std::path::PathBuf;
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/configs/dotnet_example.toml");
        gen_bindings(&env, Some(&path), &Default::default(), &mut out_texts).unwrap();
        out_texts.remove("DiplomatRuntime.cs");

        for out in out_texts.keys() {
            insta::with_settings!({ snapshot_suffix => out.clone() }, {
                insta::assert_snapshot!(out_texts.get(out).unwrap())
            });
        }
    }
}

#[test]
fn generation_using_default_config() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyLibError;

            impl MyLibError {
                pub fn to_display(out: &mut DiplomatWrite) {
                    unimplemented!()
                }
            }

            #[diplomat::opaque]
            struct MyStruct;

            impl MyStruct {
                pub fn new_slice(v: &[f64]) -> Result<Box<MyStruct>, Box<MyLibError>> {
                    unimplemented!()
                }

                pub fn set_slice(&mut self, new_slice: &[f64]) -> Result<(), Box<MyLibError>> {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn generation_using_library_config() {
    test_file_using_library_config! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyLibError;

            impl MyLibError {
                pub fn to_display(&self, out: &mut DiplomatWrite) {
                    unimplemented!()
                }
            }

            #[diplomat::opaque]
            struct MyStruct;

            impl MyStruct {
                pub fn new_slice(v: &[f64]) -> Result<Box<MyStruct>, Box<MyLibError>> {
                    unimplemented!()
                }

                pub fn set_slice(&mut self, new_slice: &[f64]) -> Result<(), Box<MyLibError>> {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn setters_getters_properties() {
    test_file_using_library_config! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct Foo;

            #[diplomat::opaque]
            struct Bar;

            impl Bar {
                pub fn get_foo(&self) -> Result<Box<Foo>, ()> {
                    unimplemented!()
                }

                pub fn set_foo(&mut self, foo: &Foo) -> Result<(), ()> {
                    unimplemented!()
                }

                pub fn get_name(&self, out: &mut DiplomatWrite) -> Result<(), ()> {
                    unimplemented!()
                }

                pub fn set_name(&mut self, new_name: &DiplomatStr) -> Result<(), ()> {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn cross_module_struct_fields() {
    test_file! {
        #[diplomat::bridge]
        mod mod1 {
            use super::mod2::Bar;

            struct Foo {
                x: Bar,
            }
        }

        #[diplomat::bridge]
        mod mod2 {
            use super::mod1::Foo;

            struct Bar {
                y: Box<Foo>,
            }
        }
    }
}

#[test]
fn cross_module_struct_methods() {
    test_file! {
        #[diplomat::bridge]
        mod mod1 {
            use super::mod2::Bar;

            #[diplomat::opaque]
            struct Foo;

            impl Foo {
                pub fn to_bar(&self) -> Bar {
                    unimplemented!()
                }
            }
        }

        #[diplomat::bridge]
        mod mod2 {
            use super::mod1::Foo;

            struct Bar {
                y: Box<Foo>,
            }
        }
    }
}

#[test]
fn enum_documentation() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            /// Documentation for MyEnum.
            ///
            /// Some remarks about this struct.
            #[diplomat::rust_link(foo::MyEnum, Enum)]
            enum MyEnum {
                /// All about A.
                A,
                /// All about B.
                #[diplomat::rust_link(foo::MyEnum::B, EnumVariant)]
                B,
                /// All about C.
                ///
                /// Some remarks about this variant.
                ///
                /// Even more remarks.
                C
            }
        }
    }
}

#[test]
fn simple_non_opaque_struct() {
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
fn simple_opaque_struct() {
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
fn method_taking_str() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyStruct(UnknownType);

            impl MyStruct {
                pub fn new_str(v: &DiplomatStr) -> Box<MyStruct> {
                    unimplemented!()
                }

                pub fn set_str(&mut self, new_str: &DiplomatStr) {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn method_taking_slice() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyStruct(UnknownType);

            impl MyStruct {
                pub fn new_slice(v: &[f64]) -> Box<MyStruct> {
                    unimplemented!()
                }

                pub fn fill_slice(&self, v: &mut [f64]) {
                    unimplemented!()
                }

                pub fn set_slice(&mut self, new_slice: &[f64]) {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn method_write_out() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyStruct(UnknownType);

            impl MyStruct {
                pub fn write(&self, out: &mut DiplomatWrite) {
                    unimplemented!()
                }

                pub fn write_unit(&self, out: &mut DiplomatWrite) -> () {
                    unimplemented!()
                }

                pub fn write_result(&self, out: &mut DiplomatWrite) -> Result<(), u8> {
                    unimplemented!()
                }

                pub fn write_no_rearrange(&self, out: &mut DiplomatWrite) -> u8 {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn struct_documentation() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            /// Documentation for Foo.
            /// Second line.
            struct Foo {
                /// Documentation for x.
                x: u8,
            }

            impl Foo {
                /// Documentation for get_x.
                pub fn get_x(&self) -> u8 {
                    x
                }
            }
        }
    }
}

#[test]
fn enum_conversion() {
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
            struct Foo;

            impl Foo {
                pub fn get_struct(&self, a: u8, b: MyEnum) -> MyStruct {
                    MyStruct { a, b }
                }
            }
        }
    }
}

#[test]
fn option_conversion() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyOtherStruct;

            #[diplomat::opaque]
            struct MyStruct;

            impl MyStruct {
                pub fn create(my_other_struct: Option<&MyOtherStruct>) -> Option<Box<MyStruct>> {
                    unimplemented!();
                }
            }
        }
    }
}

#[test]
fn pointer_types() {
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
fn boolean_type() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            struct MyStruct {
                a: bool,
            }

            impl MyStruct {
                pub fn func(a: bool) -> bool {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn option_types() {
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
fn result_types() {
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
fn string_reference() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
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
fn write_out() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyStruct;

            impl MyStruct {
                pub fn write(&self, to: &mut DiplomatWrite) {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn unit_type() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyStruct;

            impl MyStruct {
                pub fn something(&self) -> () {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn error_handling() {
    test_file! {
        #[diplomat::bridge]
        mod ffi {
            #[diplomat::opaque]
            struct MyStruct;

            #[diplomat::opaque]
            struct MyModuleOpaqueError;

            enum MyModuleError {
                BadInput,
                BadLuck,
            }

            impl MyStruct {
                pub fn foo(&self) -> Result<u32, MyModuleError> {
                    unimplemented!()
                }

                pub fn bar(&self) -> Result<DiplomatChar, MyModuleOpaqueError> {
                    unimplemented!()
                }

                pub fn baz(&self) -> Result<(), ()> {
                    unimplemented!()
                }
            }
        }
    }
}

#[test]
fn almost_properties() {
    test_file_using_library_config! {
        #[diplomat::bridge]
        mod ffi {
            /// This should not contain any property
            #[diplomat::opaque]
            struct MyStruct;

            impl MyStruct {
                /// This should not generate a property
                pub fn get_foo_by_key(&self, key: &DiplomatStr) -> Result<u64, ()> {
                    unimplemented!()
                }

                /// This should not generate a property
                pub fn set_foo_by_key(&self, key: &DiplomatStr, foo: i64) -> Result<(), ()> {
                    unimplemented!()
                }

                /// This should not generate a property
                pub fn get_str_by_key(&self, key: &DiplomatStr, writer: &mut DiplomatWrite) {
                    unimplemented!()
                }

                /// This should not generate a property
                pub fn set_str_by_key(&self, key: &DiplomatStr, s: &DiplomatStr) {
                    unimplemented!()
                }
            }
        }
    }
}
