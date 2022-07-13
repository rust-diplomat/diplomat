import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XFixedDecimal } from "./ICU4XFixedDecimal";
import { ICU4XFixedDecimalFormatOptions } from "./ICU4XFixedDecimalFormatOptions";
import { ICU4XFixedDecimalFormatResult } from "./ICU4XFixedDecimalFormatResult";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * An ICU4X Fixed Decimal Format object, capable of formatting a {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} as a string.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html Rust documentation} for more information.
 */
export class ICU4XFixedDecimalFormat {

  /**

   * Creates a new {@link ICU4XFixedDecimalFormat `ICU4XFixedDecimalFormat`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new Rust documentation} for more information.
   */
  static try_new(locale: ICU4XLocale, provider: ICU4XDataProvider, options: ICU4XFixedDecimalFormatOptions): ICU4XFixedDecimalFormatResult;

  /**

   * Formats a {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format Rust documentation} for more information.
   */
  format_write(value: ICU4XFixedDecimal): string;
}
