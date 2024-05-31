import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";


export class BorrowedFieldsWithBounds {
    #ptr: pointer;
    fieldA: String;
    fieldB: String;
    fieldC: String;


    constructor(ptr: pointer, aEdges: Array[object], bEdges: Array[object], cEdges: Array[object]): BorrowedFieldsWithBounds;
    static fromFooAndStrings(foo: Foo, dstr16X: String, utf8StrZ: String): BorrowedFieldsWithBounds;

    

}