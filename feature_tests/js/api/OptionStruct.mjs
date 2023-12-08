import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { OptionOpaque } from "./OptionOpaque.mjs"
import { OptionOpaqueChar } from "./OptionOpaqueChar.mjs"

export class OptionStruct {
  constructor(underlying) {
    this.a = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, true, []);
    })();
    this.b = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying + 4);
      return (option_ptr == 0) ? null : new OptionOpaqueChar(option_ptr, true, []);
    })();
    this.c = (new Uint32Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.d = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying + 12);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, true, []);
    })();
  }
}
