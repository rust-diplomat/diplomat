import { u8, i32, u64, char } from "./diplomat-runtime"

/**
 */
export class MyStruct {
  a: u8;
  b: boolean;
  c: u8;
  d: u64;
  e: i32;
  f: char;

  /**
   */
  static new(): MyStruct;
}
