// generated by diplomat-tool

import type { AttrOpaque1 } from "./AttrOpaque1"
import type { OpaqueIterator } from "./OpaqueIterator"
import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class OpaqueIterable {
    

    get ffiValue(): pointer;


    [Symbol.iterator](): OpaqueIterator;

    

}