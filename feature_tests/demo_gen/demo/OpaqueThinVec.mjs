import { OpaqueThinVec } from "../../js/api/index.mjs"
export function len(opaqueThinVecA, opaqueThinVecB) {
    
    let opaqueThinVec = new OpaqueThinVec(opaqueThinVecA,opaqueThinVecB);
    
    let out = opaqueThinVec.len();
    

    return out;
}
