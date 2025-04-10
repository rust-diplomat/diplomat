// generated by diplomat-tool
import { Foo } from "./Foo.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class BorrowedFieldsWithBounds {
    
    #fieldA;
    
    get fieldA()  {
        return this.#fieldA;
    } 
    set fieldA(value) {
        this.#fieldA = value;
    }
    
    #fieldB;
    
    get fieldB()  {
        return this.#fieldB;
    } 
    set fieldB(value) {
        this.#fieldB = value;
    }
    
    #fieldC;
    
    get fieldC()  {
        return this.#fieldC;
    } 
    set fieldC(value) {
        this.#fieldC = value;
    }
    
    /** Create `BorrowedFieldsWithBounds` from an object that contains all of `BorrowedFieldsWithBounds`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new BorrowedFieldsWithBounds(structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("BorrowedFieldsWithBounds's constructor takes an object of BorrowedFieldsWithBounds's fields.");
        }

        if ("fieldA" in structObj) {
            this.#fieldA = structObj.fieldA;
        } else {
            throw new Error("Missing required field fieldA.");
        }

        if ("fieldB" in structObj) {
            this.#fieldB = structObj.fieldB;
        } else {
            throw new Error("Missing required field fieldB.");
        }

        if ("fieldC" in structObj) {
            this.#fieldC = structObj.fieldC;
        } else {
            throw new Error("Missing required field fieldC.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    // If this struct contains any slices, their lifetime-edge-relevant information will be
    // set up here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    //
    // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [...diplomatRuntime.DiplomatBuf.str16(wasm, this.#fieldA).splat(), ...diplomatRuntime.DiplomatBuf.str8(wasm, this.#fieldB).splat(), ...diplomatRuntime.DiplomatBuf.str8(wasm, this.#fieldC).splat()]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof BorrowedFieldsWithBounds) {
            return obj;
        }

        return BorrowedFieldsWithBounds.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['aAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str16(wasm, this.#fieldA)).writePtrLenToArrayBuffer(arrayBuffer, offset + 0);
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['bAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#fieldB)).writePtrLenToArrayBuffer(arrayBuffer, offset + 8);
        diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['cAppendArray']).alloc(diplomatRuntime.DiplomatBuf.str8(wasm, this.#fieldC)).writePtrLenToArrayBuffer(arrayBuffer, offset + 16);
    }

    static _fromFFI(internalConstructor, ptr, aEdges, bEdges, cEdges) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("BorrowedFieldsWithBounds._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const fieldADeref = ptr;
        structObj.fieldA = new diplomatRuntime.DiplomatSliceStr(wasm, fieldADeref,  "string16", aEdges).getValue();
        const fieldBDeref = ptr + 8;
        structObj.fieldB = new diplomatRuntime.DiplomatSliceStr(wasm, fieldBDeref,  "string8", bEdges).getValue();
        const fieldCDeref = ptr + 16;
        structObj.fieldC = new diplomatRuntime.DiplomatSliceStr(wasm, fieldCDeref,  "string8", cEdges).getValue();

        return new BorrowedFieldsWithBounds(structObj);
    }

    // Return all fields corresponding to lifetime `'a` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'a`,
    // assuming that there are no `'other: a`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeA() { 
        return [fieldA];
    };

    // Return all fields corresponding to lifetime `'b` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'b`,
    // assuming that there are no `'other: b`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeB() { 
        return [fieldB];
    };

    // Return all fields corresponding to lifetime `'c` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'c`,
    // assuming that there are no `'other: c`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeC() { 
        return [fieldC];
    };
static fromFooAndStrings(foo, dstr16X, utf8StrZ) {
        let functionGarbageCollectorGrip = new diplomatRuntime.GarbageCollectorGrip();
        const dstr16XSlice = diplomatRuntime.DiplomatBuf.str16(wasm, dstr16X);
        
        const utf8StrZSlice = diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrZ);
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 24, 4, false);
        
        // This lifetime edge depends on lifetimes 'x, 'y, 'z
        let xEdges = [foo, dstr16XSlice, utf8StrZSlice];
        
        // This lifetime edge depends on lifetimes 'y, 'z
        let yEdges = [foo, utf8StrZSlice];
        
        // This lifetime edge depends on lifetimes 'z
        let zEdges = [utf8StrZSlice];
        
        const result = wasm.BorrowedFieldsWithBounds_from_foo_and_strings(diplomatReceive.buffer, foo.ffiValue, ...dstr16XSlice.splat(), ...utf8StrZSlice.splat());
    
        try {
            return BorrowedFieldsWithBounds._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer, xEdges, yEdges, zEdges);
        }
        
        finally {
            functionGarbageCollectorGrip.releaseToGarbageCollector();
        
            diplomatReceive.free();
        }
    }

    constructor(structObj) {
        return this.#internalConstructor(...arguments)
    }
}