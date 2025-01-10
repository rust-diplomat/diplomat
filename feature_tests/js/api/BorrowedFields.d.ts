// generated by diplomat-tool
import type { Bar } from "./Bar"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type BorrowedFields_obj = {
    a: string;
    b: string;
    c: string;
};



export class BorrowedFields {
    
    get a() : string; 
    set a(value: string); 
    
    get b() : string; 
    set b(value: string); 
    
    get c() : string; 
    set c(value: string); 
    
    /** Create `BorrowedFields` from an object that contains all of `BorrowedFields`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj : BorrowedFields_obj) : BorrowedFields;
    

    static fromBarAndStrings(bar: Bar, dstr16: string, utf8Str: string): BorrowedFields;

    constructor(structObj : BorrowedFields_obj);
}