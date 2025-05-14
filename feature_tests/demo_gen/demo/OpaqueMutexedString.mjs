import { OpaqueMutexedString } from "../../js/api/index.mjs"
export function getLenAndAdd(selfNumber, other) {
    
    let self = OpaqueMutexedString.fromUsize(selfNumber);
    
    let out = self.getLenAndAdd(other);
    

    return out;
}
export function toUnsignedFromUnsigned(selfNumber, input) {
    
    let self = OpaqueMutexedString.fromUsize(selfNumber);
    
    let out = self.toUnsignedFromUnsigned(input);
    

    return out;
}
