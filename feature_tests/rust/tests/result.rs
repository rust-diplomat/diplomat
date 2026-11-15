// Links in the `extern "C"` functions that the bindings call.
use diplomat_feature_tests as _;

use diplomat_feature_tests_rust::{ErrorEnum, ErrorStruct, ResultOpaque};

#[test]
fn ok_opaque() {
    let Ok(opaque) = ResultOpaque::new(5) else {
        panic!("expected Ok");
    };
    opaque.assert_integer(5);
}

#[test]
fn err_enum() {
    assert!(matches!(
        ResultOpaque::new_failing_foo(),
        Err(ErrorEnum::Foo)
    ));
    assert!(matches!(
        ResultOpaque::new_failing_bar(),
        Err(ErrorEnum::Bar)
    ));
}

#[test]
fn err_unit() {
    assert!(matches!(ResultOpaque::new_failing_unit(), Err(())));
}

#[test]
fn err_struct() {
    assert!(matches!(
        ResultOpaque::new_failing_struct(109),
        Err(ErrorStruct { i: 109, j: 12 })
    ));
}

#[test]
fn ok_int() {
    assert_eq!(ResultOpaque::new_int(109), Ok(109));
}

#[test]
fn err_opaque() {
    let Err(opaque) = ResultOpaque::new_in_err(198) else {
        panic!("expected Err");
    };
    opaque.assert_integer(198);

    let Err(opaque) = ResultOpaque::new_in_enum_err(989) else {
        panic!("expected Err");
    };
    opaque.assert_integer(989);
}
