import test from 'ava';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatOptions } from "../api.mjs";

test("multiply a fixed decimal by 0.1", t => {
    const my_decimal = ICU4XFixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);
    t.is(my_decimal.to_string(), "12.3")
});

test("format a fixed decimal", t => {
    const my_decimal = ICU4XFixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);

    let locale = ICU4XLocale.new("bn");

    const data_provider = ICU4XDataProvider.new_static();

    const fdf = ICU4XFixedDecimalFormat.try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions.default());
    if (!fdf.success) {
        
    }
    t.is(fdf.fdf.format_write(my_decimal), "১২.৩");
});
