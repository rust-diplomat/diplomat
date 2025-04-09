// generated by diplomat-tool
import { OpaqueThin } from "./OpaqueThin.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const OpaqueThinIter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.OpaqueThinIter_destroy(ptr);
});

export class OpaqueThinIter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];

    #internalConstructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("OpaqueThinIter is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#aEdge = aEdge;
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            OpaqueThinIter_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }
    #iteratorNext() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];

        const result = wasm.OpaqueThinIter_next(this.ffiValue);

        try {        return result === 0 ? null : new OpaqueThin(diplomatRuntime.internalConstructor, result, aEdges);
        }

        finally {}
    }

    next() {
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