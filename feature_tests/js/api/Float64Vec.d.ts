// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class Float64Vec {
    
    get ffiValue(): pointer;

    static newBool(v: Array<boolean>): Float64Vec;

    static newI16(v: Array<number>): Float64Vec;

    static newU16(v: Array<number>): Float64Vec;

    static newIsize(v: Array<number>): Float64Vec;

    static newUsize(v: Array<number>): Float64Vec;

    static newF64BeBytes(v: Uint8Array): Float64Vec;

    get asSlice(): Array<number>;

    fillSlice(v: Array<number>): void;

    setValue(newSlice: Array<number>): void;

    toString(): string;

    borrow(): Array<number>;

    get(i: number): number | null;

    constructor(v: Array<number>);
}