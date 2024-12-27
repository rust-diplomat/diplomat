// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class ErrorEnum {
	
    #value = undefined;

    static #values = new Map([
        ["Foo", 0],
        ["Bar", 1]
    ]);

    static getAllEntries() {
        return ErrorEnum.#values.entries();
    }
    
    
    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return ErrorEnum.#objectValues[arguments[1]];
        }

        if (value instanceof ErrorEnum) {
            return value;
        }

        let intVal = ErrorEnum.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return ErrorEnum.#objectValues[intVal];
        }

        throw TypeError(value + " is not a ErrorEnum and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...ErrorEnum.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new ErrorEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new ErrorEnum(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
    ];

    static Foo = ErrorEnum.#objectValues[0];
    static Bar = ErrorEnum.#objectValues[1];

}