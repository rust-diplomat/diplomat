// generated by diplomat-tool

import type { MyEnum } from "./MyEnum"
import type { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";


export class MyStruct {
    #a;
    get a() : number;
    set a(value: number); 
    #b;
    get b() : boolean;
    set b(value: boolean); 
    #c;
    get c() : number;
    set c(value: number); 
    #d;
    get d() : number;
    set d(value: number); 
    #e;
    get e() : number;
    set e(value: number); 
    #f;
    get f() : char;
    set f(value: char); 
    #g;
    get g() : MyEnum;
    set g(value: MyEnum); 


    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    constructor(ptr: pointer);
    constructor();

    intoA(): number;

    

}