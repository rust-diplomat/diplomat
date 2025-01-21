import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
export function cyclicOut(cyclicStructAACyclicStructAField) {
    
    let cyclicStructAA = CyclicStructB.fromFields({
        field: cyclicStructAACyclicStructAField
    });
    
    let cyclicStructA = CyclicStructA.fromFields({
        a: cyclicStructAA
    });
    
    let out = cyclicStructA.cyclicOut();
    

    return out;
}
