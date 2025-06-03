import { MyStruct } from "../../js/api/index.mjs"
export function intoA(selfA, selfB, selfC, selfD, selfE, selfF, selfG) {
    
    let self = MyStruct.fromFields({
        a: selfA,
        b: selfB,
        c: selfC,
        d: selfD,
        e: selfE,
        f: selfF,
        g: selfG
    });
    
    let out = self.intoA();
    

    return out;
}
