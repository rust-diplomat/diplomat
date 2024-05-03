import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export class BorrowedFieldsWithBounds {
  constructor(underlying, edges_a, edges_b, edges_c) {
    this.field_a = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return diplomatRuntime.readString16(wasm, ptr, size);
    })();
    this.field_b = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 8, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
    this.field_c = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 16, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
  }

  static from_foo_and_strings(arg_foo, arg_dstr16_x, arg_utf8_str_z) {
    const buf_arg_dstr16_x = diplomatRuntime.DiplomatBuf.str16(wasm, arg_dstr16_x);
    const buf_arg_utf8_str_z = diplomatRuntime.DiplomatBuf.str8(wasm, arg_utf8_str_z);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(24, 4);
      wasm.BorrowedFieldsWithBounds_from_foo_and_strings(diplomat_receive_buffer, arg_foo.underlying, buf_arg_dstr16_x.ptr, buf_arg_dstr16_x.size, buf_arg_utf8_str_z.ptr, buf_arg_utf8_str_z.size);
      const out = new BorrowedFieldsWithBounds(diplomat_receive_buffer, [arg_foo, buf_arg_dstr16_x], [arg_foo], [buf_arg_utf8_str_z]);
      wasm.diplomat_free(diplomat_receive_buffer, 24, 4);
      return out;
    })();
    buf_arg_dstr16_x.garbageCollect();
    buf_arg_utf8_str_z.garbageCollect();
    return diplomat_out;
  }
}
