import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

export class Alpha {
  constructor(underlying) {
    this.x = (new Uint32Array(wasm.memory.buffer, underlying + 0, 1))[0];
    this.y = (new Uint32Array(wasm.memory.buffer, underlying + 4, 1))[0];
  }
}

const Bar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Bar_destroy(underlying);
});

export class Bar {
  constructor(underlying) {
    this.underlying = underlying;
    Bar_box_destroy_registry.register(this, underlying);
  }
}

export class Beta {
  constructor(underlying) {
    this.alpha_field = new Alpha(underlying + 0);
  }

  static new(x, y) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.Beta_new(diplomat_receive_buffer, x, y);
      const out = new Beta(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
    return diplomat_out;
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

export class ErrorStruct {
  constructor(underlying) {
    this.i = (new Int32Array(wasm.memory.buffer, underlying + 0, 1))[0];
  }
}

const Float64Vec_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Float64Vec_destroy(underlying);
});

export class Float64Vec {
  constructor(underlying) {
    this.underlying = underlying;
    Float64Vec_box_destroy_registry.register(this, underlying);
  }

  static new(v) {
    let v_diplomat_slice = diplomatRuntime.DiplomatBuf.slice(wasm, v, 8);
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new(v_diplomat_slice.ptr, v_diplomat_slice.size));
    v_diplomat_slice.free();
    return diplomat_out;
  }

  fill_slice(v) {
    let v_diplomat_slice = diplomatRuntime.DiplomatBuf.slice(wasm, v, 8);
    const diplomat_out = wasm.Float64Vec_fill_slice(this.underlying, v_diplomat_slice.ptr, v_diplomat_slice.size);
    v_diplomat_slice.free();
  }

  set_value(new_slice) {
    let new_slice_diplomat_slice = diplomatRuntime.DiplomatBuf.slice(wasm, new_slice, 8);
    const diplomat_out = wasm.Float64Vec_set_value(this.underlying, new_slice_diplomat_slice.ptr, new_slice_diplomat_slice.size);
    new_slice_diplomat_slice.free();
  }
}

const Foo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Foo_destroy(underlying);
});

export class Foo {
  constructor(underlying) {
    this.underlying = underlying;
    Foo_box_destroy_registry.register(this, underlying);
  }

  static new(x) {
    let x_diplomat_str = diplomatRuntime.DiplomatBuf.str(wasm, x);
    const diplomat_out = new Foo(wasm.Foo_new(x_diplomat_str.ptr, x_diplomat_str.size));
    diplomat_out.__x_lifetime_guard = x_diplomat_str;
    return diplomat_out;
  }

  get_bar() {
    const diplomat_out = (() => {
      const out = new Bar(wasm.Foo_get_bar(this.underlying));
      out.__this_lifetime_guard = this;
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
    MyString_box_destroy_registry.register(this, underlying);
  }

  static new(v) {
    let v_diplomat_str = diplomatRuntime.DiplomatBuf.str(wasm, v);
    const diplomat_out = new MyString(wasm.MyString_new(v_diplomat_str.ptr, v_diplomat_str.size));
    v_diplomat_str.free();
    return diplomat_out;
  }

  set_str(new_str) {
    let new_str_diplomat_str = diplomatRuntime.DiplomatBuf.str(wasm, new_str);
    const diplomat_out = wasm.MyString_set_str(this.underlying, new_str_diplomat_str.ptr, new_str_diplomat_str.size);
    new_str_diplomat_str.free();
  }

  get_str() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.MyString_get_str(this.underlying, writeable);
    });
    return diplomat_out;
  }
}

export class MyStruct {
  constructor(underlying) {
    this.a = (new Uint8Array(wasm.memory.buffer, underlying + 0, 1))[0];
    this.b = (new Uint8Array(wasm.memory.buffer, underlying + 1, 1))[0] == 1;
    this.c = (new Uint8Array(wasm.memory.buffer, underlying + 2, 1))[0];
    this.d = (new BigUint64Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.e = (new Int32Array(wasm.memory.buffer, underlying + 16, 1))[0];
    this.f = String.fromCharCode((new Uint32Array(wasm.memory.buffer, underlying + 20, 1))[0]);
  }

  static new() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(24, 8);
      wasm.MyStruct_new(diplomat_receive_buffer);
      const out = new MyStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 24, 8);
      return out;
    })();
    return diplomat_out;
  }

  consume() {
    const diplomat_MyStruct_extracted_a = this["a"];
    const diplomat_MyStruct_extracted_b = this["b"];
    const diplomat_MyStruct_extracted_c = this["c"];
    const diplomat_MyStruct_extracted_d = this["d"];
    const diplomat_MyStruct_extracted_e = this["e"];
    const diplomat_MyStruct_extracted_f = this["f"];
    const diplomat_out = wasm.MyStruct_consume(diplomat_MyStruct_extracted_a, diplomat_MyStruct_extracted_b, diplomat_MyStruct_extracted_c, diplomat_MyStruct_extracted_d, diplomat_MyStruct_extracted_e, diplomatRuntime.extractCodePoint(diplomat_MyStruct_extracted_f, 'diplomat_MyStruct_extracted_f'));
  }
}

