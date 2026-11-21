//! An owned opaque cannot be shared with another thread.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn assert_sync<T: Sync>(_: &T) {}

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    assert_sync(&opaque);
}
