// generated by diplomat-tool
import { DefaultEnum } from "./DefaultEnum.mjs"
import { MyStruct } from "./MyStruct.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class MyStructContainingAnOption {
    
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
    
    /** Create `MyStructContainingAnOption` from an object that contains all of `MyStructContainingAnOption`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new MyStructContainingAnOption(diplomatRuntime.exposeConstructor, structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("MyStructContainingAnOption's constructor takes an object of MyStructContainingAnOption's fields.");
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

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        let buffer = new diplomatRuntime.DiplomatSendBuf(wasm, 48, 8);

        this._writeToArrayBuffer(buffer, 0, functionCleanupArena, appendArrayMap);
        
        functionCleanupArena.alloc(buffer);

        return buffer.ptr;
        
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof MyStructContainingAnOption) {
            return obj;
        }

        return MyStructContainingAnOption.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeOptionToArrayBuffer(arrayBuffer, offset + 0, this.#a, 32, 8, (arrayBuffer, offset, jsValue) => MyStruct._fromSuppliedValue(diplomatRuntime.internalConstructor, jsValue)._writeToArrayBuffer(arrayBuffer, offset + 0, functionCleanupArena, {}));
        diplomatRuntime.writeOptionToArrayBuffer(arrayBuffer, offset + 40, this.#b, 4, 4, (arrayBuffer, offset, jsValue) => diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array));
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("MyStructContainingAnOption._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const aDeref = ptr;
        structObj.a = diplomatRuntime.readOption(wasm, aDeref, 32, (wasm, offset) => { const deref = offset; return MyStruct._fromFFI(diplomatRuntime.internalConstructor, deref) });
        const bDeref = ptr + 40;
        structObj.b = diplomatRuntime.readOption(wasm, bDeref, 4, (wasm, offset) => { const deref = diplomatRuntime.enumDiscriminant(wasm, offset); return new DefaultEnum(diplomatRuntime.internalConstructor, deref) });

        return new MyStructContainingAnOption(diplomatRuntime.exposeConstructor, structObj);
    }

    #defaultConstructor() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 48, 8, false);
        
        const result = wasm.MyStructContainingAnOption_new(diplomatReceive.buffer);
    
        try {
            return MyStructContainingAnOption._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static filled() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 48, 8, false);
        
        const result = wasm.MyStructContainingAnOption_filled(diplomatReceive.buffer);
    
        try {
            return MyStructContainingAnOption._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}