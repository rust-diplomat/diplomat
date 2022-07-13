import test from 'ava';

import { ICU4XFixedDecimal } from "../api/ICU4XFixedDecimal.js"
import { ICU4XLocale } from "../api/ICU4XLocale.js"
import { ICU4XDataProvider } from "../api/ICU4XDataProvider.js"
import { ICU4XFixedDecimalFormat } from "../api/ICU4XFixedDecimalFormat.js"
import { ICU4XFixedDecimalFormatOptions } from "../api/ICU4XFixedDecimalFormatOptions.js"

test("multiply a fixed decimal by 0.1", t => {
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

    const fdf = ICU4XFixedDecimalFormat.try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions.default());
    if (!fdf.success) {
        throw "Failed to format fixed decimal";
    }
    t.is(fdf.fdf.format_write(my_decimal), "১২.৩");
});

