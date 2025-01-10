// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
*/


export class FixedDecimal {
    
    get ffiValue(): pointer;

    multiplyPow10(power: number): void;

    toString(): string | null;

    constructor(v: number);
}