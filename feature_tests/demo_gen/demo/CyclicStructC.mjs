import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
export function cyclicOut(cyclicStructCAAField) {
    
    let cyclicStructCAA = CyclicStructB.fromFields({
        field: cyclicStructCAAField
    });
    
    let cyclicStructCA = CyclicStructA.fromFields({
        a: cyclicStructCAA
    });
    
    let cyclicStructC = CyclicStructC.fromFields({
        a: cyclicStructCA
    });
    
    let out = cyclicStructC.cyclicOut();
    

    return out;
}
