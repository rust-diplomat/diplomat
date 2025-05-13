import { MyString } from "../../js/api/index.mjs"
export function str(selfV) {
    
    let self = new MyString(selfV);
    
    let out = self.str;
    

    return out;
}
export function stringTransform(foo) {
    
    let out = MyString.stringTransform(foo);
    

    return out;
}
