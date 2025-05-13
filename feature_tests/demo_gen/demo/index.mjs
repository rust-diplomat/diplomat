export * as lib from "../../js/api/index.mjs";
import * as RenamedStructWithAttrsDemo from "./RenamedStructWithAttrs.mjs";
export * as RenamedStructWithAttrsDemo from "./RenamedStructWithAttrs.mjs";
import * as CyclicStructADemo from "./CyclicStructA.mjs";
export * as CyclicStructADemo from "./CyclicStructA.mjs";
import * as CyclicStructCDemo from "./CyclicStructC.mjs";
export * as CyclicStructCDemo from "./CyclicStructC.mjs";
import * as MyStructDemo from "./MyStruct.mjs";
export * as MyStructDemo from "./MyStruct.mjs";
import * as StructWithSlicesDemo from "./StructWithSlices.mjs";
export * as StructWithSlicesDemo from "./StructWithSlices.mjs";
import * as AttrOpaque1RenamedDemo from "./AttrOpaque1Renamed.mjs";
export * as AttrOpaque1RenamedDemo from "./AttrOpaque1Renamed.mjs";
import * as OpaqueThinVecDemo from "./OpaqueThinVec.mjs";
export * as OpaqueThinVecDemo from "./OpaqueThinVec.mjs";
import * as OptionOpaqueDemo from "./OptionOpaque.mjs";
export * as OptionOpaqueDemo from "./OptionOpaque.mjs";
import * as OptionStringDemo from "./OptionString.mjs";
export * as OptionStringDemo from "./OptionString.mjs";
import * as ResultOpaqueDemo from "./ResultOpaque.mjs";
export * as ResultOpaqueDemo from "./ResultOpaque.mjs";
import * as Float64VecDemo from "./Float64Vec.mjs";
export * as Float64VecDemo from "./Float64Vec.mjs";
import * as MyStringDemo from "./MyString.mjs";
export * as MyStringDemo from "./MyString.mjs";
import * as MyOpaqueEnumDemo from "./MyOpaqueEnum.mjs";
export * as MyOpaqueEnumDemo from "./MyOpaqueEnum.mjs";
import * as OpaqueDemo from "./Opaque.mjs";
export * as OpaqueDemo from "./Opaque.mjs";
import * as OpaqueMutexedStringDemo from "./OpaqueMutexedString.mjs";
export * as OpaqueMutexedStringDemo from "./OpaqueMutexedString.mjs";
import * as Utf16WrapDemo from "./Utf16Wrap.mjs";
export * as Utf16WrapDemo from "./Utf16Wrap.mjs";
import * as DefaultEnumDemo from "./DefaultEnum.mjs";
export * as DefaultEnumDemo from "./DefaultEnum.mjs";
import * as MyEnumDemo from "./MyEnum.mjs";
export * as MyEnumDemo from "./MyEnum.mjs";



