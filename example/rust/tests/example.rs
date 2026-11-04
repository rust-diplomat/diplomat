// Links in the `extern "C"` functions that the bindings call.
use diplomat_example as _;

use diplomat_example_rust::{
    DataProvider, FixedDecimal, FixedDecimalFormatter, FixedDecimalFormatterOptions,
    FixedDecimalGroupingStrategy, Locale,
};

#[test]
fn opaque_constructor_method_and_drop() {
    let mut decimal = FixedDecimal::new(123);
    decimal.multiply_pow10(2);
    drop(decimal);

    let _provider = DataProvider::new_static();
}

#[test]
fn struct_returned_by_value() {
    let options = FixedDecimalFormatterOptions::default();
    assert_eq!(
        options,
        FixedDecimalFormatterOptions {
            grouping_strategy: FixedDecimalGroupingStrategy::Auto,
            some_other_config: false,
        }
    );
}

#[test]
fn string_param() {
    let _locale = Locale::new(b"en-US");
}

#[test]
fn result_returned() {
    assert_eq!(DataProvider::returns_result(), Ok(()));

    let formatter = FixedDecimalFormatter::try_new(
        &Locale::new(b"en-US"),
        &DataProvider::new_static(),
        FixedDecimalFormatterOptions::default(),
    );
    assert!(formatter.is_ok());
}
