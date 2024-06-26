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
    
    
    constructor(ptr, selfEdge, aEdge) {
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        Foo_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_(x) {
        
        const xSlice = diplomatRuntime.DiplomatBuf.str8(wasm, x);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [xSlice];
        const result = wasm.Foo_new(xSlice.ptr, xSlice.size);
    
        try {
    
            return new Foo(result, [], aEdges);
        } finally {
        
            xSlice.garbageCollect();
        
        }
    }

    get bar() {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        // This lifetime edge depends on lifetimes 'a, 'b
        let bEdges = [this];
        const result = wasm.Foo_get_bar(this.ffiValue);
    
        try {
    
            return new Bar(result, [], bEdges, aEdges);
        } finally {
        
        }
    }

    static newStatic(x) {
        
        const xSlice = diplomatRuntime.DiplomatBuf.str8(wasm, x);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [];
        const result = wasm.Foo_new_static(xSlice.ptr, xSlice.size);
    
        try {
    
            return new Foo(result, [], aEdges);
        } finally {
        
            xSlice.free();
        
        }
    }

    asReturning() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        const result = wasm.Foo_as_returning(diplomat_receive_buffer, this.ffiValue);
    
        try {
    
            return new BorrowedFieldsReturning()._fromFFI(diplomat_receive_buffer, aEdges);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
        
        }
    }

    static extractFromFields(fields) {
        
        let slice_cleanup_callbacks = [];
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [...fields._fieldsForLifetimeA];
        const result = wasm.Foo_extract_from_fields(...fields._intoFFI(slice_cleanup_callbacks, {aAppendArray: [aEdges],}));
    
        try {
    
            return new Foo(result, [], aEdges);
        } finally {
        
            for (let cleanup of slice_cleanup_callbacks) {
                cleanup();
            }
        
        }
    }

    static extractFromBounds(bounds, anotherString) {
        
        const anotherStringSlice = diplomatRuntime.DiplomatBuf.str8(wasm, anotherString);
        
        let slice_cleanup_callbacks = [];
        
        // This lifetime edge depends on lifetimes 'a, 'y, 'z
        let aEdges = [...bounds._fieldsForLifetimeB, ...bounds._fieldsForLifetimeC, anotherStringSlice];
        const result = wasm.Foo_extract_from_bounds(...bounds._intoFFI(slice_cleanup_callbacks, {bAppendArray: [aEdges],cAppendArray: [aEdges],}), anotherStringSlice.ptr, anotherStringSlice.size);
    
        try {
    
            return new Foo(result, [], aEdges);
        } finally {
        
            for (let cleanup of slice_cleanup_callbacks) {
                cleanup();
            }
        
            anotherStringSlice.garbageCollect();
        
        }
    }

    

}