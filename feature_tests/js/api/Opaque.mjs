import wasm from "../wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import MyStruct from "./MyStruct.mjs"

const Opaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Opaque_destroy(underlying);
});

export default class Opaque {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Opaque_box_destroy_registry.register(this, underlying);
    }
  }

  static new() {
    return new Opaque(wasm.Opaque_new(), true, []);
  }

  assert_struct(arg_s) {
    const field_a_arg_s = arg_s["a"];
    const field_b_arg_s = arg_s["b"];
    const field_c_arg_s = arg_s["c"];
    const field_d_arg_s = arg_s["d"];
    const field_e_arg_s = arg_s["e"];
    const field_f_arg_s = arg_s["f"];
    wasm.Opaque_assert_struct(this.underlying, field_a_arg_s, field_b_arg_s, field_c_arg_s, field_d_arg_s, field_e_arg_s, diplomatRuntime.extractCodePoint(field_f_arg_s, 'field_f_arg_s'));
  }
}
