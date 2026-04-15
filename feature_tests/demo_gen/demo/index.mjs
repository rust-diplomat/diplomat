import * as somelib from "../../js/api/index.mjs";
export * as somelib from "../../js/api/index.mjs";

const displayBool = (out) => out ? 'true' : 'false';
const displayOrdering = (out) => out == 0 ? '==' : out == 1 ? '>' : '<';
const displayChar = (out) => String.fromCharCode(out);
const displayByte = (out) => '0x' + out.toString(16);
const displayOptionalEnum = (out) => out?.value || 'None';

let termini = Object.assign({
    "RenamedStructWithAttrs.c": {
        func: (selfA, selfB) => somelib.RenamedStructWithAttrs.fromFields({
            a: selfA,
            b: selfB
        }).c,
        // For avoiding webpacking minifying issues:
        funcName: "RenamedStructWithAttrs.c",
        expr: (selfA, selfB) => "somelib.RenamedStructWithAttrs.fromFields({\n    a: selfA,\n    b: selfB\n}).c".replace(/([\( ])selfA([,\) \n])/, '$1' + selfA + '$2').replace(/([\( ])selfB([,\) \n])/, '$1' + selfB + '$2'),
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

    "RenamedTestMacroStruct.testFunc": {
        func: () => somelib.RenamedTestMacroStruct.testFunc(),
        // For avoiding webpacking minifying issues:
        funcName: "RenamedTestMacroStruct.testFunc",
        expr: () => "somelib.RenamedTestMacroStruct.testFunc()",
        parameters: [
            
        ]
    },

    "CyclicStructA.cyclicOut": {
        func: (selfAField) => somelib.CyclicStructA.fromFields({
            a: somelib.CyclicStructB.fromFields({
            field: selfAField
        })
        }).cyclicOut(),
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.cyclicOut",
        expr: (selfAField) => "somelib.CyclicStructA.fromFields({\n    a: somelib.CyclicStructB.fromFields({\n    field: selfAField\n})\n}).cyclicOut()".replace(/([\( ])selfAField([,\) \n])/, '$1' + selfAField + '$2'),
        parameters: [
            
            {
                name: "self_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructA.doubleCyclicOut": {
        func: (selfAField, cyclicStructAAField) => somelib.CyclicStructA.fromFields({
            a: somelib.CyclicStructB.fromFields({
            field: selfAField
        })
        }).doubleCyclicOut(somelib.CyclicStructA.fromFields({
            a: somelib.CyclicStructB.fromFields({
            field: cyclicStructAAField
        })
        })),
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.doubleCyclicOut",
        expr: (selfAField, cyclicStructAAField) => "somelib.CyclicStructA.fromFields({\n    a: somelib.CyclicStructB.fromFields({\n    field: selfAField\n})\n}).doubleCyclicOut(somelib.CyclicStructA.fromFields({\n    a: somelib.CyclicStructB.fromFields({\n    field: cyclicStructAAField\n})\n}))".replace(/([\( ])selfAField([,\) \n])/, '$1' + selfAField + '$2').replace(/([\( ])cyclicStructAAField([,\) \n])/, '$1' + cyclicStructAAField + '$2'),
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
        func: (selfAField) => somelib.CyclicStructA.fromFields({
            a: somelib.CyclicStructB.fromFields({
            field: selfAField
        })
        }).getterOut,
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructA.getterOut",
        expr: (selfAField) => "somelib.CyclicStructA.fromFields({\n    a: somelib.CyclicStructB.fromFields({\n    field: selfAField\n})\n}).getterOut".replace(/([\( ])selfAField([,\) \n])/, '$1' + selfAField + '$2'),
        parameters: [
            
            {
                name: "self_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "CyclicStructC.cyclicOut": {
        func: (selfAAField) => somelib.CyclicStructC.fromFields({
            a: somelib.CyclicStructA.fromFields({
            a: somelib.CyclicStructB.fromFields({
            field: selfAAField
        })
        })
        }).cyclicOut(),
        // For avoiding webpacking minifying issues:
        funcName: "CyclicStructC.cyclicOut",
        expr: (selfAAField) => "somelib.CyclicStructC.fromFields({\n    a: somelib.CyclicStructA.fromFields({\n    a: somelib.CyclicStructB.fromFields({\n    field: selfAAField\n})\n})\n}).cyclicOut()".replace(/([\( ])selfAAField([,\) \n])/, '$1' + selfAAField + '$2'),
        parameters: [
            
            {
                name: "self_a_a_field",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "MyStruct.intoA": {
        func: (selfA, selfB, selfC, selfD, selfE, selfF, selfG) => somelib.MyStruct.fromFields({
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
        expr: (selfA, selfB, selfC, selfD, selfE, selfF, selfG) => "somelib.MyStruct.fromFields({\n    a: selfA,\n    b: selfB,\n    c: selfC,\n    d: selfD,\n    e: selfE,\n    f: selfF,\n    g: selfG\n}).intoA()".replace(/([\( ])selfA([,\) \n])/, '$1' + selfA + '$2').replace(/([\( ])selfB([,\) \n])/, '$1' + selfB + '$2').replace(/([\( ])selfC([,\) \n])/, '$1' + selfC + '$2').replace(/([\( ])selfD([,\) \n])/, '$1' + selfD + '$2').replace(/([\( ])selfE([,\) \n])/, '$1' + selfE + '$2').replace(/([\( ])selfF([,\) \n])/, '$1' + selfF + '$2').replace(/([\( ])selfG([,\) \n])/, '$1' + selfG + '$2'),
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
                typeUse: "enumerator",
                values: ["A", "B", "C", "D", "E", "F"]
            }
            
        ]
    },

    "StructWithSlices.returnLast": {
        func: (selfFirst, selfSecond) => somelib.StructWithSlices.fromFields({
            first: selfFirst,
            second: selfSecond
        }).returnLast(),
        // For avoiding webpacking minifying issues:
        funcName: "StructWithSlices.returnLast",
        expr: (selfFirst, selfSecond) => "somelib.StructWithSlices.fromFields({\n    first: selfFirst,\n    second: selfSecond\n}).returnLast()".replace(/([\( ])selfFirst([,\) \n])/, '$1' + selfFirst + '$2').replace(/([\( ])selfSecond([,\) \n])/, '$1' + selfSecond + '$2'),
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

    "TupleStruct.takesStAsTuple": {
        func: (aX, aY, aStA, aStB, aStC, aStD, aStE, aStF, aStG) => somelib.TupleStruct.takesStAsTuple(somelib.TupleStruct.fromFields({
            x: aX,
            y: aY,
            st: somelib.MyStruct.fromFields({
            a: aStA,
            b: aStB,
            c: aStC,
            d: aStD,
            e: aStE,
            f: aStF,
            g: aStG
        }),
            op: new somelib.Opaque()
        })),
        // For avoiding webpacking minifying issues:
        funcName: "TupleStruct.takesStAsTuple",
        expr: (aX, aY, aStA, aStB, aStC, aStD, aStE, aStF, aStG) => "somelib.TupleStruct.takesStAsTuple(somelib.TupleStruct.fromFields({\n    x: aX,\n    y: aY,\n    st: somelib.MyStruct.fromFields({\n    a: aStA,\n    b: aStB,\n    c: aStC,\n    d: aStD,\n    e: aStE,\n    f: aStF,\n    g: aStG\n}),\n    op: new somelib.Opaque()\n}))".replace(/([\( ])aX([,\) \n])/, '$1' + aX + '$2').replace(/([\( ])aY([,\) \n])/, '$1' + aY + '$2').replace(/([\( ])aStA([,\) \n])/, '$1' + aStA + '$2').replace(/([\( ])aStB([,\) \n])/, '$1' + aStB + '$2').replace(/([\( ])aStC([,\) \n])/, '$1' + aStC + '$2').replace(/([\( ])aStD([,\) \n])/, '$1' + aStD + '$2').replace(/([\( ])aStE([,\) \n])/, '$1' + aStE + '$2').replace(/([\( ])aStF([,\) \n])/, '$1' + aStF + '$2').replace(/([\( ])aStG([,\) \n])/, '$1' + aStG + '$2'),
        parameters: [
            
            {
                name: "a_x",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "a_y",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "a_st_a",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "a_st_b",
                type: "boolean",
                typeUse: "boolean"
            },
            
            {
                name: "a_st_c",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "a_st_d",
                type: "bigint",
                typeUse: "bigint"
            },
            
            {
                name: "a_st_e",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "a_st_f",
                type: "codepoint",
                typeUse: "codepoint"
            },
            
            {
                name: "a_st_g",
                type: "MyEnum",
                typeUse: "enumerator",
                values: ["A", "B", "C", "D", "E", "F"]
            }
            
        ]
    },

    "TupleStruct.takesContaining": {
        func: (cInnerX, cInnerY, cInnerStA, cInnerStB, cInnerStC, cInnerStD, cInnerStE, cInnerStF, cInnerStG) => somelib.TupleStruct.takesContaining(somelib.ContainingTuple.fromFields({
            inner: somelib.TupleStruct.fromFields({
            x: cInnerX,
            y: cInnerY,
            st: somelib.MyStruct.fromFields({
            a: cInnerStA,
            b: cInnerStB,
            c: cInnerStC,
            d: cInnerStD,
            e: cInnerStE,
            f: cInnerStF,
            g: cInnerStG
        }),
            op: new somelib.Opaque()
        })
        })),
        // For avoiding webpacking minifying issues:
        funcName: "TupleStruct.takesContaining",
        expr: (cInnerX, cInnerY, cInnerStA, cInnerStB, cInnerStC, cInnerStD, cInnerStE, cInnerStF, cInnerStG) => "somelib.TupleStruct.takesContaining(somelib.ContainingTuple.fromFields({\n    inner: somelib.TupleStruct.fromFields({\n    x: cInnerX,\n    y: cInnerY,\n    st: somelib.MyStruct.fromFields({\n    a: cInnerStA,\n    b: cInnerStB,\n    c: cInnerStC,\n    d: cInnerStD,\n    e: cInnerStE,\n    f: cInnerStF,\n    g: cInnerStG\n}),\n    op: new somelib.Opaque()\n})\n}))".replace(/([\( ])cInnerX([,\) \n])/, '$1' + cInnerX + '$2').replace(/([\( ])cInnerY([,\) \n])/, '$1' + cInnerY + '$2').replace(/([\( ])cInnerStA([,\) \n])/, '$1' + cInnerStA + '$2').replace(/([\( ])cInnerStB([,\) \n])/, '$1' + cInnerStB + '$2').replace(/([\( ])cInnerStC([,\) \n])/, '$1' + cInnerStC + '$2').replace(/([\( ])cInnerStD([,\) \n])/, '$1' + cInnerStD + '$2').replace(/([\( ])cInnerStE([,\) \n])/, '$1' + cInnerStE + '$2').replace(/([\( ])cInnerStF([,\) \n])/, '$1' + cInnerStF + '$2').replace(/([\( ])cInnerStG([,\) \n])/, '$1' + cInnerStG + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "c_inner_x",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "c_inner_y",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "c_inner_st_a",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "c_inner_st_b",
                type: "boolean",
                typeUse: "boolean"
            },
            
            {
                name: "c_inner_st_c",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "c_inner_st_d",
                type: "bigint",
                typeUse: "bigint"
            },
            
            {
                name: "c_inner_st_e",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "c_inner_st_f",
                type: "codepoint",
                typeUse: "codepoint"
            },
            
            {
                name: "c_inner_st_g",
                type: "MyEnum",
                typeUse: "enumerator",
                values: ["A", "B", "C", "D", "E", "F"]
            }
            
        ]
    },

    "AttrOpaque1Renamed.macTest": {
        func: () => somelib.AttrOpaque1Renamed.macTest(),
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.macTest",
        expr: () => "somelib.AttrOpaque1Renamed.macTest()",
        parameters: [
            
        ]
    },

    "AttrOpaque1Renamed.hello": {
        func: () => somelib.AttrOpaque1Renamed.hello(),
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.hello",
        expr: () => "somelib.AttrOpaque1Renamed.hello()",
        parameters: [
            
        ]
    },

    "AttrOpaque1Renamed.methodRenamed": {
        func: () => new somelib.AttrOpaque1Renamed().methodRenamed,
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.methodRenamed",
        expr: () => "new somelib.AttrOpaque1Renamed().methodRenamed",
        parameters: [
            
        ]
    },

    "AttrOpaque1Renamed.abirenamed": {
        func: () => new somelib.AttrOpaque1Renamed().abirenamed,
        // For avoiding webpacking minifying issues:
        funcName: "AttrOpaque1Renamed.abirenamed",
        expr: () => "new somelib.AttrOpaque1Renamed().abirenamed",
        parameters: [
            
        ]
    },

    "RenamedMixinTest.hello": {
        func: () => somelib.RenamedMixinTest.hello(),
        // For avoiding webpacking minifying issues:
        funcName: "RenamedMixinTest.hello",
        expr: () => "somelib.RenamedMixinTest.hello()",
        parameters: [
            
        ]
    },

    "RenamedVectorTest.len": {
        func: () => new somelib.RenamedVectorTest().len,
        // For avoiding webpacking minifying issues:
        funcName: "RenamedVectorTest.len",
        expr: () => "new somelib.RenamedVectorTest().len",
        parameters: [
            
        ]
    },

    "RenamedVectorTest.get": {
        func: (idx) => new somelib.RenamedVectorTest().get(idx),
        // For avoiding webpacking minifying issues:
        funcName: "RenamedVectorTest.get",
        expr: (idx) => "new somelib.RenamedVectorTest().get(idx)".replace(/([\( ])idx([,\) \n])/, '$1' + idx + '$2'),
        parameters: [
            
            {
                name: "idx",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OpaqueThinVec.len": {
        func: (selfA, selfB, selfC) => new somelib.OpaqueThinVec(selfA, selfB, selfC).len(),
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueThinVec.len",
        expr: (selfA, selfB, selfC) => "new somelib.OpaqueThinVec(selfA, selfB, selfC).len()".replace(/([\( ])selfA([,\) \n])/, '$1' + selfA + '$2').replace(/([\( ])selfB([,\) \n])/, '$1' + selfB + '$2').replace(/([\( ])selfC([,\) \n])/, '$1' + selfC + '$2'),
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
            },
            
            {
                name: "self_c",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "OptionOpaque.optionIsize": {
        func: (selfI) => somelib.OptionOpaque.new_(selfI).optionIsize(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionIsize",
        expr: (selfI) => "somelib.OptionOpaque.new_(selfI).optionIsize()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionUsize": {
        func: (selfI) => somelib.OptionOpaque.new_(selfI).optionUsize(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionUsize",
        expr: (selfI) => "somelib.OptionOpaque.new_(selfI).optionUsize()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionI32": {
        func: (selfI) => somelib.OptionOpaque.new_(selfI).optionI32(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionI32",
        expr: (selfI) => "somelib.OptionOpaque.new_(selfI).optionI32()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionU32": {
        func: (selfI) => somelib.OptionOpaque.new_(selfI).optionU32(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionU32",
        expr: (selfI) => "somelib.OptionOpaque.new_(selfI).optionU32()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.optionOpaqueArgument": {
        func: (argI) => somelib.OptionOpaque.optionOpaqueArgument(somelib.OptionOpaque.new_(argI)),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.optionOpaqueArgument",
        expr: (argI) => "somelib.OptionOpaque.optionOpaqueArgument(somelib.OptionOpaque.new_(argI))".replace(/([\( ])argI([,\) \n])/, '$1' + argI + '$2'),
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
        func: (arg, sentinel) => somelib.OptionOpaque.acceptsOptionU8(arg, sentinel),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsOptionU8",
        expr: (arg, sentinel) => "somelib.OptionOpaque.acceptsOptionU8(arg, sentinel)".replace(/([\( ])arg([,\) \n])/, '$1' + arg + '$2').replace(/([\( ])sentinel([,\) \n])/, '$1' + sentinel + '$2'),
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
        func: (arg, sentinel) => somelib.OptionOpaque.acceptsOptionEnum(arg, sentinel),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsOptionEnum",
        expr: (arg, sentinel) => "somelib.OptionOpaque.acceptsOptionEnum(arg, sentinel)".replace(/([\( ])arg([,\) \n])/, '$1' + arg + '$2').replace(/([\( ])sentinel([,\) \n])/, '$1' + sentinel + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "arg",
                type: "OptionEnum",
                typeUse: "enumerator",
                values: ["Foo", "Bar", "Baz"]
            },
            
            {
                name: "sentinel",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionOpaque.acceptsMultipleOptionEnum": {
        func: (sentinel1, arg1, arg2, arg3, sentinel2) => somelib.OptionOpaque.acceptsMultipleOptionEnum(sentinel1, arg1, arg2, arg3, sentinel2),
        // For avoiding webpacking minifying issues:
        funcName: "OptionOpaque.acceptsMultipleOptionEnum",
        expr: (sentinel1, arg1, arg2, arg3, sentinel2) => "somelib.OptionOpaque.acceptsMultipleOptionEnum(sentinel1, arg1, arg2, arg3, sentinel2)".replace(/([\( ])sentinel1([,\) \n])/, '$1' + sentinel1 + '$2').replace(/([\( ])arg1([,\) \n])/, '$1' + arg1 + '$2').replace(/([\( ])arg2([,\) \n])/, '$1' + arg2 + '$2').replace(/([\( ])arg3([,\) \n])/, '$1' + arg3 + '$2').replace(/([\( ])sentinel2([,\) \n])/, '$1' + sentinel2 + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "sentinel1",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "arg1",
                type: "OptionEnum",
                typeUse: "enumerator",
                values: ["Foo", "Bar", "Baz"]
            },
            
            {
                name: "arg2",
                type: "OptionEnum",
                typeUse: "enumerator",
                values: ["Foo", "Bar", "Baz"]
            },
            
            {
                name: "arg3",
                type: "OptionEnum",
                typeUse: "enumerator",
                values: ["Foo", "Bar", "Baz"]
            },
            
            {
                name: "sentinel2",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "OptionString.write": {
        func: (selfDiplomatStr) => somelib.OptionString.new_(selfDiplomatStr).write(),
        // For avoiding webpacking minifying issues:
        funcName: "OptionString.write",
        expr: (selfDiplomatStr) => "somelib.OptionString.new_(selfDiplomatStr).write()".replace(/([\( ])selfDiplomatStr([,\) \n])/, '$1' + selfDiplomatStr + '$2'),
        parameters: [
            
            {
                name: "self_diplomatStr",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ResultOpaque.newInt": {
        func: (i) => somelib.ResultOpaque.newInt(i),
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.newInt",
        expr: (i) => "somelib.ResultOpaque.newInt(i)".replace(/([\( ])i([,\) \n])/, '$1' + i + '$2'),
        parameters: [
            
            {
                name: "i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "ResultOpaque.newInEnumErr": {
        func: (i) => somelib.ResultOpaque.newInEnumErr(i),
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.newInEnumErr",
        expr: (i) => "somelib.ResultOpaque.newInEnumErr(i)".replace(/([\( ])i([,\) \n])/, '$1' + i + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "ResultOpaque.stringifyError": {
        func: (selfI) => new somelib.ResultOpaque(selfI).stringifyError(),
        // For avoiding webpacking minifying issues:
        funcName: "ResultOpaque.stringifyError",
        expr: (selfI) => "new somelib.ResultOpaque(selfI).stringifyError()".replace(/([\( ])selfI([,\) \n])/, '$1' + selfI + '$2'),
        parameters: [
            
            {
                name: "self_i",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "Float64Vec.toString": {
        func: (selfV) => new somelib.Float64Vec(selfV).toString(),
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.toString",
        expr: (selfV) => "new somelib.Float64Vec(selfV).toString()".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "Array<number>",
                typeUse: "Array<number>"
            }
            
        ]
    },

    "Float64Vec.get": {
        func: (selfV, i) => new somelib.Float64Vec(selfV).get(i),
        // For avoiding webpacking minifying issues:
        funcName: "Float64Vec.get",
        expr: (selfV, i) => "new somelib.Float64Vec(selfV).get(i)".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2').replace(/([\( ])i([,\) \n])/, '$1' + i + '$2'),
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
        func: (selfV) => new somelib.MyString(selfV).str,
        // For avoiding webpacking minifying issues:
        funcName: "MyString.str",
        expr: (selfV) => "new somelib.MyString(selfV).str".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "MyString.stringTransform": {
        func: (foo) => somelib.MyString.stringTransform(foo),
        // For avoiding webpacking minifying issues:
        funcName: "MyString.stringTransform",
        expr: (foo) => "somelib.MyString.stringTransform(foo)".replace(/([\( ])foo([,\) \n])/, '$1' + foo + '$2'),
        parameters: [
            
            {
                name: "foo",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "MyOpaqueEnum.toString": {
        func: () => somelib.MyOpaqueEnum.new_().toString(),
        // For avoiding webpacking minifying issues:
        funcName: "MyOpaqueEnum.toString",
        expr: () => "somelib.MyOpaqueEnum.new_().toString()",
        parameters: [
            
        ]
    },

    "Opaque.getDebugStr": {
        func: () => new somelib.Opaque().getDebugStr(),
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.getDebugStr",
        expr: () => "new somelib.Opaque().getDebugStr()",
        parameters: [
            
        ]
    },

    "Opaque.returnsUsize": {
        func: () => somelib.Opaque.returnsUsize(),
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.returnsUsize",
        expr: () => "somelib.Opaque.returnsUsize()",
        parameters: [
            
        ]
    },

    "Opaque.cmp": {
        func: () => somelib.Opaque.cmp(),
        // For avoiding webpacking minifying issues:
        funcName: "Opaque.cmp",
        expr: () => "somelib.Opaque.cmp()",
        display: displayOrdering,
        parameters: [
            
        ]
    },

    "OpaqueMutexedString.getLenAndAdd": {
        func: (selfNumber, other) => somelib.OpaqueMutexedString.fromUsize(selfNumber).getLenAndAdd(other),
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueMutexedString.getLenAndAdd",
        expr: (selfNumber, other) => "somelib.OpaqueMutexedString.fromUsize(selfNumber).getLenAndAdd(other)".replace(/([\( ])selfNumber([,\) \n])/, '$1' + selfNumber + '$2').replace(/([\( ])other([,\) \n])/, '$1' + other + '$2'),
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
        func: (selfNumber, input) => somelib.OpaqueMutexedString.fromUsize(selfNumber).toUnsignedFromUnsigned(input),
        // For avoiding webpacking minifying issues:
        funcName: "OpaqueMutexedString.toUnsignedFromUnsigned",
        expr: (selfNumber, input) => "somelib.OpaqueMutexedString.fromUsize(selfNumber).toUnsignedFromUnsigned(input)".replace(/([\( ])selfNumber([,\) \n])/, '$1' + selfNumber + '$2').replace(/([\( ])input([,\) \n])/, '$1' + input + '$2'),
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
        func: (selfInput) => new somelib.Utf16Wrap(selfInput).getDebugStr(),
        // For avoiding webpacking minifying issues:
        funcName: "Utf16Wrap.getDebugStr",
        expr: (selfInput) => "new somelib.Utf16Wrap(selfInput).getDebugStr()".replace(/([\( ])selfInput([,\) \n])/, '$1' + selfInput + '$2'),
        parameters: [
            
            {
                name: "self_input",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DefaultEnum.new_": {
        func: () => new somelib.DefaultEnum(),
        // For avoiding webpacking minifying issues:
        funcName: "DefaultEnum.new_",
        expr: () => "new somelib.DefaultEnum()",
        display: displayOptionalEnum,
        parameters: [
            
        ]
    },

    "MyEnum.intoValue": {
        func: (self) => new somelib.MyEnum(self).intoValue(),
        // For avoiding webpacking minifying issues:
        funcName: "MyEnum.intoValue",
        expr: (self) => "new somelib.MyEnum(self).intoValue()".replace(/([\( ])self([,\) \n])/, '$1' + self + '$2'),
        parameters: [
            
            {
                name: "self",
                type: "MyEnum",
                typeUse: "enumerator",
                values: ["A", "B", "C", "D", "E", "F"]
            }
            
        ]
    },

    "MyEnum.getA": {
        func: () => somelib.MyEnum.getA(),
        // For avoiding webpacking minifying issues:
        funcName: "MyEnum.getA",
        expr: () => "somelib.MyEnum.getA()",
        display: displayOptionalEnum,
        parameters: [
            
        ]
    }
});

export const RenderInfo = {
    "termini": termini
};