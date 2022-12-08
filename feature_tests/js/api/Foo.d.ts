import { Bar } from "./Bar";
import { BorrowedFields } from "./BorrowedFields";

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
  static extract_from_fields(fields: BorrowedFields): Foo;
}
