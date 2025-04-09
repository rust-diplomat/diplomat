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

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("MyString is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            MyString_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }
    #defaultConstructor(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, v)));

        const result = wasm.MyString_new(vSlice.ptr);

        try {        return new MyString(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();
        }
    }
    static newUnsafe(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, v)));

        const result = wasm.MyString_new_unsafe(vSlice.ptr);

        try {        return new MyString(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();
        }
    }
    static newOwned(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, v)));

        const result = wasm.MyString_new_owned(vSlice.ptr);

        try {        return new MyString(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();
        }
    }
    static newFromFirst(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.strs(wasm, v, "string8")));

        const result = wasm.MyString_new_from_first(vSlice.ptr);

        try {        return new MyString(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
            functionCleanupArena.free();
        }
    }
    set str(newStr) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const newStrSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, newStr)));
        wasm.MyString_set_str(this.ffiValue, newStrSlice.ptr);

        try {}

        finally {
            functionCleanupArena.free();
        }
    }
    get str() {    const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.MyString_get_str(this.ffiValue, write.buffer);

        try {        return write.readString8();
        }

        finally {        write.free();
        }
    }
    static getStaticStr() {    const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);

        const result = wasm.MyString_get_static_str(diplomatReceive.buffer);

        try {        return new diplomatRuntime.DiplomatSliceStr(wasm, diplomatReceive.buffer,  "string8", []).getValue();
        }

        finally {        diplomatReceive.free();
        }
    }
    static stringTransform(foo) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const fooSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, foo)));
            const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.MyString_string_transform(fooSlice.ptr, write.buffer);

        try {        return write.readString8();
        }

        finally {
            functionCleanupArena.free();
                write.free();
        }
    }
    borrow() {    const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);

        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];

        const result = wasm.MyString_borrow(diplomatReceive.buffer, this.ffiValue);

        try {        return new diplomatRuntime.DiplomatSliceStr(wasm, diplomatReceive.buffer,  "string8", aEdges).getValue();
        }

        finally {        diplomatReceive.free();
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