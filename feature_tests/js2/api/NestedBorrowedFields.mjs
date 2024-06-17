// generated by diplomat-tool

import { Bar } from "./Bar.mjs"
import { BorrowedFields } from "./BorrowedFields.mjs"
import { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds.mjs"
import { Foo } from "./Foo.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class NestedBorrowedFields {
    #fields;
    get fields()  {
        return this.#fields;
    }
    set fields(value) {
        this.#fields = value;
    }
    #bounds;
    get bounds()  {
        return this.#bounds;
    }
    set bounds(value) {
        this.#bounds = value;
    }
    #bounds2;
    get bounds2()  {
        return this.#bounds2;
    }
    set bounds2(value) {
        this.#bounds2 = value;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    // If this struct contains any slices, their lifetime-edge-relevant objects will only
    // be constructed here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    //
    // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
    _intoFFI(xAppendArray = [], yAppendArray = [], zAppendArray = []) {
        return [
            ...fields._intoFfi(temp, [...xAppendArray]), 
            ...bounds._intoFfi(temp, [...xAppendArray], [...yAppendArray], [...yAppendArray]), 
            ...bounds2._intoFfi(temp, [...zAppendArray], [...zAppendArray], [...zAppendArray])]
    }
    

    _fromFFI(ptr, xEdges, yEdges, zEdges) {
        const fieldsDeref = ptr;
        this.#fields = BorrowedFields._fromFFI(fieldsDeref, xEdges);
        const boundsDeref = ptr;
        this.#bounds = BorrowedFieldsWithBounds._fromFFI(boundsDeref, xEdges, yEdges, yEdges);
        const bounds2Deref = ptr;
        this.#bounds2 = BorrowedFieldsWithBounds._fromFFI(bounds2Deref, zEdges, zEdges, zEdges);
    }
    static fromBarAndFooAndStrings(bar, foo, dstr16X, dstr16Z, utf8StrY, utf8StrZ) {
        
        const dstr16XSlice = diplomatRuntime.DiplomatBuf.str16(wasm, dstr16X);
        const dstr16XArena = new diplomatRuntime.DiplomatFinalizedArena();
        
        
        const dstr16ZSlice = diplomatRuntime.DiplomatBuf.str16(wasm, dstr16Z);
        const dstr16ZArena = new diplomatRuntime.DiplomatFinalizedArena();
        
        
        const utf8StrYSlice = diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrY);
        const utf8StrYArena = new diplomatRuntime.DiplomatFinalizedArena();
        
        
        const utf8StrZSlice = diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrZ);
        const utf8StrZArena = new diplomatRuntime.DiplomatFinalizedArena();
        
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(72, 4);
        
        // This lifetime edge depends on lifetimes 'x, 'y
        let xEdges = [bar, dstr16XSlice, utf8StrYSlice];
        
        // This lifetime edge depends on lifetimes 'y
        let yEdges = [bar, utf8StrYSlice];
        
        // This lifetime edge depends on lifetimes 'z
        let zEdges = [foo, dstr16ZSlice, utf8StrZSlice];
        const result = wasm.NestedBorrowedFields_from_bar_and_foo_and_strings(diplomat_receive_buffer, bar.ffiValue, foo.ffiValue, dstr16XSlice.ptr, dstr16XSlice.size, dstr16ZSlice.ptr, dstr16ZSlice.size, utf8StrYSlice.ptr, utf8StrYSlice.size, utf8StrZSlice.ptr, utf8StrZSlice.size);
    
        try {
    
            return NestedBorrowedFields._fromFFI(diplomat_receive_buffer, xEdges, yEdges, zEdges);
        } finally {
        
            dstr16XSlice.garbageCollect();
        
            dstr16ZSlice.garbageCollect();
        
            utf8StrYSlice.garbageCollect();
        
            utf8StrZSlice.garbageCollect();
        
            wasm.diplomat_free(diplomat_receive_buffer, 72, 4);
        
        }
    }

    

}