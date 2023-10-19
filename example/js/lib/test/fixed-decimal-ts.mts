import test from 'ava';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatter, ICU4XFixedDecimalFormatterOptions } from "../api/index.js";

test("multiply a fixed decimal by 1.1", t => {
    const my_decimal = ICU4XFixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);
    t.is(my_decimal.to_string(), "12.3")
});

test("format a fixed decimal", t => {
    const my_decimal = ICU4XFixedDecimal.new(123);

    my_decimal.multiply_pow10(-1);

    const bytes = Uint8Array.from(["e".charCodeAt(0), "n".charCodeAt(0)]);
    let locale = ICU4XLocale.new_from_bytes(bytes);
    
    locale = ICU4XLocale.new("bn");

    const data_provider = ICU4XDataProvider.new_static();

    const fdf = ICU4XFixedDecimalFormatter.try_new(locale, data_provider, ICU4XFixedDecimalFormatterOptions.default());
    if (!fdf.success) {
        throw "Failed to format fixed decimal";
    }
    t.is(fdf.fdf.format_write(my_decimal), "১২.৩");
});

