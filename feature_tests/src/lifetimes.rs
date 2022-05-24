#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct Foo<'a>(&'a str);

    #[diplomat::opaque]
    pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

    impl<'a> Foo<'a> {
        pub fn new(x: &'a str) -> Box<Self> {
            Box::new(Foo(x))
        }

        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
            Box::new(Bar(self))
        }
    }
}
