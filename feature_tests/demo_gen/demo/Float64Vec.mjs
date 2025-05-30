import { Float64Vec } from "../../js/api/index.mjs"
export function toString(selfV) {
    
    let self = new Float64Vec(selfV);
    
    let out = self.toString();
    

    return out;
}
export function get(selfV, i) {
    
    let self = new Float64Vec(selfV);
    
    let out = self.get(i);
    

    return out;
}
