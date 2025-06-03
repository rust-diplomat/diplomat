import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
export function cyclicOut(selfAField) {
    
    let selfA = CyclicStructB.fromFields({
        field: selfAField
    });
    
    let self = CyclicStructA.fromFields({
        a: selfA
    });
    
    let out = self.cyclicOut();
    

    return out;
}
export function doubleCyclicOut(selfAField, cyclicStructAAField) {
    
    let selfA = CyclicStructB.fromFields({
        field: selfAField
    });
    
    let self = CyclicStructA.fromFields({
        a: selfA
    });
    
    let cyclicStructAA = CyclicStructB.fromFields({
        field: cyclicStructAAField
    });
    
    let cyclicStructA = CyclicStructA.fromFields({
        a: cyclicStructAA
    });
    
    let out = self.doubleCyclicOut(cyclicStructA);
    

    return out;
}
export function getterOut(selfAField) {
    
    let selfA = CyclicStructB.fromFields({
        field: selfAField
    });
    
    let self = CyclicStructA.fromFields({
        a: selfA
    });
    
    let out = self.getterOut;
    

    return out;
}
