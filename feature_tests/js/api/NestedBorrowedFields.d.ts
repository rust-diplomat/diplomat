import { Bar } from "./Bar";
import { BorrowedFields } from "./BorrowedFields";
import { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds";
import { Foo } from "./Foo";

/**
 */
export class NestedBorrowedFields {
  fields: BorrowedFields;
  bounds: BorrowedFieldsWithBounds;
  bounds2: BorrowedFieldsWithBounds;

  /**
   */
  static from_bar_and_foo_and_strings(bar: Bar, foo: Foo, dstr16_x: string, dstr16_z: string, utf8_str_y: string, utf8_str_z: string): NestedBorrowedFields;
}
