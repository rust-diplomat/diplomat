import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { OpaqueIterator } from "./OpaqueIterator.mjs"

const OpaqueIterable_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_OpaqueIterable_destroy(underlying);
});

export class OpaqueIterable {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      OpaqueIterable_box_destroy_registry.register(this, underlying);
    }
  }

  iter() {
    return new OpaqueIterator(wasm.namespace_OpaqueIterable_iter(this.underlying), true, [this]);
  }
}
