// generated by diplomat-tool
import { OpaqueThin } from "./OpaqueThin.mjs"
import { OpaqueThinIter } from "./OpaqueThinIter.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const OpaqueThinVec_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.OpaqueThinVec_destroy(ptr);
});

export class OpaqueThinVec {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("OpaqueThinVec is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            OpaqueThinVec_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }
#defaultConstructor(a, b) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const aSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, a, "i32")));
        
        const bSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, b, "f32")));
        
        const result = wasm.OpaqueThinVec_create(aSlice.ptr, bSlice.ptr);
    
        try {
            return new OpaqueThinVec(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }
[Symbol.iterator]() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.OpaqueThinVec_iter(this.ffiValue);
    
        try {
            return new OpaqueThinIter(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {}
    }
len() {
        const result = wasm.OpaqueThinVec_len(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }
get(idx) {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.OpaqueThinVec_get(this.ffiValue, idx);
    
        try {
            return result === 0 ? null : new OpaqueThin(diplomatRuntime.internalConstructor, result, aEdges);
        }
        
        finally {}
    }
get first() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.OpaqueThinVec_first(this.ffiValue);
    
        try {
            return result === 0 ? null : new OpaqueThin(diplomatRuntime.internalConstructor, result, aEdges);
        }
        
        finally {}
    }

    constructor(a, b) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}