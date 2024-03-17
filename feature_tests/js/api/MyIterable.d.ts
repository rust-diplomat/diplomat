import { MyIterator } from "./MyIterator";

/**
 */
export class MyIterable {

  /**
   */
  static new(x: Uint8Array): MyIterable;

  /**
   */
  iter(): MyIterator;
}
