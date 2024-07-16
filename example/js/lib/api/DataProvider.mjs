import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const DataProvider_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.icu4x_DataProvider_destroy_mv1(underlying);
});

export class DataProvider {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      DataProvider_box_destroy_registry.register(this, underlying);
    }
  }

  static new_static() {
    return new DataProvider(wasm.icu4x_DataProvider_new_static_mv1(), true, []);
  }

  static returns_result() {
    return (() => {
      const is_ok = wasm.icu4x_DataProvider_returns_result_mv1() == 1;
      if (!is_ok) {
        throw new diplomatRuntime.FFIError(undefined);
      }
    })();
  }
}
