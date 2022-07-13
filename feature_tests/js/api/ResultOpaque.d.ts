import { i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ErrorEnum } from "./ErrorEnum";
import { ErrorStruct } from "./ErrorStruct";

/**
 */
export class ResultOpaque {

  /**
   * @throws {@link FFIError}<{@link ErrorEnum}>
   */
  static new(i: i32): ResultOpaque | never;

  /**
   * @throws {@link FFIError}<{@link ErrorEnum}>
   */
  static new_failing_foo(): ResultOpaque | never;

  /**
   * @throws {@link FFIError}<{@link ErrorEnum}>
   */
  static new_failing_bar(): ResultOpaque | never;

  /**
   * @throws {@link FFIError}<void>
   */
  static new_failing_unit(): ResultOpaque | never;

  /**
   * @throws {@link FFIError}<{@link ErrorStruct}>
   */
  static new_failing_struct(i: i32): ResultOpaque | never;

  /**
   * @throws {@link FFIError}<{@link ResultOpaque}>
   */
  static new_in_err(i: i32): void | never;

  /**
   * @throws {@link FFIError}<{@link ResultOpaque}>
   */
  static new_in_enum_err(i: i32): ErrorEnum | never;

  /**
   */
  assert_integer(i: i32): void;
}
