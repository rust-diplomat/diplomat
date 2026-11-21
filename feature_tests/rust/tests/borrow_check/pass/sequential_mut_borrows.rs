//! A mutable borrow can follow another once the first is no longer used.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(mut opaque) = ResultOpaque::new(5) else { panic!() };
    let a = opaque.takes_str("a");
    a.assert_integer(5);
    let b = opaque.takes_str("b");
    b.assert_integer(5);
}
