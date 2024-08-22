// generated by diplomat-tool
import { Bar } from "./Bar.mjs"
import { BorrowedFields } from "./BorrowedFields.mjs"
import { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds.mjs"
import { Foo } from "./Foo.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

export class NestedBorrowedFields {

    #fields;
    get fields()  {
        return this.#fields;
    }
    set fields(value) {
        this.#fields = value;
    }

    #bounds;
    get bounds()  {
        return this.#bounds;
    }
    set bounds(value) {
        this.#bounds = value;
    }

    #bounds2;
    get bounds2()  {
        return this.#bounds2;
    }
    set bounds2(value) {
        this.#bounds2 = value;
    }
    constructor() {
        if (arguments.length > 0 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#fromFFI(...Array.prototype.slice.call(arguments, 1));
        } else {
            
            this.#fields = arguments[0];
            this.#bounds = arguments[1];
            this.#bounds2 = arguments[2];
        }
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
        return [...this.#fields._intoFFI(functionCleanupArena, {aAppendArray: [...xAppendArray],}), ...this.#bounds._intoFFI(functionCleanupArena, {aAppendArray: [...xAppendArray],bAppendArray: [...yAppendArray],cAppendArray: [...yAppendArray],}), ...this.#bounds2._intoFFI(functionCleanupArena, {aAppendArray: [...zAppendArray],bAppendArray: [...zAppendArray],cAppendArray: [...zAppendArray],})]
    }

    #fromFFI(ptr, xEdges, yEdges, zEdges) {
        const fieldsDeref = ptr;
        this.#fields = new BorrowedFields(diplomatRuntime.internalConstructor, fieldsDeref, xEdges);
        const boundsDeref = ptr + 24;
        this.#bounds = new BorrowedFieldsWithBounds(diplomatRuntime.internalConstructor, boundsDeref, xEdges, yEdges, yEdges);
        const bounds2Deref = ptr + 48;
        this.#bounds2 = new BorrowedFieldsWithBounds(diplomatRuntime.internalConstructor, bounds2Deref, zEdges, zEdges, zEdges);
    }

    // Return all fields corresponding to lifetime `'x` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'x`,
    // assuming that there are no `'other: x`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeX() { 
        return [...fields._fieldsForLifetimeA, ...bounds._fieldsForLifetimeA];
    };

    // Return all fields corresponding to lifetime `'y` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'y`,
    // assuming that there are no `'other: y`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeY() { 
        return [...bounds._fieldsForLifetimeB, ...bounds._fieldsForLifetimeC];
    };

    // Return all fields corresponding to lifetime `'z` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'z`,
    // assuming that there are no `'other: z`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeZ() { 
        return [...bounds2._fieldsForLifetimeA, ...bounds2._fieldsForLifetimeB, ...bounds2._fieldsForLifetimeC];
    };

    static fromBarAndFooAndStrings(bar, foo, dstr16X, dstr16Z, utf8StrY, utf8StrZ) {
        let functionGarbageCollector = new diplomatRuntime.GarbageCollector();
        const dstr16XSlice = [...functionGarbageCollector.alloc(diplomatRuntime.DiplomatBuf.str16(wasm, dstr16X)).splat()];
        
        const dstr16ZSlice = [...functionGarbageCollector.alloc(diplomatRuntime.DiplomatBuf.str16(wasm, dstr16Z)).splat()];
        
        const utf8StrYSlice = [...functionGarbageCollector.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrY)).splat()];
        
        const utf8StrZSlice = [...functionGarbageCollector.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrZ)).splat()];
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 72, 4, false);
        
        // This lifetime edge depends on lifetimes 'x, 'y
        let xEdges = [bar, dstr16XSlice, utf8StrYSlice];
        
        // This lifetime edge depends on lifetimes 'y
        let yEdges = [bar, utf8StrYSlice];
        
        // This lifetime edge depends on lifetimes 'z
        let zEdges = [foo, dstr16ZSlice, utf8StrZSlice];
        
        const result = wasm.NestedBorrowedFields_from_bar_and_foo_and_strings(diplomatReceive.buffer, bar.ffiValue, foo.ffiValue, ...dstr16XSlice, ...dstr16ZSlice, ...utf8StrYSlice, ...utf8StrZSlice);
    
        try {
            return new NestedBorrowedFields(diplomatRuntime.internalConstructor, diplomatReceive.buffer, xEdges, yEdges, zEdges);
        }
        
        finally {
            functionGarbageCollector.garbageCollect();
        
            diplomatReceive.free();
        }
    }
}