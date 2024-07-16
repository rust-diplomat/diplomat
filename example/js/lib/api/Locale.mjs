import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Locale_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.icu4x_Locale_destroy_mv1(underlying);
});

export class Locale {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Locale_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_name) {
    const buf_arg_name = diplomatRuntime.DiplomatBuf.str8(wasm, arg_name);
    const diplomat_out = new Locale(wasm.icu4x_Locale_new_mv1(buf_arg_name.ptr, buf_arg_name.size), true, []);
    buf_arg_name.free();
    return diplomat_out;
  }
}
