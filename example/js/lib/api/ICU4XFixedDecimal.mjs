import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const ICU4XFixedDecimal_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimal_destroy(underlying);
});

export class ICU4XFixedDecimal {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XFixedDecimal_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_v) {
    return new ICU4XFixedDecimal(wasm.ICU4XFixedDecimal_new(arg_v), true, []);
  }

  multiply_pow10(arg_power) {
    wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, arg_power);
  }

  to_string() {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const is_ok = wasm.ICU4XFixedDecimal_to_string(this.underlying, writeable) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError(undefined);
        }
      })();
    });
  }
}
