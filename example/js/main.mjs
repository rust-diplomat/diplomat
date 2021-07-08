import { ICU4XFixedDecimal } from "./high-level.mjs";

const my_decimal = ICU4XFixedDecimal.new(123);

console.log(my_decimal.to_string());
console.log("multiplied by 0.1");
console.log(my_decimal.to_string());
