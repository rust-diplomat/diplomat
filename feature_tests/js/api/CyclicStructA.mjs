// generated by diplomat-tool
import { CyclicStructB } from "./CyclicStructB.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class CyclicStructA {
	

    #a;
    get a()  {
        return this.#a;
    }
    set a(value) {
        this.#a = value;
    }

    /** Create `CyclicStructA` from an object that contains all of `CyclicStructA`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj) {
        return new CyclicStructA(structObj);
    }
    
    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("CyclicStructA's constructor takes an object of CyclicStructA's fields.");
        }

        if ("a" in structObj) {
            this.#a = CyclicStructB._fromSuppliedValue(diplomatRuntime.internalConstructor, structObj.a);
        } else {
            throw new Error("Missing required field a.");
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
        return [...CyclicStructB._fromSuppliedValue(diplomatRuntime.internalConstructor, this.#a)._intoFFI(functionCleanupArena, {})]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof CyclicStructA) {
            return obj;
        }

        return CyclicStructA.FromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        CyclicStructB._fromSuppliedValue(diplomatRuntime.internalConstructor, this.#a)._writeToArrayBuffer(arrayBuffer, offset + 0, functionCleanupArena, {});
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, primitiveValue) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CyclicStructA._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const aDeref = primitiveValue;
        structObj.a = CyclicStructB._createFromFFI(diplomatRuntime.internalConstructor, aDeref);

        return structObj;
    }

    #setFieldsFromFFI(internalConstructor, primitiveValue) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CyclicStructA._setFieldsFromFFI is not meant to be called externally. Please use the default constructor.");
        }

        const structObj = CyclicStructA._fromFFI(...arguments);  

           
        this.#a = structObj.a;
           
    }

    static _createFromFFI(internalConstructor, primitiveValue) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CyclicStructA._createFromFFI is not meant to be called externally. Please use the default constructor.");
        }

        const structObj = CyclicStructA._fromFFI(...arguments);
        
        let self = new CyclicStructA(structObj);
        return self;
    }


    static getB() {
        const result = wasm.CyclicStructA_get_b();
    
        try {
            return CyclicStructB._createFromFFI(diplomatRuntime.internalConstructor, result);
        }
        
        finally {}
    }

    cyclicOut() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.CyclicStructA_cyclic_out(...this._intoFFI(), write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            functionCleanupArena.free();
        
            write.free();
        }
    }
}