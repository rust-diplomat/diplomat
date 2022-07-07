import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

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
        throw new diplomatRuntime.FFIError({});
      }
    })();
  }
}

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

  negate() {
    wasm.ICU4XFixedDecimal_negate(this.underlying);
  }

  to_string() {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const is_ok = wasm.ICU4XFixedDecimal_to_string(this.underlying, writeable) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError({});
        }
      })();
    });
  }
}

const ICU4XFixedDecimalFormat_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormat_destroy(underlying);
});

export class ICU4XFixedDecimalFormat {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XFixedDecimalFormat_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new(arg_locale, arg_provider, arg_options) {
    const field_grouping_strategy_arg_options = arg_options["grouping_strategy"];
    const field_sign_display_arg_options = arg_options["sign_display"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, arg_locale.underlying, arg_provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[field_grouping_strategy_arg_options], ICU4XFixedDecimalSignDisplay_js_to_rust[field_sign_display_arg_options]);
      const out = new ICU4XFixedDecimalFormatResult(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return out;
    })();
  }

  format_write(arg_value) {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.ICU4XFixedDecimalFormat_format_write(this.underlying, arg_value.underlying, writeable);
    });
  }
}

export class ICU4XFixedDecimalFormatOptions {
  constructor(underlying) {
    this.grouping_strategy = ICU4XFixedDecimalGroupingStrategy_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.sign_display = ICU4XFixedDecimalSignDisplay_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
  }

  static default() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XFixedDecimalFormatOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatOptions(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }
}

export class ICU4XFixedDecimalFormatResult {
  constructor(underlying) {
    this.fdf = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new ICU4XFixedDecimalFormat(option_ptr, true, []);
    })();
    this.success = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0] == 1;
  }
}

const ICU4XFixedDecimalGroupingStrategy_js_to_rust = {
  "Auto": 0,
  "Never": 1,
  "Always": 2,
  "Min2": 3,
};
const ICU4XFixedDecimalGroupingStrategy_rust_to_js = {
  0: "Auto",
  1: "Never",
  2: "Always",
  3: "Min2",
};

const ICU4XFixedDecimalSignDisplay_js_to_rust = {
  "Auto": 0,
  "Never": 1,
  "Always": 2,
  "ExceptZero": 3,
  "Negative": 4,
};
const ICU4XFixedDecimalSignDisplay_rust_to_js = {
  0: "Auto",
  1: "Never",
  2: "Always",
  3: "ExceptZero",
  4: "Negative",
};

const ICU4XLocale_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocale_destroy(underlying);
});

export class ICU4XLocale {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XLocale_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_name) {
    const buf_arg_name = diplomatRuntime.DiplomatBuf.str(wasm, arg_name);
    const diplomat_out = new ICU4XLocale(wasm.ICU4XLocale_new(buf_arg_name.ptr, buf_arg_name.size), true, []);
    buf_arg_name.free();
    return diplomat_out;
  }

  static new_from_bytes(arg_bytes) {
    const buf_arg_bytes = diplomatRuntime.DiplomatBuf.slice(wasm, arg_bytes, 1);
    const diplomat_out = new ICU4XLocale(wasm.ICU4XLocale_new_from_bytes(buf_arg_bytes.ptr, buf_arg_bytes.size), true, []);
    buf_arg_bytes.free();
    return diplomat_out;
  }
}
