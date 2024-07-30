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
            return result === 0 ? null : new OptionString(result, []);
        }
        
        finally {
            diplomatStrSlice.free();
        }
    }

    write() {
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        const result = wasm.OptionString_write(this.ffiValue, write.buffer);
    
        try {
            return result === 0 ? null : write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    borrow() {
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 9, 4, true);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        const result = wasm.OptionString_borrow(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return diplomatReceive.buffer.getString("string8");
        }
        
        finally {
            diplomatReceive.free();
        }
    }
}