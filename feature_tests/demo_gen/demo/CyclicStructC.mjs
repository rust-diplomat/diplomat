import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
export function cyclicOut(selfAAField) {
    
    let selfAA = CyclicStructB.fromFields({
        field: selfAAField
    });
    
    let selfA = CyclicStructA.fromFields({
        a: selfAA
    });
    
    let self = CyclicStructC.fromFields({
        a: selfA
    });
    
    let out = self.cyclicOut();
    

    return out;
}
