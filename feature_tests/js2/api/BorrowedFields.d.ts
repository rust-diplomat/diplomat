import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";


export class BorrowedFields {
    #ptr: pointer;
    a: String;
    b: String;
    c: String;


    constructor(ptr: pointer, aEdges: Array[object]): BorrowedFields;
    static fromBarAndStrings(bar: Bar, dstr16: String, utf8Str: String): BorrowedFields;

    

}