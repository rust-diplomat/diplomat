import { OpaqueThinVec } from "../../js/api/index.mjs"
export function len(selfA, selfB) {
    
    let self = new OpaqueThinVec(selfA,selfB);
    
    let out = self.len();
    

    return out;
}
