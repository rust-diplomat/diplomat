// generated by diplomat-tool
import { ErrorEnum } from "./ErrorEnum.mjs"
import { ErrorStruct } from "./ErrorStruct.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const ResultOpaque_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.ResultOpaque_destroy(ptr);
});

export class ResultOpaque {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("ResultOpaque is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            ResultOpaque_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }


    #defaultConstructor(i) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new(diplomatReceive.buffer, i);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new ErrorEnum(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('ErrorEnum: ' + cause.value, { cause });
            }
            return new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {        diplomatReceive.free();
        }
    }

    static newFailingFoo() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new_failing_foo(diplomatReceive.buffer);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new ErrorEnum(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('ErrorEnum: ' + cause.value, { cause });
            }
            return new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {        diplomatReceive.free();
        }
    }

    static newFailingBar() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new_failing_bar(diplomatReceive.buffer);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new ErrorEnum(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('ErrorEnum: ' + cause.value, { cause });
            }
            return new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {        diplomatReceive.free();
        }
    }

    static newFailingUnit() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new_failing_unit(diplomatReceive.buffer);

        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {        diplomatReceive.free();
        }
    }

    static newFailingStruct(i) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 9, 4, true);


        const result = wasm.ResultOpaque_new_failing_struct(diplomatReceive.buffer, i);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = ErrorStruct._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
                throw new globalThis.Error('ErrorStruct: ' + cause.toString(), { cause });
            }
            return new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {        diplomatReceive.free();
        }
    }

    static newInErr(i) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new_in_err(diplomatReceive.buffer, i);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
                throw new globalThis.Error('ResultOpaque: ' + cause.toString(), { cause });
            }
        }

        finally {        diplomatReceive.free();
        }
    }

    static newInt(i) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new_int(diplomatReceive.buffer, i);

        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Int32Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0];
        }

        finally {        diplomatReceive.free();
        }
    }

    static newInEnumErr(i) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.ResultOpaque_new_in_enum_err(diplomatReceive.buffer, i);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new ResultOpaque(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
                throw new globalThis.Error('ResultOpaque: ' + cause.toString(), { cause });
            }
            return new ErrorEnum(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
        }

        finally {        diplomatReceive.free();
        }
    }

    /**
     * When we take &str, the return type becomes a Result
     * Test that this interacts gracefully with returning a reference type
     */
    takesStr(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.ResultOpaque_takes_str(this.ffiValue, ...vSlice.splat());

        try {
            return new ResultOpaque(diplomatRuntime.internalConstructor, result, aEdges);
        }

        finally {
            functionCleanupArena.free();
        }
    }

    assertInteger(i) {
    wasm.ResultOpaque_assert_integer(this.ffiValue, i);

        try {}

        finally {}
    }

    constructor(i) {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}