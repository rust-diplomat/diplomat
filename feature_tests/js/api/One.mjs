import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const One_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.One_destroy(underlying);
});

export class One {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      One_box_destroy_registry.register(this, underlying);
    }
  }

  static transitivity(arg_hold, arg_nohold) {
    return new One(wasm.One_transitivity(arg_hold.underlying, arg_nohold.underlying), true, [arg_hold]);
  }

  static cycle(arg_hold, arg_nohold) {
    return new One(wasm.One_cycle(arg_hold.underlying, arg_nohold.underlying), true, [arg_hold]);
  }

  static many_dependents(arg_a, arg_b, arg_c, arg_d, arg_nohold) {
    return new One(wasm.One_many_dependents(arg_a.underlying, arg_b.underlying, arg_c.underlying, arg_d.underlying, arg_nohold.underlying), true, [arg_a, arg_b, arg_c, arg_d]);
  }

  static return_outlives_param(arg_hold, arg_nohold) {
    return new One(wasm.One_return_outlives_param(arg_hold.underlying, arg_nohold.underlying), true, [arg_hold]);
  }

  static diamond_top(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_top(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), true, [arg_bottom, arg_left, arg_right, arg_top]);
  }

  static diamond_left(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_left(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), true, [arg_bottom, arg_left]);
  }

  static diamond_right(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_right(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), true, [arg_bottom, arg_right]);
  }

  static diamond_bottom(arg_top, arg_left, arg_right, arg_bottom) {
    return new One(wasm.One_diamond_bottom(arg_top.underlying, arg_left.underlying, arg_right.underlying, arg_bottom.underlying), true, [arg_bottom]);
  }

  static diamond_and_nested_types(arg_a, arg_b, arg_c, arg_d, arg_nohold) {
    return new One(wasm.One_diamond_and_nested_types(arg_a.underlying, arg_b.underlying, arg_c.underlying, arg_d.underlying, arg_nohold.underlying), true, [arg_a, arg_b, arg_c, arg_d]);
  }

  static implicit_bounds(arg_explicit_hold, arg_implicit_hold, arg_nohold) {
    return new One(wasm.One_implicit_bounds(arg_explicit_hold.underlying, arg_implicit_hold.underlying, arg_nohold.underlying), true, [arg_explicit_hold, arg_implicit_hold]);
  }

  static implicit_bounds_deep(arg_explicit_, arg_implicit_1, arg_implicit_2, arg_nohold) {
    return new One(wasm.One_implicit_bounds_deep(arg_explicit_.underlying, arg_implicit_1.underlying, arg_implicit_2.underlying, arg_nohold.underlying), true, [arg_explicit_, arg_implicit_1, arg_implicit_2]);
  }
}
