// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"



const MyString_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.MyString_destroy(ptr);
});

export class MyString {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        if (this.#selfEdge.length === 0) {
            MyString_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new MyString(result, []);
    }

    static newUnsafe(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new_unsafe(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new MyString(result, []);
    }

    static newOwned(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new_owned(vSlice.ptr, vSlice.size);
    
        return new MyString(result, []);
    }

    static newFromFirst(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        const result = wasm.MyString_new_from_first(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new MyString(result, []);
    }

    set str(newStr) {
        
        const newStrSlice = diplomatRuntime.DiplomatBuf.str8(wasm, newStr);
        wasm.MyString_set_str(this.ffiValue, newStrSlice.ptr, newStrSlice.size);
    
        newStrSlice.free();
        
    }

    get str() {
        wasm.MyString_get_str(this.ffiValue);
    
        return writeable;
    }

    getBoxedStr() {
        const result = wasm.MyString_get_boxed_str(this.ffiValue);
    
        return result // TODO: Slice c_to_js;
    }

    

}