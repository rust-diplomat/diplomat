import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

// Base enumerator definition
export class MyEnum {
	#value = undefined;

	static #internal_map = new Map([
		["A", -2],
		["B", -1],
		["C", 0],
		["D", 1],
		["E", 2],
		["F", 3]
	]);

	constructor(value) {
		if (value instanceof MyEnum) {
			this.#value = value.value;
			return;
		}

		if (MyEnum.#internal_map.has(value)) {
			this.#value = value;
			return;
		}

		throw TypeError(value + " is not a MyEnum and does not correspond to any of its enumerator values.");
	}

	get value() {
		return this.#value;
	}

	get ffiValue() {
		return MyEnum.#internal_map.get(this.#value);
	}

	static A = new MyEnum("A");

	static B = new MyEnum("B");

	static C = new MyEnum("C");

	static D = new MyEnum("D");

	static E = new MyEnum("E");

	static F = new MyEnum("F");
}