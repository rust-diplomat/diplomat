import { FixedDecimal } from "../../js/lib/api/index.mjs"
export function toString(fixedDecimalV) {
    
    let fixedDecimal = new FixedDecimal(fixedDecimalV);
    
    let out = fixedDecimal.toString();
    

    return out;
}
