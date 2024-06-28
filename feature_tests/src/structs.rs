#[allow(clippy::needless_lifetimes)]
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatStr16;

    use crate::imports::ffi::ImportedStruct;
    use std::fmt::Write;
    use std::sync::Mutex;

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    pub struct Opaque(String);

    #[diplomat::opaque]
    pub struct OpaqueMutexedString(Mutex<String>);

    #[diplomat::opaque]
    pub struct Utf16Wrap(Vec<u16>);

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

    #[diplomat::skip_if_ast]
    pub struct MyZst;

    impl Opaque {
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn new() -> Box<Opaque> {
            Box::new(Opaque("".into()))
        }

        pub fn try_from_utf8(input: &DiplomatStr) -> Option<Box<Self>> {
            let s = std::str::from_utf8(input).ok()?;
            Some(Box::new(Self(s.into())))
        }

        pub fn from_str(input: &str) -> Box<Self> {
            Box::new(Self(input.into()))
        }

        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(write, "{:?}", &self.0);
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

        pub fn internal_len(&self) -> usize {
            self.0.len()
        }

        pub fn returns_imported() -> ImportedStruct {
            unimplemented!()
        }

        pub fn cmp() -> core::cmp::Ordering {
            unimplemented!()
        }
    }

    impl OpaqueMutexedString {
        pub fn from_usize(number: usize) -> Box<OpaqueMutexedString> {
            Box::new(OpaqueMutexedString(Mutex::new(format!("{number}"))))
        }

        pub fn change(&self, number: usize) {
            let mut guard = self.0.lock().expect("Failed to lock mutex");
            *guard = format!("{number}");
        }

        #[diplomat::skip_if_ast]
        pub fn borrow<'a>(&'a self) -> &'a OpaqueMutexedString {
            self
        }

        #[diplomat::skip_if_ast]
        pub fn borrow_other<'a>(other: &'a OpaqueMutexedString) -> &'a OpaqueMutexedString {
            other
        }

        #[diplomat::skip_if_ast]
        pub fn borrow_self_or_other<'a>(
            &'a self,
            other: &'a OpaqueMutexedString,
        ) -> &'a OpaqueMutexedString {
            let guard = self.0.lock().expect("Failed to lock mutex");
            if guard.len() % 2 == 0 {
                self
            } else {
                other
            }
        }

        pub fn get_len_and_add(&self, other: usize) -> usize {
            let guard = self.0.lock().expect("Failed to lock mutex");
            guard.len() + other
        }

        pub fn dummy_str<'a>(&'a self) -> &'a DiplomatStr {
            "A const str with non byte char: 餐 which is a DiplomatChar,".as_bytes()
        }

        pub fn wrapper<'a>(&'a self) -> Box<Utf16Wrap> {
            let chars = "A const str with non byte char: 𐐷 which is a DiplomatChar,"
                .encode_utf16()
                .collect();
            Box::new(Utf16Wrap(chars))
        }
    }

    impl Utf16Wrap {
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn from_utf16(input: &DiplomatStr16) -> Box<Self> {
            Box::new(Self(input.into()))
        }

        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(write, "{:?}", &self.0);
        }

        pub fn borrow_cont<'a>(&'a self) -> &'a DiplomatStr16 {
            &self.0
        }

        pub fn owned<'a>(&'a self) -> Box<DiplomatStr16> {
            self.0.clone().into()
        }
    }

    impl MyEnum {
        pub fn into_value(self) -> i8 {
            self as i8
        }

        pub fn get_a() -> MyEnum {
            MyEnum::A
        }
    }

    impl MyStruct {
        #[diplomat::attr(supports = constructors, constructor)]
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

        #[diplomat::skip_if_ast]
        pub fn returns_zst_result() -> Result<(), MyZst> {
            Ok(())
        }
    }
}

#[allow(unused)]
fn test_transparent_convert_exists(s: &String) -> &ffi::Opaque {
    ffi::Opaque::transparent_convert(s)
}
