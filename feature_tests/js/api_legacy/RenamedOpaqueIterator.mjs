// generated by diplomat-tool
import { AttrOpaque1Renamed } from "./AttrOpaque1Renamed.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const RenamedOpaqueIterator_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.namespace_OpaqueIterator_destroy(ptr);
});

export class RenamedOpaqueIterator {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];

    #internalConstructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("RenamedOpaqueIterator is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#aEdge = aEdge;
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            RenamedOpaqueIterator_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }
    #iteratorNext() {
        const result = wasm.namespace_OpaqueIterator_next(this.ffiValue);

        try {        return result === 0 ? null : new AttrOpaque1Renamed(diplomatRuntime.internalConstructor, result, []);

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