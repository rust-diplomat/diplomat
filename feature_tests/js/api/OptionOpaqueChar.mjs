// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const OptionOpaqueChar_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.OptionOpaqueChar_destroy(ptr);
});

export class OptionOpaqueChar {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        OptionOpaqueChar_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }
    
    assertChar(ch) {
        wasm.OptionOpaqueChar_assert_char(this.ffiValue, ch);
    
        try {
    
        } finally {
        
        }
    }

    

}