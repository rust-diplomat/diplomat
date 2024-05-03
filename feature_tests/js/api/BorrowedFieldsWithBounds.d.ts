import { Foo } from "./Foo";

/**
 */
export class BorrowedFieldsWithBounds {
  field_a: string;
  field_b: string;
  field_c: string;

  /**
   */
  static from_foo_and_strings(foo: Foo, dstr16_x: string, utf8_str_z: string): BorrowedFieldsWithBounds;
}
