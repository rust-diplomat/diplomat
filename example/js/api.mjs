import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"]);
});

const ICU4XDataProvider_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataProvider_destroy(underlying);
});

export class ICU4XDataProvider {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new_static() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XDataProvider(wasm.ICU4XDataProvider_new_static());
        out.owner = null;
        return out;
      })();
      ICU4XDataProvider_box_destroy_registry.register(out, out.underlying)
      return out;
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
  }

  static new(v) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XFixedDecimal(wasm.ICU4XFixedDecimal_new(v));
        out.owner = null;
        return out;
      })();
      ICU4XFixedDecimal_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
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
          throw {};
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
  }

  static try_new(locale, provider, options) {
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy = options["grouping_strategy"];
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display = options["sign_display"];
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5);
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy], ICU4XFixedDecimalSignDisplay_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display]);
      const out = new ICU4XFixedDecimalFormatResult(diplomat_receive_buffer);
      const out_fdf_value = out.fdf;
      ICU4XFixedDecimalFormat_box_destroy_registry.register(out_fdf_value, out_fdf_value.underlying);
      Object.defineProperty(out, "fdf", { value: out_fdf_value });
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5
      });
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

const ICU4XFixedDecimalFormatOptions_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormatOptions_destroy(underlying);
});

export class ICU4XFixedDecimalFormatOptions {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static default() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8);
      wasm.ICU4XFixedDecimalFormatOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatOptions(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 8
      });
      return out;
    })();
    return diplomat_out;
  }

  get grouping_strategy() {
    return ICU4XFixedDecimalGroupingStrategy_rust_to_js[(new Int32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]];
  }

  get sign_display() {
    return ICU4XFixedDecimalSignDisplay_rust_to_js[(new Int32Array(wasm.memory.buffer, this.underlying + 4, 1))[0]];
  }
}

const ICU4XFixedDecimalFormatResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormatResult_destroy(underlying);
});

export class ICU4XFixedDecimalFormatResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get fdf() {
    return (() => {
      const out = new ICU4XFixedDecimalFormat((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
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
    let name_diplomat_bytes = (new TextEncoder()).encode(name);
    let name_diplomat_ptr = wasm.diplomat_alloc(name_diplomat_bytes.length);
    let name_diplomat_buf = new Uint8Array(wasm.memory.buffer, name_diplomat_ptr, name_diplomat_bytes.length);
    name_diplomat_buf.set(name_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_new(name_diplomat_ptr, name_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(name_diplomat_ptr, name_diplomat_bytes.length);
    return diplomat_out;
  }
}
