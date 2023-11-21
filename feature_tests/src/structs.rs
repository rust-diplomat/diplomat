#[diplomat::bridge]
pub mod ffi {
    use crate::imports::ffi::ImportedStruct;

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    pub struct Opaque(String);

    #[derive(Debug, PartialEq, Eq)]
    pub enum MyEnum {
        A = -2,
        B = -1,
        C = 0,
        D = 1,
        E = 2,
        F = 3,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum ContiguousEnum {
        C = 0,
        D = 1,
        E = 2,
        F = 3,
    }

    pub struct MyStruct {
        a: u8,
        b: bool,
        c: u8,
        d: u64,
        e: i32,
        f: DiplomatChar,
        g: MyEnum,
    }

    impl Opaque {
        pub fn new() -> Box<Opaque> {
            Box::new(Opaque("".into()))
        }

        #[diplomat::rust_link(Something::something, FnInStruct)]
        #[diplomat::rust_link(Something::something_else, FnInStruct)]
        #[diplomat::rust_link(Something::something_small, FnInStruct, compact)]
        #[diplomat::rust_link(SomethingElse::something, FnInStruct, compact)]
        #[diplomat::rust_link(SomethingElse::something_else, FnInStruct, hidden)]
        pub fn assert_struct(&self, s: MyStruct) {
            s.assert_value();
        }

        pub fn returns_usize() -> usize {
            412
        }

        pub fn returns_imported() -> ImportedStruct {
            unimplemented!()
        }
    }

    impl MyEnum {
        pub fn into_value(self) -> i8 {
            self as i8
        }
    }

    impl MyStruct {
        pub fn new() -> MyStruct {
            MyStruct {
                a: 17,
                b: true,
                c: 209,
                d: 1234,
                e: 5991,
                f: '餐' as DiplomatChar,
                g: MyEnum::B,
            }
        }

        pub fn into_a(self) -> u8 {
            self.a
        }

        fn assert_value(&self) {
            assert_eq!(self.a, 17);
            assert!(self.b);
            assert_eq!(self.c, 209);
            assert_eq!(self.d, 1234);
            assert_eq!(self.e, 5991);
            assert_eq!(self.f, '餐' as DiplomatChar);
            assert_eq!(self.g, MyEnum::B);
        }
    }
}

#[allow(unused)]
fn test_transparent_convert_exists(s: &String) -> &ffi::Opaque {
    ffi::Opaque::transparent_convert(s)
}
