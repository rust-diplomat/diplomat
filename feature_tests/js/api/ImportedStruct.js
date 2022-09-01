import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { UnimportedEnum_js_to_rust, UnimportedEnum_rust_to_js } from "./UnimportedEnum.js"

export class ImportedStruct {
  constructor(underlying) {
    this.foo = UnimportedEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.int = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0];
  }
}
