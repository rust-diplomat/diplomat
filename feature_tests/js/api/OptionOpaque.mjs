import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { OptionStruct } from "./OptionStruct.mjs"

const OptionOpaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionOpaque_destroy(underlying);
});

export class OptionOpaque {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      OptionOpaque_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_i) {
    return (() => {
      const option_ptr = wasm.OptionOpaque_new(arg_i);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, true, []);
    })();
  }

  static new_none() {
    return (() => {
      const option_ptr = wasm.OptionOpaque_new_none();
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, true, []);
    })();
  }

  static new_struct() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 16, 4);
      return out;
    })();
  }

  static new_struct_nones() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct_nones(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 16, 4);
      return out;
    })();
  }

  assert_integer(arg_i) {
    wasm.OptionOpaque_assert_integer(this.underlying, arg_i);
  }

  static option_opaque_argument(arg_arg) {
    return wasm.OptionOpaque_option_opaque_argument(arg_arg);
  }
}
