// generated by diplomat-tool

import { Bar } from "./Bar.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class BorrowedFields {
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

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    // If this struct contains any slices, their lifetime-edge-relevant information will be
    // set up here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    _intoFFI(
        slice_cleanup_callbacks,
        appendArrayMap
    ) {
        slice_cleanup_callbacks.push((appendArrayMap[aAppendArray] || []).length > 0 ? () => { for (let lifetime of appendArrayMap[aAppendArray]) { appendArrayMap[aAppendArray].push(a); } a.garbageCollect(); } : a.free);
        
        slice_cleanup_callbacks.push((appendArrayMap[aAppendArray] || []).length > 0 ? () => { for (let lifetime of appendArrayMap[aAppendArray]) { appendArrayMap[aAppendArray].push(b); } b.garbageCollect(); } : b.free);
        
        slice_cleanup_callbacks.push((appendArrayMap[aAppendArray] || []).length > 0 ? () => { for (let lifetime of appendArrayMap[aAppendArray]) { appendArrayMap[aAppendArray].push(c); } c.garbageCollect(); } : c.free);
        
        return [diplomatRuntime.DiplomatBuf.str16(wasm, this.#a), diplomatRuntime.DiplomatBuf.str8(wasm, this.#b), diplomatRuntime.DiplomatBuf.str8(wasm, this.#c)]
    }

    _fromFFI(ptr, aEdges) {
        const aDeref = ptr;
        this.#a = diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, aDeref, "string16");
        const bDeref = ptr + 8;
        this.#b = diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, bDeref, "string8");
        const cDeref = ptr + 16;
        this.#c = diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, cDeref, "string8");

        return this;
    }

    // Return all fields corresponding to lifetime `'a` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'a`,
    // assuming that there are no `'other: a`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeA() { 
        return [a, b, c];
    };
    static fromBarAndStrings(bar, dstr16, utf8Str) {
        
        const dstr16Slice = diplomatRuntime.DiplomatBuf.str16(wasm, dstr16);
        
        const utf8StrSlice = diplomatRuntime.DiplomatBuf.str8(wasm, utf8Str);
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(24, 4);
        
        // This lifetime edge depends on lifetimes 'x
        let xEdges = [bar, dstr16Slice, utf8StrSlice];
        const result = wasm.BorrowedFields_from_bar_and_strings(diplomat_receive_buffer, bar.ffiValue, dstr16Slice.ptr, dstr16Slice.size, utf8StrSlice.ptr, utf8StrSlice.size);
    
        try {
    
            return new BorrowedFields()._fromFFI(diplomat_receive_buffer, xEdges);
        } finally {
        
            dstr16Slice.garbageCollect();
        
            utf8StrSlice.garbageCollect();
        
            wasm.diplomat_free(diplomat_receive_buffer, 24, 4);
        
        }
    }

    

}