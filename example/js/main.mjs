import { ICU4XFixedDecimal, ICU4XLocale } from "./high-level.mjs";

const my_decimal = ICU4XFixedDecimal.new(123);

console.log(my_decimal.to_string());

my_decimal.multiply_pow10(-1);
console.log("multiplied by 0.1");

console.log(my_decimal.to_string());

const my_locale = ICU4XLocale.new("en-US");
console.log(my_locale.underlying);
