import { u8, i8 } from "./diplomat-runtime"

/**
 */
export class Comparable {

  /**
   */
  static new(int: u8): Comparable;

  /**
   */
  cmp(other: Comparable): i8;
}
