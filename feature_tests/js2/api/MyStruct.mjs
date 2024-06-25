// generated by diplomat-tool

import { MyEnum } from "./MyEnum.mjs"
import { MyZst } from "./MyZst.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class MyStruct {
    #a;
    get a()  {
        return this.#a;
    }
    set a(value) {
        this.#a = value;
    }
    #b;
    get b()  {
        return this.#b;
    }
    set b(value) {
        this.#b = value;
    }
    #c;
    get c()  {
        return this.#c;
    }
    set c(value) {
        this.#c = value;
    }
    #d;
    get d()  {
        return this.#d;
    }
    set d(value) {
        this.#d = value;
    }
    #e;
    get e()  {
        return this.#e;
    }
    set e(value) {
        this.#e = value;
    }
    #f;
    get f()  {
        return this.#f;
    }
    set f(value) {
        this.#f = value;
    }
    #g;
    get g()  {
        return this.#g;
    }
    set g(value) {
        this.#g = value;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI() {
        return [
            this.#a, 
            this.#b, 
            this.#c, 
            this.#d, 
            this.#e, 
            diplomatRuntime.extractCodePoint(this.#f, 'this.#f'), 
            this.#g.ffiValue]
    }
    

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    _fromFFI(ptr) {
        const aDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0];
        this.#a = aDeref;
        const bDeref = (new Uint8Array(wasm.memory.buffer, ptr + 1, 1))[0] == 1;
        this.#b = bDeref;
        const cDeref = (new Uint8Array(wasm.memory.buffer, ptr + 2, 1))[0];
        this.#c = cDeref;
        const dDeref = (new BigUint64Array(wasm.memory.buffer, ptr + 8, 1))[0];
        this.#d = dDeref;
        const eDeref = (new Int32Array(wasm.memory.buffer, ptr + 16, 1))[0];
        this.#e = eDeref;
        const fDeref = String.fromCharCode((new Uint32Array(wasm.memory.buffer, ptr + 20, 1))[0]);
        this.#f = fDeref;
        const gDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 24);
        this.#g = (() => {for (let i of MyEnum.values) { if(i[1] === gDeref) return MyEnum[i[0]]; } return null;})();;

        return this;
    }
    static new_() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(28, 8);
        const result = wasm.MyStruct_new(diplomat_receive_buffer);
    
        try {
    
            return new MyStruct()._fromFFI(diplomat_receive_buffer);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 28, 8);
        
        }
    }

    intoA() {
        const result = wasm.MyStruct_into_a(...this._intoFFI());
    
        try {
    
            return result;
        } finally {
        
        }
    }

    static returnsZstResult() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.MyStruct_returns_zst_result(diplomat_receive_buffer);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                throw new diplomatRuntime.FFIError(new MyZst()._fromFFI(diplomat_receive_buffer));
            }
    
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    

}