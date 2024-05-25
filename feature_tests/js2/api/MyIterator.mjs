// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"



const MyIterator_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_MyIterator_destroy(ptr);
});

export class MyIterator {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    #selfEdge = [];
    
    #aEdge = [];
    
    
    constructor(ptr, selfEdge, aEdge) {
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        if (this.#selfEdge.length === 0) {
            MyIterator_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }


    #iteratorNext() {
        const result = wasm.namespace_MyIterator_next(this.ffiValue);
    
        if (!result.isOk) {
            return null
        }
         return result.union.ok;
    }

    
    #value;
    
    get value() {
    	return this.#value;
    }
    
    next() {
    	const out = this.#iteratorNext();
    
    	this.#value = out;
    
    	return out;
    }

}