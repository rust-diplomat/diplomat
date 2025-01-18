import { OptionString } from "../../js/api/index.mjs"
export function write(optionStringDiplomatStr) {
    
    let optionString = OptionString.new_(optionStringDiplomatStr);
    
    let out = optionString.write();
    

    return out;
}
