//! A borrowed opaque cannot be shared with another thread.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    let borrowed: &ResultOpaque = &opaque;
    std::thread::scope(|s| {
        s.spawn(|| borrowed.assert_integer(5));
    });
}
