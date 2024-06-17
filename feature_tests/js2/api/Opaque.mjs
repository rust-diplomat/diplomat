// generated by diplomat-tool

import { ImportedStruct } from "./ImportedStruct.mjs"
import { MyStruct } from "./MyStruct.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";




const Opaque_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Opaque_destroy(ptr);
});

export class Opaque {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        Opaque_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_() {
        const result = wasm.Opaque_new();
    
        try {
    
            return new Opaque(result, []);
        } finally {
        
        }
    }

    assertStruct(s) {
        wasm.Opaque_assert_struct(this.ffiValue, ...s._intoFfi(temp));
    
        try {
    
        } finally {
        
            this.free(); /* TODO: Does this work? */
        
        }
    }

    static returnsUsize() {
        const result = wasm.Opaque_returns_usize();
    
        try {
    
            return result;
        } finally {
        
        }
    }

    static returnsImported() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.Opaque_returns_imported(diplomat_receive_buffer);
    
        try {
    
            return new ImportedStruct()._fromFFI(diplomat_receive_buffer);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static cmp() {
        const result = wasm.Opaque_cmp();
    
        try {
    
            return result;
        } finally {
        
        }
    }

    

}