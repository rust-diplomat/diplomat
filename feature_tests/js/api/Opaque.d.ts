// generated by diplomat-tool
import type { ImportedStruct } from "./ImportedStruct"
import type { MyStruct } from "./MyStruct"
import type { MyStruct_obj } from "./MyStruct"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class Opaque {
    
    get ffiValue(): pointer;

    static tryFromUtf8(input: string): Opaque | null;

    static fromStr(input: string): Opaque;

    getDebugStr(): string;

    assertStruct(s: MyStruct_obj): void;

    static returnsUsize(): number;

    static returnsImported(): ImportedStruct;

    static cmp(): number;

    constructor();
}