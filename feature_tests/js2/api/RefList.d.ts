import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class RefList {
    

    get ffiValue(): pointer;


    static node(data: RefListParameter): RefList;

    

}