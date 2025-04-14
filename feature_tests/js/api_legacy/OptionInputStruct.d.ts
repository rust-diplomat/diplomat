// generated by diplomat-tool
import type { OptionEnum } from "./OptionEnum"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type OptionInputStruct_obj = {
    a?: number | null;
    b?: codepoint | null;
    c?: OptionEnum | null;
};



export class OptionInputStruct {
    
    get a() : number | null; 
    set a(value: number | null); 
    
    get b() : codepoint | null; 
    set b(value: codepoint | null); 
    
    get c() : OptionEnum | null; 
    set c(value: OptionEnum | null); 
    
    /** Create `OptionInputStruct` from an object that contains all of `OptionInputStruct`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj : OptionInputStruct_obj) : OptionInputStruct;


    constructor(structObj : OptionInputStruct_obj);
}