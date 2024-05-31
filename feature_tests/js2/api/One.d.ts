import { u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64, pointer, char } from "./diplomat-runtime.d.ts";



export class One {
    

    get ffiValue(): pointer;


    static transitivity(hold: One, nohold: One): One;

    static cycle(hold: Two, nohold: One): One;

    static manyDependents(a: One, b: One, c: Two, d: Two, nohold: Two): One;

    static returnOutlivesParam(hold: Two, nohold: One): One;

    static diamondTop(top: One, left: One, right: One, bottom: One): One;

    static diamondLeft(top: One, left: One, right: One, bottom: One): One;

    static diamondRight(top: One, left: One, right: One, bottom: One): One;

    static diamondBottom(top: One, left: One, right: One, bottom: One): One;

    static diamondAndNestedTypes(a: One, b: One, c: One, d: One, nohold: One): One;

    static implicitBounds(explicitHold: One, implicitHold: One, nohold: One): One;

    static implicitBoundsDeep(explicit: One, implicit1: One, implicit2: One, nohold: One): One;

    

}