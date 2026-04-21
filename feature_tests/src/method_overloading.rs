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
    #[diplomat::cfg(supports = method_overloading)]
    pub struct MethodOverloading;

    impl MethodOverloading {
        #[diplomat::attr(supports = method_overloading, rename = "from")]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_int32(_v: i32) -> Box<MethodOverloading> {
            Box::new(MethodOverloading)
        }

        #[diplomat::attr(supports = method_overloading, rename = "from")]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_int64(_v: i64) -> Box<MethodOverloading> {
            Box::new(MethodOverloading)
        }

        #[diplomat::attr(supports = method_overloading, rename = "from")]
        #[diplomat::attr(auto, named_constructor)]
        pub fn from_uint32(_v: u32) -> Box<MethodOverloading> {
            Box::new(MethodOverloading)
        }
    }
}
