#[diplomat::bridge]
pub mod ffi {

    #[diplomat::opaque]
    pub struct Opaque();

    pub struct MyStruct<'a> {
        a: u8,
        b: bool,
        c: u8,
        d: u64,
        e: i32,
        f: char,
        g: &'a str,
    }

    impl Opaque {
        pub fn new() -> Box<Opaque> {
            Box::new(Opaque())
        }

        pub fn assert_struct<'b>(&self, s: MyStruct<'b>) {
            s.assert_value();
        }

        pub fn read_g<'a>(&self, s: MyStruct<'a>) -> &'a str {
            s.g
        }
    }

    impl<'c> MyStruct<'c> {
        pub fn new(g: &'c str) -> MyStruct<'c> {
            MyStruct {
                a: 17,
                b: true,
                c: 209,
                d: 1234,
                e: 5991,
                f: '餐',
                g,
            }
        }

        fn assert_value(&self) {
            assert_eq!(self.a, 17);
            assert!(self.b);
            assert_eq!(self.c, 209);
            assert_eq!(self.d, 1234);
            assert_eq!(self.e, 5991);
            assert_eq!(self.f, '餐');
        }
    }

    pub struct Alpha<'alpha> {
        alpha_field: &'alpha str,
        a: u8,
    }

    pub struct Beta<'beta> {
        beta_field: Alpha<'beta>,
        b: u8,
    }

    impl<'imp> Beta<'imp> {
        pub fn new(my_str: &'imp str) -> Self {
            Beta {
                beta_field: Alpha {
                    alpha_field: my_str,
                    a: 0,
                },
                b: 0,
            }
        }
    }
}
