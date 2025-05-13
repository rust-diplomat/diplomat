import { OptionOpaque } from "../../js/api/index.mjs"
export function optionIsize(optionOpaqueI) {
    
    let optionOpaque = OptionOpaque.new_(optionOpaqueI);
    
    let out = optionOpaque.optionIsize();
    

    return out;
}
export function optionUsize(optionOpaqueI) {
    
    let optionOpaque = OptionOpaque.new_(optionOpaqueI);
    
    let out = optionOpaque.optionUsize();
    

    return out;
}
export function optionI32(optionOpaqueI) {
    
    let optionOpaque = OptionOpaque.new_(optionOpaqueI);
    
    let out = optionOpaque.optionI32();
    

    return out;
}
export function optionU32(optionOpaqueI) {
    
    let optionOpaque = OptionOpaque.new_(optionOpaqueI);
    
    let out = optionOpaque.optionU32();
    

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
    
    out = out.value;;
    

    return out;
}
