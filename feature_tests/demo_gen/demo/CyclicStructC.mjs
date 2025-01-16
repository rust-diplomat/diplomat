import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
export function cyclicOut(field) {
    
    let a = CyclicStructB.fromFields({
        field: field
    });
    
    let a = CyclicStructA.fromFields({
        a: a
    });
    
    let self = CyclicStructC.fromFields({
        a: a
    });
    
    let out = self.cyclicOut();
    

    return out;
}
