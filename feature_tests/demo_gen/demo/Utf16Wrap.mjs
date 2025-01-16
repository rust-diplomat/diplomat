import { Utf16Wrap } from "../../js/api/index.mjs"
export function getDebugStr(input) {
    
    let self = new Utf16Wrap(input);
    
    let out = self.getDebugStr();
    

    return out;
}
