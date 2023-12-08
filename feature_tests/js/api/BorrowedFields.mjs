import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export class BorrowedFields {
  constructor(underlying, edges_a) {
    this.a = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return diplomatRuntime.readString16(wasm, ptr, size);
    })();
    this.b = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 8, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
    this.c = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 16, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
  }
}
