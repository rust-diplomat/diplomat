// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class BorrowedFieldsReturning {
	

    #bytes;
    get bytes()  {
        return this.#bytes;
    }
    set bytes(value) {
        this.#bytes = value;
    }

    /** Create `BorrowedFieldsReturning` from an object that contains all of `BorrowedFieldsReturning`'s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj) {
        return new BorrowedFieldsReturning(structObj);
    }
    
    constructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("BorrowedFieldsReturning's constructor takes an object of BorrowedFieldsReturning's fields.");
        }

        if ("bytes" in structObj) {
            this.#bytes = structObj.bytes;
        } else {
            throw new Error("Missing required field bytes.");
        }

    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    // If this struct contains any slices, their lifetime-edge-relevant information will be
    // set up here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [...diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#bytes)).splat()]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof BorrowedFieldsReturning) {
            return obj;
        }

        return new BorrowedFieldsReturning(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#bytes)).writePtrLenToArrayBuffer(arrayBuffer, offset + 0);
    }

    static _fromFFI(internalConstructor, ptr, aEdges) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BorrowedFieldsReturning._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const bytesDeref = ptr;
        structObj.bytes = new diplomatRuntime.DiplomatSliceStr(wasm, bytesDeref,  "string8", aEdges).getValue();

        return new BorrowedFieldsReturning(structObj, internalConstructor);
    }

    // Return all fields corresponding to lifetime `'a` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'a`,
    // assuming that there are no `'other: a`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeA() { 
        return [bytes];
    };

}