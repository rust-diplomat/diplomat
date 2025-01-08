// generated by diplomat-tool
import { RefListParameter } from "./RefListParameter.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const RefList_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.RefList_destroy(ptr);
});

export class RefList {
	
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];
    
    constructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("RefList is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            RefList_box_destroy_registry.register(this, this.#ptr);
        }
    }
    get ffiValue() {
        return this.#ptr;
    }

    static node(data) {
        // This lifetime edge depends on lifetimes 'b
        let bEdges = [data];
        
        const result = wasm.RefList_node(data.ffiValue);
    
        try {
            return new RefList(diplomatRuntime.internalConstructor, result, [], bEdges);
        }
        
        finally {}
    }
}