const One_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.One_destroy(underlying);
});

export class One {
  constructor(underlying) {
    this.underlying = underlying;
    One_box_destroy_registry.register(this, underlying);
  }

  static transitivity(hold, nohold) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_transitivity(hold.underlying, nohold.underlying));
      out.__hold_lifetime_guard = hold;
      return out;
    })();
    return diplomat_out;
  }

  static cycle(hold, nohold) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_cycle(hold.underlying, nohold.underlying));
      out.__hold_lifetime_guard = hold;
      return out;
    })();
    return diplomat_out;
  }

  static many_dependents(a, b, c, d, nohold) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_many_dependents(a.underlying, b.underlying, c.underlying, d.underlying, nohold.underlying));
      out.__a_lifetime_guard = a;
      out.__b_lifetime_guard = b;
      out.__c_lifetime_guard = c;
      out.__d_lifetime_guard = d;
      return out;
    })();
    return diplomat_out;
  }

  static return_outlives_param(hold, nohold) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_return_outlives_param(hold.underlying, nohold.underlying));
      out.__hold_lifetime_guard = hold;
      return out;
    })();
    return diplomat_out;
  }

  static diamond_top(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_diamond_top(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__top_lifetime_guard = top;
      out.__left_lifetime_guard = left;
      out.__right_lifetime_guard = right;
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
    return diplomat_out;
  }

  static diamond_left(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_diamond_left(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__left_lifetime_guard = left;
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
    return diplomat_out;
  }

  static diamond_right(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_diamond_right(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__right_lifetime_guard = right;
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
    return diplomat_out;
  }

  static diamond_bottom(top, left, right, bottom) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_diamond_bottom(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
    return diplomat_out;
  }

  static diamond_and_nested_types(a, b, c, d, nohold) {
    const diplomat_out = (() => {
      const out = new One(wasm.One_diamond_and_nested_types(a.underlying, b.underlying, c.underlying, d.underlying, nohold.underlying));
      out.__a_lifetime_guard = a;
      out.__b_lifetime_guard = b;
      out.__c_lifetime_guard = c;
      out.__d_lifetime_guard = d;
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
    Opaque_box_destroy_registry.register(this, underlying);
  }

  static new() {
    const diplomat_out = new Opaque(wasm.Opaque_new());
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
    OptionOpaque_box_destroy_registry.register(this, underlying);
  }

  static new(i) {
    const diplomat_out = (() => {
      const option_ptr = (new Uint32Array(wasm.memory.buffer, wasm.OptionOpaque_new(i), 1))[0];
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
    return diplomat_out;
  }

  static new_none() {
    const diplomat_out = (() => {
      const option_ptr = (new Uint32Array(wasm.memory.buffer, wasm.OptionOpaque_new_none(), 1))[0];
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
    return diplomat_out;
  }

  static new_struct() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 16, 4);
      return out;
    })();
    return diplomat_out;
  }

  static new_struct_nones() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct_nones(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 16, 4);
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
    OptionOpaqueChar_box_destroy_registry.register(this, underlying);
  }

  assert_char(ch) {
    const diplomat_out = wasm.OptionOpaqueChar_assert_char(this.underlying, diplomatRuntime.extractCodePoint(ch, 'ch'));
  }
}

export class OptionStruct {
  constructor(underlying) {
    this.a = (() => {
      const option_ptr = (new Uint32Array(wasm.memory.buffer, underlying + 0, 1))[0];
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
    this.b = (() => {
      const option_ptr = (new Uint32Array(wasm.memory.buffer, underlying + 4, 1))[0];
      return (option_ptr == 0) ? null : new OptionOpaqueChar(option_ptr);
    })();
    this.c = (new Uint32Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.d = (() => {
      const option_ptr = (new Uint32Array(wasm.memory.buffer, underlying + 12, 1))[0];
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
  }
}

const RefList_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.RefList_destroy(underlying);
});

export class RefList {
  constructor(underlying) {
    this.underlying = underlying;
    RefList_box_destroy_registry.register(this, underlying);
  }

  static node(data) {
    const diplomat_out = (() => {
      const out = new RefList(wasm.RefList_node(data.underlying));
      out.__data_lifetime_guard = data;
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
    ResultOpaque_box_destroy_registry.register(this, underlying);
  }

  static new(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_foo() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_foo(diplomat_receive_buffer);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_bar() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_bar(diplomat_receive_buffer);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_unit() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_unit(diplomat_receive_buffer);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_failing_struct(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_struct(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = new ErrorStruct(diplomat_receive_buffer);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_in_err(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_in_err(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static new_in_enum_err(i) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_in_enum_err(diplomat_receive_buffer, i);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = ErrorEnum_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = new ResultOpaque((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
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
    Two_box_destroy_registry.register(this, underlying);
  }
}
