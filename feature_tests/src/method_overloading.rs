//! Tests for method overloading with constructors, named constructors, and regular methods.
//!
//! This test addresses the panic that occurred when multiple methods were renamed to Python
//! keywords (like "from") which get escaped to "from_". The code detected these as duplicate
//! entries but failed to handle `NamedConstructor` special methods, only accounting for
//! regular `Constructor` types.

#[diplomat::bridge]
#[diplomat::attr(auto, namespace = "mylib")]
pub mod ffi {

    #[diplomat::opaque]
    pub struct Decimal;

    impl Decimal {
        /// Test that method overloading works when methods are renamed to Python keywords.
        ///
        /// Problem: ICU4X has methods like from_int32(), from_int64(), from_uint32() that
        /// are all renamed to "from" for a nice Python API. Since "from" is a Python keyword,
        /// Diplomat escapes it to "from_". This caused a panic because the code only handled
        /// Constructor overloading, not NamedConstructor.
        ///
        /// Expected Python API after fix:
        ///   d = Decimal.from_(42)       # calls from_int32
        ///   d = Decimal.from_(999999)   # calls from_int64
        ///
        /// Generated C++ should look like:
        ///   .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int32_t>(&mylib::Decimal::from))), "v"_a)
        ///   .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int64_t>(&mylib::Decimal::from))), "v"_a)
        ///   .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<uint32_t>(&mylib::Decimal::from))), "v"_a)
        #[diplomat::attr(supports = method_overloading, rename = "from")]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_int32(_v: i32) -> Box<Decimal> {
            Box::new(Decimal)
        }

        #[diplomat::attr(supports = method_overloading, rename = "from")]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_int64(_v: i64) -> Box<Decimal> {
            Box::new(Decimal)
        }

        #[diplomat::attr(supports = method_overloading, rename = "from")]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_uint32(_v: u32) -> Box<Decimal> {
            Box::new(Decimal)
        }
    }
}
