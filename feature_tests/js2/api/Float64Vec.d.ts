// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"



export class Float64Vec {
    

    get ffiValue(): pointer;


    static bool(v: Array<bool>): Float64Vec;

    static i16(v: Array<number>): Float64Vec;

    static u16(v: Array<number>): Float64Vec;

    static isize(v: Array<number>): Float64Vec;

    static usize(v: Array<number>): Float64Vec;

    static f64BeBytes(v: Uint8Array): Float64Vec;

    constructor(v: Array<number>): Float64Vec;

    get asBoxedSlice(): Array<number>;

    get asSlice(): Array<number>;

    fillSlice(v: Array<number>): void;

    setValue(newSlice: Array<number>): void;

    toString(): String;

    borrow(): Array<number>;

    get(i: number): number?;

}