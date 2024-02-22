import { i32 } from "./diplomat-runtime"
import { OptionStruct } from "./OptionStruct";

/**
 */
export class OptionOpaque {

  /**
   */
  static new(i: i32): OptionOpaque | undefined;

  /**
   */
  static new_none(): OptionOpaque | undefined;

  /**
   */
  static returns(): OptionStruct | undefined;

  /**
   */
  static new_struct(): OptionStruct;

  /**
   */
  static new_struct_nones(): OptionStruct;

  /**
   */
  assert_integer(i: i32): void;

  /**
   */
  static option_opaque_argument(arg?: OptionOpaque): boolean;
}
