import { StructWithSlices } from "../../js/api/index.mjs"
export function returnLast(selfFirst, selfSecond) {
    
    let self = StructWithSlices.fromFields({
        first: selfFirst,
        second: selfSecond
    });
    
    let out = self.returnLast();
    

    return out;
}
