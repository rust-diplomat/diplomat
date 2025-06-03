import { DefaultEnum } from "../../js/api/index.mjs"
export function new_() {
    
    let out = new DefaultEnum();
    
    out = out?.value || 'None';;
    

    return out;
}
