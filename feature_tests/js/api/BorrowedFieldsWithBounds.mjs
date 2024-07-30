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
    // If this struct contains any slices, their lifetime-edge-relevant information will be
    // set up here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
    // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
    // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
    //
    // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
    _intoFFI(
        slice_cleanup_callbacks,
        appendArrayMap
    ) {
        slice_cleanup_callbacks.push((appendArrayMap[aAppendArray] || []).length > 0 ? () => { for (let lifetime of appendArrayMap[aAppendArray]) { appendArrayMap[aAppendArray].push(fieldA); } fieldA.garbageCollect(); } : fieldA.free);
        
        slice_cleanup_callbacks.push((appendArrayMap[bAppendArray] || []).length > 0 ? () => { for (let lifetime of appendArrayMap[bAppendArray]) { appendArrayMap[bAppendArray].push(fieldB); } fieldB.garbageCollect(); } : fieldB.free);
        
        slice_cleanup_callbacks.push((appendArrayMap[cAppendArray] || []).length > 0 ? () => { for (let lifetime of appendArrayMap[cAppendArray]) { appendArrayMap[cAppendArray].push(fieldC); } fieldC.garbageCollect(); } : fieldC.free);
        
        return [diplomatRuntime.DiplomatBuf.str16(wasm, this.#fieldA), diplomatRuntime.DiplomatBuf.str8(wasm, this.#fieldB), diplomatRuntime.DiplomatBuf.str8(wasm, this.#fieldC)]
    }

    _fromFFI(ptr, aEdges, bEdges, cEdges) {
        const fieldADeref = ptr;
        this.#fieldA = fieldADeref.getString("string16");
        const fieldBDeref = ptr + 8;
        this.#fieldB = fieldBDeref.getString("string8");
        const fieldCDeref = ptr + 16;
        this.#fieldC = fieldCDeref.getString("string8");

        return this;
    }

    // Return all fields corresponding to lifetime `'a` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'a`,
    // assuming that there are no `'other: a`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeA() { 
        return [fieldA];
    };

    // Return all fields corresponding to lifetime `'b` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'b`,
    // assuming that there are no `'other: b`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeB() { 
        return [fieldB];
    };

    // Return all fields corresponding to lifetime `'c` 
    // without handling lifetime dependencies (this is the job of the caller)
    // This is all fields that may be borrowed from if borrowing `'c`,
    // assuming that there are no `'other: c`. bounds. In case of such bounds,
    // the caller should take care to also call _fieldsForLifetimeOther
    get _fieldsForLifetimeC() { 
        return [fieldC];
    };

    static fromFooAndStrings(foo, dstr16X, utf8StrZ) {
        
        const dstr16XSlice = diplomatRuntime.DiplomatBuf.str16(wasm, dstr16X);
        
        const utf8StrZSlice = diplomatRuntime.DiplomatBuf.str8(wasm, utf8StrZ);
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 24, 4, false);
        
        // This lifetime edge depends on lifetimes 'x, 'y, 'z
        let xEdges = [foo, dstr16XSlice, utf8StrZSlice];
        
        // This lifetime edge depends on lifetimes 'y, 'z
        let yEdges = [foo, utf8StrZSlice];
        
        // This lifetime edge depends on lifetimes 'z
        let zEdges = [utf8StrZSlice];
        const result = wasm.BorrowedFieldsWithBounds_from_foo_and_strings(diplomatReceive.buffer, foo.ffiValue, dstr16XSlice.ptr, dstr16XSlice.size, utf8StrZSlice.ptr, utf8StrZSlice.size);
    
        try {
            return new BorrowedFieldsWithBounds()._fromFFI(diplomatReceive.buffer, xEdges, yEdges, zEdges);
        }
        
        finally {
            dstr16XSlice.garbageCollect();
        
            utf8StrZSlice.garbageCollect();
        
            diplomatReceive.free();
        }
    }
}