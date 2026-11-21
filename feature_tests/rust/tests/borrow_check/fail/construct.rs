//! An opaque cannot be created outside the library.
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let _opaque = ResultOpaque {
        _private: [],
        _marker: core::marker::PhantomData,
    };
}
