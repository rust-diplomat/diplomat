import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Comparable_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_Comparable_destroy(underlying);
});

export class Comparable {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Comparable_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_int) {
    return new Comparable(wasm.namespace_Comparable_new(arg_int), true, []);
  }

  cmp(arg_other) {
    return wasm.namespace_Comparable_cmp(this.underlying, arg_other.underlying);
  }
}
