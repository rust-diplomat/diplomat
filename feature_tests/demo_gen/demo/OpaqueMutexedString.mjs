import { OpaqueMutexedString } from "../../js/api/index.mjs"
export function getLenAndAdd(opaqueMutexedStringNumber, other) {
    
    let opaqueMutexedString = OpaqueMutexedString.fromUsize(opaqueMutexedStringNumber);
    
    let out = opaqueMutexedString.getLenAndAdd(other);
    

    return out;
}
export function toUnsignedFromUnsigned(opaqueMutexedStringNumber, input) {
    
    let opaqueMutexedString = OpaqueMutexedString.fromUsize(opaqueMutexedStringNumber);
    
    let out = opaqueMutexedString.toUnsignedFromUnsigned(input);
    

    return out;
}
