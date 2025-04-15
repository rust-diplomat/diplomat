// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const RenamedMyIterator_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_MyIterator_destroy(ptr);
});

export class RenamedMyIterator {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];

    #internalConstructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("RenamedMyIterator is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#aEdge = aEdge;
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            RenamedMyIterator_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }


    #iteratorNext() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);


        const result = wasm.namespace_MyIterator_next(diplomatReceive.buffer, this.ffiValue);

        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0];
        }

        finally {        diplomatReceive.free();
        }
    }

    next(){
        const out = this.#iteratorNext();

        return {
            value: out,
            done: out === null,
        };
    }

    constructor(symbol, ptr, selfEdge, aEdge) {
        return this.#internalConstructor(...arguments)
    }
}