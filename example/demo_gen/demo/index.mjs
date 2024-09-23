export * as lib from "../../js/lib/api/index.mjs";
import * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
export * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
import * as FixedDecimalDemo from "./FixedDecimal.mjs";
export * as FixedDecimalDemo from "./FixedDecimal.mjs";


export const RenderInfo = {
    termini: {
        "FixedDecimalFormatter.formatWrite": {
            func: FixedDecimalFormatterDemo.formatWrite,
            // For avoiding webpacking minifying issues:
            funcName: "FixedDecimalFormatter.formatWrite",
            parameters: [
                
                {
                    name: "Locale Name",
                    type: "string"
                },
                
                {
                    name: "ICU4X Fixed Decimal Grouping Strategy",
                    type: "FixedDecimalGroupingStrategy"
                },
                
                {
                    name: "Useless Config (Ignore)",
                    type: "boolean",
                    defaultValue: "true"
                },
                
                {
                    name: "ICU4XFixedDecimal Value",
                    type: "number",
                    defaultValue: "1000"
                }
                
            ]
        },
        
        "FixedDecimal.multiplyPow10": {
            func: FixedDecimalDemo.multiplyPow10,
            // For avoiding webpacking minifying issues:
            funcName: "FixedDecimal.multiplyPow10",
            parameters: [
                
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
                    defaultValue: "1000"
                }
                
            ]
        }
        },
};