
/**
 * An ICU4X Locale, capable of representing strings like `"en-US"`.

 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
 */
export class ICU4XLocale {

  /**
   * Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
   */
  static new(name: string): ICU4XLocale;

  /**
   * Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
   */
  static new_from_bytes(bytes: Uint8Array): ICU4XLocale;
}
