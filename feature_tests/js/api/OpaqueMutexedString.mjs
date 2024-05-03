import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { Utf16Wrap } from "./Utf16Wrap.mjs"

const OpaqueMutexedString_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OpaqueMutexedString_destroy(underlying);
});

export class OpaqueMutexedString {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      OpaqueMutexedString_box_destroy_registry.register(this, underlying);
    }
  }

  static from_usize(arg_number) {
    return new OpaqueMutexedString(wasm.OpaqueMutexedString_from_usize(arg_number), true, []);
  }

  change(arg_number) {
    wasm.OpaqueMutexedString_change(this.underlying, arg_number);
  }

  get_len_and_add(arg_other) {
    return wasm.OpaqueMutexedString_get_len_and_add(this.underlying, arg_other);
  }

  dummy_str() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.OpaqueMutexedString_dummy_str(diplomat_receive_buffer, this.underlying);
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 2);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
  }

  wrapper() {
    return new Utf16Wrap(wasm.OpaqueMutexedString_wrapper(this.underlying), true, []);
  }
}
