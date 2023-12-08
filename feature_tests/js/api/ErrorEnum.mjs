import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ErrorEnum_js_to_rust = {
  "Foo": 0,
  "Bar": 1,
};

export const ErrorEnum_rust_to_js = {
  [0]: "Foo",
  [1]: "Bar",
};

export const ErrorEnum = {
  "Foo": "Foo",
  "Bar": "Bar",
};
