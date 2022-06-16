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
    ICU4XDataProvider_box_destroy_registry.register(this, underlying);
  }

  static new_static() {
    const diplomat_out = new ICU4XDataProvider(wasm.ICU4XDataProvider_new_static());
    return diplomat_out;
  }

  static returns_result() {
    const diplomat_out = (() => {
      const is_ok = wasm.ICU4XDataProvider_returns_result() == 1;
      if (!is_ok) {
        throw new diplomatRuntime.FFIError({});
      }
    })();
    return diplomat_out;
  }
}

const ICU4XFixedDecimal_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimal_destroy(underlying);
});

export class ICU4XFixedDecimal {
  constructor(underlying) {
    this.underlying = underlying;
    ICU4XFixedDecimal_box_destroy_registry.register(this, underlying);
  }

  static new(v) {
    const diplomat_out = new ICU4XFixedDecimal(wasm.ICU4XFixedDecimal_new(v));
    return diplomat_out;
  }

  multiply_pow10(power) {
    const diplomat_out = wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, power);
  }

  negate() {
    const diplomat_out = wasm.ICU4XFixedDecimal_negate(this.underlying);
  }

  to_string() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const is_ok = wasm.ICU4XFixedDecimal_to_string(this.underlying, writeable) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError({});
        }
      })();
    });
    return diplomat_out;
  }
}

const ICU4XFixedDecimalFormat_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormat_destroy(underlying);
});

export class ICU4XFixedDecimalFormat {
  constructor(underlying) {
    this.underlying = underlying;
    ICU4XFixedDecimalFormat_box_destroy_registry.register(this, underlying);
  }

  static try_new(locale, provider, options) {
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy = options["grouping_strategy"];
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display = options["sign_display"];
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy], ICU4XFixedDecimalSignDisplay_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display]);
      const out = new ICU4XFixedDecimalFormatResult(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return out;
    })();
    return diplomat_out;
  }

  format_write(value) {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.ICU4XFixedDecimalFormat_format_write(this.underlying, value.underlying, writeable);
    });
    return diplomat_out;
  }
}

export class ICU4XFixedDecimalFormatOptions {
  constructor(underlying) {
    this.grouping_strategy = ICU4XFixedDecimalGroupingStrategy_rust_to_js[(new Int32Array(wasm.memory.buffer, underlying + 0, 1))[0]];
    this.sign_display = ICU4XFixedDecimalSignDisplay_rust_to_js[(new Int32Array(wasm.memory.buffer, underlying + 4, 1))[0]];
  }

  static default() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XFixedDecimalFormatOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatOptions(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
    return diplomat_out;
  }
}

export class ICU4XFixedDecimalFormatResult {
  constructor(underlying) {
    this.fdf = (() => {
      const option_ptr = (new Uint32Array(wasm.memory.buffer, underlying + 0, 1))[0];
      return (option_ptr == 0) ? null : new ICU4XFixedDecimalFormat(option_ptr);
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
    ICU4XLocale_box_destroy_registry.register(this, underlying);
  }

  static new(name) {
    let name_diplomat_str = diplomatRuntime.DiplomatBuf.str(wasm, name);
    const diplomat_out = new ICU4XLocale(wasm.ICU4XLocale_new(name_diplomat_str.ptr, name_diplomat_str.size));
    name_diplomat_str.free();
    return diplomat_out;
  }

  static new_from_bytes(bytes) {
    let bytes_diplomat_slice = diplomatRuntime.DiplomatBuf.slice(wasm, bytes, 1);
    const diplomat_out = new ICU4XLocale(wasm.ICU4XLocale_new_from_bytes(bytes_diplomat_slice.ptr, bytes_diplomat_slice.size));
    bytes_diplomat_slice.free();
    return diplomat_out;
  }
}
