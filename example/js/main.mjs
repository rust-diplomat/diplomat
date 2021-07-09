import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormat } from "./high-level.mjs";

const my_decimal = ICU4XFixedDecimal.new(123);

console.log(my_decimal.to_string());

my_decimal.multiply_pow10(-1);
console.log("multiplied by 0.1");

console.log(my_decimal.to_string());

const locale = ICU4XLocale.new("bn");

const data_provider = ICU4XDataProvider.new_static();

const fdf = ICU4XFixedDecimalFormat.new(locale, data_provider);
console.log(fdf.format_write(my_decimal));
