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

    // TODO: add after #191 is resolved.
    // impl<'b, 'a: 'b> Bar<'b, 'a> {
    //     pub fn foo(&'b self) -> &'b Foo<'a> {
    //         self.0
    //     }
    // }

    #[diplomat::opaque]
    pub struct One<'a>(super::One<'a>);

    #[diplomat::opaque]
    pub struct Two<'a, 'b>(super::Two<'a, 'b>);

    impl<'o> One<'o> {
        // Holds: [hold]
        #[allow(clippy::extra_unused_lifetimes)]
        pub fn transitivity<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'e: 'd, 'x>(
            hold: &'x One<'e>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [hold]
        #[allow(clippy::extra_unused_lifetimes)]
        pub fn cycle<'a: 'b, 'b: 'c, 'c: 'a, 'x>(
            hold: &Two<'x, 'b>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [a, b, c, d]
        pub fn many_dependents<'a, 'b: 'a, 'c: 'a, 'd: 'b, 'x, 'y>(
            a: &'x One<'a>,
            b: &'b One<'x>,
            c: &Two<'x, 'c>,
            d: &'x Two<'d, 'y>,
            nohold: &'x Two<'x, 'y>,
        ) -> Box<One<'a>> {
            let _ = (a, b, c, d, nohold);
            unimplemented!()
        }

        // Holds: [hold]
        pub fn return_outlives_param<'short, 'long: 'short>(
            hold: &Two<'long, 'short>,
            nohold: &'short One<'short>,
        ) -> Box<One<'long>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [top, left, right, bottom]
        pub fn diamond_top<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'top>> {
            let _ = (top, left, right, bottom);
            unimplemented!()
        }

        // Holds: [left, bottom]
        pub fn diamond_left<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'left>> {
            let _ = (top, left, right, bottom);
            unimplemented!()
        }

        // Holds: [right, bottom]
        pub fn diamond_right<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'right>> {
            let _ = (top, left, right, bottom);
            unimplemented!()
        }

        // Holds: [bottom]
        pub fn diamond_bottom<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'bottom>> {
            let _ = (top, left, right, bottom);
            unimplemented!()
        }

        // Holds: [a, b, c, d]
        pub fn diamond_and_nested_types<'a, 'b: 'a, 'c: 'b, 'd: 'b + 'c, 'x, 'y>(
            a: &'x One<'a>,
            b: &'y One<'b>,
            c: &One<'c>,
            d: &'d One<'x>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            let _ = (a, b, c, d, nohold);
            unimplemented!()
        }
    }
}

pub struct One<'a>(&'a ());

pub struct Two<'a, 'b>(&'a (), &'b ());
