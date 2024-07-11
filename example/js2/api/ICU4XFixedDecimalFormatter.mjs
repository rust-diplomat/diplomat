// generated by diplomat-tool
import { ICU4XDataProvider } from "./ICU4XDataProvider.mjs"
import { ICU4XFixedDecimal } from "./ICU4XFixedDecimal.mjs"
import { ICU4XFixedDecimalFormatterOptions } from "./ICU4XFixedDecimalFormatterOptions.mjs"
import { ICU4XLocale } from "./ICU4XLocale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
*
*See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
*/

const ICU4XFixedDecimalFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_ICU4XFixedDecimalFormatter_destroy_mv1(ptr);
});
export class ICU4XFixedDecimalFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        ICU4XFixedDecimalFormatter_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static tryNew(locale, provider, options) {
        
        let slice_cleanup_callbacks = [];
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_ICU4XFixedDecimalFormatter_try_new_mv1(diplomat_receive_buffer, locale.ffiValue, provider.ffiValue, ...options._intoFFI(slice_cleanup_callbacks, {}));
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                throw diplomatRuntime.FFIError(null);
            }
            return new ICU4XFixedDecimalFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            for (let cleanup of slice_cleanup_callbacks) {
                cleanup();
            }
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    formatWrite(value) {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.icu4x_ICU4XFixedDecimalFormatter_format_write_mv1(this.ffiValue, value.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    

}