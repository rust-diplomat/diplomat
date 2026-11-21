//! Two mutable borrows of one owner cannot be used at the same time.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(mut opaque) = ResultOpaque::new(5) else { panic!() };
    let a = opaque.takes_str("a");
    let b = opaque.takes_str("b");
    a.assert_integer(5);
    b.assert_integer(5);
}
