// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
export class ContiguousEnum {
    #value = undefined;

    static #values = new Map([
        ["C", 0],
        ["D", 1],
        ["E", 2],
        ["F", 3]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return ContiguousEnum.#objectValues[arguments[1]];
        }

        if (value instanceof ContiguousEnum) {
            return value;
        }

        let intVal = ContiguousEnum.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return ContiguousEnum.#objectValues[intVal];
        }

        throw TypeError(value + " is not a ContiguousEnum and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...ContiguousEnum.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new ContiguousEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new ContiguousEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new ContiguousEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new ContiguousEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
    ];


    static C = ContiguousEnum.#objectValues[0];
    static D = ContiguousEnum.#objectValues[1];
    static E = ContiguousEnum.#objectValues[2];
    static F = ContiguousEnum.#objectValues[3];
}