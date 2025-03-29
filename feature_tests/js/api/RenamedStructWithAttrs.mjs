// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class RenamedStructWithAttrs {
    
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
    
    /** Create `RenamedStructWithAttrs` from an object that contains all of `RenamedStructWithAttrs`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new RenamedStructWithAttrs(diplomatRuntime.exposeConstructor, structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("RenamedStructWithAttrs's constructor takes an object of RenamedStructWithAttrs's fields.");
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

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    // JS structs need to be generated with or without padding depending on whether they are being passed as aggregates or splatted out into fields.
    // Most of the time this is known beforehand: large structs (>2 scalar fields) always get padding, and structs passed directly in parameters omit padding
    // if they are small. However small structs within large structs also get padding, and we signal that by setting forcePadding.
    _intoFFI(
        functionCleanupArena,
        appendArrayMap,
        forcePadding
    ) {
        return [this.#a, ...diplomatRuntime.maybePaddingFields(forcePadding, 3 /* x i8 */), this.#b]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof RenamedStructWithAttrs) {
            return obj;
        }

        return RenamedStructWithAttrs.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap,
        forcePadding
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#a, Uint8Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#b, Uint32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("RenamedStructWithAttrs._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const aDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0] === 1;
        structObj.a = aDeref;
        const bDeref = (new Uint32Array(wasm.memory.buffer, ptr + 4, 1))[0];
        structObj.b = bDeref;

        return new RenamedStructWithAttrs(diplomatRuntime.exposeConstructor, structObj);
    }
#defaultConstructor(a, b) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 9, 4, true);
        
        const result = wasm.namespace_StructWithAttrs_new_fallible(diplomatReceive.buffer, a, b);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return RenamedStructWithAttrs._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }
get c() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const result = wasm.namespace_StructWithAttrs_c(...this._intoFFI());
    
        try {
            return result;
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    constructor(a, b) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}