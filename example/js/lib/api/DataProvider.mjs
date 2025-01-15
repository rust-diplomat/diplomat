// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An  data provider, capable of loading  data keys from some source.
*
*See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
*/
const DataProvider_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_DataProvider_destroy_mv1(ptr);
});

export class DataProvider {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("DataProvider is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            DataProvider_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    static newStatic() {
        const result = wasm.icu4x_DataProvider_new_static_mv1();
    
        try {
            return new DataProvider(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static returnsResult() {
        const result = wasm.icu4x_DataProvider_returns_result_mv1();
    
        try {
            return result === 1;
        }
        
        finally {}
    }

    constructor(symbol, ptr, selfEdge) {
        return this.#internalConstructor(...arguments)
    }
}