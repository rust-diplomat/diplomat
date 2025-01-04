// generated by diplomat-tool
import { Bar } from "./Bar.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class BorrowedFields {
	

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

    #c;
    get c()  {
        return this.#c;
    }
    set c(value) {
        this.#c = value;
    }

    /** Create `BorrowedFields` from an object that contains all of `BorrowedFields`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj) {
        return new BorrowedFields(structObj);
    }
    
    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("BorrowedFields's constructor takes an object of BorrowedFields's fields.");
        }

        if ("a" in structObj) {
            this.#a = structObj.a;
        } else {
            throw new Error("Missing required field a.");
        }

        if ("b" in structObj) {
            this.#b = structObj.b;
        } else {
            throw new Error("Missing required field b.");
        }

        if ("c" in structObj) {
            this.#c = structObj.c;
        } else {
            throw new Error("Missing required field c.");
        }

    }
    constructor(structObj) {
        this.#internalConstructor(structObj);
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
        return [...diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str16(wasm, this.#a)).splat(), ...diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#b)).splat(), ...diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#c)).splat()]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof BorrowedFields) {
            return obj;
        }

        return new BorrowedFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str16(wasm, this.#a)).writePtrLenToArrayBuffer(arrayBuffer, offset + 0);
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#b)).writePtrLenToArrayBuffer(arrayBuffer, offset + 8);
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#c)).writePtrLenToArrayBuffer(arrayBuffer, offset + 16);
    }

    static _fromFFI(internalConstructor, ptr, aEdges) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BorrowedFields._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const aDeref = ptr;
        structObj.a = new diplomatRuntime.DiplomatSliceStr(wasm, aDeref,  "string16", aEdges).getValue();
        const bDeref = ptr + 8;
        structObj.b = new diplomatRuntime.DiplomatSliceStr(wasm, bDeref,  "string8", aEdges).getValue();
        const cDeref = ptr + 16;
        structObj.c = new diplomatRuntime.DiplomatSliceStr(wasm, cDeref,  "string8", aEdges).getValue();

        return structObj;
    }

    static _createFromFFI(internalConstructor, ptr, aEdges) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BorrowedFields._createFromFFI is not meant to be called externally. Please use the default constructor.");
        }

        const structObj = BorrowedFields._fromFFI(...arguments);
        
        let self = new BorrowedFields(structObj);
        return self;
    }

    // Return all fields corresponding to lifetime `'a` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'a`,
    // assuming that there are no `'other: a`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeA() { 
        return [a, b, c];
    };


    static fromBarAndStrings(bar, dstr16, utf8Str) {
        let functionGarbageCollectorGrip = new diplomatRuntime.GarbageCollectorGrip();
        const dstr16Slice = functionGarbageCollectorGrip.alloc(diplomatRuntime.DiplomatBuf.str16(wasm, dstr16));
        
        const utf8StrSlice = functionGarbageCollectorGrip.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, utf8Str));
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 24, 4, false);
        
        // This lifetime edge depends on lifetimes 'x
        let xEdges = [bar, dstr16Slice, utf8StrSlice];
        
        const result = wasm.BorrowedFields_from_bar_and_strings(diplomatReceive.buffer, bar.ffiValue, ...dstr16Slice.splat(), ...utf8StrSlice.splat());
    
        try {
            return BorrowedFields._createFromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer, xEdges);
        }
        
        finally {
            functionGarbageCollectorGrip.releaseToGarbageCollector();
        
            diplomatReceive.free();
        }
    }
}