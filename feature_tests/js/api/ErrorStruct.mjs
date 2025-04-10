// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class ErrorStruct {
    #i;
    get i() {
        return this.#i;
    }
    set i(value){
        this.#i = value;
    }
    #j;
    get j() {
        return this.#j;
    }
    set j(value){
        this.#j = value;
    }
    /** Create `ErrorStruct` from an object that contains all of `ErrorStruct`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new ErrorStruct(structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("ErrorStruct's constructor takes an object of ErrorStruct's fields.");
        }

        if ("i" in structObj) {
            this.#i = structObj.i;
        } else {
            throw new Error("Missing required field i.");
        }

        if ("j" in structObj) {
            this.#j = structObj.j;
        } else {
            throw new Error("Missing required field j.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        let buffer = diplomatRuntime.DiplomatBuf.struct(wasm, 8, 4);

        this._writeToArrayBuffer(wasm.memory.buffer, buffer.ptr, functionCleanupArena, appendArrayMap);
        
        functionCleanupArena.alloc(buffer);

        return buffer.ptr;
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof ErrorStruct) {
            return obj;
        }

        return ErrorStruct.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#i, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#j, Int32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("ErrorStruct._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const iDeref = (new Int32Array(wasm.memory.buffer, ptr, 1))[0];
        structObj.i = iDeref;
        const jDeref = (new Int32Array(wasm.memory.buffer, ptr + 4, 1))[0];
        structObj.j = jDeref;

        return new ErrorStruct(structObj);
    }


    constructor(structObj) {
        return this.#internalConstructor(...arguments)
    }
}