// generated by diplomat-tool
import type { ErrorEnum } from "./ErrorEnum"
import type { ErrorStruct } from "./ErrorStruct"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class ResultOpaque {
    
    get ffiValue(): pointer;

    static newFailingFoo(): ResultOpaque;

    static newFailingBar(): ResultOpaque;

    static newFailingUnit(): ResultOpaque | null;

    static newFailingStruct(i: number): ResultOpaque;

    static newInErr(i: number): void;

    static newInt(i: number): number | null;

    static newInEnumErr(i: number): ErrorEnum;

    assertInteger(i: number): void;

    constructor(i: number);
}