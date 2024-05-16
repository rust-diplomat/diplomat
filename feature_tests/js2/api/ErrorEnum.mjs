import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

// Internal conversion from JS types to Rust types.
export const ErrorEnum_js_to_rust = {
	"Foo": 0,
	"Bar": 1
};

export const ErrorEnum_rust_to_js = {
	[0]: "Foo",
	[1]: "Bar"
};

// Base enumerator definition
export const ErrorEnum = {
	"Foo": "Foo",
	"Bar": "Bar"
};