import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

const ErrorEnum_js_to_rust = {
  "Foo": 0,
  "Bar": 1,
};
const ErrorEnum_rust_to_js = {
  0: "Foo",
  1: "Bar",
};

const ErrorStruct_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ErrorStruct_destroy(underlying);
});

export class ErrorStruct {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get i() {
    return (new Int32Array(wasm.memory.buffer, this.underlying + 0, 1))[0];
  }
}

const MyStruct_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.MyStruct_destroy(underlying);
});

export class MyStruct {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(24, 8);
      wasm.MyStruct_new(diplomat_receive_buffer);
      const out = new MyStruct(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 24,
        align: 8,
      });
      return out;
    })();
    return diplomat_out;
  }

  get a() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 0, 1))[0];
  }

  get b() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 1, 1))[0] == 1;
  }

  get c() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 2, 1))[0];
  }

  get d() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 8, 1))[0];
  }

  get e() {
    return (new Int32Array(wasm.memory.buffer, this.underlying + 16, 1))[0];
  }

  get f() {
    return String.fromCharCode((new Uint32Array(wasm.memory.buffer, this.underlying + 20, 1))[0]);
  }
}

const Opaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Opaque_destroy(underlying);
});

export class Opaque {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new Opaque(wasm.Opaque_new());
        out.owner = null;
        return out;
      })();
      Opaque_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  assert_struct(s) {
    const diplomat_MyStruct_extracted_a = s["a"];
    const diplomat_MyStruct_extracted_b = s["b"];
    const diplomat_MyStruct_extracted_c = s["c"];
    const diplomat_MyStruct_extracted_d = s["d"];
    const diplomat_MyStruct_extracted_e = s["e"];
    const diplomat_MyStruct_extracted_f = s["f"];
    if (!diplomat_MyStruct_extracted_f.length || !diplomat_MyStruct_extracted_f.codePointAt || [...diplomat_MyStruct_extracted_f].length != 1) { throw new TypeError("Expected single-character string for `char` parameter diplomat_MyStruct_extracted_f, found " + diplomat_MyStruct_extracted_f); }
    const diplomat_out = wasm.Opaque_assert_struct(this.underlying, diplomat_MyStruct_extracted_a, diplomat_MyStruct_extracted_b, diplomat_MyStruct_extracted_c, diplomat_MyStruct_extracted_d, diplomat_MyStruct_extracted_e, diplomat_MyStruct_extracted_f.codePointAt(0));
  }
}

const OptionOpaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionOpaque_destroy(underlying);
});

export class OptionOpaque {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(i) {
    const diplomat_out = (() => {
      if (wasm.OptionOpaque_new(i) !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new OptionOpaque(wasm.OptionOpaque_new(i));
            out.owner = null;
            return out;
          })();
          OptionOpaque_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
      }
    })();
    return diplomat_out;
  }

  static new_none() {
    const diplomat_out = (() => {
      if (wasm.OptionOpaque_new_none() !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new OptionOpaque(wasm.OptionOpaque_new_none());
            out.owner = null;
            return out;
          })();
          OptionOpaque_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
      }
    })();
    return diplomat_out;
  }

  static new_struct() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      if (out.a.underlying !== 0) {
        const out_a_value = out.a;
        OptionOpaque_box_destroy_registry.register(out_a_value, out_a_value.underlying);
        Object.defineProperty(out, "a", { value: out_a_value });
      } else {
        Object.defineProperty(out, "a", { value: null });
      }
      if (out.b.underlying !== 0) {
        const out_b_value = out.b;
        OptionOpaqueChar_box_destroy_registry.register(out_b_value, out_b_value.underlying);
        Object.defineProperty(out, "b", { value: out_b_value });
      } else {
        Object.defineProperty(out, "b", { value: null });
      }
      if (out.d.underlying !== 0) {
        const out_d_value = out.d;
        OptionOpaque_box_destroy_registry.register(out_d_value, out_d_value.underlying);
        Object.defineProperty(out, "d", { value: out_d_value });
      } else {
        Object.defineProperty(out, "d", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 16,
        align: 4,
      });
      return out;
    })();
    return diplomat_out;
  }

  static new_struct_nones() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct_nones(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      if (out.a.underlying !== 0) {
        const out_a_value = out.a;
        OptionOpaque_box_destroy_registry.register(out_a_value, out_a_value.underlying);
        Object.defineProperty(out, "a", { value: out_a_value });
      } else {
        Object.defineProperty(out, "a", { value: null });
      }
      if (out.b.underlying !== 0) {
        const out_b_value = out.b;
        OptionOpaqueChar_box_destroy_registry.register(out_b_value, out_b_value.underlying);
        Object.defineProperty(out, "b", { value: out_b_value });
      } else {
        Object.defineProperty(out, "b", { value: null });
      }
      if (out.d.underlying !== 0) {
        const out_d_value = out.d;
        OptionOpaque_box_destroy_registry.register(out_d_value, out_d_value.underlying);
        Object.defineProperty(out, "d", { value: out_d_value });
      } else {
        Object.defineProperty(out, "d", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 16,
        align: 4,
      });
      return out;
    })();
    return diplomat_out;
  }

  assert_integer(i) {
    const diplomat_out = wasm.OptionOpaque_assert_integer(this.underlying, i);
  }
}

const OptionOpaqueChar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionOpaqueChar_destroy(underlying);
});

export class OptionOpaqueChar {
  constructor(underlying) {
    this.underlying = underlying;
  }

  assert_char(ch) {
    if (!ch.length || !ch.codePointAt || [...ch].length != 1) { throw new TypeError("Expected single-character string for `char` parameter ch, found " + ch); }
    const diplomat_out = wasm.OptionOpaqueChar_assert_char(this.underlying, ch.codePointAt(0));
  }
}

const OptionStruct_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionStruct_destroy(underlying);
});

export class OptionStruct {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get a() {
    return (() => {
      const out = new OptionOpaque((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get b() {
    return (() => {
      const out = new OptionOpaqueChar((new Uint32Array(wasm.memory.buffer, this.underlying + 4, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get c() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 8, 1))[0];
  }

  get d() {
    return (() => {
      const out = new OptionOpaque((new Uint32Array(wasm.memory.buffer, this.underlying + 12, 1))[0]);
      out.owner = null;
      return out;
    })();
  }
}

const ResultOpaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ResultOpaque_destroy(underlying);
});

export class ResultOpaque {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_foo() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new_failing_foo(diplomat_receive_buffer);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_bar() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new_failing_bar(diplomat_receive_buffer);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_unit() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new_failing_unit(diplomat_receive_buffer);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = {};
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_struct(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new_failing_struct(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = (() => {
          const out = new ErrorStruct(diplomat_receive_buffer);
          out.owner = result_tag;
          return out;
        })();
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_in_err(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new_in_err(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = {};
        return ok_value;
      } else {
        const throw_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_in_enum_err(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ResultOpaque_new_in_enum_err(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        return ok_value;
      } else {
        const throw_value = (() => {
          const out = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  assert_integer(i) {
    const diplomat_out = wasm.ResultOpaque_assert_integer(this.underlying, i);
  }
}
