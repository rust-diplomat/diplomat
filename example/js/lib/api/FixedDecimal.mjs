// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
*/
const FixedDecimal_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_FixedDecimal_destroy_mv1(ptr);
});

export class FixedDecimal {
	
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("FixedDecimal is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            FixedDecimal_box_destroy_registry.register(this, this.#ptr);
        }
    }
    get ffiValue() {
        return this.#ptr;
    }
	
    #defaultConstructor(v) {
        const result = wasm.icu4x_FixedDecimal_new_mv1(v);
    
        try {
            return new FixedDecimal(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }
	
    multiplyPow10(power) {wasm.icu4x_FixedDecimal_multiply_pow10_mv1(this.ffiValue, power);
    
        try {}
        
        finally {}
    }
	
    toString() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_FixedDecimal_to_string_mv1(this.ffiValue, write.buffer);
    
        try {
            return result === 0 ? null : write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    constructor(v) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}