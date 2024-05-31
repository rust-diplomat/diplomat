



const Unnamespaced_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_Unnamespaced_destroy(ptr);
});

export class Unnamespaced {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        Unnamespaced_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static make(e) {
        const result = wasm.namespace_Unnamespaced_make(e.ffiValue);
    
        return new Unnamespaced(result, []);
    }

    useNamespaced(n) {
        wasm.namespace_Unnamespaced_use_namespaced(this.ffiValue, n.ffiValue);
    
    }

    

}