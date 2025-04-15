// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class StructWithSlices {
    #first;
    get first() {
        return this.#first;
    }
    set first(value){
        this.#first = value;
    }
    #second;
    get second() {
        return this.#second;
    }
    set second(value){
        this.#second = value;
    }
    /** Create `StructWithSlices` from an object that contains all of `StructWithSlices`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new StructWithSlices(structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("StructWithSlices's constructor takes an object of StructWithSlices's fields.");
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
    // Returns an array that can be expanded with spread syntax (...)// If this struct contains any slices, their lifetime-edge-relevant information will be
    // set up here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        let buffer = diplomatRuntime.DiplomatBuf.struct(wasm, 16, 4);

        this._writeToArrayBuffer(wasm.memory.buffer, buffer.ptr, functionCleanupArena, appendArrayMap);

        functionCleanupArena.alloc(buffer);

        return buffer.ptr;
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof StructWithSlices) {
            return obj;
        }

        return StructWithSlices.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#first)).writePtrLenToArrayBuffer(arrayBuffer, offset + 0);
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.slice(wasm, this.#second, "u16")).writePtrLenToArrayBuffer(arrayBuffer, offset + 8);
    }

    static _fromFFI(internalConstructor, ptr, aEdges) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("StructWithSlices._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const firstDeref = ptr;
        structObj.first = new diplomatRuntime.DiplomatSliceStr(wasm, firstDeref,  "string8", aEdges).getValue();
        const secondDeref = ptr + 8;
        structObj.second = Array.from(new diplomatRuntime.DiplomatSlicePrimitive(wasm, secondDeref, "u16", aEdges).getValue());

        return new StructWithSlices(structObj);
    }

    // Return all fields corresponding to lifetime `'a`
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'a`,
    // assuming that there are no `'other: a`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeA() {
        return [this.#first, this.#second];
    };


    returnLast() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);

        // This lifetime edge depends on lifetimes 'a
        let aEdges = [...this._fieldsForLifetimeA];

    wasm.StructWithSlices_return_last(StructWithSlices._fromSuppliedValue(diplomatRuntime.internalConstructor, this)._intoFFI(functionCleanupArena, {aAppendArray: [aEdges],}, false), write.buffer);

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