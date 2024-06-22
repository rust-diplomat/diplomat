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

  static from_utf16(arg_input) {
    const buf_arg_input = diplomatRuntime.DiplomatBuf.str16(wasm, arg_input);
    const diplomat_out = new Utf16Wrap(wasm.Utf16Wrap_from_utf16(buf_arg_input.ptr, buf_arg_input.size), true, []);
    buf_arg_input.free();
    return diplomat_out;
  }

  get_debug_str() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.Utf16Wrap_get_debug_str(this.underlying, write);
    });
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
