// generated by diplomat-tool

import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



/** An ICU4X data provider, capable of loading ICU4X data keys from some source.
*
*See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
*/


const ICU4XDataProvider_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.ICU4XDataProvider_destroy(ptr);
});

export class ICU4XDataProvider {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        ICU4XDataProvider_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static newStatic() {
        const result = wasm.ICU4XDataProvider_new_static();
    
        try {
    
            return new ICU4XDataProvider(result, []);
        } finally {
        
        }
    }

    static returnsResult() {
        const result = wasm.ICU4XDataProvider_returns_result();
    
        try {
    
            return result == 1;
        } finally {
        
        }
    }

    

}