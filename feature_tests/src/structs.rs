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

    #[diplomat::opaque]
    pub enum MyOpaqueEnum {
        A(String),
        B(Utf16Wrap),
        C,
        D(i32, ImportedStruct),
    }

    pub enum DefaultEnum {
        A,
        B,
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

    // Related to issue https://github.com/rust-diplomat/diplomat/issues/803
    // `diplomat-tool js` was crashing when trying to process options-in-structs
    // Not supported in kotlin
    #[diplomat::attr(kotlin, disable)]
    pub struct MyStructContainingAnOption {
        a: DiplomatOption<MyStruct>,
        b: DiplomatOption<DefaultEnum>,
    }

    #[diplomat::attr(auto, error)]
    pub struct MyZst;

    impl Opaque {
        #[diplomat::attr(auto, constructor)]
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

        pub fn borrow<'a>(&'a self) -> &'a OpaqueMutexedString {
            self
        }

        pub fn borrow_other<'a>(other: &'a OpaqueMutexedString) -> &'a OpaqueMutexedString {
            other
        }

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

        pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
            input
        }
    }

    impl Utf16Wrap {
        #[diplomat::attr(auto, constructor)]
        pub fn from_utf16(input: &DiplomatStr16) -> Box<Self> {
            Box::new(Self(input.into()))
        }

        pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(write, "{:?}", &self.0);
        }

        pub fn borrow_cont<'a>(&'a self) -> &'a DiplomatStr16 {
            &self.0
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

    impl MyOpaqueEnum {
        #[diplomat::demo(default_constructor)]
        pub fn new() -> Box<MyOpaqueEnum> {
            Box::new(MyOpaqueEnum::A("a".into()))
        }

        pub fn to_string(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(
                write,
                "MyOpaqueEnum::{}",
                match self {
                    MyOpaqueEnum::A(..) => "A",
                    MyOpaqueEnum::B(..) => "B",
                    MyOpaqueEnum::C => "C",
                    MyOpaqueEnum::D(..) => "D",
                }
            );
        }
    }

    impl DefaultEnum {
        #[diplomat::attr(all(supports=constructors, not(dart)), constructor)]
        pub fn new() -> DefaultEnum {
            DefaultEnum::A
        }
    }

    impl MyStruct {
        #[diplomat::attr(auto, constructor)]
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

        pub fn returns_zst_result() -> Result<(), MyZst> {
            Ok(())
        }

        pub fn fails_zst_result() -> Result<(), MyZst> {
            Err(MyZst {})
        }
    }

    impl MyStructContainingAnOption {
        #[diplomat::attr(auto, constructor)]
        pub fn new() -> Self {
            MyStructContainingAnOption {
                a: None.into(),
                b: None.into(),
            }
        }

        pub fn filled() -> Self {
            MyStructContainingAnOption {
                a: Some(MyStruct::new()).into(),
                b: Some(DefaultEnum::new()).into(),
            }
        }
    }

    // Test that cycles between structs work even when
    // they reference each other in the methods
    #[derive(Default)]
    pub struct CyclicStructA {
        pub a: CyclicStructB,
    }
    #[derive(Default)]
    pub struct CyclicStructB {
        pub field: u8,
    }

    // For demo_gen testing. How many layers in are we going?
    #[derive(Default)]
    pub struct CyclicStructC {
        pub a: CyclicStructA,
    }

    impl CyclicStructA {
        pub fn get_b() -> CyclicStructB {
            Default::default()
        }

        pub fn cyclic_out(self, out: &mut DiplomatWrite) {
            out.write_str(&self.a.field.to_string()).unwrap();
        }

        // For demo gen: tests having the same variables in the namespace
        pub fn double_cyclic_out(self, cyclic_struct_a: Self, out: &mut DiplomatWrite) {
            out.write_fmt(format_args!(
                "{} {}",
                &self.a.field, cyclic_struct_a.a.field
            ))
            .unwrap();
        }

        #[diplomat::attr(auto, getter)]
        pub fn getter_out(self, out: &mut DiplomatWrite) {
            out.write_str(&self.a.field.to_string()).unwrap();
        }
    }

    impl CyclicStructB {
        pub fn get_a() -> CyclicStructA {
            Default::default()
        }

        pub fn get_a_option() -> Option<CyclicStructA> {
            Some(Default::default())
        }
    }

    impl CyclicStructC {
        pub fn takes_nested_parameters(c: CyclicStructC) -> CyclicStructC {
            c
        }

        pub fn cyclic_out(self, out: &mut DiplomatWrite) {
            out.write_str(&self.a.a.field.to_string()).unwrap();
        }
    }

    /// Testing JS-specific layout/padding behavior
    #[diplomat::attr(not(js), disable)]
    pub struct ScalarPairWithPadding {
        pub first: u8,
        // Padding: [3 x u8]
        pub second: u32,
    }

    impl ScalarPairWithPadding {
        pub fn assert_value(self) {
            assert_eq!(self.first, 122);
            assert_eq!(self.second, 414);
        }
    }

    /// Testing JS-specific layout/padding behavior
    #[diplomat::attr(not(js), disable)]
    pub struct BigStructWithStuff {
        pub first: u8,
        // Padding: [1 x u8]
        pub second: u16,
        pub third: u16,
        // Padding: [1 x u16]
        pub fourth: ScalarPairWithPadding,
        pub fifth: u8,
    }

    impl BigStructWithStuff {
        pub fn assert_value(self, extra_val: u16) {
            assert_eq!(self.first, 101);
            assert_eq!(self.second, 505);
            assert_eq!(self.third, 9345);
            self.fourth.assert_value();
            assert_eq!(self.fifth, 99);
            assert_eq!(extra_val, 853);
        }
    }

    #[diplomat::attr(not(supports = arithmetic), disable)]
    struct StructArithmetic {
        x: i32,
        y: i32,
    }

    impl StructArithmetic {
        #[diplomat::attr(auto, getter)]
        #[allow(non_snake_case)]
        pub fn ORIGIN() -> Self {
            Self { x: 0, y: 0 }
        }

        #[diplomat::attr(auto, setter = "ORIGIN")]
        pub fn set_origin(_new_origin: StructArithmetic) {}

        #[diplomat::attr(auto, constructor)]
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        #[diplomat::attr(auto, add)]
        pub fn add(self, o: Self) -> Self {
            Self {
                x: self.x + o.x,
                y: self.y + o.y,
            }
        }

        #[diplomat::attr(auto, sub)]
        pub fn sub(self, o: Self) -> Self {
            Self {
                x: self.x - o.x,
                y: self.y - o.y,
            }
        }

        #[diplomat::attr(auto, mul)]
        pub fn mul(self, o: Self) -> Self {
            Self {
                x: self.x * o.x,
                y: self.y * o.y,
            }
        }

        #[diplomat::attr(auto, div)]
        pub fn div(self, o: Self) -> Self {
            Self {
                x: self.x / o.x,
                y: self.y / o.y,
            }
        }
    }
}

#[allow(unused)]
fn test_transparent_convert_exists(s: &String) -> &ffi::Opaque {
    ffi::Opaque::transparent_convert(s)
}
