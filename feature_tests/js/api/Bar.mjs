// generated by diplomat-tool
import { Foo } from "./Foo.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const Bar_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Bar_destroy(ptr);
});

export class Bar {
	
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #bEdge = [];
    #aEdge = [];
    
    constructor(symbol, ptr, selfEdge, bEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Bar is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#bEdge = bEdge;
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Bar_box_destroy_registry.register(this, this.#ptr);
        }
    }
    get ffiValue() {
        return this.#ptr;
    }

    get foo() {
        // This lifetime edge depends on lifetimes 'b, 'a
        let bEdges = [this];
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.Bar_foo(this.ffiValue);
    
        try {
            return new Foo(diplomatRuntime.internalConstructor, result, bEdges, aEdges);
        }
        
        finally {}
    }
}