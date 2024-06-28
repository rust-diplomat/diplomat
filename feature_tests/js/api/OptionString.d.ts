import { FFIError } from "./diplomat-runtime"

/**
 */
export class OptionString {

  /**
   */
  static new(diplomat_str: string): OptionString | undefined;

  /**
   * @throws {@link FFIError}<void>
   */
  write(): string | never;

  /**
   */
  borrow(): string | undefined;
}
