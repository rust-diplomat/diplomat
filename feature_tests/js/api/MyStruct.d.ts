// generated by diplomat-tool
import type { MyEnum } from "./MyEnum"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

export class MyStruct {

    get a() : number;
    set a(value: number); 

    get b() : boolean;
    set b(value: boolean); 

    get c() : number;
    set c(value: number); 

    get d() : bigint;
    set d(value: bigint); 

    get e() : number;
    set e(value: number); 

    get f() : codepoint;
    set f(value: codepoint); 

    get g() : MyEnum;
    set g(value: MyEnum); 


    static new_(): MyStruct;

    intoA(): number;

    static returnsZstResult(): void;
}