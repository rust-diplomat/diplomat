import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
export function cyclicOut(cyclicStructCACyclicStructCaCyclicStructCaField) {
    
    let cyclicStructCA = CyclicStructB.fromFields({
        field: cyclicStructCACyclicStructCaCyclicStructCaField
    });
    
    let a = CyclicStructA.fromFields({
        a: cyclicStructCA
    });
    
    let CyclicStructC = CyclicStructC.fromFields({
        a: a
    });
    
    let out = CyclicStructC.cyclicOut();
    

    return out;
}
