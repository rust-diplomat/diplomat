import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { AttrEnum_js_to_rust, AttrEnum_rust_to_js } from "./AttrEnum.mjs"

const AttrOpaque1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.namespace_AttrOpaque1_destroy(underlying);
});

export class AttrOpaque1 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      AttrOpaque1_box_destroy_registry.register(this, underlying);
    }
  }

  static new() {
    return new AttrOpaque1(wasm.namespace_AttrOpaque1_new(), true, []);
  }

  method() {
    return wasm.namespace_AttrOpaque1_method(this.underlying);
  }

  abirenamed() {
    return wasm.renamed_on_abi_only(this.underlying);
  }

  method_disabled() {
    wasm.namespace_AttrOpaque1_method_disabled(this.underlying);
  }

  use_unnamespaced(arg__un) {
    wasm.namespace_AttrOpaque1_use_unnamespaced(this.underlying, arg__un.underlying);
  }

  use_namespaced(arg__n) {
    wasm.namespace_AttrOpaque1_use_namespaced(this.underlying, AttrEnum_js_to_rust[arg__n]);
  }
}
