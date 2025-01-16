import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
export function cyclicOut(field) {
    
    let a = CyclicStructB.fromFields({
        field: field
    });
    
    let self = CyclicStructA.fromFields({
        a: a
    });
    
    let out = self.cyclicOut();
    

    return out;
}
