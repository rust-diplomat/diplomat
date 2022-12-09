import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export class BorrowedFields {
  constructor(underlying, edges_a) {
    this.a = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return new Uint16Array(wasm.memory.buffer, ptr, size);
    })();
    this.b = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 8, 2);
      return diplomatRuntime.readString(wasm, ptr, size);
    })();
  }
}
