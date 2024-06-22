import { usize, i8 } from "./diplomat-runtime"
import { ImportedStruct } from "./ImportedStruct";
import { MyStruct } from "./MyStruct";

/**
 */
export class Opaque {

  /**
   */
  static new(): Opaque;

  /**
   */
  static try_from_utf8(input: string): Opaque | undefined;

  /**
   */
  static from_str(input: string): Opaque;

  /**
   */
  get_debug_str(): string;

  /**

   * See the {@link https://docs.rs/Something/latest/struct.Something.html#method.something Rust documentation for `something`} for more information.

   * See the {@link https://docs.rs/Something/latest/struct.Something.html#method.something_else Rust documentation for `something_else`} for more information.

   * Additional information: {@link https://docs.rs/Something/latest/struct.Something.html#method.something_small 1}, {@link https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something 2}
   */
  assert_struct(s: MyStruct): void;

  /**
   */
  static returns_usize(): usize;

  /**
   */
  static returns_imported(): ImportedStruct;

  /**
   */
  static cmp(): i8;
}
