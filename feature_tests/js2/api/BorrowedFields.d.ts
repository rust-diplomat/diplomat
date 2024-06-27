// generated by diplomat-tool
import type { Bar } from "./Bar"
import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";

export class BorrowedFields {
    #a;
    get a() : string;
    set a(value: string); 
    #b;
    get b() : string;
    set b(value: string); 
    #c;
    get c() : string;
    set c(value: string); 

    static fromBarAndStrings(bar: Bar, dstr16: string, utf8Str: string): BorrowedFields;

    

}