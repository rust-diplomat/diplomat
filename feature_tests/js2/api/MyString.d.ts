import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class MyString {
    

    get ffiValue(): pointer;


    static new_(v: String): MyString;

    static newUnsafe(v: String): MyString;

    static newOwned(v: String): MyString;

    static newFromFirst(v: Array<String>): MyString;

    set str(newStr: String): void;

    get str(): String;

    getBoxedStr(): String;

    

}