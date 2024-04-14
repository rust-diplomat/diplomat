import { usize, f64 } from "./diplomat-runtime"

/**
 */
export class Float64Vec {

  /**
   */
  static new(v: Float64Array): Float64Vec;

  /**
   */
  static new_bool(v: Uint8Array): Float64Vec;

  /**
   */
  static new_i16(v: Int16Array): Float64Vec;

  /**
   */
  static new_u16(v: Uint16Array): Float64Vec;

  /**
   */
  static new_isize(v: Int32Array): Float64Vec;

  /**
   */
  static new_usize(v: Uint32Array): Float64Vec;

  /**
   */
  static new_f64_be_bytes(v: Uint8Array): Float64Vec;

  /**
   */
  static new_from_owned(v: Float64Array): Float64Vec;

  /**
   */
  as_boxed_slice(): Float64Array;

  /**
   */
  as_slice(): Float64Array;

  /**
   */
  fill_slice(v: Float64Array): void;

  /**
   */
  set_value(new_slice: Float64Array): void;

  /**
   */
  to_string(): string;

  /**
   */
  borrow(): Float64Array;

  /**
   */
  get(i: usize): f64 | undefined;
}
