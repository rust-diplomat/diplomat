import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XFixedDecimal } from "./ICU4XFixedDecimal";
import { ICU4XFixedDecimalFormatterOptions } from "./ICU4XFixedDecimalFormatterOptions";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * An ICU4X Fixed Decimal Format object, capable of formatting a {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} as a string.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html Rust documentation for `FixedDecimalFormatter`} for more information.
 */
export class ICU4XFixedDecimalFormatter {

  /**

   * Creates a new {@link ICU4XFixedDecimalFormatter `ICU4XFixedDecimalFormatter`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<void>
   */
  static try_new(locale: ICU4XLocale, provider: ICU4XDataProvider, options: ICU4XFixedDecimalFormatterOptions): ICU4XFixedDecimalFormatter | never;

  /**

   * Formats a {@link ICU4XFixedDecimal `ICU4XFixedDecimal`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_write(value: ICU4XFixedDecimal): string;
}
