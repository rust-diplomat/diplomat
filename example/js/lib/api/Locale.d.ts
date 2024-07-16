
/**

 * An  Locale, capable of representing strings like `"en-US"`.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html Rust documentation for `Locale`} for more information.
 */
export class Locale {

  /**

   * Construct an {@link Locale `Locale`} from a locale identifier represented as a string.
   */
  static new(name: string): Locale;
}
