//! The owner cannot be dropped while a borrow from it is still used.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    let Err(borrowed) = opaque.give_self() else { panic!() };
    drop(opaque);
    borrowed.assert_integer(5);
}
