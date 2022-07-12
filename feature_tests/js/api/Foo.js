import wasm from "../wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { Bar } from "./Bar.js"

const Foo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Foo_destroy(underlying);
});

export class Foo {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Foo_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_x) {
    const buf_arg_x = diplomatRuntime.DiplomatBuf.str(wasm, arg_x);
    return new Foo(wasm.Foo_new(buf_arg_x.ptr, buf_arg_x.size), true, [buf_arg_x]);
  }

  get_bar() {
    return new Bar(wasm.Foo_get_bar(this.underlying), true, [this]);
  }
}
