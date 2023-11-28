import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

const MyString_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.MyString_destroy(underlying);
});

export class MyString {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      MyString_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.str8(wasm, arg_v);
    const diplomat_out = new MyString(wasm.MyString_new(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_unsafe(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.str8(wasm, arg_v);
    const diplomat_out = new MyString(wasm.MyString_new_unsafe(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  set_str(arg_new_str) {
    const buf_arg_new_str = diplomatRuntime.DiplomatBuf.str8(wasm, arg_new_str);
    wasm.MyString_set_str(this.underlying, buf_arg_new_str.ptr, buf_arg_new_str.size);
    buf_arg_new_str.free();
  }

  get_str() {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.MyString_get_str(this.underlying, writeable);
    });
  }
}
