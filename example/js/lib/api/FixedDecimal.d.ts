import { i16, i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html Rust documentation for `FixedDecimal`} for more information.
 */
export class FixedDecimal {

  /**

   * Construct an {@link FixedDecimal `FixedDecimal`} from an integer.
   */
  static new(v: i32): FixedDecimal;

  /**

   * Multiply the {@link FixedDecimal `FixedDecimal`} by a given power of ten.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10 Rust documentation for `multiply_pow10`} for more information.
   */
  multiply_pow10(power: i16): void;

  /**

   * Format the {@link FixedDecimal `FixedDecimal`} as a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to Rust documentation for `write_to`} for more information.
   * @throws {@link FFIError}<void>
   */
  to_string(): string | never;
}
