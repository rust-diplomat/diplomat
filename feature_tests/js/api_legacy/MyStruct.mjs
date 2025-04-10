// generated by diplomat-tool
import { MyEnum } from "./MyEnum.mjs"
import { MyZst } from "./MyZst.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class MyStruct {
    #a;
    get a() {
        return this.#a;
    }
    set a(value){
        this.#a = value;
    }
    #b;
    get b() {
        return this.#b;
    }
    set b(value){
        this.#b = value;
    }
    #c;
    get c() {
        return this.#c;
    }
    set c(value){
        this.#c = value;
    }
    #d;
    get d() {
        return this.#d;
    }
    set d(value){
        this.#d = value;
    }
    #e;
    get e() {
        return this.#e;
    }
    set e(value){
        this.#e = value;
    }
    #f;
    get f() {
        return this.#f;
    }
    set f(value){
        this.#f = value;
    }
    #g;
    get g() {
        return this.#g;
    }
    set g(value){
        this.#g = value;
    }
    /** Create `MyStruct` from an object that contains all of `MyStruct`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new MyStruct(diplomatRuntime.exposeConstructor, structObj);
    }

    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("MyStruct's constructor takes an object of MyStruct's fields.");
        }

        if ("a" in structObj) {
            this.#a = structObj.a;
        } else {
            throw new Error("Missing required field a.");
        }

        if ("b" in structObj) {
            this.#b = structObj.b;
        } else {
            throw new Error("Missing required field b.");
        }

        if ("c" in structObj) {
            this.#c = structObj.c;
        } else {
            throw new Error("Missing required field c.");
        }

        if ("d" in structObj) {
            this.#d = structObj.d;
        } else {
            throw new Error("Missing required field d.");
        }

        if ("e" in structObj) {
            this.#e = structObj.e;
        } else {
            throw new Error("Missing required field e.");
        }

        if ("f" in structObj) {
            this.#f = structObj.f;
        } else {
            throw new Error("Missing required field f.");
        }

        if ("g" in structObj) {
            this.#g = structObj.g;
        } else {
            throw new Error("Missing required field g.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#a, this.#b, this.#c, /* [5 x i8] padding */ 0, 0, 0, 0, 0 /* end padding */, this.#d, this.#e, this.#f, this.#g.ffiValue, /* [1 x i32] padding */ 0 /* end padding */]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof MyStruct) {
            return obj;
        }

        return MyStruct.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#a, Uint8Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 1, this.#b, Uint8Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 2, this.#c, Uint8Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 8, this.#d, BigUint64Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 16, this.#e, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 20, this.#f, Uint32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 24, this.#g.ffiValue, Int32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("MyStruct._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const aDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0];
        structObj.a = aDeref;
        const bDeref = (new Uint8Array(wasm.memory.buffer, ptr + 1, 1))[0] === 1;
        structObj.b = bDeref;
        const cDeref = (new Uint8Array(wasm.memory.buffer, ptr + 2, 1))[0];
        structObj.c = cDeref;
        const dDeref = (new BigUint64Array(wasm.memory.buffer, ptr + 8, 1))[0];
        structObj.d = dDeref;
        const eDeref = (new Int32Array(wasm.memory.buffer, ptr + 16, 1))[0];
        structObj.e = eDeref;
        const fDeref = (new Uint32Array(wasm.memory.buffer, ptr + 20, 1))[0];
        structObj.f = fDeref;
        const gDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 24);
        structObj.g = new MyEnum(diplomatRuntime.internalConstructor, gDeref);

        return new MyStruct(diplomatRuntime.exposeConstructor, structObj);
    }
    #defaultConstructor() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 32, 8, false);


        const result = wasm.MyStruct_new(diplomatReceive.buffer);

        try {        return MyStruct._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }

        finally {        diplomatReceive.free();
        }
    }
    intoA() {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();


        const result = wasm.MyStruct_into_a(...MyStruct._fromSuppliedValue(diplomatRuntime.internalConstructor, this)._intoFFI(functionCleanupArena, {}));

        try {        return result;
        }

        finally {
            functionCleanupArena.free();
        }
    }
    static returnsZstResult() {

        const result = wasm.MyStruct_returns_zst_result();

        try {        if (result !== 1) {
                const cause = MyZst.fromFields({}, diplomatRuntime.internalConstructor);
                throw new globalThis.Error('MyZst', { cause });
            }
        }

        finally {}
    }
    static failsZstResult() {

        const result = wasm.MyStruct_fails_zst_result();

        try {        if (result !== 1) {
                const cause = MyZst.fromFields({}, diplomatRuntime.internalConstructor);
                throw new globalThis.Error('MyZst', { cause });
            }
        }

        finally {}
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}