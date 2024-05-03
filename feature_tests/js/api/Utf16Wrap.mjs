import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Utf16Wrap_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Utf16Wrap_destroy(underlying);
});

export class Utf16Wrap {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Utf16Wrap_box_destroy_registry.register(this, underlying);
    }
  }

  borrow_cont() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.Utf16Wrap_borrow_cont(diplomat_receive_buffer, this.underlying);
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 2);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return diplomatRuntime.readString16(wasm, ptr, size);
    })();
  }

  owned() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.Utf16Wrap_owned(diplomat_receive_buffer, this.underlying);
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 2);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return diplomatRuntime.readString16(wasm, ptr, size);
    })();
  }
}
