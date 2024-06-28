import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const MyIndexer_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_MyIndexer_destroy(underlying);
});

export class MyIndexer {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      MyIndexer_box_destroy_registry.register(this, underlying);
    }
  }

  get(arg_i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
      wasm.namespace_MyIndexer_get(diplomat_receive_buffer, this.underlying, arg_i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 8);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        return;
      }
      const value = (() => {
        const [ptr, size] = new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 2);
        return diplomatRuntime.readString8(wasm, ptr, size);
      })();
      wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
      return value;
    })();
  }
}
