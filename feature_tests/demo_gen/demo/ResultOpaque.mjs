import { ResultOpaque } from "../../js/api/index.mjs"
export function newInt(i) {
    
    let out = ResultOpaque.newInt(i);
    

    return out;
}
export function newInEnumErr(i) {
    
    let out = ResultOpaque.newInEnumErr(i);
    
    out = out?.value || 'None';;
    

    return out;
}
