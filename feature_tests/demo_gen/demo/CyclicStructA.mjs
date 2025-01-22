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
    
    let cyclicStructAA_1 = CyclicStructB.fromFields({
        field: cyclicStructAAField_1
    });
    
    let cyclicStructA_1 = CyclicStructA.fromFields({
        a: cyclicStructAA_1
    });
    
    let out = cyclicStructA.doubleCyclicOut(cyclicStructA_1);
    

    return out;
}
export function getterOut(cyclicStructAAField) {
    
    let cyclicStructAA = CyclicStructB.fromFields({
        field: cyclicStructAAField
    });
    
    let cyclicStructA = CyclicStructA.fromFields({
        a: cyclicStructAA
    });
    
    let out = cyclicStructA.getterOut;
    

    return out;
}
