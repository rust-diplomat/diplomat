// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type MyZst_obj = {
};



export class MyZst {
	
    /** Create `MyZst` from an object that contains all of `MyZst`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj : MyZst_obj) : MyZst;
    
    constructor(structObj : MyZst_obj);
}