// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

export type StructWithSlices_obj = {
    first: string;
    second: Array<number>;
};



export class StructWithSlices {
    get first(): string;
    set first(value: string);
    get second(): Array<number>;
    set second(value: Array<number>);
    /** @internal */
    static fromFields(structObj : StructWithSlices_obj) : StructWithSlices;

    /**
    * Create `StructWithSlices` from an object that contains all of `StructWithSlices`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    constructor(structObj: StructWithSlices_obj);


    returnLast(): string;
}