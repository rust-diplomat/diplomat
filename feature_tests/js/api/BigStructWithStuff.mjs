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

    /** Create `BigStructWithStuff` from an object that contains all of `BigStructWithStuff`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj) {
        return new BigStructWithStuff(structObj);
    }
    
    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("BigStructWithStuff's constructor takes an object of BigStructWithStuff's fields.");
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

        if ("third" in structObj) {
            this.#third = structObj.third;
        } else {
            throw new Error("Missing required field third.");
        }

        if ("fourth" in structObj) {
            this.#fourth = ScalarPairWithPadding._fromSuppliedValue(diplomatRuntime.internalConstructor, structObj.fourth);
        } else {
            throw new Error("Missing required field fourth.");
        }

        if ("fifth" in structObj) {
            this.#fifth = structObj.fifth;
        } else {
            throw new Error("Missing required field fifth.");
        }

    }
    constructor(structObj) {
        this.#internalConstructor(structObj);
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#first, /* [1 x i8] padding */ 0 /* end padding */, this.#second, this.#third, /* [1 x i16] padding */ 0 /* end padding */, ...ScalarPairWithPadding._fromSuppliedValue(diplomatRuntime.internalConstructor, this.#fourth)._intoFFI(functionCleanupArena, {}, true), this.#fifth, /* [3 x i8] padding */ 0, 0, 0 /* end padding */]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof BigStructWithStuff) {
            return obj;
        }

        return BigStructWithStuff.FromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#first, Uint8Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 2, this.#second, Uint16Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#third, Uint16Array);
        ScalarPairWithPadding._fromSuppliedValue(diplomatRuntime.internalConstructor, this.#fourth)._writeToArrayBuffer(arrayBuffer, offset + 8, functionCleanupArena, {});
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 16, this.#fifth, Uint8Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BigStructWithStuff._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const firstDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0];
        structObj.first = firstDeref;
        const secondDeref = (new Uint16Array(wasm.memory.buffer, ptr + 2, 1))[0];
        structObj.second = secondDeref;
        const thirdDeref = (new Uint16Array(wasm.memory.buffer, ptr + 4, 1))[0];
        structObj.third = thirdDeref;
        const fourthDeref = ptr + 8;
        structObj.fourth = ScalarPairWithPadding._createFromFFI(diplomatRuntime.internalConstructor, fourthDeref);
        const fifthDeref = (new Uint8Array(wasm.memory.buffer, ptr + 16, 1))[0];
        structObj.fifth = fifthDeref;

        return structObj;
    }

    #setFieldsFromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BigStructWithStuff._setFieldsFromFFI is not meant to be called externally. Please use the default constructor.");
        }

        const structObj = BigStructWithStuff._fromFFI(...arguments);  

           
        this.#first = structObj.first;
           
        this.#second = structObj.second;
           
        this.#third = structObj.third;
           
        this.#fourth = structObj.fourth;
           
        this.#fifth = structObj.fifth;
           
    }

    static _createFromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BigStructWithStuff._createFromFFI is not meant to be called externally. Please use the default constructor.");
        }

        const structObj = BigStructWithStuff._fromFFI(...arguments);
        
        let self = new BigStructWithStuff(structObj);
        return self;
    }


    assertValue(extraVal) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        wasm.BigStructWithStuff_assert_value(...this._intoFFI(), extraVal);
    
        try {}
        
        finally {
            functionCleanupArena.free();
        }
    }
}