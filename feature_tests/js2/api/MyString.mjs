// generated by diplomat-tool

import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";




const MyString_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.MyString_destroy(ptr);
});

export class MyString {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        MyString_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new(vSlice.ptr, vSlice.size);
    
        try {
    
            return new MyString(result, []);
        } finally {
        
            vSlice.free();
        
        }
    }

    static newUnsafe(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new_unsafe(vSlice.ptr, vSlice.size);
    
        try {
    
            return new MyString(result, []);
        } finally {
        
            vSlice.free();
        
        }
    }

    static newOwned(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new_owned(vSlice.ptr, vSlice.size);
    
        try {
    
            return new MyString(result, []);
        } finally {
        
        }
    }

    static newFromFirst(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new_from_first(vSlice.ptr, vSlice.size);
    
        try {
    
            return new MyString(result, []);
        } finally {
        
            vSlice.free();
        
        }
    }

    set str(newStr) {
        
        const newStrSlice = diplomatRuntime.DiplomatBuf.str8(wasm, newStr);
        wasm.MyString_set_str(this.ffiValue, newStrSlice.ptr, newStrSlice.size);
    
        try {
    
        } finally {
        
            newStrSlice.free();
        
        }
    }

    get str() {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.MyString_get_str(this.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    getBoxedStr() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
        const result = wasm.MyString_get_boxed_str(diplomat_receive_buffer, this.ffiValue);
    
        try {
    
            return diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, diplomat_receive_buffer, "string8");
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
        
        }
    }

    

}