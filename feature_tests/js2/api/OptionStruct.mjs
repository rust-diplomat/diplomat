


export class OptionStruct {
    #ptr
    const a;
    const b;
    const c;
    const d;

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    #intoFFI() {
        return [
            a.ffiValue ?? 0, 
            b.ffiValue ?? 0, 
            c, 
            d.ffiValue ?? 0]
    }
    

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    constructor(ptr) {
        this.#ptr = ptr;
        a = (a === 0) ? undefined : new OptionOpaque(a, []);,
        b = (b === 0) ? undefined : new OptionOpaqueChar(b, []);,
        c = c,
        d = (d === 0) ? undefined : new OptionOpaque(d, []);;
    }
    

}