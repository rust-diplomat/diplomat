// generated by diplomat-tool
import type { CyclicStructB } from "./CyclicStructB"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

export class CyclicStructA {

    get a() : CyclicStructB;
    set a(value: CyclicStructB); 
    constructor(a: CyclicStructB);

    static getB(): CyclicStructB;
}