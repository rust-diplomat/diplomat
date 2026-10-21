// Links in the `extern "C"` functions that the bindings call.
use diplomat_example as _;

use diplomat_example_rust::{
    DataProvider, FixedDecimal, FixedDecimalFormatterOptions, FixedDecimalGroupingStrategy,
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
