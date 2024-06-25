// generated by diplomat-tool

import { Utf16Wrap } from "./Utf16Wrap.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";




const OpaqueMutexedString_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.OpaqueMutexedString_destroy(ptr);
});

export class OpaqueMutexedString {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        OpaqueMutexedString_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static fromUsize(number) {
        const result = wasm.OpaqueMutexedString_from_usize(number);
    
        try {
    
            return new OpaqueMutexedString(result, []);
        } finally {
        
        }
    }

    change(number) {
        wasm.OpaqueMutexedString_change(this.ffiValue, number);
    
        try {
    
        } finally {
        
        }
    }

    borrow() {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        const result = wasm.OpaqueMutexedString_borrow(this.ffiValue);
    
        try {
    
            return new OpaqueMutexedString(result, aEdges);
        } finally {
        
        }
    }

    static borrowOther(other) {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [other];
        const result = wasm.OpaqueMutexedString_borrow_other(other.ffiValue);
    
        try {
    
            return new OpaqueMutexedString(result, aEdges);
        } finally {
        
        }
    }

    borrowSelfOrOther(other) {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this, other];
        const result = wasm.OpaqueMutexedString_borrow_self_or_other(this.ffiValue, other.ffiValue);
    
        try {
    
            return new OpaqueMutexedString(result, aEdges);
        } finally {
        
        }
    }

    getLenAndAdd(other) {
        const result = wasm.OpaqueMutexedString_get_len_and_add(this.ffiValue, other);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    dummyStr() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        const result = wasm.OpaqueMutexedString_dummy_str(diplomat_receive_buffer, this.ffiValue);
    
        try {
    
            return diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, diplomat_receive_buffer, "string8");
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
        
        }
    }

    wrapper() {
        const result = wasm.OpaqueMutexedString_wrapper(this.ffiValue);
    
        try {
    
            return new Utf16Wrap(result, []);
        } finally {
        
        }
    }

    

}