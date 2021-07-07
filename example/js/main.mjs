import { ICU4XFixedDecimal } from "./high-level.mjs";

const my_decimal = ICU4XFixedDecimal.new(123);

console.log(my_decimal.digit_at(2));
console.log(my_decimal.digit_at(1));
console.log(my_decimal.digit_at(0));
console.log(my_decimal.digit_at(-1));

my_decimal.multiply_pow10(-1);
console.log("multiplied by 0.1");

console.log(my_decimal.digit_at(2));
console.log(my_decimal.digit_at(1));
console.log(my_decimal.digit_at(0));
console.log(my_decimal.digit_at(-1));
