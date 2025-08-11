#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;

    use diplomat_runtime::DiplomatStr16;

    #[diplomat::opaque]
    pub struct Foo<'a>(&'a DiplomatStr);

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

    pub struct BorrowedFields<'a> {
        a: DiplomatStr16Slice<'a>,
        b: DiplomatStrSlice<'a>,
        c: DiplomatUtf8StrSlice<'a>,
    }

    pub struct BorrowedFieldsWithBounds<'a, 'b: 'a, 'c: 'b> {
        field_a: DiplomatStr16Slice<'a>,
        field_b: DiplomatStrSlice<'b>,
        field_c: DiplomatUtf8StrSlice<'c>,
    }

    pub struct BorrowedFieldsReturning<'a> {
        bytes: DiplomatStrSlice<'a>,
    }
    impl<'a> Foo<'a> {
        #[diplomat::attr(auto, constructor)]
        pub fn new(x: &'a DiplomatStr) -> Box<Self> {
            Box::new(Foo(x))
        }

        #[diplomat::attr(auto, getter = "bar")]
        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
            Box::new(Bar(self))
        }

        #[diplomat::attr(auto, named_constructor = "static")]
        #[diplomat::attr(not(supports = static_slices), disable)]
        pub fn new_static(x: &'static DiplomatStr) -> Box<Self> {
            Box::new(Foo(x))
        }

        pub fn as_returning(&self) -> BorrowedFieldsReturning<'a> {
            BorrowedFieldsReturning {
                bytes: self.0.into(),
            }
        }

        #[diplomat::attr(auto, named_constructor)]
        pub fn extract_from_fields(fields: BorrowedFields<'a>) -> Box<Self> {
            Box::new(Foo(fields.b.into()))
        }

        #[diplomat::attr(auto, named_constructor)]
        /// Test that the extraction logic correctly pins the right fields
        pub fn extract_from_bounds<'x, 'y: 'x + 'a, 'z: 'x + 'y>(
            bounds: BorrowedFieldsWithBounds<'x, 'y, 'z>,
            another_string: &'a DiplomatStr,
        ) -> Box<Self> {
            if bounds.field_b.is_empty() {
                Box::new(Self(another_string))
            } else {
                Box::new(Self(bounds.field_b.into()))
            }
        }
    }

    impl<'x> BorrowedFields<'x> {
        pub fn from_bar_and_strings(
            bar: &'x Bar<'x, 'x>,
            dstr16: &'x DiplomatStr16,
            utf8_str: &'x str,
        ) -> Self {
            BorrowedFields {
                a: dstr16.into(),
                b: bar.0 .0.into(),
                c: utf8_str.into(),
            }
        }
    }

    impl<'x, 'y: 'x, 'z: 'y> BorrowedFieldsWithBounds<'x, 'y, 'z> {
        pub fn from_foo_and_strings(
            foo: &'x Foo<'y>,
            dstr16_x: &'x DiplomatStr16,
            utf8_str_z: &'z str,
        ) -> Self {
            BorrowedFieldsWithBounds {
                field_a: dstr16_x.into(),
                field_b: foo.0.into(),
                field_c: utf8_str_z.into(),
            }
        }
    }

    pub struct NestedBorrowedFields<'x, 'y: 'x, 'z> {
        fields: BorrowedFields<'x>,
        bounds: BorrowedFieldsWithBounds<'x, 'y, 'y>,
        bounds2: BorrowedFieldsWithBounds<'z, 'z, 'z>,
    }

    impl<'x, 'y: 'x, 'z> NestedBorrowedFields<'x, 'y, 'z> {
        pub fn from_bar_and_foo_and_strings(
            bar: &'x Bar<'x, 'y>,
            foo: &'z Foo<'z>,
            dstr16_x: &'x DiplomatStr16,
            dstr16_z: &'z DiplomatStr16,
            utf8_str_y: &'y str,
            utf8_str_z: &'z str,
        ) -> Self {
            let fields = BorrowedFields::from_bar_and_strings(bar, dstr16_x, utf8_str_y);
            let bounds =
                BorrowedFieldsWithBounds::from_foo_and_strings(bar.0, dstr16_x, utf8_str_y);
            let bounds2 = BorrowedFieldsWithBounds::from_foo_and_strings(foo, dstr16_z, utf8_str_z);
            Self {
                fields,
                bounds,
                bounds2,
            }
        }
    }

    // FIXME(#191): This test breaks the C++ codegen
    impl<'b, 'a: 'b> Bar<'b, 'a> {
        #[diplomat::attr(auto, getter)]
        pub fn foo(&'b self) -> &'b Foo<'a> {
            self.0
        }
    }

    #[derive(Copy, Clone)]
    #[diplomat::opaque]
    pub struct One<'a>(super::One<'a>);

    #[derive(Copy, Clone)]
    #[diplomat::opaque]
    pub struct Two<'a, 'b>(super::Two<'a, 'b>);

    impl<'o> One<'o> {
        // Holds: [hold]
        #[allow(clippy::extra_unused_lifetimes)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn transitivity<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'e: 'd, 'x>(
            hold: &'x One<'e>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [hold]
        #[allow(clippy::extra_unused_lifetimes)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn cycle<'a: 'b, 'b: 'c, 'c: 'a, 'x>(
            hold: &Two<'x, 'b>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [a, b, c, d]
        #[diplomat::attr(auto, named_constructor)]
        pub fn many_dependents<'a, 'b: 'a, 'c: 'a, 'd: 'b + 'x, 'x, 'y>(
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
        #[diplomat::attr(auto, named_constructor)]
        pub fn return_outlives_param<'short, 'long: 'short>(
            hold: &Two<'long, 'short>,
            nohold: &'short One<'short>,
        ) -> Box<One<'long>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [top, left, right, bottom]
        #[diplomat::attr(auto, named_constructor)]
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
        #[diplomat::attr(auto, named_constructor)]
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
        #[diplomat::attr(auto, named_constructor)]
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
        #[diplomat::attr(auto, named_constructor)]
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
        #[diplomat::attr(auto, named_constructor)]
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
        #[diplomat::attr(auto, named_constructor)]
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
        #[diplomat::attr(auto, named_constructor)]
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

    // Test a common iterator pattern for exposing vectors of native elements
    // The Vec type stores the underlying type, and when it returns accessors, it does so
    // via transparent_convert and non-owning references. Iterators, iterables, and getters
    // are all handled via attributes, which may have slightly different codepaths.
    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    #[diplomat::attr(demo_gen, disable)]
    pub struct OpaqueThin(pub crate::lifetimes::Internal);

    impl OpaqueThin {
        #[diplomat::attr(auto, getter)]
        pub fn a(&self) -> i32 {
            self.0.a
        }
        #[diplomat::attr(auto, getter)]
        pub fn b(&self) -> f32 {
            self.0.b
        }

        #[diplomat::attr(auto, getter)]
        pub fn c(&self, w: &mut DiplomatWrite) {
            w.write_str(&self.0.c).unwrap();
        }
    }

    #[diplomat::opaque]
    pub struct OpaqueThinIter<'a>(pub std::slice::Iter<'a, crate::lifetimes::Internal>);

    impl<'a> OpaqueThinIter<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&'a mut self) -> Option<&'a OpaqueThin> {
            self.0.next().map(OpaqueThin::transparent_convert)
        }
    }

    #[diplomat::opaque]
    pub struct OpaqueThinVec(std::vec::Vec<crate::lifetimes::Internal>);

    impl OpaqueThinVec {
        #[diplomat::attr(auto, constructor)]
        pub fn create(a: &[i32], b: &[f32], c: &DiplomatStr) -> Box<Self> {
            assert!(a.len() == b.len(), "arrays must be of equal size");
            Box::new(Self(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| crate::lifetimes::Internal {
                        a: *a,
                        b: *b,
                        c: String::from_utf8(c.to_vec()).unwrap(),
                    })
                    .collect(),
            ))
        }

        #[diplomat::attr(auto, iterable)]
        #[allow(clippy::should_implement_trait)]
        pub fn iter<'a>(&'a self) -> Box<OpaqueThinIter<'a>> {
            Box::new(OpaqueThinIter(self.0.iter()))
        }

        #[diplomat::attr(nanobind, rename = "__len__")]
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        #[diplomat::attr(auto, indexer)]
        pub fn get<'a>(&'a self, idx: usize) -> Option<&'a OpaqueThin> {
            self.0.get(idx).map(OpaqueThin::transparent_convert)
        }

        #[diplomat::attr(auto, getter)]
        #[diplomat::attr(dart, rename = "firstelement")]
        pub fn first<'a>(&'a self) -> Option<&'a OpaqueThin> {
            self.0.get(0).map(OpaqueThin::transparent_convert)
        }
    }
}

#[derive(Copy, Clone)]
pub struct One<'a>(&'a ());

#[derive(Copy, Clone)]
pub struct Two<'a, 'b>(&'a (), &'b ());

pub struct Internal {
    a: i32,
    b: f32,
    c: String,
}
