// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class DefaultEnum {
    #value = undefined;

    static #values = new Map([
        ["A", 0],
        ["B", 1]
    ]);

    static getAllEntries() {
        return DefaultEnum.#values.entries();
    }

    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return DefaultEnum.#objectValues[arguments[1]];
        }

        if (value instanceof DefaultEnum) {
            return value;
        }

        let intVal = DefaultEnum.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return DefaultEnum.#objectValues[intVal];
        }

        throw TypeError(value + " is not a DefaultEnum and does not correspond to any of its enumerator values.");
    }

    static fromValue(value) {
        return new DefaultEnum(diplomatRuntime.exposeConstructor, value);
    }

    get value(){
        return [...DefaultEnum.#values.keys()][this.#value];
    }

    get ffiValue(){
        return this.#value;
    }
    static #objectValues = [
        new DefaultEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new DefaultEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
    ];

    static A = DefaultEnum.#objectValues[0];
    static B = DefaultEnum.#objectValues[1];


    #defaultConstructor() {

        const result = wasm.DefaultEnum_new();

        try {
            return new DefaultEnum(diplomatRuntime.internalConstructor, result);
        }

        finally {}
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}