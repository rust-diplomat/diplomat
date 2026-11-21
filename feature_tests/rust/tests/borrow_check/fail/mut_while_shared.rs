//! The owner cannot be borrowed mutably while a shared borrow is still used.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(mut opaque) = ResultOpaque::new(5) else { panic!() };
    let Err(shared) = opaque.give_self() else { panic!() };
    opaque.takes_str("x");
    shared.assert_integer(5);
}
