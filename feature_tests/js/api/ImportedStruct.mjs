import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { UnimportedEnum_js_to_rust, UnimportedEnum_rust_to_js } from "./UnimportedEnum.mjs"

export class ImportedStruct {
  constructor(underlying) {
    this.foo = UnimportedEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.count = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0];
  }
}
