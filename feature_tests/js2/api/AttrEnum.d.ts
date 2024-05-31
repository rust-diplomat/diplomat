import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
export class AttrEnum {
    constructor(value : AttrEnum | string);

    get value() : string;

    get ffiValue() : number;

    static A : AttrEnum;

    static B : AttrEnum;

    static C : AttrEnum;


    

}