//! An owned opaque cannot be sent to another thread.
use diplomat_feature_tests as _;
use diplomat_feature_tests_rust::ResultOpaque;

fn main() {
    let Ok(opaque) = ResultOpaque::new(5) else { panic!() };
    std::thread::spawn(move || opaque.assert_integer(5));
}
