import { usize } from "./diplomat-runtime"
import { Utf16Wrap } from "./Utf16Wrap";

/**
 */
export class OpaqueMutexedString {

  /**
   */
  static from_usize(number: usize): OpaqueMutexedString;

  /**
   */
  change(number: usize): void;

  /**
   */
  get_len_and_add(other: usize): usize;

  /**
   */
  dummy_str(): string;

  /**
   */
  wrapper(): Utf16Wrap;
}
