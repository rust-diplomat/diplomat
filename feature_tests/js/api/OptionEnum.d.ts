// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class OptionEnum {
    

    static fromValue(value : OptionEnum | string) : OptionEnum; 

    get value() : string;

    get ffiValue() : number;

    static Foo : OptionEnum;
    static Bar : OptionEnum;

    constructor(value: OptionEnum | string );
}