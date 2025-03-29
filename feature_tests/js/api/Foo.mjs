// generated by diplomat-tool
import { Bar } from "./Bar.mjs"
import { BorrowedFields } from "./BorrowedFields.mjs"
import { BorrowedFieldsReturning } from "./BorrowedFieldsReturning.mjs"
import { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const Foo_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Foo_destroy(ptr);
});

export class Foo {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Foo is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Foo_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    #defaultConstructor(x) {
        let functionGarbageCollectorGrip = new diplomatRuntime.GarbageCollectorGrip();
        const xSlice = functionGarbageCollectorGrip.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, x)));
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [xSlice];
        
        const result = wasm.Foo_new(xSlice.ptr);
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionGarbageCollectorGrip.releaseToGarbageCollector();
        }
    }

    get bar() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        // This lifetime edge depends on lifetimes 'a, 'b
        let bEdges = [this];
        
        const result = wasm.Foo_get_bar(this.ffiValue);
    
        try {
            return new Bar(diplomatRuntime.internalConstructor, result, [], bEdges, aEdges);
        }
        
        finally {}
    }

    asReturning() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.Foo_as_returning(diplomatReceive.buffer, this.ffiValue);
    
        try {
            return BorrowedFieldsReturning._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer, aEdges);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static extractFromFields(fields) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [...fields._fieldsForLifetimeA];
        
        const result = wasm.Foo_extract_from_fields(BorrowedFields._fromSuppliedValue(diplomatRuntime.internalConstructor, fields)._intoFFI(functionCleanupArena, [], false));
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    static extractFromBounds(bounds, anotherString) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        let functionGarbageCollectorGrip = new diplomatRuntime.GarbageCollectorGrip();
        const anotherStringSlice = functionGarbageCollectorGrip.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, anotherString)));
        
        // This lifetime edge depends on lifetimes 'a, 'y, 'z
        let aEdges = [...bounds._fieldsForLifetimeB, ...bounds._fieldsForLifetimeC, anotherStringSlice];
        
        const result = wasm.Foo_extract_from_bounds(BorrowedFieldsWithBounds._fromSuppliedValue(diplomatRuntime.internalConstructor, bounds)._intoFFI(functionCleanupArena, [], false), anotherStringSlice.ptr);
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionCleanupArena.free();
        
            functionGarbageCollectorGrip.releaseToGarbageCollector();
        }
    }

    constructor(x) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}