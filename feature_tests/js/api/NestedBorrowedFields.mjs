import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { BorrowedFields } from "./BorrowedFields.mjs"
import { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds.mjs"

export class NestedBorrowedFields {
  constructor(underlying, edges_x, edges_y, edges_z) {
    this.fields = new BorrowedFields(underlying, edges_x, edges_y);
    this.bounds = new BorrowedFieldsWithBounds(underlying + 24, edges_x, edges_y);
    this.bounds2 = new BorrowedFieldsWithBounds(underlying + 48, edges_z);
  }

  static from_bar_and_foo_and_strings(arg_bar, arg_foo, arg_dstr16_x, arg_dstr16_z, arg_utf8_str_y, arg_utf8_str_z) {
    const buf_arg_dstr16_x = diplomatRuntime.DiplomatBuf.str16(wasm, arg_dstr16_x);
    const buf_arg_dstr16_z = diplomatRuntime.DiplomatBuf.str16(wasm, arg_dstr16_z);
    const buf_arg_utf8_str_y = diplomatRuntime.DiplomatBuf.str8(wasm, arg_utf8_str_y);
    const buf_arg_utf8_str_z = diplomatRuntime.DiplomatBuf.str8(wasm, arg_utf8_str_z);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(72, 4);
      wasm.NestedBorrowedFields_from_bar_and_foo_and_strings(diplomat_receive_buffer, arg_bar.underlying, arg_foo.underlying, buf_arg_dstr16_x.ptr, buf_arg_dstr16_x.size, buf_arg_dstr16_z.ptr, buf_arg_dstr16_z.size, buf_arg_utf8_str_y.ptr, buf_arg_utf8_str_y.size, buf_arg_utf8_str_z.ptr, buf_arg_utf8_str_z.size);
      const out = new NestedBorrowedFields(diplomat_receive_buffer, [arg_bar, buf_arg_dstr16_x], [arg_bar, buf_arg_utf8_str_y], [arg_foo, buf_arg_dstr16_z, buf_arg_utf8_str_z]);
      wasm.diplomat_free(diplomat_receive_buffer, 72, 4);
      return out;
    })();
    buf_arg_dstr16_x.garbageCollect();
    buf_arg_dstr16_z.garbageCollect();
    buf_arg_utf8_str_y.garbageCollect();
    buf_arg_utf8_str_z.garbageCollect();
    return diplomat_out;
  }
}
