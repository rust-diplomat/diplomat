// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const Float64Vec_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Float64Vec_destroy(ptr);
});

export class Float64Vec {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Float64Vec is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Float64Vec_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }


    static newBool(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "boolean")));

        const result = wasm.Float64Vec_new_bool(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    static newI16(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "i16")));

        const result = wasm.Float64Vec_new_i16(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    static newU16(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "u16")));

        const result = wasm.Float64Vec_new_u16(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    static newIsize(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "i32")));

        const result = wasm.Float64Vec_new_isize(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    static newUsize(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "u32")));

        const result = wasm.Float64Vec_new_usize(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    static newF64BeBytes(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "u8")));

        const result = wasm.Float64Vec_new_f64_be_bytes(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    #defaultConstructor(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "f64")));

        const result = wasm.Float64Vec_new_from_owned(vSlice.ptr);

        try {
            return new Float64Vec(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();

        }
    }

    get asSlice() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);

        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.Float64Vec_as_slice(diplomatReceive.buffer, this.ffiValue);

        try {
            return Array.from(new diplomatRuntime.DiplomatSlicePrimitive(wasm, diplomatReceive.buffer, "f64", aEdges).getValue());
        }

        finally {
            diplomatReceive.free();
        }
    }

    fillSlice(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, v, "f64")));
    wasm.Float64Vec_fill_slice(this.ffiValue, vSlice.ptr);

        try {}

        finally {
            functionCleanupArena.free();

        }
    }

    setValue(newSlice) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const newSliceSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.slice(wasm, newSlice, "f64")));
    wasm.Float64Vec_set_value(this.ffiValue, newSliceSlice.ptr);

        try {}

        finally {
            functionCleanupArena.free();

        }
    }

    toString() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);

    wasm.Float64Vec_to_string(this.ffiValue, write.buffer);

        try {
            return write.readString8();
        }

        finally {
            write.free();
        }
    }

    borrow() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);

        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.Float64Vec_borrow(diplomatReceive.buffer, this.ffiValue);

        try {
            return Array.from(new diplomatRuntime.DiplomatSlicePrimitive(wasm, diplomatReceive.buffer, "f64", aEdges).getValue());
        }

        finally {
            diplomatReceive.free();
        }
    }

    get(i) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 9, 8, true);


        const result = wasm.Float64Vec_get(diplomatReceive.buffer, this.ffiValue, i);

        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Float64Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0];
        }

        finally {
            diplomatReceive.free();
        }
    }

    constructor(v) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}