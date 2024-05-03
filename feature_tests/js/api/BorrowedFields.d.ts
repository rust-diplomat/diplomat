import { Bar } from "./Bar";

/**
 */
export class BorrowedFields {
  a: string;
  b: string;
  c: string;

  /**
   */
  static from_bar_and_strings(bar: Bar, dstr16: string, utf8_str: string): BorrowedFields;
}
