// generated by diplomat-tool
import type { Bar } from "./Bar"
import type { BorrowedFields } from "./BorrowedFields"
import type { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds"
import type { BorrowedFieldsWithBounds_obj } from "./BorrowedFieldsWithBounds"
import type { BorrowedFields_obj } from "./BorrowedFields"
import type { Foo } from "./Foo"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type NestedBorrowedFields_obj = {
    fields: BorrowedFields_obj;
    bounds: BorrowedFieldsWithBounds_obj;
    bounds2: BorrowedFieldsWithBounds_obj;
};

export class NestedBorrowedFields {

    get fields() : BorrowedFields;
    set fields(value: BorrowedFields); 

    get bounds() : BorrowedFieldsWithBounds;
    set bounds(value: BorrowedFieldsWithBounds); 

    get bounds2() : BorrowedFieldsWithBounds;
    set bounds2(value: BorrowedFieldsWithBounds); 
    constructor(structObj : NestedBorrowedFields_obj);

    static fromBarAndFooAndStrings(bar: Bar, foo: Foo, dstr16X: string, dstr16Z: string, utf8StrY: string, utf8StrZ: string): NestedBorrowedFields;
}