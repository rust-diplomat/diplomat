// generated by diplomat-tool
import { UnimportedEnum } from "./UnimportedEnum.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class ImportedStruct {
	

    #foo;
    get foo()  {
        return this.#foo;
    }
    set foo(value) {
        this.#foo = value;
    }

    #count;
    get count()  {
        return this.#count;
    }
    set count(value) {
        this.#count = value;
    }

    /** Create `ImportedStruct` from an object that contains all of `ImportedStruct`'s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj) {
        return new ImportedStruct(structObj);
    }
    
    constructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("ImportedStruct's constructor takes an object of ImportedStruct's fields.");
        }

        if ("foo" in structObj) {
            this.#foo = structObj.foo;
        } else {
            throw new Error("Missing required field foo.");
        }

        if ("count" in structObj) {
            this.#count = structObj.count;
        } else {
            throw new Error("Missing required field count.");
        }

    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    // JS structs need to be generated with or without padding depending on whether they are being passed as aggregates or splatted out into fields.
    // Most of the time this is known beforehand: large structs (>2 scalar fields) always get padding, and structs passed directly in parameters omit padding
    // if they are small. However small structs within large structs also get padding, and we signal that by setting forcePadding.
    _intoFFI(
        functionCleanupArena,
        appendArrayMap,
        forcePadding
    ) {
        return [this.#foo.ffiValue, this.#count, ...diplomatRuntime.maybePaddingFields(forcePadding, 3 /* x i8 */)]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof ImportedStruct) {
            return obj;
        }

        return new ImportedStruct(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap,
        forcePadding
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#foo.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#count, Uint8Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("ImportedStruct._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        var structObj = {};
        const fooDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        structObj.foo = new UnimportedEnum(diplomatRuntime.internalConstructor, fooDeref);
        const countDeref = (new Uint8Array(wasm.memory.buffer, ptr + 4, 1))[0];
        structObj.count = countDeref;

        return new ImportedStruct(structObj, internalConstructor);
    }

}