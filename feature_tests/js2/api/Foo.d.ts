// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"



export class Foo {
    

    get ffiValue(): pointer;


    constructor(x: String): Foo;

    get bar(): Bar;

    static static(x: String): Foo;

    asReturning(): BorrowedFieldsReturning;

    static extractFromFields(fields: BorrowedFields): Foo;

    static extractFromBounds(bounds: BorrowedFieldsWithBounds, anotherString: String): Foo;

}