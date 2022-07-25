import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XFixedDecimalGroupingStrategy_js_to_rust, ICU4XFixedDecimalGroupingStrategy_rust_to_js } from "./ICU4XFixedDecimalGroupingStrategy.js"
import { ICU4XFixedDecimalSignDisplay_js_to_rust, ICU4XFixedDecimalSignDisplay_rust_to_js } from "./ICU4XFixedDecimalSignDisplay.js"

export class ICU4XFixedDecimalFormatOptions {
  constructor(underlying) {
    this.grouping_strategy = ICU4XFixedDecimalGroupingStrategy_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.sign_display = ICU4XFixedDecimalSignDisplay_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
  }

  static default() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XFixedDecimalFormatOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatOptions(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }
}
