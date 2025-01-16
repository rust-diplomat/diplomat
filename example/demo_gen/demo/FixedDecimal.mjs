import { FixedDecimal } from "../../js/lib/api/index.mjs"
export function toString(v) {
    
    let self = new FixedDecimal(v);
    
    let out = self.toString();
    

    return out;
}
