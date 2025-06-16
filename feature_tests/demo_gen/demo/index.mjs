export * as lib from "../../js/api/index.mjs";
import { AttrOpaque1Renamed } from "../../js/api/index.mjs"
import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
import { DefaultEnum } from "../../js/api/index.mjs"
import { Float64Vec } from "../../js/api/index.mjs"
import { MyEnum } from "../../js/api/index.mjs"
import { MyOpaqueEnum } from "../../js/api/index.mjs"
import { MyString } from "../../js/api/index.mjs"
import { MyStruct } from "../../js/api/index.mjs"
import { Opaque } from "../../js/api/index.mjs"
import { OpaqueMutexedString } from "../../js/api/index.mjs"
import { OpaqueThinVec } from "../../js/api/index.mjs"
import { OptionOpaque } from "../../js/api/index.mjs"
import { OptionString } from "../../js/api/index.mjs"
import { RenamedStructWithAttrs } from "../../js/api/index.mjs"
import { ResultOpaque } from "../../js/api/index.mjs"
import { StructWithSlices } from "../../js/api/index.mjs"
import { Utf16Wrap } from "../../js/api/index.mjs"

const displayBool = (out) => out ? 'true' : 'false';
const displayOrdering = (out) => out == 0 ? '==' : out == 1 ? '>' : '<';
const displayChar = (out) => String.fromCharCode(out);
const displayByte = (out) => '0x' + out.toString(16);
const displayOptionalEnum = (out) => out?.value || 'None';

