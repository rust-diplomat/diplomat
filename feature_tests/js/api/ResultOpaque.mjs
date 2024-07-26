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
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        ResultOpaque_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static new_(i) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new(diplomat_receive_buffer, i);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = ErrorEnum[Array.from(ErrorEnum.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('ErrorEnum: ' + cause.value, { cause });
            }
            return new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static newFailingFoo() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new_failing_foo(diplomat_receive_buffer);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = ErrorEnum[Array.from(ErrorEnum.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('ErrorEnum: ' + cause.value, { cause });
            }
            return new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static newFailingBar() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new_failing_bar(diplomat_receive_buffer);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = ErrorEnum[Array.from(ErrorEnum.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('ErrorEnum: ' + cause.value, { cause });
            }
            return new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static newFailingUnit() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new_failing_unit(diplomat_receive_buffer);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                return null;
            }
            return new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static newFailingStruct(i) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
        const result = wasm.ResultOpaque_new_failing_struct(diplomat_receive_buffer, i);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 8)) {
                const cause = new ErrorStruct()._fromFFI(diplomat_receive_buffer);
                throw new Error('ErrorStruct: ' + cause.toString(), { cause });
            }
            return new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
        
        }
    }

    static newInErr(i) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new_in_err(diplomat_receive_buffer, i);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
                throw new Error('ResultOpaque: ' + cause.toString(), { cause });
            }
    
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static newInt(i) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new_int(diplomat_receive_buffer, i);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                return null;
            }
            return (new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0];
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static newInEnumErr(i) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ResultOpaque_new_in_enum_err(diplomat_receive_buffer, i);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = new ResultOpaque(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
                throw new Error('ResultOpaque: ' + cause.toString(), { cause });
            }
            return ErrorEnum[Array.from(ErrorEnum.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    assertInteger(i) {
        wasm.ResultOpaque_assert_integer(this.ffiValue, i);
    
        try {
    
        } finally {
        
        }
    }

    

}