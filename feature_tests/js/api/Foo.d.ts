// generated by diplomat-tool
import type { Bar } from "./Bar"
import type { BorrowedFields } from "./BorrowedFields"
import type { BorrowedFieldsReturning } from "./BorrowedFieldsReturning"
import type { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds"
import type { BorrowedFieldsWithBounds_obj } from "./BorrowedFieldsWithBounds"
import type { BorrowedFields_obj } from "./BorrowedFields"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class Foo {
	
    get ffiValue(): pointer;

    get bar(): Bar;

    asReturning(): BorrowedFieldsReturning;

    static extractFromFields(fields: BorrowedFields_obj): Foo;

    static extractFromBounds(bounds: BorrowedFieldsWithBounds_obj, anotherString: string): Foo;

    constructor(x: string);
}