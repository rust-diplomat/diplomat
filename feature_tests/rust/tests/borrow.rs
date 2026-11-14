// Links in the `extern "C"` functions that the bindings call.
use diplomat_feature_tests as _;

use diplomat_feature_tests_rust::ResultOpaque;

#[test]
fn borrowed_err() {
    let Ok(opaque) = ResultOpaque::new(5) else {
        panic!("expected Ok");
    };
    let Err(borrowed) = opaque.give_self() else {
        panic!("expected Err");
    };
    assert!(core::ptr::eq(borrowed, &*opaque));
    borrowed.assert_integer(5);
}

#[test]
fn borrowed_mut() {
    let Ok(mut opaque) = ResultOpaque::new(5) else {
        panic!("expected Ok");
    };
    let expected: *const ResultOpaque = &*opaque;
    let borrowed = opaque.takes_str("x");
    assert!(core::ptr::eq(borrowed, expected));
    borrowed.assert_integer(5);
}
