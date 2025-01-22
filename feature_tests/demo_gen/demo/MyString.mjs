import { MyString } from "../../js/api/index.mjs"
export function getStr(myStringV) {
    
    let myString = new MyString(myStringV);
    
    let out = myString.getStr;
    

    return out;
}
export function stringTransform(foo) {
    
    let out = MyString.stringTransform(foo);
    

    return out;
}
