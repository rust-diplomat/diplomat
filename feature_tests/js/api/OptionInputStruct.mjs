// generated by diplomat-tool
import { OptionEnum } from "./OptionEnum.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class OptionInputStruct {
	

    #a;
    get a()  {
        return this.#a;
    }
    set a(value) {
        this.#a = value;
    }

    #b;
    get b()  {
        return this.#b;
    }
    set b(value) {
        this.#b = value;
    }

    #c;
    get c()  {
        return this.#c;
    }
    set c(value) {
        this.#c = value;
    }

    
    constructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("OptionInputStruct's constructor takes an object of OptionInputStruct's fields.");
        }

        if ("a" in structObj) {
            this.#a = structObj.a;
        } else {
            this.#a = null;
        }

        if ("b" in structObj) {
            this.#b = structObj.b;
        } else {
            this.#b = null;
        }

        if ("c" in structObj) {
            this.#c = structObj.c;
        } else {
            this.#c = null;
        }

    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [...diplomatRuntime.optionToArgsForCalling(this.#a, 1, 1, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue, Uint8Array)]), /* [2 x i8] padding */ 0, 0 /* end padding */, ...diplomatRuntime.optionToArgsForCalling(this.#b, 4, 4, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue, Uint32Array)]), ...diplomatRuntime.optionToArgsForCalling(this.#c, 4, 4, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array)])]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof OptionInputStruct) {
            return obj;
        }

        return new OptionInputStruct(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeOptionToArrayBuffer(arrayBuffer, offset + 0, this.#a, 1, 1, (arrayBuffer, offset, jsValue) => diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue, Uint8Array));
        diplomatRuntime.writeOptionToArrayBuffer(arrayBuffer, offset + 4, this.#b, 4, 4, (arrayBuffer, offset, jsValue) => diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue, Uint32Array));
        diplomatRuntime.writeOptionToArrayBuffer(arrayBuffer, offset + 12, this.#c, 4, 4, (arrayBuffer, offset, jsValue) => diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array));
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("OptionInputStruct._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const aDeref = ptr;
        structObj.a = diplomatRuntime.readOption(wasm, aDeref, 1, (wasm, offset) => { const deref = (new Uint8Array(wasm.memory.buffer, offset, 1))[0]; return deref });
        const bDeref = ptr + 4;
        structObj.b = diplomatRuntime.readOption(wasm, bDeref, 4, (wasm, offset) => { const deref = (new Uint32Array(wasm.memory.buffer, offset, 1))[0]; return deref });
        const cDeref = ptr + 12;
        structObj.c = diplomatRuntime.readOption(wasm, cDeref, 4, (wasm, offset) => { const deref = diplomatRuntime.enumDiscriminant(wasm, offset); return new OptionEnum(diplomatRuntime.internalConstructor, deref) });

        return new OptionInputStruct(structObj, internalConstructor);
    }

}