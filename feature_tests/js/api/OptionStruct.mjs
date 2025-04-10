// generated by diplomat-tool
import { OptionOpaque } from "./OptionOpaque.mjs"
import { OptionOpaqueChar } from "./OptionOpaqueChar.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class OptionStruct {
    #a;
    get a() {
        return this.#a;
    }
    #b;
    get b() {
        return this.#b;
    }
    #c;
    get c() {
        return this.#c;
    }
    #d;
    get d() {
        return this.#d;
    }
    #internalConstructor(structObj, internalConstructor) {
        if (typeof structObj !== "object") {
            throw new Error("OptionStruct's constructor takes an object of OptionStruct's fields.");
        }

        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("OptionStruct is an out struct and can only be created internally.");
        }
        if ("a" in structObj) {
            this.#a = structObj.a;
        } else {
            throw new Error("Missing required field a.");
        }

        if ("b" in structObj) {
            this.#b = structObj.b;
        } else {
            throw new Error("Missing required field b.");
        }

        if ("c" in structObj) {
            this.#c = structObj.c;
        } else {
            throw new Error("Missing required field c.");
        }

        if ("d" in structObj) {
            this.#d = structObj.d;
        } else {
            throw new Error("Missing required field d.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        let buffer = diplomatRuntime.DiplomatBuf.struct(wasm, 16, 4);

        this._writeToArrayBuffer(wasm.memory.buffer, buffer.ptr, functionCleanupArena, appendArrayMap);
        
        functionCleanupArena.alloc(buffer);

        return buffer.ptr;
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof OptionStruct) {
            return obj;
        }

        return OptionStruct.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#a.ffiValue ?? 0, Uint32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#b.ffiValue ?? 0, Uint32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 8, this.#c, Uint32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 12, this.#d.ffiValue, Uint32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("OptionStruct._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const aDeref = diplomatRuntime.ptrRead(wasm, ptr);
        structObj.a = aDeref === 0 ? null : new OptionOpaque(diplomatRuntime.internalConstructor, aDeref, []);
        const bDeref = diplomatRuntime.ptrRead(wasm, ptr + 4);
        structObj.b = bDeref === 0 ? null : new OptionOpaqueChar(diplomatRuntime.internalConstructor, bDeref, []);
        const cDeref = (new Uint32Array(wasm.memory.buffer, ptr + 8, 1))[0];
        structObj.c = cDeref;
        const dDeref = diplomatRuntime.ptrRead(wasm, ptr + 12);
        structObj.d = new OptionOpaque(diplomatRuntime.internalConstructor, dDeref, []);

        return new OptionStruct(structObj, internalConstructor);
    }


    constructor(structObj, internalConstructor) {
        return this.#internalConstructor(...arguments)
    }
}