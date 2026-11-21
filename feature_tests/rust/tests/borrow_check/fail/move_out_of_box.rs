//! An opaque cannot be moved out of the `DiplomatBox` that owns it.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    let moved: ResultOpaque = *opaque;
    moved.assert_integer(5);
}
