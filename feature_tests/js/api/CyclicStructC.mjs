// generated by diplomat-tool
import { CyclicStructA } from "./CyclicStructA.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class CyclicStructC {
    
    #a;
    
    get a()  {
        return this.#a;
    } 
    set a(value) {
        this.#a = value;
    }
    
    /** Create `CyclicStructC` from an object that contains all of `CyclicStructC`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new CyclicStructC(structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("CyclicStructC's constructor takes an object of CyclicStructC's fields.");
        }

        if ("a" in structObj) {
            this.#a = CyclicStructA._fromSuppliedValue(diplomatRuntime.internalConstructor, structObj.a);
        } else {
            throw new Error("Missing required field a.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        let buffer = diplomatRuntime.DiplomatBuf.struct(wasm, 1, 1);

        this._writeToArrayBuffer(wasm.memory.buffer, buffer.ptr, functionCleanupArena, appendArrayMap);
        
        functionCleanupArena.alloc(buffer);

        return buffer.ptr;
        
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof CyclicStructC) {
            return obj;
        }

        return CyclicStructC.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        CyclicStructA._fromSuppliedValue(diplomatRuntime.internalConstructor, this.#a)._writeToArrayBuffer(arrayBuffer, offset + 0, functionCleanupArena, {});
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, primitiveValue) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CyclicStructC._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const aDeref = primitiveValue;
        structObj.a = CyclicStructA._fromFFI(diplomatRuntime.internalConstructor, aDeref);

        return new CyclicStructC(structObj);
    }

    static takesNestedParameters(c) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const result = wasm.CyclicStructC_takes_nested_parameters(CyclicStructC._intoFFI(functionCleanupArena, [], false));
    
        try {
            return CyclicStructC._fromFFI(diplomatRuntime.internalConstructor, result);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    cyclicOut() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.CyclicStructC_cyclic_out(this._intoFFI(functionCleanupArena), write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            functionCleanupArena.free();
        
            write.free();
        }
    }

    constructor(structObj) {
        return this.#internalConstructor(...arguments)
    }
}