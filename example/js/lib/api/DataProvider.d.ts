// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/**
 * An  data provider, capable of loading  data keys from some source.
 *
 * See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
 */


export class DataProvider {
    get ffiValue(): pointer;


    /**
     * See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
     */
    static newStatic(): DataProvider;

    /**
     * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
     */
    static returnsResult(): boolean;
}