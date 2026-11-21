//! A borrow cannot outlive its owner.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn borrow() -> &'static ResultOpaque {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    let Err(borrowed) = opaque.give_self() else { panic!() };
    borrowed
}

fn main() {
    borrow().assert_integer(5);
}
