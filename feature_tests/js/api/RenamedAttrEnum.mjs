// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class RenamedAttrEnum {
    #value = undefined;

    static #values = new Map([
        ["A", 0],
        ["B", 1],
        ["Renamed", 2]
    ]);

    static getAllEntries() {
        return RenamedAttrEnum.#values.entries();
    }

    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return RenamedAttrEnum.#objectValues[arguments[1]];
        }

        if (value instanceof RenamedAttrEnum) {
            return value;
        }

        let intVal = RenamedAttrEnum.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return RenamedAttrEnum.#objectValues[intVal];
        }

        throw TypeError(value + " is not a RenamedAttrEnum and does not correspond to any of its enumerator values.");
    }

    static fromValue(value) {
        return new RenamedAttrEnum(value);
    }

    get value(){
        return [...RenamedAttrEnum.#values.keys()][this.#value];
    }

    get ffiValue(){
        return this.#value;
    }
    static #objectValues = [
        new RenamedAttrEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new RenamedAttrEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new RenamedAttrEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
    ];

    static A = RenamedAttrEnum.#objectValues[0];
    static B = RenamedAttrEnum.#objectValues[1];
    static Renamed = RenamedAttrEnum.#objectValues[2];

    constructor(value) {
        return this.#internalConstructor(...arguments)
    }
}