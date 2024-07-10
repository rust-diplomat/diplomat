import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XFixedDecimalGroupingStrategy_js_to_rust, ICU4XFixedDecimalGroupingStrategy_rust_to_js } from "./ICU4XFixedDecimalGroupingStrategy.mjs"

const ICU4XFixedDecimalFormatter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.icu4x_ICU4XFixedDecimalFormatter_mv1_destroy(underlying);
});

export class ICU4XFixedDecimalFormatter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XFixedDecimalFormatter_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new(arg_locale, arg_provider, arg_options) {
    const field_grouping_strategy_arg_options = arg_options["grouping_strategy"];
    const field_some_other_config_arg_options = arg_options["some_other_config"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.icu4x_ICU4XFixedDecimalFormatter_try_new_mv1(diplomat_receive_buffer, arg_locale.underlying, arg_provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[field_grouping_strategy_arg_options], field_some_other_config_arg_options);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XFixedDecimalFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  format_write(arg_value) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.icu4x_ICU4XFixedDecimalFormatter_format_write_mv1(this.underlying, arg_value.underlying, write);
    });
  }
}