let termini = Object.assign({
    "RenamedStructWithAttrs.c": {
        func: RenamedStructWithAttrsDemo.c,
        // For avoiding webpacking minifying issues:
        funcName: "RenamedStructWithAttrs.c",
        parameters: [
            
            {
                name: "RenamedStructWithAttrs:A",
                type: "boolean",
                typeUse: "boolean"
            },
            
            {
                name: "RenamedStructWithAttrs:B",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

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

    "MyStruct.intoA": {
        func: MyStructDemo.intoA,
        // For avoiding webpacking minifying issues:
        funcName: "MyStruct.intoA",
        parameters: [
            
            {
                name: "MyStruct:A",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "MyStruct:B",
                type: "boolean",
                typeUse: "boolean"
            },
            
            {
                name: "MyStruct:C",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "MyStruct:D",
                type: "bigint",
                typeUse: "bigint"
            },
            
            {
                name: "MyStruct:E",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "MyStruct:F",
                type: "codepoint",
                typeUse: "codepoint"
            },
            
            {
                name: "MyStruct:G",
                type: "MyEnum",
                typeUse: "enumerator"
            }
            
        ]
    },

    "StructWithSlices.returnLast": {
        func: StructWithSlicesDemo.returnLast,
        // For avoiding webpacking minifying issues:
        funcName: "StructWithSlices.returnLast",
        parameters: [
            
            {
                name: "StructWithSlices:First",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "StructWithSlices:Second",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "AttrOpaque1Renamed.methodRenamed": {
        func: AttrOpaque1RenamedDemo.methodRenamed,
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.methodRenamed",
        parameters: [
            
        ]
    },

    "AttrOpaque1Renamed.abirenamed": {
        func: AttrOpaque1RenamedDemo.abirenamed,
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.abirenamed",
        parameters: [
            
        ]
    },

    "OpaqueThinVec.len": {
        func: OpaqueThinVecDemo.len,
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueThinVec.len",
        parameters: [
            
            {
                name: "OpaqueThinVec:A",
                type: "Array<number>",
                typeUse: "Array<number>"
            },
            
            {
                name: "OpaqueThinVec:B",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "OptionOpaque.optionIsize": {
        func: OptionOpaqueDemo.optionIsize,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionIsize",
        parameters: [
            
            {
                name: "OptionOpaque:I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionUsize": {
        func: OptionOpaqueDemo.optionUsize,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionUsize",
        parameters: [
            
            {
                name: "OptionOpaque:I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionI32": {
        func: OptionOpaqueDemo.optionI32,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionI32",
        parameters: [
            
            {
                name: "OptionOpaque:I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionU32": {
        func: OptionOpaqueDemo.optionU32,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionU32",
        parameters: [
            
            {
                name: "OptionOpaque:I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionOpaqueArgument": {
        func: OptionOpaqueDemo.optionOpaqueArgument,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionOpaqueArgument",
        parameters: [
            
            {
                name: "Arg:I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.acceptsOptionU8": {
        func: OptionOpaqueDemo.acceptsOptionU8,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsOptionU8",
        parameters: [
            
            {
                name: "Arg",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Sentinel",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.acceptsOptionEnum": {
        func: OptionOpaqueDemo.acceptsOptionEnum,
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsOptionEnum",
        parameters: [
            
            {
                name: "Arg",
                type: "OptionEnum",
                typeUse: "enumerator"
            },
            
            {
                name: "Sentinel",
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

    "ResultOpaque.newInt": {
        func: ResultOpaqueDemo.newInt,
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.newInt",
        parameters: [
            
            {
                name: "I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "ResultOpaque.newInEnumErr": {
        func: ResultOpaqueDemo.newInEnumErr,
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.newInEnumErr",
        parameters: [
            
            {
                name: "I",
                type: "number",
                typeUse: "number"
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

    "Float64Vec.get": {
        func: Float64VecDemo.get,
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.get",
        parameters: [
            
            {
                name: "Float64Vec:V",
                type: "Array<number>",
                typeUse: "Array<number>"
            },
            
            {
                name: "I",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "MyString.str": {
        func: MyStringDemo.str,
        // For avoiding webpacking minifying issues:
        funcName: "MyString.str",
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

    "Opaque.returnsUsize": {
        func: OpaqueDemo.returnsUsize,
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.returnsUsize",
        parameters: [
            
        ]
    },

    "Opaque.cmp": {
        func: OpaqueDemo.cmp,
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.cmp",
        parameters: [
            
        ]
    },

    "OpaqueMutexedString.getLenAndAdd": {
        func: OpaqueMutexedStringDemo.getLenAndAdd,
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueMutexedString.getLenAndAdd",
        parameters: [
            
            {
                name: "OpaqueMutexedString:Number",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Other",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OpaqueMutexedString.toUnsignedFromUnsigned": {
        func: OpaqueMutexedStringDemo.toUnsignedFromUnsigned,
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueMutexedString.toUnsignedFromUnsigned",
        parameters: [
            
            {
                name: "OpaqueMutexedString:Number",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Input",
                type: "number",
                typeUse: "number"
            }
            
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
    },

    "DefaultEnum.new_": {
        func: DefaultEnumDemo.new_,
        // For avoiding webpacking minifying issues:
        funcName: "DefaultEnum.new_",
        parameters: [
            
        ]
    },

    "MyEnum.intoValue": {
        func: MyEnumDemo.intoValue,
        // For avoiding webpacking minifying issues:
        funcName: "MyEnum.intoValue",
        parameters: [
            
            {
                name: "MyEnum",
                type: "MyEnum",
                typeUse: "enumerator"
            }
            
        ]
    },

    "MyEnum.getA": {
        func: MyEnumDemo.getA,
        // For avoiding webpacking minifying issues:
        funcName: "MyEnum.getA",
        parameters: [
            
        ]
    }
});

export const RenderInfo = {
    "termini": termini
};