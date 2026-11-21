//! Several shared borrows of one owner can be used at the same time.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    let Err(a) = opaque.give_self() else { panic!() };
    let Err(b) = opaque.give_self() else { panic!() };
    a.assert_integer(5);
    b.assert_integer(5);
    opaque.assert_integer(5);
}
