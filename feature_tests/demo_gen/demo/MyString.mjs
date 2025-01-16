import { MyString } from "../../js/api/index.mjs"
export function getStr(v) {
    
    let self = new MyString(v);
    
    let out = self.getStr();
    

    return out;
}
export function stringTransform(foo) {
    
    let out = MyString.stringTransform(foo);
    

    return out;
}
