// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class MyString {
    get ffiValue(): pointer;
    static newUnsafe(v: string): MyString;
    static newOwned(v: string): MyString;
    static newFromFirst(v: Array<string>): MyString;
    set str(newStr: string);
    get str(): string;
    static getStaticStr(): string;
    static stringTransform(foo: string): string;
    borrow(): string;

    constructor(v: string);
}