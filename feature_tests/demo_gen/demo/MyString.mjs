import { MyString } from "../../js/api/index.mjs"
export function str(myStringV) {
    
    let myString = new MyString(myStringV);
    
    let out = myString.str;
    

    return out;
}
export function stringTransform(foo) {
    
    let out = MyString.stringTransform(foo);
    

    return out;
}
