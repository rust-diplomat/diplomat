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
    constructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("CyclicStructC's constructor takes an object of CyclicStructC's fields.");
        }

        if ("a" in structObj) {
            this.#a = CyclicStructA._fromSuppliedValue(diplomatRuntime.internalConstructor, structObj.a);
        } else {
            throw new Error("Missing required field a.");
        }

    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [...CyclicStructA._fromSuppliedValue(diplomatRuntime.internalConstructor, this.#a)._intoFFI(functionCleanupArena, {})]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof CyclicStructC) {
            return obj;
        }

        return new CyclicStructC(obj);
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
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CyclicStructC._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const aDeref = ptr;
        structObj.a = CyclicStructA._fromFFI(diplomatRuntime.internalConstructor, aDeref);

        return new CyclicStructC(structObj, internalConstructor);
    }

    static takesNestedParameters(c) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 1, 1, false);
        
        const result = wasm.CyclicStructC_takes_nested_parameters(diplomatReceive.buffer, ...CyclicStructC._fromSuppliedValue(diplomatRuntime.internalConstructor, c)._intoFFI(functionCleanupArena, {}));
    
        try {
            return CyclicStructC._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    cyclicOut() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.CyclicStructC_cyclic_out(...this._intoFFI(), write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            functionCleanupArena.free();
        
            write.free();
        }
    }
}