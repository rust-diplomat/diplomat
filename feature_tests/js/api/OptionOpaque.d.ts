import { i32, u32, isize, usize } from "./diplomat-runtime"
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
  option_isize(): isize | undefined;

  /**
   */
  option_usize(): usize | undefined;

  /**
   */
  option_i32(): i32 | undefined;

  /**
   */
  option_u32(): u32 | undefined;

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
