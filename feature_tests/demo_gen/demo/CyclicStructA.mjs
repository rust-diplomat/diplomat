import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
export function cyclicOut(cyclicStructAField) {
    
    let a = CyclicStructB.fromFields({
        field: cyclicStructAField
    });
    
    let CyclicStructA = CyclicStructA.fromFields({
        a: a
    });
    
    let out = CyclicStructA.cyclicOut();
    

    return out;
}
