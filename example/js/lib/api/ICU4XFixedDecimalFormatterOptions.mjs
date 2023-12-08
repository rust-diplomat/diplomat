import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XFixedDecimalGroupingStrategy_js_to_rust, ICU4XFixedDecimalGroupingStrategy_rust_to_js } from "./ICU4XFixedDecimalGroupingStrategy.mjs"

export class ICU4XFixedDecimalFormatterOptions {
  constructor(underlying) {
    this.grouping_strategy = ICU4XFixedDecimalGroupingStrategy_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.some_other_config = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0] == 1;
  }

  static default() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimalFormatterOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatterOptions(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return out;
    })();
  }
}
