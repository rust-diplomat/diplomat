import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ImportedStruct } from "./ImportedStruct.mjs"
import { MyEnum_js_to_rust, MyEnum_rust_to_js } from "./MyEnum.mjs"
import { UnimportedEnum_js_to_rust, UnimportedEnum_rust_to_js } from "./UnimportedEnum.mjs"

const Opaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Opaque_destroy(underlying);
});

export class Opaque {
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
    const field_g_arg_s = arg_s["g"];
    wasm.Opaque_assert_struct(this.underlying, field_a_arg_s, field_b_arg_s, field_c_arg_s, field_d_arg_s, field_e_arg_s, diplomatRuntime.extractCodePoint(field_f_arg_s, 'field_f_arg_s'), MyEnum_js_to_rust[field_g_arg_s]);
  }

  static returns_usize() {
    return wasm.Opaque_returns_usize();
  }

  static returns_imported() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.Opaque_returns_imported(diplomat_receive_buffer);
      const out = new ImportedStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return out;
    })();
  }
}
