#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatChar, DiplomatOption, DiplomatWrite};

    #[diplomat::opaque]
    pub struct OptionOpaque(i32);

    #[diplomat::opaque]
    pub struct OptionOpaqueChar(char);

    #[diplomat::opaque]
    #[diplomat::attr(dart, disable)]
    pub struct OptionString(String);

    impl OptionString {
        #[diplomat::demo(default_constructor)]
        pub fn new<'a>(diplomat_str: &'a DiplomatStr) -> Option<Box<Self>> {
            let string = std::str::from_utf8(diplomat_str).ok()?.into();
            Some(Box::new(OptionString(string)))
        }

        pub fn write<'a>(&'a self, write: &'a mut DiplomatWrite) -> Result<(), ()> {
            use std::fmt::Write;
            write!(write, "{}", self.0).map_err(|_| ())?;
            Ok(())
        }

        pub fn borrow<'a>(&'a self) -> Option<&'a DiplomatStr> {
            Some(self.0.as_bytes())
        }
    }

    #[diplomat::out]
    pub struct OptionStruct {
        a: Option<Box<OptionOpaque>>,
        b: Option<Box<OptionOpaqueChar>>,
        c: u32,
        d: Box<OptionOpaque>,
    }

    #[diplomat::attr(not(supports = option), disable)]
    #[derive(Debug)]
    pub struct OptionInputStruct {
        a: DiplomatOption<u8>,
        b: DiplomatOption<DiplomatChar>,
        c: DiplomatOption<OptionEnum>,
    }

    impl OptionInputStruct {
        // Specifically test the Dart default constructor generation code
        // around Options
        #[diplomat::attr(not(dart), disable)]
        #[diplomat::attr(auto, constructor)]
        pub fn default_ctor() -> Self {
            Self {
                a: None.into(),
                b: None.into(),
                c: None.into(),
            }
        }
    }

    #[diplomat::attr(not(supports = option), disable)]
    #[derive(Debug)]
    pub enum OptionEnum {
        Foo,
        Bar,
    }

    impl OptionOpaque {
        pub fn new(i: i32) -> Option<Box<OptionOpaque>> {
            Some(Box::new(OptionOpaque(i)))
        }

        pub fn new_none() -> Option<Box<OptionOpaque>> {
            None
        }

        pub fn returns() -> Option<OptionStruct> {
            None
        }

        pub fn option_isize(&self) -> Option<isize> {
            Some(10)
        }

        pub fn option_usize(&self) -> Option<usize> {
            Some(10)
        }

        pub fn option_i32(&self) -> Option<i32> {
            Some(10)
        }

        pub fn option_u32(&self) -> Option<u32> {
            Some(10)
        }

        pub fn new_struct() -> OptionStruct {
            OptionStruct {
                a: Some(Box::new(OptionOpaque(101))),
                b: Some(Box::new(OptionOpaqueChar('餐'))),
                c: 904,
                d: Box::new(OptionOpaque(926535)),
            }
        }

        pub fn new_struct_nones() -> OptionStruct {
            OptionStruct {
                a: None,
                b: None,
                c: 908,
                d: Box::new(OptionOpaque(926535)),
            }
        }

        pub fn assert_integer(&self, i: i32) {
            assert_eq!(i, self.0);
        }

        pub fn option_opaque_argument(arg: Option<&OptionOpaque>) -> bool {
            arg.is_some()
        }

        #[diplomat::attr(not(supports = option), disable)]
        pub fn accepts_option_u8(arg: Option<u8>) -> Option<u8> {
            arg
        }

        #[diplomat::attr(not(supports = option), disable)]
        pub fn accepts_option_enum(arg: Option<OptionEnum>) -> Option<OptionEnum> {
            arg
        }
        #[diplomat::attr(not(supports = option), disable)]
        pub fn accepts_option_input_struct(
            arg: Option<OptionInputStruct>,
        ) -> Option<OptionInputStruct> {
            arg
        }
        #[diplomat::attr(not(supports = option), disable)]
        pub fn returns_option_input_struct() -> OptionInputStruct {
            OptionInputStruct {
                a: Some(6).into(),
                b: None.into(),
                c: Some(OptionEnum::Bar).into(),
            }
        }

        #[diplomat::attr(any(not(supports = option), not(any(c, cpp))), disable)]
        pub fn accepts_option_str(arg: Option<&str>) -> usize {
            arg.unwrap_or_default().len()
        }

        #[diplomat::attr(any(not(supports = option), not(any(c, cpp))), disable)]
        pub fn accepts_option_str_slice(arg: Option<&[DiplomatStrSlice]>) -> bool {
            arg.is_some()
        }

        #[diplomat::attr(any(not(supports = option), not(any(c, cpp))), disable)]
        pub fn accepts_option_primitive(arg: Option<&[u32]>) -> i64 {
            arg.map(|v| v.iter().sum::<u32>().into()).unwrap_or(-1)
        }
    }

    impl OptionOpaqueChar {
        pub fn assert_char(&self, ch: DiplomatChar) {
            assert_eq!(ch, self.0 as u32)
        }
    }
}
