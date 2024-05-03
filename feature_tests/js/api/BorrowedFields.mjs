import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export class BorrowedFields {
  constructor(underlying, edges_a) {
    this.a = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying, 2);
      return diplomatRuntime.readString16(wasm, ptr, size);
    })();
    this.b = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 8, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
    this.c = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 16, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
  }

  static from_bar_and_strings(arg_bar, arg_dstr16, arg_utf8_str) {
    const buf_arg_dstr16 = diplomatRuntime.DiplomatBuf.str16(wasm, arg_dstr16);
    const buf_arg_utf8_str = diplomatRuntime.DiplomatBuf.str8(wasm, arg_utf8_str);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(24, 4);
      wasm.BorrowedFields_from_bar_and_strings(diplomat_receive_buffer, arg_bar.underlying, buf_arg_dstr16.ptr, buf_arg_dstr16.size, buf_arg_utf8_str.ptr, buf_arg_utf8_str.size);
      const out = new BorrowedFields(diplomat_receive_buffer, [arg_bar, buf_arg_dstr16, buf_arg_utf8_str]);
      wasm.diplomat_free(diplomat_receive_buffer, 24, 4);
      return out;
    })();
    buf_arg_dstr16.garbageCollect();
    buf_arg_utf8_str.garbageCollect();
    return diplomat_out;
  }
}
