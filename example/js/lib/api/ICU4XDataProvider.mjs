import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const ICU4XDataProvider_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataProvider_destroy(underlying);
});

export class ICU4XDataProvider {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XDataProvider_box_destroy_registry.register(this, underlying);
    }
  }

  static new_static() {
    return new ICU4XDataProvider(wasm.ICU4XDataProvider_new_static(), true, []);
  }

  static returns_result() {
    return (() => {
      const is_ok = wasm.ICU4XDataProvider_returns_result() == 1;
      if (!is_ok) {
        throw new diplomatRuntime.FFIError(undefined);
      }
    })();
  }
}
