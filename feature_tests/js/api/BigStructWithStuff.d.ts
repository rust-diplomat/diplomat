// generated by diplomat-tool
import type { ScalarPairWithPadding } from "./ScalarPairWithPadding"
import type { ScalarPairWithPadding_obj } from "./ScalarPairWithPadding"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** Testing JS-specific layout/padding behavior
*/
type BigStructWithStuff_obj = {
    first: number;
    second: number;
    third: number;
    fourth: ScalarPairWithPadding_obj;
    fifth: number;
};



export class BigStructWithStuff {
    
    get first() : number; 
    set first(value: number); 
    
    get second() : number; 
    set second(value: number); 
    
    get third() : number; 
    set third(value: number); 
    
    get fourth() : ScalarPairWithPadding; 
    set fourth(value: ScalarPairWithPadding); 
    
    get fifth() : number; 
    set fifth(value: number); 
    
    /** Create `BigStructWithStuff` from an object that contains all of `BigStructWithStuff`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj : BigStructWithStuff_obj) : BigStructWithStuff;
    

    assertValue(extraVal: number): void;

    constructor(structObj : BigStructWithStuff_obj);
}