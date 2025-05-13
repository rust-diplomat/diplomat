import { RenamedStructWithAttrs } from "../../js/api/index.mjs"
export function c(renamedStructWithAttrsA, renamedStructWithAttrsB) {
    
    let renamedStructWithAttrs = RenamedStructWithAttrs.fromFields({
        a: renamedStructWithAttrsA,
        b: renamedStructWithAttrsB
    });
    
    let out = renamedStructWithAttrs.c;
    

    return out;
}
