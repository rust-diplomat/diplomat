import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { Foo } from "./Foo.mjs"

const Bar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Bar_destroy(underlying);
});

export class Bar {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Bar_box_destroy_registry.register(this, underlying);
    }
  }

  foo() {
    return new Foo(wasm.Bar_foo(this.underlying), false, [this]);
  }
}
