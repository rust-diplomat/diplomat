import { OptionString } from "../../js/api/index.mjs"
export function write(diplomatStr) {
    
    let self = OptionString.new_(diplomatStr);
    
    let out = self.write();
    

    return out;
}
