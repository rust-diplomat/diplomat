import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const OptionOpaqueChar_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.OptionOpaqueChar_destroy(underlying);
});

export class OptionOpaqueChar {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      OptionOpaqueChar_box_destroy_registry.register(this, underlying);
    }
  }

  assert_char(arg_ch) {
    wasm.OptionOpaqueChar_assert_char(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }
}
