import { FFIError } from "./diplomat-runtime"

/**

 * An  data provider, capable of loading  data keys from some source.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html Rust documentation for `icu_provider`} for more information.
 */
export class DataProvider {

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html Rust documentation for `get_static_provider`} for more information.
   */
  static new_static(): DataProvider;

  /**

   * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
   * @throws {@link FFIError}<void>
   */
  static returns_result(): void | never;
}
