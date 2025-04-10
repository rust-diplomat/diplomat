// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const Utf16Wrap_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Utf16Wrap_destroy(ptr);
});

export class Utf16Wrap {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Utf16Wrap is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Utf16Wrap_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }
#defaultConstructor(input) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const inputSlice = diplomatRuntime.DiplomatBuf.str16(wasm, input);
        
        const result = wasm.Utf16Wrap_from_utf16(...inputSlice.splat());
    
        try {
            return new Utf16Wrap(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }
getDebugStr() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.Utf16Wrap_get_debug_str(this.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }
borrowCont() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.Utf16Wrap_borrow_cont(diplomatReceive.buffer, this.ffiValue);
    
        try {
            return new diplomatRuntime.DiplomatSliceStr(wasm, diplomatReceive.buffer,  "string16", aEdges).getValue();
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    constructor(input) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}