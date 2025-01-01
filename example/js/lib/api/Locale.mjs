// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An  Locale, capable of representing strings like `"en-US"`.
*
*See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
*/
const Locale_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_Locale_destroy_mv1(ptr);
});



export class Locale {
	
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Locale is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Locale_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    #defaultConstructor(name) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const nameSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, name));
        
        const result = wasm.icu4x_Locale_new_mv1(...nameSlice.splat());
    
        try {
            this.#internalConstructor(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.internalConstructor) {
            this.#internalConstructor(...arguments);
        } else {
            this.#defaultConstructor(...arguments);
        }
    }
}