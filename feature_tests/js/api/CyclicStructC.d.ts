// generated by diplomat-tool
import type { CyclicStructA } from "./CyclicStructA"
import type { CyclicStructA_obj } from "./CyclicStructA"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type CyclicStructC_obj = {
    a: CyclicStructA_obj;
};



export class CyclicStructC {
	

    get a() : CyclicStructA;
    set a(value: CyclicStructA); 

    /** Create `CyclicStructC` from an object that contains all of `CyclicStructC`'s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj : CyclicStructC_obj) : CyclicStructC;
    
    constructor(structObj : CyclicStructC_obj);


    static takesNestedParameters(c: CyclicStructC_obj): CyclicStructC;

    cyclicOut(): string;
}