
/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html Rust documentation} for more information.
 */
export class ICU4XFixedDecimal {

  /**

   * Construct an {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} from an integer.
   */
  static new(v: number): ICU4XFixedDecimal;

  /**

   * Multiply the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} by a given power of ten.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10 Rust documentation} for more information.
   */
  multiply_pow10(power: number): void;

  /**

   * Invert the sign of the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.negate Rust documentation} for more information.
   */
  negate(): void;

  /**

   * Format the {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} as a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to Rust documentation} for more information.
   */
  to_string(): string | never;
}
