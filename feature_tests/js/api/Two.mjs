import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Two_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Two_destroy(underlying);
});

export class Two {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Two_box_destroy_registry.register(this, underlying);
    }
  }
}
