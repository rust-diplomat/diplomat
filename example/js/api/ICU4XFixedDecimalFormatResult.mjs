import wasm from "../wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import ICU4XFixedDecimalFormat from "./ICU4XFixedDecimalFormat.mjs"

export default class ICU4XFixedDecimalFormatResult {
  constructor(underlying) {
    this.fdf = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new ICU4XFixedDecimalFormat(option_ptr, true, []);
    })();
    this.success = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0] == 1;
  }
}
