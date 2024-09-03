// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
export class ErrorEnum {
    #value = undefined;

    static values = new Map([
        ["Foo", 0],
        ["Bar", 1]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#value = arguments[1];
            return;
        }

        if (value instanceof ErrorEnum) {
            this.#value = value.value;
            return;
        }

        let intVal = ErrorEnum.values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            this.#value = intVal;
            return;
        }

        throw TypeError(value + " is not a ErrorEnum and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...ErrorEnum.values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }

    static Foo = new ErrorEnum(diplomatRuntime.internalConstructor, 0);
    static Bar = new ErrorEnum(diplomatRuntime.internalConstructor, 1);
}