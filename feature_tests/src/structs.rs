#[diplomat::bridge]
pub mod ffi {
    use crate::imports::ffi::ImportedStruct;
    use std::sync::Mutex;

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    #[diplomat::attr(kotlin, disable)]
    pub struct Opaque(String);

    #[diplomat::opaque]
    pub struct OtherOpaque(Mutex<String>);

    #[derive(Debug, PartialEq, Eq)]
    #[diplomat::attr(kotlin, disable)]
    pub enum MyEnum {
        A = -2,
        B = -1,
        C = 0,
        D = 1,
        E = 2,
        F = 3,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[diplomat::attr(kotlin, disable)]
    pub enum ContiguousEnum {
        C = 0,
        D = 1,
        E = 2,
        F = 3,
    }

    #[diplomat::attr(kotlin, disable)]
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

    impl OtherOpaque {
        pub fn from_usize(number: usize) -> Box<OtherOpaque> {
            Box::new(OtherOpaque(Mutex::new(format!("{number}"))))
        }

        pub fn change(&self, number: usize) {
            let mut guard = self.0.lock().expect("Failed to lock mutex");
            *guard = format!("{number}");
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn borrow<'a>(&'a self) -> &'a OtherOpaque {
            self
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn borrow_other<'a>(other: &'a OtherOpaque) -> &'a OtherOpaque {
            other
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn get_len_and_add(&self, other: usize) -> usize {
            let guard = self.0.lock().expect("Failed to lock mutex");
            guard.len() + other
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
