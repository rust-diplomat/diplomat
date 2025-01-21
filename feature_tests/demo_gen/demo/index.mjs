export * as lib from "../../js/api/index.mjs";
import * as CyclicStructADemo from "./CyclicStructA.mjs";
export * as CyclicStructADemo from "./CyclicStructA.mjs";
import * as CyclicStructCDemo from "./CyclicStructC.mjs";
export * as CyclicStructCDemo from "./CyclicStructC.mjs";
import * as OptionStringDemo from "./OptionString.mjs";
export * as OptionStringDemo from "./OptionString.mjs";
import * as Float64VecDemo from "./Float64Vec.mjs";
export * as Float64VecDemo from "./Float64Vec.mjs";
import * as MyStringDemo from "./MyString.mjs";
export * as MyStringDemo from "./MyString.mjs";
import * as MyOpaqueEnumDemo from "./MyOpaqueEnum.mjs";
export * as MyOpaqueEnumDemo from "./MyOpaqueEnum.mjs";
import * as OpaqueDemo from "./Opaque.mjs";
export * as OpaqueDemo from "./Opaque.mjs";
import * as Utf16WrapDemo from "./Utf16Wrap.mjs";
export * as Utf16WrapDemo from "./Utf16Wrap.mjs";



let termini = Object.assign({
    "CyclicStructA.cyclicOut": {
        func: CyclicStructADemo.cyclicOut,
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.cyclicOut",
        parameters: [
            
            {
                name: "CyclicStructA:A:Field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructA.doubleCyclicOut": {
        func: CyclicStructADemo.doubleCyclicOut,
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.doubleCyclicOut",
        parameters: [
            
            {
                name: "CyclicStructA:A:Field",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "CyclicStructA:A:Field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructA.getterOut": {
        func: CyclicStructADemo.getterOut,
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.getterOut",
        parameters: [
            
            {
                name: "CyclicStructA:A:Field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructC.cyclicOut": {
        func: CyclicStructCDemo.cyclicOut,
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructC.cyclicOut",
        parameters: [
            
            {
                name: "CyclicStructC:A:A:Field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionString.write": {
        func: OptionStringDemo.write,
        // For avoiding webpacking minifying issues:
        funcName: "OptionString.write",
        parameters: [
            
            {
                name: "OptionString:DiplomatStr",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Float64Vec.toString": {
        func: Float64VecDemo.toString,
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.toString",
        parameters: [
            
            {
                name: "Float64Vec:V",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "MyString.getStr": {
        func: MyStringDemo.getStr,
        // For avoiding webpacking minifying issues:
        funcName: "MyString.getStr",
        parameters: [
            
            {
                name: "MyString:V",
                type: "string",
                typeUse: "string"
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
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "MyOpaqueEnum.toString": {
        func: MyOpaqueEnumDemo.toString,
        // For avoiding webpacking minifying issues:
        funcName: "MyOpaqueEnum.toString",
        parameters: [
            
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
                name: "Utf16Wrap:Input",
                type: "string",
                typeUse: "string"
            }
            
        ]
    }
});

export const RenderInfo = {
    "termini": termini
};