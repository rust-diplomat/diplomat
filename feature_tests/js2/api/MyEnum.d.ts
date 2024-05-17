// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

// Base enumerator definition
export class MyEnum {
    constructor(value : MyEnum | string);

    get value() : string;

    get ffiValue() : number;

    static A : MyEnum;

    static B : MyEnum;

    static C : MyEnum;

    static D : MyEnum;

    static E : MyEnum;

    static F : MyEnum;


    intoValue(): number;

    static getA(): MyEnum;

}