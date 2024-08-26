// generated by diplomat-tool
import { ScalarPairWithPadding } from "./ScalarPairWithPadding.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** Testing JS-specific layout/padding behavior
*/
export class BigStructWithStuff {

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

    #third;
    get third()  {
        return this.#third;
    }
    set third(value) {
        this.#third = value;
    }

    #fourth;
    get fourth()  {
        return this.#fourth;
    }
    set fourth(value) {
        this.#fourth = value;
    }

    #fifth;
    get fifth()  {
        return this.#fifth;
    }
    set fifth(value) {
        this.#fifth = value;
    }
    constructor() {
        if (arguments.length > 0 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#fromFFI(...Array.prototype.slice.call(arguments, 1));
        } else {
            
            this.#first = arguments[0];
            this.#second = arguments[1];
            this.#third = arguments[2];
            this.#fourth = arguments[3];
            this.#fifth = arguments[4];
        }
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#first,/* Padding for first */ 0 /* End Padding */, this.#second, this.#third,/* Padding for third */ 0, 0 /* End Padding */, ...this.#fourth._intoFFI(functionCleanupArena, {}), this.#fifth]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    #fromFFI(ptr) {
        const firstDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0];
        this.#first = firstDeref;
        const secondDeref = (new Uint16Array(wasm.memory.buffer, ptr + 2, 1))[0];
        this.#second = secondDeref;
        const thirdDeref = (new Uint16Array(wasm.memory.buffer, ptr + 4, 1))[0];
        this.#third = thirdDeref;
        const fourthDeref = ptr + 8;
        this.#fourth = new ScalarPairWithPadding(diplomatRuntime.internalConstructor, fourthDeref);
        const fifthDeref = (new Uint8Array(wasm.memory.buffer, ptr + 16, 1))[0];
        this.#fifth = fifthDeref;
    }

    assertValue() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        wasm.BigStructWithStuff_assert_value(...this._intoFFI());
    
        try {}
        
        finally {
            functionCleanupArena.free();
        }
    }
}