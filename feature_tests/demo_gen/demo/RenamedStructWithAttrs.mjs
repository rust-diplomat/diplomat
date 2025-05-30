import { RenamedStructWithAttrs } from "../../js/api/index.mjs"
export function c(selfA, selfB) {
    
    let self = RenamedStructWithAttrs.fromFields({
        a: selfA,
        b: selfB
    });
    
    let out = self.c;
    

    return out;
}
