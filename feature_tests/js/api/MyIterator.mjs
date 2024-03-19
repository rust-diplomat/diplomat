import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const MyIterator_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_MyIterator_destroy(underlying);
});

export class MyIterator {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      MyIterator_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.namespace_MyIterator_next(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0];
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }
}
