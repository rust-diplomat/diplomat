import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { AttrOpaque1 } from "./AttrOpaque1.mjs"

const OpaqueIterator_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_OpaqueIterator_destroy(underlying);
});

export class OpaqueIterator {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      OpaqueIterator_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return (() => {
      const option_ptr = wasm.namespace_OpaqueIterator_next(this.underlying);
      return (option_ptr == 0) ? undefined : new AttrOpaque1(option_ptr, true, []);
    })();
  }
}
