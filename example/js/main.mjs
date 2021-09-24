import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatOptions } from "./api.mjs";

const my_decimal = ICU4XFixedDecimal.new(123);

console.log(my_decimal.to_string());

my_decimal.multiply_pow10(-1);
console.log("multiplied by 0.1");

console.log(my_decimal.to_string());

const bytes = Uint8Array.from(["e".charCodeAt(0), "n".charCodeAt(0)]);
let locale = ICU4XLocale.new_from_bytes(bytes);

locale = ICU4XLocale.new("bn");

const data_provider = ICU4XDataProvider.new_static();

const fdf = ICU4XFixedDecimalFormat.try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions.default());
console.log(fdf.success);
console.log(fdf.fdf.format_write(my_decimal));
