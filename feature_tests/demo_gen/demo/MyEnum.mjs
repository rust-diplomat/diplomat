import { MyEnum } from "../../js/api/index.mjs"
export function intoValue(self) {
    
    let out = new MyEnum(self).intoValue();
    

    return out;
}
export function getA() {
    
    let out = MyEnum.getA();
    
    out = out?.value || 'None';;
    

    return out;
}
