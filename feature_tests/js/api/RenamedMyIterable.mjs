// generated by diplomat-tool
import { RenamedMyIterator } from "./RenamedMyIterator.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const RenamedMyIterable_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_MyIterable_destroy(ptr);
});
export class RenamedMyIterable {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        RenamedMyIterable_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_(x) {
        
        const xSlice = diplomatRuntime.DiplomatBuf.slice(wasm, x, "u8");
        const result = wasm.namespace_MyIterable_new(xSlice.ptr, xSlice.size);
    
        try {
    
            return new RenamedMyIterable(result, []);
        } finally {
        
            xSlice.free();
        
        }
    }

    [Symbol.iterator]() {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        const result = wasm.namespace_MyIterable_iter(this.ffiValue);
    
        try {
    
            return new RenamedMyIterator(result, [], aEdges);
        } finally {
        
        }
    }

    

}