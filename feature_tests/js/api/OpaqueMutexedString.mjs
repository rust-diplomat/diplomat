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

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("OpaqueMutexedString is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            OpaqueMutexedString_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }


    static fromUsize(number) {

        const result = wasm.OpaqueMutexedString_from_usize(number);

        try {
            return new OpaqueMutexedString(diplomatRuntime.internalConstructor, result, []);
        }

        finally {}
    }

    change(number) {
    wasm.OpaqueMutexedString_change(this.ffiValue, number);

        try {}

        finally {}
    }

    borrow() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.OpaqueMutexedString_borrow(this.ffiValue);

        try {
            return new OpaqueMutexedString(diplomatRuntime.internalConstructor, result, aEdges);
        }

        finally {}
    }

    static borrowOther(other) {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [other];


        const result = wasm.OpaqueMutexedString_borrow_other(other.ffiValue);

        try {
            return new OpaqueMutexedString(diplomatRuntime.internalConstructor, result, aEdges);
        }

        finally {}
    }

    borrowSelfOrOther(other) {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this, other];


        const result = wasm.OpaqueMutexedString_borrow_self_or_other(this.ffiValue, other.ffiValue);

        try {
            return new OpaqueMutexedString(diplomatRuntime.internalConstructor, result, aEdges);
        }

        finally {}
    }

    getLenAndAdd(other) {

        const result = wasm.OpaqueMutexedString_get_len_and_add(this.ffiValue, other);

        try {
            return result;
        }

        finally {}
    }

    dummyStr() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);

        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.OpaqueMutexedString_dummy_str(diplomatReceive.buffer, this.ffiValue);

        try {
            return new diplomatRuntime.DiplomatSliceStr(wasm, diplomatReceive.buffer,  "string8", aEdges).getValue();
        }

        finally {        diplomatReceive.free();
        }
    }

    wrapper() {

        const result = wasm.OpaqueMutexedString_wrapper(this.ffiValue);

        try {
            return new Utf16Wrap(diplomatRuntime.internalConstructor, result, []);
        }

        finally {}
    }

    toUnsignedFromUnsigned(input) {

        const result = wasm.OpaqueMutexedString_to_unsigned_from_unsigned(this.ffiValue, input);

        try {
            return result;
        }

        finally {}
    }

    constructor(symbol, ptr, selfEdge) {
        return this.#internalConstructor(...arguments)
    }
}