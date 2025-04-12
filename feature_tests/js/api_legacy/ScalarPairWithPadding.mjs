// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** 
 * Testing JS-specific layout/padding behavior
 */


export class ScalarPairWithPadding {
    
    #first;
    
    get first()  {
        return this.#first;
    } 
    set first(value) {
        this.#first = value;
    }
    
    #second;
    
    get second()  {
        return this.#second;
    } 
    set second(value) {
        this.#second = value;
    }
    
    /** Create `ScalarPairWithPadding` from an object that contains all of `ScalarPairWithPadding`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new ScalarPairWithPadding(structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("ScalarPairWithPadding's constructor takes an object of ScalarPairWithPadding's fields.");
        }

        if ("first" in structObj) {
            this.#first = structObj.first;
        } else {
            throw new Error("Missing required field first.");
        }

        if ("second" in structObj) {
            this.#second = structObj.second;
        } else {
            throw new Error("Missing required field second.");
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
        return [this.#first, ...diplomatRuntime.maybePaddingFields(forcePadding, 3 /* x i8 */), this.#second]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof ScalarPairWithPadding) {
            return obj;
        }

        return ScalarPairWithPadding.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap,
        forcePadding
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#first, Uint8Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#second, Uint32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("ScalarPairWithPadding._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const firstDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0];
        structObj.first = firstDeref;
        const secondDeref = (new Uint32Array(wasm.memory.buffer, ptr + 4, 1))[0];
        structObj.second = secondDeref;

        return new ScalarPairWithPadding(structObj);
    }

    assertValue() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        wasm.ScalarPairWithPadding_assert_value(...ScalarPairWithPadding._fromSuppliedValue(diplomatRuntime.internalConstructor, this)._intoFFI(functionCleanupArena, {}));
    
        try {}
        
        finally {
            functionCleanupArena.free();
        }
    }

    constructor(structObj) {
        return this.#internalConstructor(...arguments)
    }
}