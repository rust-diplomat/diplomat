import { MyStruct } from "../../js/api/index.mjs"
export function intoA(myStructA, myStructB, myStructC, myStructD, myStructE, myStructF, myStructG) {
    
    let myStruct = MyStruct.fromFields({
        a: myStructA,
        b: myStructB,
        c: myStructC,
        d: myStructD,
        e: myStructE,
        f: myStructF,
        g: myStructG
    });
    
    let out = myStruct.intoA();
    

    return out;
}
