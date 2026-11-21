//! The owner can be dropped once its borrows are no longer used.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    let Err(borrowed) = opaque.give_self() else { panic!() };
    borrowed.assert_integer(5);
    drop(opaque);
}
