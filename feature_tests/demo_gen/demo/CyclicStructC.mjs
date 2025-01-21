import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
export function cyclicOut(cyclicStructCACyclicStructCaCyclicStructCaField) {
    
    let cyclicStructCACyclicStructCa = CyclicStructB.fromFields({
        field: cyclicStructCACyclicStructCaCyclicStructCaField
    });
    
    let cyclicStructCA = CyclicStructA.fromFields({
        a: cyclicStructCACyclicStructCa
    });
    
    let cyclicStructC = CyclicStructC.fromFields({
        a: cyclicStructCA
    });
    
    let out = cyclicStructC.cyclicOut();
    

    return out;
}
