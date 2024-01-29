import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export class BorrowedFieldsWithBounds {
  constructor(underlying, edges_a, edges_b, edges_c) {
    this.field_a = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return diplomatRuntime.readString16(wasm, ptr, size);
    })();
    this.field_b = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 8, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
    this.field_c = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 16, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
  }
}
