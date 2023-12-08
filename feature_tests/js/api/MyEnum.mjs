import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const MyEnum_js_to_rust = {
  "A": -2,
  "B": -1,
  "C": 0,
  "D": 1,
  "E": 2,
  "F": 3,
};

export const MyEnum_rust_to_js = {
  [-2]: "A",
  [-1]: "B",
  [0]: "C",
  [1]: "D",
  [2]: "E",
  [3]: "F",
};

export const MyEnum = {
  "A": "A",
  "B": "B",
  "C": "C",
  "D": "D",
  "E": "E",
  "F": "F",
};
