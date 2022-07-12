
/**
 * An ICU4X data provider, capable of loading ICU4X data keys from some source.

 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html) for more information.
 */
export class ICU4XDataProvider {

  /**


   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html) for more information.
   */
  static new_static(): ICU4XDataProvider;

  /**
   * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
   */
  static returns_result(): void | never;
}
