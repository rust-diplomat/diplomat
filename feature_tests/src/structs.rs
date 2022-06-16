#[diplomat::bridge]
pub mod ffi {

    #[diplomat::opaque]
    pub struct Opaque();

    pub struct MyStruct {
        a: u8,
        b: bool,
        c: u8,
        d: u64,
        e: i32,
        f: char,
    }

    impl Opaque {
        pub fn new() -> Box<Opaque> {
            Box::new(Opaque())
        }

        #[allow(clippy::needless_lifetimes)] // macro doesn't support elision yet
        pub fn assert_struct(&self, s: MyStruct) {
            s.assert_value();
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
                f: '餐',
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

        pub fn consume(self) {}
    }

    pub struct Alpha {
        x: u32,
        y: u32,
    }

    pub struct Beta {
        alpha_field: Alpha,
    }

    impl Beta {
        pub fn new(x: u32, y: u32) -> Self {
            Beta {
                alpha_field: Alpha { x, y },
            }
        }
    }
}
