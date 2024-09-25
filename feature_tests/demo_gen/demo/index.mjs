export * as lib from "../../js/api/index.mjs";
import * as OptionStringDemo from "./OptionString.mjs";
export * as OptionStringDemo from "./OptionString.mjs";
import * as Float64VecDemo from "./Float64Vec.mjs";
export * as Float64VecDemo from "./Float64Vec.mjs";
import * as MyStringDemo from "./MyString.mjs";
export * as MyStringDemo from "./MyString.mjs";
import * as OpaqueDemo from "./Opaque.mjs";
export * as OpaqueDemo from "./Opaque.mjs";
import * as Utf16WrapDemo from "./Utf16Wrap.mjs";
export * as Utf16WrapDemo from "./Utf16Wrap.mjs";



let termini = Object.assign({
    "OptionString.write": {
        func: OptionStringDemo.write,
        // For avoiding webpacking minifying issues:
        funcName: "OptionString.write",
        parameters: [
            
            {
                name: "DiplomatStr",
                type: "string"
            }
            
        ]
    },

    "Float64Vec.toString": {
        func: Float64VecDemo.toString,
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.toString",
        parameters: [
            
            {
                name: "V",
                type: "Array<number>"
            }
            
        ]
    },

    "MyString.getStr": {
        func: MyStringDemo.getStr,
        // For avoiding webpacking minifying issues:
        funcName: "MyString.getStr",
        parameters: [
            
            {
                name: "V",
                type: "string"
            }
            
        ]
    },

    "MyString.stringTransform": {
        func: MyStringDemo.stringTransform,
        // For avoiding webpacking minifying issues:
        funcName: "MyString.stringTransform",
        parameters: [
            
            {
                name: "Foo",
                type: "string"
            }
            
        ]
    },

    "Opaque.getDebugStr": {
        func: OpaqueDemo.getDebugStr,
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.getDebugStr",
        parameters: [
            
        ]
    },

    "Utf16Wrap.getDebugStr": {
        func: Utf16WrapDemo.getDebugStr,
        // For avoiding webpacking minifying issues:
        funcName: "Utf16Wrap.getDebugStr",
        parameters: [
            
            {
                name: "Input",
                type: "string"
            }
            
        ]
    }
});

export const RenderInfo = {
    "termini": termini
};