import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { MyIterator } from "./MyIterator.mjs"

const MyIterable_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_MyIterable_destroy(underlying);
});

export class MyIterable {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      MyIterable_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_x) {
    const buf_arg_x = diplomatRuntime.DiplomatBuf.slice(wasm, arg_x, "u8");
    const diplomat_out = new MyIterable(wasm.namespace_MyIterable_new(buf_arg_x.ptr, buf_arg_x.size), true, []);
    buf_arg_x.free();
    return diplomat_out;
  }

  iter() {
    return new MyIterator(wasm.namespace_MyIterable_iter(this.underlying), true, [this]);
  }
}
