// generated by diplomat-tool
import { Two } from "./Two.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const One_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.One_destroy(ptr);
});

export class One {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("One is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            One_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    static transitivity(hold, nohold) {
        // This lifetime edge depends on lifetimes 'a, 'b, 'c, 'd, 'e
        let aEdges = [hold];
        
        const result = wasm.One_transitivity(hold.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }

    static cycle(hold, nohold) {
        // This lifetime edge depends on lifetimes 'a, 'b, 'c
        let aEdges = [hold];
        
        const result = wasm.One_cycle(hold.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }

    static manyDependents(a, b, c, d, nohold) {
        // This lifetime edge depends on lifetimes 'a, 'b, 'c, 'd
        let aEdges = [a, b, c, d];
        
        const result = wasm.One_many_dependents(a.ffiValue, b.ffiValue, c.ffiValue, d.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }

    static returnOutlivesParam(hold, nohold) {
        // This lifetime edge depends on lifetimes 'long
        let longEdges = [hold];
        
        const result = wasm.One_return_outlives_param(hold.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], longEdges);
        }
        
        finally {}
    }

    static diamondTop(top, left, right, bottom) {
        // This lifetime edge depends on lifetimes 'top, 'left, 'right, 'bottom
        let topEdges = [top, left, right, bottom];
        
        const result = wasm.One_diamond_top(top.ffiValue, left.ffiValue, right.ffiValue, bottom.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], topEdges);
        }
        
        finally {}
    }

    static diamondLeft(top, left, right, bottom) {
        // This lifetime edge depends on lifetimes 'left, 'bottom
        let leftEdges = [left, bottom];
        
        const result = wasm.One_diamond_left(top.ffiValue, left.ffiValue, right.ffiValue, bottom.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], leftEdges);
        }
        
        finally {}
    }

    static diamondRight(top, left, right, bottom) {
        // This lifetime edge depends on lifetimes 'right, 'bottom
        let rightEdges = [right, bottom];
        
        const result = wasm.One_diamond_right(top.ffiValue, left.ffiValue, right.ffiValue, bottom.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], rightEdges);
        }
        
        finally {}
    }

    static diamondBottom(top, left, right, bottom) {
        // This lifetime edge depends on lifetimes 'bottom
        let bottomEdges = [bottom];
        
        const result = wasm.One_diamond_bottom(top.ffiValue, left.ffiValue, right.ffiValue, bottom.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], bottomEdges);
        }
        
        finally {}
    }

    static diamondAndNestedTypes(a, b, c, d, nohold) {
        // This lifetime edge depends on lifetimes 'a, 'b, 'c, 'd
        let aEdges = [a, b, c, d];
        
        const result = wasm.One_diamond_and_nested_types(a.ffiValue, b.ffiValue, c.ffiValue, d.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }

    static implicitBounds(explicitHold, implicitHold, nohold) {
        // This lifetime edge depends on lifetimes 'a, 'b, 'c, 'd, 'x
        let aEdges = [explicitHold, implicitHold];
        
        const result = wasm.One_implicit_bounds(explicitHold.ffiValue, implicitHold.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }

    static implicitBoundsDeep(explicit, implicit1, implicit2, nohold) {
        // This lifetime edge depends on lifetimes 'a, 'b, 'c, 'd
        let aEdges = [explicit, implicit1, implicit2];
        
        const result = wasm.One_implicit_bounds_deep(explicit.ffiValue, implicit1.ffiValue, implicit2.ffiValue, nohold.ffiValue);
    
        try {
            return new One(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }

    constructor(symbol, ptr, selfEdge, aEdge) {
        return this.#internalConstructor(...arguments)
    }
}