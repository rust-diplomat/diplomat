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
    Bar_box_destroy_registry.register(this, underlying);
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
    this.i = (new Int32Array(wasm.memory.buffer, underlying, 1))[0];
    this.j = (new Int32Array(wasm.memory.buffer, underlying + 4, 1))[0];
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
    let v_diplomat_bytes = new Uint8Array(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 8);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new(v_diplomat_ptr, v_diplomat_bytes.length));
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 8);
    return diplomat_out;
  }

  fill_slice(v) {
    let v_diplomat_bytes = new Uint8Array(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 8);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    wasm.Float64Vec_fill_slice(this.underlying, v_diplomat_ptr, v_diplomat_bytes.length);
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 8);
  }

  set_value(new_slice) {
    let new_slice_diplomat_bytes = new Uint8Array(new_slice);
    let new_slice_diplomat_ptr = wasm.diplomat_alloc(new_slice_diplomat_bytes.length, 8);
    let new_slice_diplomat_buf = new Uint8Array(wasm.memory.buffer, new_slice_diplomat_ptr, new_slice_diplomat_bytes.length);
    new_slice_diplomat_buf.set(new_slice_diplomat_bytes, 0);
    wasm.Float64Vec_set_value(this.underlying, new_slice_diplomat_ptr, new_slice_diplomat_bytes.length);
    wasm.diplomat_free(new_slice_diplomat_ptr, new_slice_diplomat_bytes.length, 8);
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
    let x_diplomat_bytes = (new TextEncoder()).encode(x);
    let x_diplomat_ptr = wasm.diplomat_alloc(x_diplomat_bytes.length, 1);
    let x_diplomat_buf = new Uint8Array(wasm.memory.buffer, x_diplomat_ptr, x_diplomat_bytes.length);
    x_diplomat_buf.set(x_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = new Foo(wasm.Foo_new(x_diplomat_ptr, x_diplomat_bytes.length));
      out.__x_lifetime_guard = x;
      return out;
    })();
    wasm.diplomat_free(x_diplomat_ptr, x_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  get_bar() {
    return (() => {
      const out = new Bar(wasm.Foo_get_bar(this.underlying));
      out.__this_lifetime_guard = this;
      return out;
    })();
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
    let v_diplomat_bytes = (new TextEncoder()).encode(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 1);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = new MyString(wasm.MyString_new(v_diplomat_ptr, v_diplomat_bytes.length));
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  set_str(new_str) {
    let new_str_diplomat_bytes = (new TextEncoder()).encode(new_str);
    let new_str_diplomat_ptr = wasm.diplomat_alloc(new_str_diplomat_bytes.length, 1);
    let new_str_diplomat_buf = new Uint8Array(wasm.memory.buffer, new_str_diplomat_ptr, new_str_diplomat_bytes.length);
    new_str_diplomat_buf.set(new_str_diplomat_bytes, 0);
    wasm.MyString_set_str(this.underlying, new_str_diplomat_ptr, new_str_diplomat_bytes.length);
    wasm.diplomat_free(new_str_diplomat_ptr, new_str_diplomat_bytes.length, 1);
  }

  get_str() {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.MyString_get_str(this.underlying, writeable);
    });
  }
}

