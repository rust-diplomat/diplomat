import { OptionStruct } from "./OptionStruct";

/**
 */
export class OptionOpaque {

  /**
   */
  static new(i: number): OptionOpaque | undefined;

  /**
   */
  static new_none(): OptionOpaque | undefined;

  /**
   */
  static new_struct(): OptionStruct;

  /**
   */
  static new_struct_nones(): OptionStruct;

  /**
   */
  assert_integer(i: number): void;
}
