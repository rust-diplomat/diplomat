// generated by diplomat-tool
import { AttrOpaque1Renamed } from "./AttrOpaque1Renamed.mjs"
import { RenamedAttrEnum } from "./RenamedAttrEnum.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const Unnamespaced_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_Unnamespaced_destroy(ptr);
});

export class Unnamespaced {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Unnamespaced is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Unnamespaced_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    static make(e) {
        const result = wasm.namespace_Unnamespaced_make(e.ffiValue);
    
        try {
            return new Unnamespaced(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    useNamespaced(n) {wasm.namespace_Unnamespaced_use_namespaced(this.ffiValue, n.ffiValue);
    
        try {}
        
        finally {}
    }

    constructor(symbol, ptr, selfEdge) {
        return this.#internalConstructor(...arguments)
    }
}