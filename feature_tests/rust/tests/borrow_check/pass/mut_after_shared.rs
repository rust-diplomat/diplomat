//! A mutable borrow can follow a shared one once the shared one is no longer used.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(mut opaque) = ResultOpaque::new(5) else { panic!() };
    let Err(shared) = opaque.give_self() else { panic!() };
    shared.assert_integer(5);
    opaque.takes_str("x").assert_integer(5);
}
