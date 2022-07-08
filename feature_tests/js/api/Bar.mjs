import wasm from "../wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Bar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Bar_destroy(underlying);
});

export default class Bar {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Bar_box_destroy_registry.register(this, underlying);
    }
  }
}
