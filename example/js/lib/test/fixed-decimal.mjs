import test from 'ava';

import { FixedDecimal, Locale, DataProvider, FixedDecimalFormatter, FixedDecimalFormatterOptions } from "demo";

test("multiply a fixed decimal by 0.1", t => {
    const my_decimal = FixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);
    t.is(my_decimal.to_string(), "12.3");
});

test("format a fixed decimal", t => {
    const my_decimal = FixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);

    const locale = Locale.new("bn");

    const data_provider = DataProvider.new_static();

    const fdf = FixedDecimalFormatter.try_new(locale, data_provider, FixedDecimalFormatterOptions.default());

    t.is(fdf.format_write(my_decimal), "১২.৩");
});
