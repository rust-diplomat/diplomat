import { usize } from "./diplomat-runtime"
import { MyStruct } from "./MyStruct";

/**
 */
export class Opaque {

  /**
   */
  static new(): Opaque;

  /**

   * See the {@link https://docs.rs/Something/latest/struct.Something.html#method.something Rust documentation for `something`} for more information.

   * See the {@link https://docs.rs/Something/latest/struct.Something.html#method.something_else Rust documentation for `something_else`} for more information.

   * Additional information: {@link https://docs.rs/Something/latest/struct.Something.html#method.something_small 1}, {@link https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something 2}
   */
  assert_struct(s: MyStruct): void;

  /**
   */
  static returns_usize(): usize;
}
