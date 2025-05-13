import { FixedDecimal } from "../../js/lib/api/index.mjs"
export function toString(selfV) {
    
    let self = new FixedDecimal(selfV);
    
    let out = self.toString();
    

    return out;
}
