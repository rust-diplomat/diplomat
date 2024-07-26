// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const OptionString_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.OptionString_destroy(ptr);
});

export class OptionString {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        OptionString_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }

    static new_(diplomatStr) {
        
        const diplomatStrSlice = diplomatRuntime.DiplomatBuf.str8(wasm, diplomatStr);
        const result = wasm.OptionString_new(diplomatStrSlice.ptr, diplomatStrSlice.size);
    
        try {
    
            return result == 0 ? null : new OptionString(result, []);
        } finally {
        
            diplomatStrSlice.free();
        
        }
    }

    write() {
        
        const write = wasm.diplomat_buffer_write_create(0);
        const result = wasm.OptionString_write(this.ffiValue, write);
    
        try {
    
            return result == 0 ? null : diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    borrow() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        const result = wasm.OptionString_borrow(diplomat_receive_buffer, this.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 8)) {
                return null;
            }
            return diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, diplomat_receive_buffer, "string8");
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        
        }
    }
}