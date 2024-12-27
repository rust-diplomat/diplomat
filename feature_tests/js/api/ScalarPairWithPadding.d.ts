// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** Testing JS-specific layout/padding behavior
*/
type ScalarPairWithPadding_obj = {
    first: number;
    second: number;
};



export class ScalarPairWithPadding {
	

    get first() : number;
    set first(value: number); 

    get second() : number;
    set second(value: number); 

    /** Create `ScalarPairWithPadding` from an object that contains all of `ScalarPairWithPadding`'s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj : ScalarPairWithPadding_obj) : ScalarPairWithPadding;
    
    constructor(structObj : ScalarPairWithPadding_obj);


    assertValue(): void;
}