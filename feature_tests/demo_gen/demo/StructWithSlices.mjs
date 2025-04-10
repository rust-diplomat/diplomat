import { StructWithSlices } from "../../js/api/index.mjs"
export function returnLast(structWithSlicesFirst, structWithSlicesSecond) {
    
    let structWithSlices = StructWithSlices.fromFields({
        first: structWithSlicesFirst,
        second: structWithSlicesSecond
    });
    
    let out = structWithSlices.returnLast();
    

    return out;
}
