import { Bar } from "./Bar";
import { BorrowedFields } from "./BorrowedFields";
import { BorrowedFieldsReturning } from "./BorrowedFieldsReturning";

/**
 */
export class Foo {

  /**
   */
  static new(x: string): Foo;

  /**
   */
  get_bar(): Bar;

  /**
   */
  static new_static(x: string): Foo;

  /**
   */
  as_returning(): BorrowedFieldsReturning;

  /**
   */
  static extract_from_fields(fields: BorrowedFields): Foo;
}
