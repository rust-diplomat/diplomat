// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class ErrorEnum {

    static fromValue(value: ErrorEnum | string): ErrorEnum;

    get value(): string;

    get ffiValue(): number;

    static Foo : ErrorEnum;
    static Bar : ErrorEnum;


    constructor(value: ErrorEnum | string );
}