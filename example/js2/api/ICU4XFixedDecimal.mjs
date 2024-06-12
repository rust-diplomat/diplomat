// generated by diplomat-tool

import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



/** See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
*/


const ICU4XFixedDecimal_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.ICU4XFixedDecimal_destroy(ptr);
});

export class ICU4XFixedDecimal {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        ICU4XFixedDecimal_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_(v) {
        const result = wasm.ICU4XFixedDecimal_new(v);
    
        try {
    
            return new ICU4XFixedDecimal(result, []);
        } finally {
        
        }
    }

    multiplyPow10(power) {
        wasm.ICU4XFixedDecimal_multiply_pow10(this.ffiValue, power);
    
        try {
    
        } finally {
        
        }
    }

    toString() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        
        const write = wasm.diplomat_buffer_write_create(0);
        const result = wasm.ICU4XFixedDecimal_to_string(diplomat_receive_buffer, this.ffiValue);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    

}