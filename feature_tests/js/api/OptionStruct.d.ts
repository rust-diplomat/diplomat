import { u32 } from "./diplomat-runtime"
import { OptionOpaque } from "./OptionOpaque";
import { OptionOpaqueChar } from "./OptionOpaqueChar";

/**
 */
export class OptionStruct {
  a?: OptionOpaque;
  b?: OptionOpaqueChar;
  c: u32;
  d?: OptionOpaque;
}
