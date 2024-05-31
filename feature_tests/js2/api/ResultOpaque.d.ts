import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class ResultOpaque {
    

    get ffiValue(): pointer;


    static new_(i: number): ResultOpaque;

    static newFailingFoo(): ResultOpaque;

    static newFailingBar(): ResultOpaque;

    static newFailingUnit(): ResultOpaque | undefined;

    static newFailingStruct(i: number): ResultOpaque;

    static newInErr(i: number): void;

    static newInt(i: number): number | undefined;

    static newInEnumErr(i: number): ErrorEnum;

    assertInteger(i: number): void;

    

}