// generated by diplomat-tool

import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";




const Two_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Two_destroy(ptr);
});

export class Two {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #aEdge = [];
    
    #bEdge = [];
    
    
    _fromFFI(ptr, selfEdge, aEdge, bEdge) {
        
        
        this.#aEdge = aEdge;
        
        
        this.#bEdge = bEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        Two_box_destroy_registry.register(this, this.#ptr);
    }
    constructor() {
        throw new Error("You cannot create the opaque type Two without a valid constructor. You may call one of the static methods below, or you may label the default opaque constructor in the diplomat FFI definition with #[diplomat::attr(constructor)].");
    }

    get ffiValue() {
        return this.#ptr;
    }


    

}