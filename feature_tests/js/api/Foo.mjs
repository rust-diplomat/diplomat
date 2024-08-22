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
    
    constructor(symbol, ptr, selfEdge, aEdge) {
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
    }

    get ffiValue() {
        return this.#ptr;
    }

    static new_(x) {
        let functionGarbageCollector = new diplomatRuntime.GarbageCollector();
        const xSlice = [...functionGarbageCollector.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, x)).splat()];
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [xSlice];
        
        const result = wasm.Foo_new(...xSlice);
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionGarbageCollector.garbageCollect();
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
            return new BorrowedFieldsReturning(diplomatRuntime.internalConstructor, diplomatReceive.buffer, aEdges);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static extractFromFields(fields) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [...fields._fieldsForLifetimeA];
        
        const result = wasm.Foo_extract_from_fields(...fields._intoFFI(functionCleanupArena, {aAppendArray: [aEdges],}));
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    static extractFromBounds(bounds, anotherString) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        let functionGarbageCollector = new diplomatRuntime.GarbageCollector();
        const anotherStringSlice = [...functionGarbageCollector.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, anotherString)).splat()];
        
        // This lifetime edge depends on lifetimes 'a, 'y, 'z
        let aEdges = [...bounds._fieldsForLifetimeB, ...bounds._fieldsForLifetimeC, anotherStringSlice];
        
        const result = wasm.Foo_extract_from_bounds(...bounds._intoFFI(functionCleanupArena, {bAppendArray: [aEdges],cAppendArray: [aEdges],}), ...anotherStringSlice);
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionCleanupArena.free();
        
            functionGarbageCollector.garbageCollect();
        }
    }
}