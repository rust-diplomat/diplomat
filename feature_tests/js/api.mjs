import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

const Bar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Bar_destroy(underlying);
});

export class Bar {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      Bar_box_destroy_registry.register(this, underlying);
    }
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
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      Float64Vec_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, 8);
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new(buf_arg_v.ptr, buf_arg_v.size), [], true);
    buf_arg_v.free();
    return diplomat_out;
  }

  fill_slice(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, 8);
    wasm.Float64Vec_fill_slice(this.underlying, buf_arg_v.ptr, buf_arg_v.size);
    buf_arg_v.free();
  }

  set_value(arg_new_slice) {
    const buf_arg_new_slice = diplomatRuntime.DiplomatBuf.slice(wasm, arg_new_slice, 8);
    wasm.Float64Vec_set_value(this.underlying, buf_arg_new_slice.ptr, buf_arg_new_slice.size);
    buf_arg_new_slice.free();
  }
}

const Foo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Foo_destroy(underlying);
});

export class Foo {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      Foo_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_x) {
    const buf_arg_x = diplomatRuntime.DiplomatBuf.str(wasm, arg_x);
    return new Foo(wasm.Foo_new(buf_arg_x.ptr, buf_arg_x.size), [buf_arg_x], true);
  }

  get_bar() {
    return new Bar(wasm.Foo_get_bar(this.underlying), [this, this], true);
  }
}

const MyString_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.MyString_destroy(underlying);
});

export class MyString {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      MyString_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.str(wasm, arg_v);
    const diplomat_out = new MyString(wasm.MyString_new(buf_arg_v.ptr, buf_arg_v.size), [], true);
    buf_arg_v.free();
    return diplomat_out;
  }

  set_str(arg_new_str) {
    const buf_arg_new_str = diplomatRuntime.DiplomatBuf.str(wasm, arg_new_str);
    wasm.MyString_set_str(this.underlying, buf_arg_new_str.ptr, buf_arg_new_str.size);
    buf_arg_new_str.free();
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
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      One_box_destroy_registry.register(this, underlying);
    }
  }

  static transitivity(arg_hold, arg_nohold) {
    return new One(wasm.One_transitivity(arg_hold.underlying, arg_nohold.underlying), [arg_hold], true);
  }

  static cycle(arg_hold, arg_nohold) {
    return new One(wasm.One_cycle(arg_hold.underlying, arg_nohold.underlying), [arg_hold, arg_hold, arg_hold], true);
  }

  static many_dependents(arg_a, arg_b, arg_c, arg_d, arg_nohold) {
    return new One(wasm.One_many_dependents(arg_a.underlying, arg_b.underlying, arg_c.underlying, arg_d.underlying, arg_nohold.underlying), [arg_a, arg_a, arg_b, arg_a, arg_c, arg_a, arg_b, arg_d], true);
  }

  static return_outlives_param(arg_hold, arg_nohold) {
    return new One(wasm.One_return_outlives_param(arg_hold.underlying, arg_nohold.underlying), [arg_hold, arg_nohold], true);
  }

  static diamond_top(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_top(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), [arg_top, arg_left, arg_right, arg_bottom, arg_top, arg_left, arg_top, arg_right, arg_top], true);
  }

  static diamond_left(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_left(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), [arg_top, arg_left, arg_right, arg_bottom, arg_top, arg_left], true);
  }

  static diamond_right(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_right(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), [arg_top, arg_left, arg_right, arg_bottom, arg_top, arg_right], true);
  }

  static diamond_bottom(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_bottom(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), [arg_top, arg_left, arg_right, arg_bottom], true);
  }

  static diamond_and_nested_types(arg_a, arg_b, arg_c, arg_d, arg_nohold) {
    return new One(wasm.One_diamond_and_nested_types(arg_a.underlying, arg_b.underlying, arg_c.underlying, arg_d.underlying, arg_nohold.underlying), [arg_a, arg_a, arg_b, arg_a, arg_b, arg_c, arg_a, arg_b, arg_c, arg_d], true);
  }
}

const Opaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Opaque_destroy(underlying);
});

export class Opaque {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      Opaque_box_destroy_registry.register(this, underlying);
    }
  }

  static new() {
    return new Opaque(wasm.Opaque_new(), [], true);
  }

  assert_struct(arg_s) {
    const field_a_arg_s = arg_s["a"];
    const field_b_arg_s = arg_s["b"];
    const field_c_arg_s = arg_s["c"];
    const field_d_arg_s = arg_s["d"];
    const field_e_arg_s = arg_s["e"];
    const field_f_arg_s = arg_s["f"];
    wasm.Opaque_assert_struct(this.underlying, field_a_arg_s, field_b_arg_s, field_c_arg_s, field_d_arg_s, field_e_arg_s, diplomatRuntime.extractCodePoint(field_f_arg_s, 'field_f_arg_s'));
  }
}

const OptionOpaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionOpaque_destroy(underlying);
});

export class OptionOpaque {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      OptionOpaque_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_i) {
    return (() => {
      const option_ptr = wasm.OptionOpaque_new(arg_i);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, [], true);
    })();
  }

  static new_none() {
    return (() => {
      const option_ptr = wasm.OptionOpaque_new_none();
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, [], true);
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

  assert_integer(arg_i) {
    wasm.OptionOpaque_assert_integer(this.underlying, arg_i);
  }
}

const OptionOpaqueChar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionOpaqueChar_destroy(underlying);
});

export class OptionOpaqueChar {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      OptionOpaqueChar_box_destroy_registry.register(this, underlying);
    }
  }

  assert_char(arg_ch) {
    wasm.OptionOpaqueChar_assert_char(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }
}

export class OptionStruct {
  constructor(underlying) {
    this.a = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, [], true);
    })();
    this.b = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying + 4);
      return (option_ptr == 0) ? null : new OptionOpaqueChar(option_ptr, [], true);
    })();
    this.c = (new Uint32Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.d = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying + 12);
      return (option_ptr == 0) ? null : new OptionOpaque(option_ptr, [], true);
    })();
  }
}

const RefList_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.RefList_destroy(underlying);
});

export class RefList {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      RefList_box_destroy_registry.register(this, underlying);
    }
  }

  static node(arg_data) {
    return new RefList(wasm.RefList_node(arg_data.underlying), [arg_data], true);
  }
}

const ResultOpaque_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ResultOpaque_destroy(underlying);
});

export class ResultOpaque {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      ResultOpaque_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new(diplomat_receive_buffer, arg_i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
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
        const ok_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
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
        const ok_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
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
        const ok_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_failing_struct(arg_i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
      wasm.ResultOpaque_new_failing_struct(diplomat_receive_buffer, arg_i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 8);
      if (is_ok) {
        const ok_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        return ok_value;
      } else {
        const throw_value = new ErrorStruct(diplomat_receive_buffer);
        wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_in_err(arg_i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_in_err(diplomat_receive_buffer, arg_i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_in_enum_err(arg_i) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ResultOpaque_new_in_enum_err(diplomat_receive_buffer, arg_i);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = ErrorEnum_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), [], true);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  assert_integer(arg_i) {
    wasm.ResultOpaque_assert_integer(this.underlying, arg_i);
  }
}

const Two_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Two_destroy(underlying);
});

export class Two {
  constructor(underlying, edges, owned) {
    this.underlying = underlying;
    this.__edges_lifetime_guard = edges;
    if (owned) {
      Two_box_destroy_registry.register(this, underlying);
    }
  }
}
