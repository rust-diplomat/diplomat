import test from 'ava';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatter, ICU4XFixedDecimalFormatterOptions } from "demo";

test("multiply a fixed decimal by 0.1", t => {
    const my_decimal = ICU4XFixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);
    t.is(my_decimal.to_string(), "12.3");
});

test("format a fixed decimal", t => {
    const my_decimal = ICU4XFixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);

    const locale = ICU4XLocale.new("bn");

    const data_provider = ICU4XDataProvider.new_static();

    const fdf = ICU4XFixedDecimalFormatter.try_new(locale, data_provider, ICU4XFixedDecimalFormatterOptions.default());

    t.is(fdf.format_write(my_decimal), "১২.৩");
});
