import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export class BorrowedFieldsReturning {
  constructor(underlying, edges_a) {
    this.bytes = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
  }
}
