import { Float64Vec } from "../../js/api/index.mjs"
export function toString(float64VecV) {
    
    let float64Vec = new Float64Vec(float64VecV);
    
    let out = float64Vec.toString();
    

    return out;
}
export function get(float64VecV, i) {
    
    let float64Vec = new Float64Vec(float64VecV);
    
    let out = float64Vec.get(i);
    

    return out;
}
