import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const RefList_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.RefList_destroy(underlying);
});

export class RefList {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      RefList_box_destroy_registry.register(this, underlying);
    }
  }

  static node(arg_data) {
    return new RefList(wasm.RefList_node(arg_data.underlying), true, [arg_data]);
  }
}
