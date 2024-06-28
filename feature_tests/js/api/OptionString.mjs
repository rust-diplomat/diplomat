import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const OptionString_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionString_destroy(underlying);
});

export class OptionString {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      OptionString_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_diplomat_str) {
    const buf_arg_diplomat_str = diplomatRuntime.DiplomatBuf.str8(wasm, arg_diplomat_str);
    const diplomat_out = (() => {
      const option_ptr = wasm.OptionString_new(buf_arg_diplomat_str.ptr, buf_arg_diplomat_str.size);
      return (option_ptr == 0) ? undefined : new OptionString(option_ptr, true, []);
    })();
    buf_arg_diplomat_str.free();
    return diplomat_out;
  }

  write() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.OptionString_write(this.underlying, write) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError(undefined);
        }
      })();
    });
  }

  borrow() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
      wasm.OptionString_borrow(diplomat_receive_buffer, this.underlying);
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
