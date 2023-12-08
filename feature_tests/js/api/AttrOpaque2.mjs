import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const AttrOpaque2_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.AttrOpaque2_destroy(underlying);
});

export class AttrOpaque2 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      AttrOpaque2_box_destroy_registry.register(this, underlying);
    }
  }
}
