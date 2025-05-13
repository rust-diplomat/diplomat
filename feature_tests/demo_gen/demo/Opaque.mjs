import { Opaque } from "../../js/api/index.mjs"
export function getDebugStr() {
    
    let opaque = new Opaque();
    
    let out = opaque.getDebugStr();
    

    return out;
}
export function returnsUsize() {
    
    let out = Opaque.returnsUsize();
    

    return out;
}
export function cmp() {
    
    let out = Opaque.cmp();
    
    out = out == 0 ? '==' : out == 1 ? '>' : '<';;
    

    return out;
}
