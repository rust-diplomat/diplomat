import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { MyEnum_js_to_rust, MyEnum_rust_to_js } from "./MyEnum.mjs"

export class MyStruct {
  constructor(underlying) {
    this.a = (new Uint8Array(wasm.memory.buffer, underlying, 1))[0];
    this.b = (new Uint8Array(wasm.memory.buffer, underlying + 1, 1))[0] == 1;
    this.c = (new Uint8Array(wasm.memory.buffer, underlying + 2, 1))[0];
    this.d = (new BigUint64Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.e = (new Int32Array(wasm.memory.buffer, underlying + 16, 1))[0];
    this.f = String.fromCharCode((new Uint32Array(wasm.memory.buffer, underlying + 20, 1))[0]);
    this.g = MyEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 24)];
  }

  static new() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(28, 8);
      wasm.MyStruct_new(diplomat_receive_buffer);
      const out = new MyStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 28, 8);
      return out;
    })();
  }

  into_a() {
    const field_a_this = this["a"];
    const field_b_this = this["b"];
    const field_c_this = this["c"];
    const field_d_this = this["d"];
    const field_e_this = this["e"];
    const field_f_this = this["f"];
    const field_g_this = this["g"];
    return wasm.MyStruct_into_a(field_a_this, field_b_this, field_c_this, field_d_this, field_e_this, diplomatRuntime.extractCodePoint(field_f_this, 'field_f_this'), MyEnum_js_to_rust[field_g_this]);
  }
}
