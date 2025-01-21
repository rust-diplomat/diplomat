import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
export function cyclicOut(cyclicStructAAField) {
    
    let cyclicStructAA = CyclicStructB.fromFields({
        field: cyclicStructAAField
    });
    
    let cyclicStructA = CyclicStructA.fromFields({
        a: cyclicStructAA
    });
    
    let out = cyclicStructA.cyclicOut();
    

    return out;
}
export function doubleCyclicOut(cyclicStructAAField, cyclicStructAAField_1) {
    
    let cyclicStructAA = CyclicStructB.fromFields({
        field: cyclicStructAAField
    });
    
    let cyclicStructA = CyclicStructA.fromFields({
        a: cyclicStructAA
    });
    
    let cyclicStructAA = CyclicStructB.fromFields({
        field: cyclicStructAAField_1
    });
    
    let cyclicStructA = CyclicStructA.fromFields({
        a: cyclicStructAA
    });
    
    let out = cyclicStructA.doubleCyclicOut(cyclicStructA);
    

    return out;
}
