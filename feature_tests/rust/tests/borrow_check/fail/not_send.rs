//! An owned opaque cannot be sent to another thread.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn assert_send<T: Send>(_: T) {}

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    assert_send(opaque);
}
