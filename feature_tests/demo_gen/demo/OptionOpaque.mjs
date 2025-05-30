import { OptionOpaque } from "../../js/api/index.mjs"
export function optionIsize(selfI) {
    
    let self = OptionOpaque.new_(selfI);
    
    let out = self.optionIsize();
    

    return out;
}
export function optionUsize(selfI) {
    
    let self = OptionOpaque.new_(selfI);
    
    let out = self.optionUsize();
    

    return out;
}
export function optionI32(selfI) {
    
    let self = OptionOpaque.new_(selfI);
    
    let out = self.optionI32();
    

    return out;
}
export function optionU32(selfI) {
    
    let self = OptionOpaque.new_(selfI);
    
    let out = self.optionU32();
    

    return out;
}
export function optionOpaqueArgument(argI) {
    
    let arg = OptionOpaque.new_(argI);
    
    let out = OptionOpaque.optionOpaqueArgument(arg);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function acceptsOptionU8(arg, sentinel) {
    
    let out = OptionOpaque.acceptsOptionU8(arg,sentinel);
    

    return out;
}
export function acceptsOptionEnum(arg, sentinel) {
    
    let out = OptionOpaque.acceptsOptionEnum(arg,sentinel);
    
    out = out?.value || 'None';;
    

    return out;
}
