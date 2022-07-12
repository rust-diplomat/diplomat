import { ErrorEnum } from "./ErrorEnum";
import { ErrorStruct } from "./ErrorStruct";

/**
 */
export class ResultOpaque {

  /**
   */
  static new(i: number): ResultOpaque | never;

  /**
   */
  static new_failing_foo(): ResultOpaque | never;

  /**
   */
  static new_failing_bar(): ResultOpaque | never;

  /**
   */
  static new_failing_unit(): ResultOpaque | never;

  /**
   */
  static new_failing_struct(i: number): ResultOpaque | never;

  /**
   */
  static new_in_err(i: number): void | never;

  /**
   */
  static new_in_enum_err(i: number): ErrorEnum | never;

  /**
   */
  assert_integer(i: number): void;
}
