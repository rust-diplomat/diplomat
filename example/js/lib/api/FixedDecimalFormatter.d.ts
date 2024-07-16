import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { FixedDecimal } from "./FixedDecimal";
import { FixedDecimalFormatterOptions } from "./FixedDecimalFormatterOptions";
import { Locale } from "./Locale";

/**

 * An  Fixed Decimal Format object, capable of formatting a {@link FixedDecimal `FixedDecimal`} as a string.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html Rust documentation for `FixedDecimalFormatter`} for more information.
 */
export class FixedDecimalFormatter {

  /**

   * Creates a new {@link FixedDecimalFormatter `FixedDecimalFormatter`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<void>
   */
  static try_new(locale: Locale, provider: DataProvider, options: FixedDecimalFormatterOptions): FixedDecimalFormatter | never;

  /**

   * Formats a {@link FixedDecimal `FixedDecimal`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format_write(value: FixedDecimal): string;
}
