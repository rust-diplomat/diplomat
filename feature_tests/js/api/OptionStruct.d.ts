import { OptionOpaque } from "./OptionOpaque";
import { OptionOpaqueChar } from "./OptionOpaqueChar";

/**
 */
export class OptionStruct {
  a?: OptionOpaque;
  b?: OptionOpaqueChar;
  c: number;
  d?: OptionOpaque;
}
