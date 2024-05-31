import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class MyIterable {
    

    get ffiValue(): pointer;


    static new_(x: Array<number>): MyIterable;

    [Symbol.iterator](): MyIterator;

    

}