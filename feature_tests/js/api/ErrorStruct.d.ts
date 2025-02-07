// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type ErrorStruct_obj = {
    i: number;
    j: number;
};



export class ErrorStruct {
    
    get i() : number; 
    set i(value: number); 
    
    get j() : number; 
    set j(value: number); 
    
    /** Create `ErrorStruct` from an object that contains all of `ErrorStruct`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj : ErrorStruct_obj) : ErrorStruct;


    constructor(structObj : ErrorStruct_obj);
}