import { usize } from "./diplomat-runtime"
import { MyStruct } from "./MyStruct";

/**
 */
export class Opaque {

  /**
   */
  static new(): Opaque;

  /**
   */
  assert_struct(s: MyStruct): void;

  /**
   */
  static returns_usize(): usize;
}
