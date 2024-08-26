// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** Testing JS-specific layout/padding behavior
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
    constructor() {
        if (arguments.length > 0 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#fromFFI(...Array.prototype.slice.call(arguments, 1));
        } else {
            
            this.#first = arguments[0];
            this.#second = arguments[1];
        }
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#first, /* Padding (u8) for second */ 0, 0, 0 /* End Padding */,this.#second]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    #fromFFI(ptr) {
        const firstDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0];
        this.#first = firstDeref;
        const secondDeref = (new Uint32Array(wasm.memory.buffer, ptr + 4, 1))[0];
        this.#second = secondDeref;
    }

    assertValue() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        wasm.ScalarPairWithPadding_assert_value(...this._intoFFI());
    
        try {}
        
        finally {
            functionCleanupArena.free();
        }
    }
}