let termini = Object.assign({
    "RenamedStructWithAttrs.c": {
        func: (selfA, selfB) => RenamedStructWithAttrs.fromFields({
            a: selfA,
            b: selfB
        }).c,
        // For avoiding webpacking minifying issues:
        funcName: "RenamedStructWithAttrs.c",
        expr: (selfA, selfB) => "RenamedStructWithAttrs.fromFields({\n    a: selfA,\n    b: selfB\n}).c".replace(/([\( ])selfA([,\) \n])/, '$1' + selfA + '$2').replace(/([\( ])selfB([,\) \n])/, '$1' + selfB + '$2'),
        parameters: [
            
            {
                name: "self_a",
                type: "boolean",
                typeUse: "boolean"
            },
            
            {
                name: "self_b",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructA.cyclicOut": {
        func: (selfAField) => CyclicStructA.fromFields({
            a: CyclicStructB.fromFields({
            field: selfAField
        })
        }).cyclicOut(),
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.cyclicOut",
        expr: (selfAField) => "CyclicStructA.fromFields({\n    a: CyclicStructB.fromFields({\n    field: selfAField\n})\n}).cyclicOut()".replace(/([\( ])selfAField([,\) \n])/, '$1' + selfAField + '$2'),
        parameters: [
            
            {
                name: "self_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructA.doubleCyclicOut": {
        func: (selfAField, cyclicStructAAField) => CyclicStructA.fromFields({
            a: CyclicStructB.fromFields({
            field: selfAField
        })
        }).doubleCyclicOut(CyclicStructA.fromFields({
            a: CyclicStructB.fromFields({
            field: cyclicStructAAField
        })
        })),
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.doubleCyclicOut",
        expr: (selfAField, cyclicStructAAField) => "CyclicStructA.fromFields({\n    a: CyclicStructB.fromFields({\n    field: selfAField\n})\n}).doubleCyclicOut(CyclicStructA.fromFields({\n    a: CyclicStructB.fromFields({\n    field: cyclicStructAAField\n})\n}))".replace(/([\( ])selfAField([,\) \n])/, '$1' + selfAField + '$2').replace(/([\( ])cyclicStructAAField([,\) \n])/, '$1' + cyclicStructAAField + '$2'),
        parameters: [
            
            {
                name: "self_a_field",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "cyclicStructA_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructA.getterOut": {
        func: (selfAField) => CyclicStructA.fromFields({
            a: CyclicStructB.fromFields({
            field: selfAField
        })
        }).getterOut,
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.getterOut",
        expr: (selfAField) => "CyclicStructA.fromFields({\n    a: CyclicStructB.fromFields({\n    field: selfAField\n})\n}).getterOut".replace(/([\( ])selfAField([,\) \n])/, '$1' + selfAField + '$2'),
        parameters: [
            
            {
                name: "self_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructC.cyclicOut": {
        func: (selfAAField) => CyclicStructC.fromFields({
            a: CyclicStructA.fromFields({
            a: CyclicStructB.fromFields({
            field: selfAAField
        })
        })
        }).cyclicOut(),
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructC.cyclicOut",
        expr: (selfAAField) => "CyclicStructC.fromFields({\n    a: CyclicStructA.fromFields({\n    a: CyclicStructB.fromFields({\n    field: selfAAField\n})\n})\n}).cyclicOut()".replace(/([\( ])selfAAField([,\) \n])/, '$1' + selfAAField + '$2'),
        parameters: [
            
            {
                name: "self_a_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "MyStruct.intoA": {
        func: (selfA, selfB, selfC, selfD, selfE, selfF, selfG) => MyStruct.fromFields({
            a: selfA,
            b: selfB,
            c: selfC,
            d: selfD,
            e: selfE,
            f: selfF,
            g: selfG
        }).intoA(),
        // For avoiding webpacking minifying issues:
        funcName: "MyStruct.intoA",
        expr: (selfA, selfB, selfC, selfD, selfE, selfF, selfG) => "MyStruct.fromFields({\n    a: selfA,\n    b: selfB,\n    c: selfC,\n    d: selfD,\n    e: selfE,\n    f: selfF,\n    g: selfG\n}).intoA()".replace(/([\( ])selfA([,\) \n])/, '$1' + selfA + '$2').replace(/([\( ])selfB([,\) \n])/, '$1' + selfB + '$2').replace(/([\( ])selfC([,\) \n])/, '$1' + selfC + '$2').replace(/([\( ])selfD([,\) \n])/, '$1' + selfD + '$2').replace(/([\( ])selfE([,\) \n])/, '$1' + selfE + '$2').replace(/([\( ])selfF([,\) \n])/, '$1' + selfF + '$2').replace(/([\( ])selfG([,\) \n])/, '$1' + selfG + '$2'),
        parameters: [
            
            {
                name: "self_a",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_b",
                type: "boolean",
                typeUse: "boolean"
            },
            
            {
                name: "self_c",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_d",
                type: "bigint",
                typeUse: "bigint"
            },
            
            {
                name: "self_e",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_f",
                type: "codepoint",
                typeUse: "codepoint"
            },
            
            {
                name: "self_g",
                type: "MyEnum",
                typeUse: "enumerator"
            }
            
        ]
    },

    "StructWithSlices.returnLast": {
        func: (selfFirst, selfSecond) => StructWithSlices.fromFields({
            first: selfFirst,
            second: selfSecond
        }).returnLast(),
        // For avoiding webpacking minifying issues:
        funcName: "StructWithSlices.returnLast",
        expr: (selfFirst, selfSecond) => "StructWithSlices.fromFields({\n    first: selfFirst,\n    second: selfSecond\n}).returnLast()".replace(/([\( ])selfFirst([,\) \n])/, '$1' + selfFirst + '$2').replace(/([\( ])selfSecond([,\) \n])/, '$1' + selfSecond + '$2'),
        parameters: [
            
            {
                name: "self_first",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_second",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "AttrOpaque1Renamed.methodRenamed": {
        func: () => new AttrOpaque1Renamed().methodRenamed,
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.methodRenamed",
        expr: () => "new AttrOpaque1Renamed().methodRenamed",
        parameters: [
            
        ]
    },

    "AttrOpaque1Renamed.abirenamed": {
        func: () => new AttrOpaque1Renamed().abirenamed,
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.abirenamed",
        expr: () => "new AttrOpaque1Renamed().abirenamed",
        parameters: [
            
        ]
    },

    "OpaqueThinVec.len": {
        func: (selfA, selfB) => new OpaqueThinVec(selfA, selfB).len(),
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueThinVec.len",
        expr: (selfA, selfB) => "new OpaqueThinVec(selfA, selfB).len()".replace(/([\( ])selfA([,\) \n])/, '$1' + selfA + '$2').replace(/([\( ])selfB([,\) \n])/, '$1' + selfB + '$2'),
        parameters: [
            
            {
                name: "self_a",
                type: "Array<number>",
                typeUse: "Array<number>"
            },
            
            {
                name: "self_b",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "OptionOpaque.optionIsize": {
        func: (selfI) => OptionOpaque.new_(selfI).optionIsize(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionIsize",
        expr: (selfI) => "OptionOpaque.new_(selfI).optionIsize()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionUsize": {
        func: (selfI) => OptionOpaque.new_(selfI).optionUsize(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionUsize",
        expr: (selfI) => "OptionOpaque.new_(selfI).optionUsize()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionI32": {
        func: (selfI) => OptionOpaque.new_(selfI).optionI32(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionI32",
        expr: (selfI) => "OptionOpaque.new_(selfI).optionI32()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionU32": {
        func: (selfI) => OptionOpaque.new_(selfI).optionU32(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionU32",
        expr: (selfI) => "OptionOpaque.new_(selfI).optionU32()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionOpaqueArgument": {
        func: (argI) => OptionOpaque.optionOpaqueArgument(OptionOpaque.new_(argI)),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionOpaqueArgument",
        expr: (argI) => "OptionOpaque.optionOpaqueArgument(OptionOpaque.new_(argI))".replace(/([\( ])argI([,\) \n])/, '$1' + argI + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "arg_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.acceptsOptionU8": {
        func: (arg, sentinel) => OptionOpaque.acceptsOptionU8(arg, sentinel),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsOptionU8",
        expr: (arg, sentinel) => "OptionOpaque.acceptsOptionU8(arg, sentinel)".replace(/([\( ])arg([,\) \n])/, '$1' + arg + '$2').replace(/([\( ])sentinel([,\) \n])/, '$1' + sentinel + '$2'),
        parameters: [
            
            {
                name: "arg",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "sentinel",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.acceptsOptionEnum": {
        func: (arg, sentinel) => OptionOpaque.acceptsOptionEnum(arg, sentinel),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsOptionEnum",
        expr: (arg, sentinel) => "OptionOpaque.acceptsOptionEnum(arg, sentinel)".replace(/([\( ])arg([,\) \n])/, '$1' + arg + '$2').replace(/([\( ])sentinel([,\) \n])/, '$1' + sentinel + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "arg",
                type: "OptionEnum",
                typeUse: "enumerator"
            },
            
            {
                name: "sentinel",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionString.write": {
        func: (selfDiplomatStr) => OptionString.new_(selfDiplomatStr).write(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionString.write",
        expr: (selfDiplomatStr) => "OptionString.new_(selfDiplomatStr).write()".replace(/([\( ])selfDiplomatStr([,\) \n])/, '$1' + selfDiplomatStr + '$2'),
        parameters: [
            
            {
                name: "self_diplomatStr",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ResultOpaque.newInt": {
        func: (i) => ResultOpaque.newInt(i),
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.newInt",
        expr: (i) => "ResultOpaque.newInt(i)".replace(/([\( ])i([,\) \n])/, '$1' + i + '$2'),
        parameters: [
            
            {
                name: "i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "ResultOpaque.newInEnumErr": {
        func: (i) => ResultOpaque.newInEnumErr(i),
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.newInEnumErr",
        expr: (i) => "ResultOpaque.newInEnumErr(i)".replace(/([\( ])i([,\) \n])/, '$1' + i + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "Float64Vec.toString": {
        func: (selfV) => new Float64Vec(selfV).toString(),
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.toString",
        expr: (selfV) => "new Float64Vec(selfV).toString()".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "Float64Vec.get": {
        func: (selfV, i) => new Float64Vec(selfV).get(i),
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.get",
        expr: (selfV, i) => "new Float64Vec(selfV).get(i)".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2').replace(/([\( ])i([,\) \n])/, '$1' + i + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "Array<number>",
                typeUse: "Array<number>"
            },
            
            {
                name: "i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "MyString.str": {
        func: (selfV) => new MyString(selfV).str,
        // For avoiding webpacking minifying issues:
        funcName: "MyString.str",
        expr: (selfV) => "new MyString(selfV).str".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "MyString.stringTransform": {
        func: (foo) => MyString.stringTransform(foo),
        // For avoiding webpacking minifying issues:
        funcName: "MyString.stringTransform",
        expr: (foo) => "MyString.stringTransform(foo)".replace(/([\( ])foo([,\) \n])/, '$1' + foo + '$2'),
        parameters: [
            
            {
                name: "foo",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "MyOpaqueEnum.toString": {
        func: () => MyOpaqueEnum.new_().toString(),
        // For avoiding webpacking minifying issues:
        funcName: "MyOpaqueEnum.toString",
        expr: () => "MyOpaqueEnum.new_().toString()",
        parameters: [
            
        ]
    },

    "Opaque.getDebugStr": {
        func: () => new Opaque().getDebugStr(),
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.getDebugStr",
        expr: () => "new Opaque().getDebugStr()",
        parameters: [
            
        ]
    },

    "Opaque.returnsUsize": {
        func: () => Opaque.returnsUsize(),
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.returnsUsize",
        expr: () => "Opaque.returnsUsize()",
        parameters: [
            
        ]
    },

    "Opaque.cmp": {
        func: () => Opaque.cmp(),
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.cmp",
        expr: () => "Opaque.cmp()",
        display: displayOrdering,
        parameters: [
            
        ]
    },

    "OpaqueMutexedString.getLenAndAdd": {
        func: (selfNumber, other) => OpaqueMutexedString.fromUsize(selfNumber).getLenAndAdd(other),
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueMutexedString.getLenAndAdd",
        expr: (selfNumber, other) => "OpaqueMutexedString.fromUsize(selfNumber).getLenAndAdd(other)".replace(/([\( ])selfNumber([,\) \n])/, '$1' + selfNumber + '$2').replace(/([\( ])other([,\) \n])/, '$1' + other + '$2'),
        parameters: [
            
            {
                name: "self_number",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "other",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OpaqueMutexedString.toUnsignedFromUnsigned": {
        func: (selfNumber, input) => OpaqueMutexedString.fromUsize(selfNumber).toUnsignedFromUnsigned(input),
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueMutexedString.toUnsignedFromUnsigned",
        expr: (selfNumber, input) => "OpaqueMutexedString.fromUsize(selfNumber).toUnsignedFromUnsigned(input)".replace(/([\( ])selfNumber([,\) \n])/, '$1' + selfNumber + '$2').replace(/([\( ])input([,\) \n])/, '$1' + input + '$2'),
        parameters: [
            
            {
                name: "self_number",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "input",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "Utf16Wrap.getDebugStr": {
        func: (selfInput) => new Utf16Wrap(selfInput).getDebugStr(),
        // For avoiding webpacking minifying issues:
        funcName: "Utf16Wrap.getDebugStr",
        expr: (selfInput) => "new Utf16Wrap(selfInput).getDebugStr()".replace(/([\( ])selfInput([,\) \n])/, '$1' + selfInput + '$2'),
        parameters: [
            
            {
                name: "self_input",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DefaultEnum.new_": {
        func: () => new DefaultEnum(),
        // For avoiding webpacking minifying issues:
        funcName: "DefaultEnum.new_",
        expr: () => "new DefaultEnum()",
        display: displayOptionalEnum,
        parameters: [
            
        ]
    },

    "MyEnum.intoValue": {
        func: (self) => new MyEnum(self).intoValue(),
        // For avoiding webpacking minifying issues:
        funcName: "MyEnum.intoValue",
        expr: (self) => "new MyEnum(self).intoValue()".replace(/([\( ])self([,\) \n])/, '$1' + self + '$2'),
        parameters: [
            
            {
                name: "self",
                type: "MyEnum",
                typeUse: "enumerator"
            }
            
        ]
    },

    "MyEnum.getA": {
        func: () => MyEnum.getA(),
        // For avoiding webpacking minifying issues:
        funcName: "MyEnum.getA",
        expr: () => "MyEnum.getA()",
        display: displayOptionalEnum,
        parameters: [
            
        ]
    }
});

export const RenderInfo = {
    "termini": termini
};