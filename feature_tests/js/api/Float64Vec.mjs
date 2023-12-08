import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Float64Vec_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Float64Vec_destroy(underlying);
});

export class Float64Vec {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Float64Vec_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "f64");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  fill_slice(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "f64");
    wasm.Float64Vec_fill_slice(this.underlying, buf_arg_v.ptr, buf_arg_v.size);
    buf_arg_v.free();
  }

  set_value(arg_new_slice) {
    const buf_arg_new_slice = diplomatRuntime.DiplomatBuf.slice(wasm, arg_new_slice, "f64");
    wasm.Float64Vec_set_value(this.underlying, buf_arg_new_slice.ptr, buf_arg_new_slice.size);
    buf_arg_new_slice.free();
  }
}
