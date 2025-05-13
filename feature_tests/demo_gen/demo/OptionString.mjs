import { OptionString } from "../../js/api/index.mjs"
export function write(selfDiplomatStr) {
    
    let self = OptionString.new_(selfDiplomatStr);
    
    let out = self.write();
    

    return out;
}
