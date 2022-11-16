#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct OptionOpaque(i32);

    #[diplomat::opaque]
    pub struct OptionOpaqueChar(char);

    #[diplomat::out]
    pub struct OptionStruct {
        a: Option<Box<OptionOpaque>>,
        b: Option<Box<OptionOpaqueChar>>,
        c: u32,
        d: Option<Box<OptionOpaque>>,
    }

    impl OptionOpaque {
        pub fn new(i: i32) -> Option<Box<OptionOpaque>> {
            Some(Box::new(OptionOpaque(i)))
        }

        pub fn new_none() -> Option<Box<OptionOpaque>> {
            None
        }

        pub fn new_struct() -> OptionStruct {
            OptionStruct {
                a: Some(Box::new(OptionOpaque(101))),
                b: Some(Box::new(OptionOpaqueChar('餐'))),
                c: 904,
                d: Some(Box::new(OptionOpaque(926535))),
            }
        }

        pub fn new_struct_nones() -> OptionStruct {
            OptionStruct {
                a: None,
                b: None,
                c: 908,
                d: None,
            }
        }

        pub fn assert_integer(&self, i: i32) {
            assert_eq!(i, self.0);
        }
    }

    impl OptionOpaqueChar {
        pub fn assert_char(&self, ch: char) {
            assert_eq!(ch, self.0)
        }
    }
}
