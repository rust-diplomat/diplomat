export * as lib from "../../js/lib/api/index.mjs";
import * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
export * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
import * as FixedDecimalDemo from "./FixedDecimal.mjs";
export * as FixedDecimalDemo from "./FixedDecimal.mjs";

import RenderTerminiFixedDecimal from "./a.mjs";


let termini = Object.assign({
    "FixedDecimalFormatter.formatWrite": {
        func: FixedDecimalFormatterDemo.formatWrite,
        // For avoiding webpacking minifying issues:
        funcName: "FixedDecimalFormatter.formatWrite",
        parameters: [
            
            {
                name: "FixedDecimalFormatter:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "ICU4X Fixed Decimal Grouping Strategy",
                type: "FixedDecimalGroupingStrategy",
                typeUse: "enumerator"
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
        func: FixedDecimalDemo.toString,
        // For avoiding webpacking minifying issues:
        funcName: "FixedDecimal.toString",
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