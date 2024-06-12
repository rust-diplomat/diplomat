// generated by diplomat-tool

import type { Bar } from "./Bar"
import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";


export class BorrowedFields {
    #a;
    get a() : String;
    set a(value: String); 
    #b;
    get b() : String;
    set b(value: String); 
    #c;
    get c() : String;
    set c(value: String); 


    constructor(ptr: pointer, aEdges: Array[object]);
    static fromBarAndStrings(bar: Bar, dstr16: String, utf8Str: String): BorrowedFields;

    

}