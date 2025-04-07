// generated by diplomat-tool
import { ImportedStruct } from "./ImportedStruct.mjs"
import { MyStruct } from "./MyStruct.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const Opaque_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Opaque_destroy(ptr);
});

export class Opaque {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Opaque is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Opaque_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }
#defaultConstructor() {
        const result = wasm.Opaque_new();
    
        try {
            return new Opaque(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }
static tryFromUtf8(input) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const inputSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, input)));
        
        const result = wasm.Opaque_try_from_utf8(...inputSlice.splat());
    
        try {
            return result === 0 ? null : new Opaque(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }
static fromStr(input) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const inputSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, input)));
        
        const result = wasm.Opaque_from_str(...inputSlice.splat());
    
        try {
            return new Opaque(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }
getDebugStr() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.Opaque_get_debug_str(this.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    /** 
     * See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
     *
     * See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
     *
     * Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
     */
    assertStruct(s) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        wasm.Opaque_assert_struct(this.ffiValue, ...MyStruct._fromSuppliedValue(diplomatRuntime.internalConstructor, s)._intoFFI(functionCleanupArena, {}));
    
        try {}
        
        finally {
            functionCleanupArena.free();
        }
    }
static returnsUsize() {
        const result = wasm.Opaque_returns_usize();
    
        try {
            return result;
        }
        
        finally {}
    }
static returnsImported() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);
        
        const result = wasm.Opaque_returns_imported(diplomatReceive.buffer);
    
        try {
            return ImportedStruct._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }
static cmp() {
        const result = wasm.Opaque_cmp();
    
        try {
            return result;
        }
        
        finally {}
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}