import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class AttrOpaque1 {
    

    get ffiValue(): pointer;


    static new_(): AttrOpaque1;

    get method(): number;

    get abirenamed(): number;

    methodDisabledcpp(): void;

    useUnnamespaced(un: Unnamespaced): void;

    useNamespaced(n: AttrEnum): void;

    

}