export class MyStruct {
  constructor(underlying) {
    this.a = (new Uint8Array(wasm.memory.buffer, underlying, 1))[0];
    this.b = (new Uint8Array(wasm.memory.buffer, underlying + 1, 1))[0] == 1;
    this.c = (new Uint8Array(wasm.memory.buffer, underlying + 2, 1))[0];
    this.d = (new BigUint64Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.e = (new Int32Array(wasm.memory.buffer, underlying + 16, 1))[0];
    this.f = String.fromCharCode((new Uint32Array(wasm.memory.buffer, underlying + 20, 1))[0]);
  }

  static new() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(24, 8);
      wasm.MyStruct_new(diplomat_receive_buffer);
      const out = new MyStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 24, 8);
      return out;
    })();
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
    return (() => {
      const out = new One(wasm.One_transitivity(hold.underlying, nohold.underlying));
      out.__hold_lifetime_guard = hold;
      return out;
    })();
  }

  static cycle(hold, nohold) {
    return (() => {
      const out = new One(wasm.One_cycle(hold.underlying, nohold.underlying));
      out.__hold_lifetime_guard = hold;
      return out;
    })();
  }

  static many_dependents(a, b, c, d, nohold) {
    return (() => {
      const out = new One(wasm.One_many_dependents(a.underlying, b.underlying, c.underlying, d.underlying, nohold.underlying));
      out.__a_lifetime_guard = a;
      out.__b_lifetime_guard = b;
      out.__c_lifetime_guard = c;
      out.__d_lifetime_guard = d;
      return out;
    })();
  }

  static return_outlives_param(hold, nohold) {
    return (() => {
      const out = new One(wasm.One_return_outlives_param(hold.underlying, nohold.underlying));
      out.__hold_lifetime_guard = hold;
      return out;
    })();
  }

  static diamond_top(top, left, right, bottom) {
    return (() => {
      const out = new One(wasm.One_diamond_top(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__top_lifetime_guard = top;
      out.__left_lifetime_guard = left;
      out.__right_lifetime_guard = right;
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
  }

  static diamond_left(top, left, right, bottom) {
    return (() => {
      const out = new One(wasm.One_diamond_left(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__left_lifetime_guard = left;
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
  }

  static diamond_right(top, left, right, bottom) {
    return (() => {
      const out = new One(wasm.One_diamond_right(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__right_lifetime_guard = right;
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
  }

  static diamond_bottom(top, left, right, bottom) {
    return (() => {
      const out = new One(wasm.One_diamond_bottom(top.underlying, left.underlying, right.underlying, bottom.underlying));
      out.__bottom_lifetime_guard = bottom;
      return out;
    })();
  }

  static diamond_and_nested_types(a, b, c, d, nohold) {
    return (() => {
      const out = new One(wasm.One_diamond_and_nested_types(a.underlying, b.underlying, c.underlying, d.underlying, nohold.underlying));
      out.__a_lifetime_guard = a;
      out.__b_lifetime_guard = b;
      out.__c_lifetime_guard = c;
      out.__d_lifetime_guard = d;
      return out;
    })();
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
    return new Opaque(wasm.Opaque_new());
  }

  assert_struct(s) {
    const diplomat_MyStruct_extracted_a = s["a"];
    const diplomat_MyStruct_extracted_b = s["b"];
    const diplomat_MyStruct_extracted_c = s["c"];
    const diplomat_MyStruct_extracted_d = s["d"];
    const diplomat_MyStruct_extracted_e = s["e"];
    const diplomat_MyStruct_extracted_f = s["f"];
    wasm.Opaque_assert_struct(this.underlying, diplomat_MyStruct_extracted_a, diplomat_MyStruct_extracted_b, diplomat_MyStruct_extracted_c, diplomat_MyStruct_extracted_d, diplomat_MyStruct_extracted_e, diplomatRuntime.extractCodePoint(diplomat_MyStruct_extracted_f, 'diplomat_MyStruct_extracted_f'));
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
    return (() => {
      const option_ptr = wasm.OptionOpaque_new(i);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
  }

  static new_none() {
    return (() => {
      const option_ptr = wasm.OptionOpaque_new_none();
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
  }

  static new_struct() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 16, 4);
      return out;
    })();
  }

  static new_struct_nones() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(16, 4);
      wasm.OptionOpaque_new_struct_nones(diplomat_receive_buffer);
      const out = new OptionStruct(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 16, 4);
      return out;
    })();
  }

  assert_integer(i) {
    wasm.OptionOpaque_assert_integer(this.underlying, i);
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
    wasm.OptionOpaqueChar_assert_char(this.underlying, diplomatRuntime.extractCodePoint(ch, 'ch'));
  }
}

export class OptionStruct {
  constructor(underlying) {
    this.a = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr);
    })();
    this.b = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying + 4);
      return (option_ptr == 0) ? null : new OptionOpaqueChar(option_ptr);
    })();
    this.c = (new Uint32Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.d = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying + 12);
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
    return (() => {
      const out = new RefList(wasm.RefList_node(data.underlying));
      out.__data_lifetime_guard = data;
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
    ResultOpaque_box_destroy_registry.register(this, underlying);
  }

  static new(i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new(diplomat_receive_buffer, i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const out = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return out;
      } else {
        const throw_value = ErrorEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_failing_foo() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_foo(diplomat_receive_buffer);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const out = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return out;
      } else {
        const throw_value = ErrorEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_failing_bar() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_bar(diplomat_receive_buffer);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const out = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return out;
      } else {
        const throw_value = ErrorEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_failing_unit() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_failing_unit(diplomat_receive_buffer);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const out = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return out;
      } else {
        const throw_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_failing_struct(i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
      wasm.ResultOpaque_new_failing_struct(diplomat_receive_buffer, i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 8);
      if (is_ok) {
        const out = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        return out;
      } else {
        const throw_value = new ErrorStruct(diplomat_receive_buffer);
        wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_in_err(i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_in_err(diplomat_receive_buffer, i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const out = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return out;
      } else {
        const throw_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_in_enum_err(i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_in_enum_err(diplomat_receive_buffer, i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const out = ErrorEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return out;
      } else {
        const throw_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer));
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  assert_integer(i) {
    wasm.ResultOpaque_assert_integer(this.underlying, i);
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
