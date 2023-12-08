import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const AttrOpaque1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.AttrOpaque1_destroy(underlying);
});

export class AttrOpaque1 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      AttrOpaque1_box_destroy_registry.register(this, underlying);
    }
  }

  method() {
    wasm.AttrOpaque1_method(this.underlying);
  }

  method_disabledcpp() {
    wasm.AttrOpaque1_method_disabledcpp(this.underlying);
  }
}
