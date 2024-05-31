import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class Foo {
    

    get ffiValue(): pointer;


    static new_(x: String): Foo;

    get bar(): Bar;

    static newStatic(x: String): Foo;

    asReturning(): BorrowedFieldsReturning;

    static extractFromFields(fields: BorrowedFields): Foo;

    static extractFromBounds(bounds: BorrowedFieldsWithBounds, anotherString: String): Foo;

    

}