import { u8 } from "./diplomat-runtime"
import { AttrEnum } from "./AttrEnum";
import { Unnamespaced } from "./Unnamespaced";

/**
 */
export class AttrOpaque1 {

  /**
   */
  static new(): AttrOpaque1;

  /**
   */
  method(): u8;

  /**
   */
  abirenamed(): u8;

  /**
   */
  method_disabledcpp(): void;

  /**
   */
  use_unnamespaced(_un: Unnamespaced): void;

  /**
   */
  use_namespaced(_n: AttrEnum): void;
}
