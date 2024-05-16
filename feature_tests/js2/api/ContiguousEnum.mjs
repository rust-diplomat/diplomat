import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

// Internal conversion from JS types to Rust types.
export const ContiguousEnum_js_to_rust = {
	"C": 0,
	"D": 1,
	"E": 2,
	"F": 3
};

export const ContiguousEnum_rust_to_js = {
	[0]: "C",
	[1]: "D",
	[2]: "E",
	[3]: "F"
};

// Base enumerator definition
export const ContiguousEnum = {
	"C": "C",
	"D": "D",
	"E": "E",
	"F": "F"
};