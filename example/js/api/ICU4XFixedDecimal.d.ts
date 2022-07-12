
/**


 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html) for more information.
 */
export class ICU4XFixedDecimal {

  /**
   * Construct an [`ICU4XFixedDecimal`] from an integer.
   */
  static new(v: number): ICU4XFixedDecimal;

  /**
   * Multiply the [`ICU4XFixedDecimal`] by a given power of ten.

   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
   */
  multiply_pow10(power: number): void;

  /**
   * Invert the sign of the [`ICU4XFixedDecimal`].

   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.negate) for more information.
   */
  negate(): void;

  /**
   * Format the [`ICU4XFixedDecimal`] as a string.

   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  to_string(): string | never;
}
