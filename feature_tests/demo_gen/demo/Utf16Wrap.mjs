import { Utf16Wrap } from "../../js/api/index.mjs"
export function getDebugStr(selfInput) {
    
    let self = new Utf16Wrap(selfInput);
    
    let out = self.getDebugStr();
    

    return out;
}
