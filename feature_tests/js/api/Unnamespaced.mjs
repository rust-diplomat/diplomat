import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { AttrEnum_js_to_rust, AttrEnum_rust_to_js } from "./AttrEnum.mjs"

const Unnamespaced_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_Unnamespaced_destroy(underlying);
});

export class Unnamespaced {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Unnamespaced_box_destroy_registry.register(this, underlying);
    }
  }

  static make(arg_e) {
    return new Unnamespaced(wasm.namespace_Unnamespaced_make(AttrEnum_js_to_rust[arg_e]), true, []);
  }

  use_namespaced(arg__n) {
    wasm.namespace_Unnamespaced_use_namespaced(this.underlying, arg__n.underlying);
  }
}
