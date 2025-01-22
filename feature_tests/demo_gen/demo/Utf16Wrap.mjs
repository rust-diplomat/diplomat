import { Utf16Wrap } from "../../js/api/index.mjs"
export function getDebugStr(utf16WrapInput) {
    
    let utf16Wrap = new Utf16Wrap(utf16WrapInput);
    
    let out = utf16Wrap.getDebugStr();
    

    return out;
}
