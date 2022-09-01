import { u8 } from "./diplomat-runtime"
import { UnimportedEnum } from "./UnimportedEnum";

/**
 */
export class ImportedStruct {
  foo: UnimportedEnum;
  count: u8;
}
