// generated by diplomat-tool

import { Foo } from "./Foo.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class BorrowedFieldsWithBounds {
    #fieldA;
    get fieldA()  {
        return this.#fieldA;
    }
    set fieldA(value) {
        this.#fieldA = value;
    }
    #fieldB;
    get fieldB()  {
        return this.#fieldB;
    }
    set fieldB(value) {
        this.#fieldB = value;
    }
    #fieldC;
    get fieldC()  {
        return this.#fieldC;
    }
    set fieldC(value) {
        this.#fieldC = value;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    // If this struct contains any slices, their lifetime-edge-relevant objects will only
    // be constructed here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    //
    // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
    _intoFFI(aAppendArray = [], bAppendArray = [], cAppendArray = []) {
        return [
            diplomatRuntime.DiplomatBuf.str16(wasm, fieldA) /* TODO: Freeing code */, 
            diplomatRuntime.DiplomatBuf.str8(wasm, fieldB) /* TODO: Freeing code */, 
            diplomatRuntime.DiplomatBuf.str8(wasm, fieldC) /* TODO: Freeing code */]
    }
    

    _fromFFI(ptr, aEdges, bEdges, cEdges) {
        const fieldADeref = /* TODO: gen_c_to_js_deref */null;
        this.#fieldA = fieldADeref(aEdges) // TODO: Slice c_to_js;
        const fieldBDeref = /* TODO: gen_c_to_js_deref */null;
        this.#fieldB = fieldBDeref(bEdges) // TODO: Slice c_to_js;
        const fieldCDeref = /* TODO: gen_c_to_js_deref */null;
        this.#fieldC = fieldCDeref(cEdges) // TODO: Slice c_to_js;

        return this;
    }
    static fromFooAndStrings(foo, dstr16X, utf8StrZ) {
        
        const dstr16XSlice = diplomatRuntime.DiplomatBuf.str16(wasm, dstr16X);
        const dstr16XArena = new diplomatRuntime.DiplomatFinalizedArena();
        
        
        const utf8StrZSlice = diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrZ);
        const utf8StrZArena = new diplomatRuntime.DiplomatFinalizedArena();
        
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(24, 4);
        
        // This lifetime edge depends on lifetimes 'x, 'y, 'z
        let xEdges = [foo, dstr16XSlice, utf8StrZSlice];
        
        // This lifetime edge depends on lifetimes 'y, 'z
        let yEdges = [foo, utf8StrZSlice];
        
        // This lifetime edge depends on lifetimes 'z
        let zEdges = [utf8StrZSlice];
        const result = wasm.BorrowedFieldsWithBounds_from_foo_and_strings(diplomat_receive_buffer, foo.ffiValue, dstr16XSlice.ptr, dstr16XSlice.size, utf8StrZSlice.ptr, utf8StrZSlice.size);
    
        try {
    
            return new BorrowedFieldsWithBounds()._fromFFI(diplomat_receive_buffer, xEdges, yEdges, zEdges);
        } finally {
        
            dstr16XSlice.garbageCollect();
        
            utf8StrZSlice.garbageCollect();
        
            wasm.diplomat_free(diplomat_receive_buffer, 24, 4);
        
        }
    }

    

}