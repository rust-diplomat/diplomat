import * as somelib from "../../js/lib/api/index.mjs";
export * as somelib from "../../js/lib/api/index.mjs";
import RenderTerminiFixedDecimal from "./a.mjs";

const displayBool = (out) => out ? 'true' : 'false';
const displayOrdering = (out) => out == 0 ? '==' : out == 1 ? '>' : '<';
const displayChar = (out) => String.fromCharCode(out);
const displayByte = (out) => '0x' + out.toString(16);
const displayOptionalEnum = (out) => out?.value || 'None';

let termini = Object.assign({
    "FixedDecimalFormatter.formatWrite": {
        func: (selfLocaleName, selfOptionsGroupingStrategy, selfOptionsSomeOtherConfig, valueV) => somelib.FixedDecimalFormatter.tryNew(new somelib.Locale(selfLocaleName), somelib.DataProvider.newStatic(), somelib.FixedDecimalFormatterOptions.fromFields({
            groupingStrategy: selfOptionsGroupingStrategy,
            someOtherConfig: selfOptionsSomeOtherConfig
        })).formatWrite(new somelib.FixedDecimal(valueV)),
        // For avoiding webpacking minifying issues:
        funcName: "FixedDecimalFormatter.formatWrite",
        expr: (selfLocaleName, selfOptionsGroupingStrategy, selfOptionsSomeOtherConfig, valueV) => "somelib.FixedDecimalFormatter.tryNew(new somelib.Locale(selfLocaleName), somelib.DataProvider.newStatic(), somelib.FixedDecimalFormatterOptions.fromFields({\n    groupingStrategy: selfOptionsGroupingStrategy,\n    someOtherConfig: selfOptionsSomeOtherConfig\n})).formatWrite(new somelib.FixedDecimal(valueV))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfOptionsGroupingStrategy([,\) \n])/, '$1' + selfOptionsGroupingStrategy + '$2').replace(/([\( ])selfOptionsSomeOtherConfig([,\) \n])/, '$1' + selfOptionsSomeOtherConfig + '$2').replace(/([\( ])valueV([,\) \n])/, '$1' + valueV + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "ICU4X Fixed Decimal Grouping Strategy",
                type: "FixedDecimalGroupingStrategy",
                typeUse: "enumerator",
                values: ["Auto", "Never", "Always", "Min2"]
            },
            
            {
                name: "Useless Config (Ignore)",
                type: "boolean",
                typeUse: "boolean",
                defaultValue: "true"
            },
            
            {
                name: "ICU4XFixedDecimal Value",
                type: "number",
                typeUse: "number",
                defaultValue: "1000"
            }
            
        ]
    },

    "FixedDecimal.toString": {
        func: (selfV) => new somelib.FixedDecimal(selfV).toString(),
        // For avoiding webpacking minifying issues:
        funcName: "FixedDecimal.toString",
        expr: (selfV) => "new somelib.FixedDecimal(selfV).toString()".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "ICU4XFixedDecimal Value",
                type: "number",
                typeUse: "number",
                defaultValue: "1000"
            }
            
        ]
    }
}, RenderTerminiFixedDecimal);

export const RenderInfo = {
    "termini": termini
};