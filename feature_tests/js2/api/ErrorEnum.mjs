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
        if (value instanceof ErrorEnum) {
            this.#value = value.value;
            return;
        }

        if (ErrorEnum.values.has(value)) {
            this.#value = value;
            return;
        }

        throw TypeError(value + " is not a ErrorEnum and does not correspond to any of its enumerator values.");
    }

    get value() {
        return this.#value;
    }

    get ffiValue() {
        return ErrorEnum.values.get(this.#value);
    }

    static Foo = new ErrorEnum("Foo");

    static Bar = new ErrorEnum("Bar");


    

}