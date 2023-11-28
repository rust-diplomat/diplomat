import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { Bar } from "./Bar.js"
import { BorrowedFieldsReturning } from "./BorrowedFieldsReturning.js"

const Foo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.Foo_destroy(underlying);
});

export class Foo {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Foo_box_destroy_registry.register(this, underlying);
    }
  }

  static new(arg_x) {
    const buf_arg_x = diplomatRuntime.DiplomatBuf.str8(wasm, arg_x);
    const diplomat_out = new Foo(wasm.Foo_new(buf_arg_x.ptr, buf_arg_x.size), true, [buf_arg_x]);
    buf_arg_x.garbageCollect();
    return diplomat_out;
  }

  get_bar() {
    return new Bar(wasm.Foo_get_bar(this.underlying), true, [this]);
  }

  static new_static(arg_x) {
    const buf_arg_x = diplomatRuntime.DiplomatBuf.str8(wasm, arg_x);
    const diplomat_out = new Foo(wasm.Foo_new_static(buf_arg_x.ptr, buf_arg_x.size), true, []);
    buf_arg_x.leak();
    return diplomat_out;
  }

  as_returning() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.Foo_as_returning(diplomat_receive_buffer, this.underlying);
      const out = new BorrowedFieldsReturning(diplomat_receive_buffer, [this]);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }

  static extract_from_fields(arg_fields) {
    const field_a_arg_fields = arg_fields["a"];
    const buf_field_a_arg_fields = diplomatRuntime.DiplomatBuf.str16(wasm, field_a_arg_fields);
    const field_b_arg_fields = arg_fields["b"];
    const buf_field_b_arg_fields = diplomatRuntime.DiplomatBuf.str8(wasm, field_b_arg_fields);
    const field_c_arg_fields = arg_fields["c"];
    const buf_field_c_arg_fields = diplomatRuntime.DiplomatBuf.str8(wasm, field_c_arg_fields);
    const diplomat_out = new Foo(wasm.Foo_extract_from_fields(buf_field_a_arg_fields.ptr, buf_field_a_arg_fields.size, buf_field_b_arg_fields.ptr, buf_field_b_arg_fields.size, buf_field_c_arg_fields.ptr, buf_field_c_arg_fields.size), true, [buf_field_a_arg_fields, buf_field_b_arg_fields, buf_field_c_arg_fields]);
    buf_field_a_arg_fields.garbageCollect();
    buf_field_b_arg_fields.garbageCollect();
    buf_field_c_arg_fields.garbageCollect();
    return diplomat_out;
  }
}
