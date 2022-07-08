import wasm from "../wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export default class MyStruct {
  constructor(underlying) {
    this.a = (new Uint8Array(wasm.memory.buffer, underlying, 1))[0];
    this.b = (new Uint8Array(wasm.memory.buffer, underlying + 1, 1))[0] == 1;
    this.c = (new Uint8Array(wasm.memory.buffer, underlying + 2, 1))[0];
    this.d = (new BigUint64Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.e = (new Int32Array(wasm.memory.buffer, underlying + 16, 1))[0];
    this.f = String.fromCharCode((new Uint32Array(wasm.memory.buffer, underlying + 20, 1))[0]);
  }

  static new() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(24, 8);
      wasm.MyStruct_new(diplomat_receive_buffer);
      const out = new MyStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 24, 8);
      return out;
    })();
  }
}
