import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

const Bar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Bar_destroy(underlying);
});

export class Bar {
  constructor(underlying) {
    this.underlying = underlying;
  }
}

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

const Float64Vec_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Float64Vec_destroy(underlying);
});

export class Float64Vec {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(v) {
    let v_diplomat_bytes = new Uint8Array(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 8);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new Float64Vec(wasm.Float64Vec_new(v_diplomat_ptr, v_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      Float64Vec_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 8);
    return diplomat_out;
  }

  fill_slice(v) {
    let v_diplomat_bytes = new Uint8Array(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 8);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = wasm.Float64Vec_fill_slice(this.underlying, v_diplomat_ptr, v_diplomat_bytes.length);
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 8);
  }

  set_value(new_slice) {
    let new_slice_diplomat_bytes = new Uint8Array(new_slice);
    let new_slice_diplomat_ptr = wasm.diplomat_alloc(new_slice_diplomat_bytes.length, 8);
    let new_slice_diplomat_buf = new Uint8Array(wasm.memory.buffer, new_slice_diplomat_ptr, new_slice_diplomat_bytes.length);
    new_slice_diplomat_buf.set(new_slice_diplomat_bytes, 0);
    const diplomat_out = wasm.Float64Vec_set_value(this.underlying, new_slice_diplomat_ptr, new_slice_diplomat_bytes.length);
    wasm.diplomat_free(new_slice_diplomat_ptr, new_slice_diplomat_bytes.length, 8);
  }
}

const Foo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Foo_destroy(underlying);
});

export class Foo {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(x) {
    let x_diplomat_bytes = (new TextEncoder()).encode(x);
    let x_diplomat_ptr = wasm.diplomat_alloc(x_diplomat_bytes.length, 1);
    let x_diplomat_buf = new Uint8Array(wasm.memory.buffer, x_diplomat_ptr, x_diplomat_bytes.length);
    x_diplomat_buf.set(x_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new Foo(wasm.Foo_new(x_diplomat_ptr, x_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      Foo_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(x_diplomat_ptr, x_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  get_bar() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new Bar(wasm.Foo_get_bar(this.underlying));
        out.owner = null;
        out.__this_lifetime_guard = this;
        return out;
      })();
      Bar_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }
}

const MyString_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.MyString_destroy(underlying);
});

export class MyString {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static new(v) {
    let v_diplomat_bytes = (new TextEncoder()).encode(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 1);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new MyString(wasm.MyString_new(v_diplomat_ptr, v_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      MyString_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  set_str(new_str) {
    let new_str_diplomat_bytes = (new TextEncoder()).encode(new_str);
    let new_str_diplomat_ptr = wasm.diplomat_alloc(new_str_diplomat_bytes.length, 1);
    let new_str_diplomat_buf = new Uint8Array(wasm.memory.buffer, new_str_diplomat_ptr, new_str_diplomat_bytes.length);
    new_str_diplomat_buf.set(new_str_diplomat_bytes, 0);
    const diplomat_out = wasm.MyString_set_str(this.underlying, new_str_diplomat_ptr, new_str_diplomat_bytes.length);
    wasm.diplomat_free(new_str_diplomat_ptr, new_str_diplomat_bytes.length, 1);
  }

  get_str() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.MyString_get_str(this.underlying, writeable);
    });
    return diplomat_out;
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

const One_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.One_destroy(underlying);
});

export class One {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static transitivity(hold, nohold) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_transitivity(hold.underlying, nohold.underlying));
        out.owner = null;
        out.__hold_lifetime_guard = hold;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static cycle(hold, nohold) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_cycle(hold.underlying, nohold.underlying));
        out.owner = null;
        out.__hold_lifetime_guard = hold;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static many_dependents(a, b, c, d, nohold) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_many_dependents(a.underlying, b.underlying, c.underlying, d.underlying, nohold.underlying));
        out.owner = null;
        out.__a_lifetime_guard = a;
        out.__b_lifetime_guard = b;
        out.__c_lifetime_guard = c;
        out.__d_lifetime_guard = d;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static return_outlives_param(hold, nohold) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_return_outlives_param(hold.underlying, nohold.underlying));
        out.owner = null;
        out.__hold_lifetime_guard = hold;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static diamond_top(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_diamond_top(top.underlying, left.underlying, right.underlying, bottom.underlying));
        out.owner = null;
        out.__top_lifetime_guard = top;
        out.__left_lifetime_guard = left;
        out.__right_lifetime_guard = right;
        out.__bottom_lifetime_guard = bottom;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static diamond_left(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_diamond_left(top.underlying, left.underlying, right.underlying, bottom.underlying));
        out.owner = null;
        out.__left_lifetime_guard = left;
        out.__bottom_lifetime_guard = bottom;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static diamond_right(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_diamond_right(top.underlying, left.underlying, right.underlying, bottom.underlying));
        out.owner = null;
        out.__right_lifetime_guard = right;
        out.__bottom_lifetime_guard = bottom;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static diamond_bottom(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_diamond_bottom(top.underlying, left.underlying, right.underlying, bottom.underlying));
        out.owner = null;
        out.__bottom_lifetime_guard = bottom;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static diamond_and_nested_types(a, b, c, d, nohold) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new One(wasm.One_diamond_and_nested_types(a.underlying, b.underlying, c.underlying, d.underlying, nohold.underlying));
        out.owner = null;
        out.__a_lifetime_guard = a;
        out.__b_lifetime_guard = b;
        out.__c_lifetime_guard = c;
        out.__d_lifetime_guard = d;
        return out;
      })();
      One_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
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
    const diplomat_out = wasm.Opaque_assert_struct(this.underlying, diplomat_MyStruct_extracted_a, diplomat_MyStruct_extracted_b, diplomat_MyStruct_extracted_c, diplomat_MyStruct_extracted_d, diplomat_MyStruct_extracted_e, diplomatRuntime.extractCodePoint(diplomat_MyStruct_extracted_f, 'diplomat_MyStruct_extracted_f'));
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
      const option_value = wasm.OptionOpaque_new(i)
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new OptionOpaque(option_value);
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
      const option_value = wasm.OptionOpaque_new_none()
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new OptionOpaque(option_value);
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
    const diplomat_out = wasm.OptionOpaqueChar_assert_char(this.underlying, diplomatRuntime.extractCodePoint(ch, 'ch'));
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

const RefList_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.RefList_destroy(underlying);
});

export class RefList {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static node(data) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new RefList(wasm.RefList_node(data.underlying));
        out.owner = null;
        out.__data_lifetime_guard = data;
        return out;
      })();
      RefList_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
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

const Two_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Two_destroy(underlying);
});

export class Two {
  constructor(underlying) {
    this.underlying = underlying;
  }
}
