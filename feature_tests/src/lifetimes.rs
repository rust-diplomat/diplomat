#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct Foo<'a>(&'a DiplomatStr);

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

    pub struct BorrowedFields<'a> {
        a: &'a DiplomatStr16,
        b: &'a DiplomatStr,
        c: &'a str,
    }

    pub struct BorrowedFieldsReturning<'a> {
        bytes: &'a DiplomatStr,
    }
    impl<'a> Foo<'a> {
        pub fn new(x: &'a DiplomatStr) -> Box<Self> {
            Box::new(Foo(x))
        }

        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
            Box::new(Bar(self))
        }

        pub fn new_static(x: &'static DiplomatStr) -> Box<Self> {
            Box::new(Foo(x))
        }

        pub fn as_returning(&self) -> BorrowedFieldsReturning<'a> {
            BorrowedFieldsReturning { bytes: self.0 }
        }

        pub fn extract_from_fields(fields: BorrowedFields<'a>) -> Box<Self> {
            Box::new(Foo(fields.b))
        }
    }

    // FIXME(#191): This test breaks the C++ codegen
    // impl<'b, 'a: 'b> Bar<'b, 'a> {
    //     pub fn foo(&'b self) -> &'b Foo<'a> {
    //         self.0
    //     }
    // }

    #[derive(Copy, Clone)]
    #[diplomat::opaque]
    pub struct One<'a>(super::One<'a>);

    #[derive(Copy, Clone)]
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
            b: &'b One<'a>,
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
            Box::new(match 0 {
                0 => *bottom,
                1 => *left,
                2 => *right,
                _ => *top,
            })
        }

        // Holds: [left, bottom]
        pub fn diamond_left<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'left>> {
            let _ = (top, right);
            Box::new(match 0 {
                0 => *bottom,
                _ => *left,
            })
        }

        // Holds: [right, bottom]
        pub fn diamond_right<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'right>> {
            let _ = (top, left);
            Box::new(match 0 {
                0 => *bottom,
                _ => *right,
            })
        }

        // Holds: [bottom]
        pub fn diamond_bottom<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'bottom>> {
            let _ = (top, left, right);
            Box::new(*bottom)
        }

        // Holds: [a, b, c, d]
        pub fn diamond_and_nested_types<'a, 'b: 'a, 'c: 'b, 'd: 'b + 'c, 'x, 'y>(
            a: &One<'a>,
            b: &'y One<'b>,
            c: &One<'c>,
            d: &One<'d>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            let _ = nohold;
            Box::new(match 0 {
                0 => *a,
                1 => *b,
                2 => *c,
                _ => *d,
            })
        }

        // Holds: [implicit_hold, explicit_hold]
        #[allow(clippy::extra_unused_lifetimes)]
        pub fn implicit_bounds<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'x, 'y>(
            explicit_hold: &'d One<'x>, // implies that 'x: 'd
            implicit_hold: &One<'x>,
            nohold: &One<'y>,
        ) -> Box<One<'a>> {
            let _ = nohold;
            Box::new(match 0 {
                0 => *explicit_hold,
                _ => *implicit_hold,
            })
        }

        // Holds: [a, b, c]
        #[allow(clippy::needless_lifetimes)]
        pub fn implicit_bounds_deep<'a, 'b, 'c, 'd, 'x>(
            explicit_: &'a One<'b>,
            implicit_1: &'b One<'c>,
            implicit_2: &'c One<'d>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            let _ = nohold;
            Box::new(match 0 {
                0 => *explicit_,
                1 => *implicit_1,
                _ => *implicit_2,
            })
        }
    }
}

#[derive(Copy, Clone)]
pub struct One<'a>(&'a ());

#[derive(Copy, Clone)]
pub struct Two<'a, 'b>(&'a (), &'b ());
