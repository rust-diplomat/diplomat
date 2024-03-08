import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Float64Vec_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Float64Vec_destroy(underlying);
});

export class Float64Vec {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Float64Vec_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "f64");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_bool(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "bool");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new_bool(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_i16(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "i16");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new_i16(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_u16(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "u16");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new_u16(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_isize(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "isize");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new_isize(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_usize(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "usize");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new_usize(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  static new_f64_be_bytes(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "u8");
    const diplomat_out = new Float64Vec(wasm.Float64Vec_new_f64_be_bytes(buf_arg_v.ptr, buf_arg_v.size), true, []);
    buf_arg_v.free();
    return diplomat_out;
  }

  fill_slice(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.slice(wasm, arg_v, "f64");
    wasm.Float64Vec_fill_slice(this.underlying, buf_arg_v.ptr, buf_arg_v.size);
    buf_arg_v.free();
  }

  set_value(arg_new_slice) {
    const buf_arg_new_slice = diplomatRuntime.DiplomatBuf.slice(wasm, arg_new_slice, "f64");
    wasm.Float64Vec_set_value(this.underlying, buf_arg_new_slice.ptr, buf_arg_new_slice.size);
    buf_arg_new_slice.free();
  }

  to_string() {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.Float64Vec_to_string(this.underlying, writeable);
    });
  }

  borrow() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.Float64Vec_borrow(diplomat_receive_buffer, this.underlying);
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 2);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return Float64Array.from(new Float64Array(wasm.memory.buffer, ptr, size));
    })();
  }
}
