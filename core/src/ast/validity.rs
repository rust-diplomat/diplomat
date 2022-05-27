use super::{Path, TypeName};

#[cfg_attr(feature = "displaydoc", derive(displaydoc::Display))]
#[derive(Debug, Clone)]
pub enum ValidityError {
    #[cfg_attr(
        feature = "displaydoc",
        doc = "An opaque type crossed the FFI boundary as a value: {0}"
    )]
    OpaqueAsValue(TypeName),
    #[cfg_attr(
        feature = "displaydoc",
        doc = "A non-opaque zero-sized struct or enum has been defined: {0}"
    )]
    NonOpaqueZST(Path),
    #[cfg_attr(
        feature = "displaydoc",
        doc = "A non-opaque type was found behind a Box or reference, these can \
               only be handled by-move as they get converted at the FFI boundary: {0}"
    )]
    NonOpaqueBehindRef(TypeName),
    #[cfg_attr(
        feature = "displaydoc",
        doc = "A non-reference type was found inside an Option<T>: {0}"
    )]
    OptionNotContainingPointer(TypeName),
    #[cfg_attr(
        feature = "displaydoc",
        doc = "Return types cannot elide lifetimes, expected {expected} lifetimes: {sub_type} in {full_type}"
    )]
    LifetimeElisionInReturn {
        full_type: TypeName,
        sub_type: TypeName,
        expected: usize,
    },
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    macro_rules! uitest_validity {
        ($($file:tt)*) => {
            let parsed: syn::File = syn::parse_quote! { $($file)* };
            let custom_types = crate::ast::File::from(&parsed);
            let env = custom_types.all_types();

            let errors = custom_types.check_validity(&env);

            let mut output = String::new();
            for error in errors {
                write!(&mut output, "{}\n", error).unwrap();
            }
            insta::with_settings!({}, {
                insta::assert_display_snapshot!(output)
            });
        }
    }

    #[test]
    fn test_opaque_ffi() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct(UnknownType);

                impl MyOpaqueStruct {
                    pub fn new() -> Box<MyOpaqueStruct> {}
                    pub fn new_broken() -> MyOpaqueStruct {}
                    pub fn do_thing(&self) {}
                    pub fn do_thing_broken(self) {}
                    pub fn broken_differently(&self, x: &MyOpaqueStruct) {}
                }
            }
        }
    }

    #[test]
    fn opaque_checks_with_safe_use() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                struct NonOpaqueStruct {}

                impl NonOpaqueStruct {
                    fn new(x: i32) -> NonOpaqueStruct {
                        unimplemented!();
                    }
                }

                #[diplomat::opaque]
                struct OpaqueStruct {}

                impl OpaqueStruct {
                    pub fn new() -> Box<OpaqueStruct> {
                        unimplemented!();
                    }

                    pub fn get_i32(&self) -> i32 {
                        unimplemented!()
                    }
                }
            }
        };
    }

    #[test]
    fn opaque_checks_with_error() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct OpaqueStruct {}

                impl OpaqueStruct {
                    pub fn new() -> OpaqueStruct {
                        unimplemented!();
                    }

                    pub fn get_i32(self) -> i32 {
                        unimplemented!()
                    }
                }
            }
        };
    }

    #[test]
    fn zst_non_opaque() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                struct OpaqueStruct;

                enum OpaqueEnum {}
            }
        };
    }

    #[test]
    fn option_invalid() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatResult;
                struct Foo {
                    field: Option<u8>,
                }

                impl Foo {
                    pub fn do_thing(opt: Option<Option<u16>>) {

                    }

                    pub fn do_thing2(opt: DiplomatResult<Option<char>, u8>) {

                    }
                    pub fn do_thing2(opt: Option<u16>) {

                    }
                }
            }
        };
    }

    #[test]
    fn option_valid() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                struct Foo {
                    field: Option<Box<u8>>,
                }

                impl Foo {
                    pub fn do_thing(opt: Option<Box<u32>>) {

                    }
                    pub fn do_thing2(opt: Option<&u32>) {

                    }
                }
            }
        };
    }

    #[test]
    fn non_opaque_move() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                struct NonOpaque {
                    num: u8,
                }

                impl NonOpaque {
                    pub fn foo(&self) {}
                }

                #[diplomat::opaque]
                struct Opaque;

                impl Opaque {
                    pub fn bar(&self) -> &NonOpaque {}
                    pub fn baz(&self, x: &NonOpaque) {}
                    pub fn quux(&self) -> Box<NonOpaque> {}
                }
            }
        };
    }
}
