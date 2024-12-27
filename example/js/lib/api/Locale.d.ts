// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An  Locale, capable of representing strings like `"en-US"`.
*
*See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
*/


export class Locale {
	
    

    get ffiValue(): pointer;

    #defaultConstructor(name: string): Locale;

    constructor(name: string);
}