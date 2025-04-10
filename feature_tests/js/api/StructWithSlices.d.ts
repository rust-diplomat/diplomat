// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type StructWithSlices_obj = {
    first: string;
    second: Array<number>;
};



export class StructWithSlices {
    
    get first() : string; 
    set first(value: string); 
    
    get second() : Array<number>; 
    set second(value: Array<number>); 
    
    /** Create `StructWithSlices` from an object that contains all of `StructWithSlices`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj : StructWithSlices_obj) : StructWithSlices;

returnLast(): string;

    constructor(structObj : StructWithSlices_obj);
}