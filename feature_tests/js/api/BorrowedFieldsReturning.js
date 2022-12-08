import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export class BorrowedFieldsReturning {
  constructor(underlying, edges_a) {
    this.bytes = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return new Uint8Array(wasm.memory.buffer, ptr, size);
    })();
  }
}
