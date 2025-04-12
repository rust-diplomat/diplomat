// generated by diplomat-tool
import type { CyclicStructB } from "./CyclicStructB"
import type { CyclicStructB_obj } from "./CyclicStructB"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type CyclicStructA_obj = {
    a: CyclicStructB_obj;
};



export class CyclicStructA {
    
    get a() : CyclicStructB; 
    set a(value: CyclicStructB); 
    
    /** Create `CyclicStructA` from an object that contains all of `CyclicStructA`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj : CyclicStructA_obj) : CyclicStructA;


    static getB(): CyclicStructB;

    cyclicOut(): string;

    doubleCyclicOut(cyclicStructA: CyclicStructA_obj): string;

    get getterOut(): string;

    constructor(structObj : CyclicStructA_obj);
}