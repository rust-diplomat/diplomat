import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { FixedDecimalGroupingStrategy_js_to_rust, FixedDecimalGroupingStrategy_rust_to_js } from "./FixedDecimalGroupingStrategy.mjs"

export class FixedDecimalFormatterOptions {
  constructor(underlying) {
    this.grouping_strategy = FixedDecimalGroupingStrategy_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.some_other_config = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0] == 1;
  }

  static default() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.icu4x_FixedDecimalFormatterOptions_default_mv1(diplomat_receive_buffer);
      const out = new FixedDecimalFormatterOptions(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return out;
    })();
  }
}
