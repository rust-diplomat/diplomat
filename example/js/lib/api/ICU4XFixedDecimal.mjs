import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const ICU4XFixedDecimal_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.icu4x_ICU4XFixedDecimal_mv1_destroy(underlying);
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
    return new ICU4XFixedDecimal(wasm.icu4x_ICU4XFixedDecimal_new_mv1(arg_v), true, []);
  }

  multiply_pow10(arg_power) {
    wasm.icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(this.underlying, arg_power);
  }

  to_string() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.icu4x_ICU4XFixedDecimal_to_string_mv1(this.underlying, write) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError(undefined);
        }
      })();
    });
  }
}
