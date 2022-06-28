import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

const ICU4XDataProvider_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataProvider_destroy(underlying);
});

export class ICU4XDataProvider {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new_static() {
    return (() => {
      const underlying = wasm.ICU4XDataProvider_new_static();
      const out = new ICU4XDataProvider(underlying);
      ICU4XDataProvider_box_destroy_registry.register(out, underlying);
      return out;
    })();
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
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(v) {
    return (() => {
      const underlying = wasm.ICU4XFixedDecimal_new(v);
      const out = new ICU4XFixedDecimal(underlying);
      ICU4XFixedDecimal_box_destroy_registry.register(out, underlying);
      return out;
    })();
  }

  multiply_pow10(power) {
    wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, power);
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
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(locale, provider, options) {
    const f_options_grouping_strategy = options["grouping_strategy"];
    const f_options_sign_display = options["sign_display"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[f_options_grouping_strategy], ICU4XFixedDecimalSignDisplay_js_to_rust[f_options_sign_display]);
      const out = new ICU4XFixedDecimalFormatResult(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return out;
    })();
  }

  format_write(value) {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.ICU4XFixedDecimalFormat_format_write(this.underlying, value.underlying, writeable);
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
      return (option_ptr == 0) ? null : (() => {
        const underlying = option_ptr;
        const out = new ICU4XFixedDecimalFormat(underlying);
        ICU4XFixedDecimalFormat_box_destroy_registry.register(out, underlying);
        return out;
      })();
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
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(name) {
    name = diplomatRuntime.DiplomatBuf.str(wasm, name);
    const diplomat_out = (() => {
      const underlying = wasm.ICU4XLocale_new(name.ptr, name.size);
      const out = new ICU4XLocale(underlying);
      ICU4XLocale_box_destroy_registry.register(out, underlying);
      return out;
    })();
    name.free();
    return diplomat_out;
  }

  static new_from_bytes(bytes) {
    bytes = diplomatRuntime.DiplomatBuf.slice(wasm, bytes, 1);
    const diplomat_out = (() => {
      const underlying = wasm.ICU4XLocale_new_from_bytes(bytes.ptr, bytes.size);
      const out = new ICU4XLocale(underlying);
      ICU4XLocale_box_destroy_registry.register(out, underlying);
      return out;
    })();
    bytes.free();
    return diplomat_out;
  }
}
