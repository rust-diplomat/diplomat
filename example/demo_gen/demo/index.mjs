import * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
export * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
import * as FixedDecimalDemo from "./FixedDecimal.mjs";
export * as FixedDecimalDemo from "./FixedDecimal.mjs";


export const RenderInfo = {
    termini: [
        {
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
                    type: "boolean"
                },
                
                {
                    name: "ICU4XFixedDecimal Value",
                    type: "number"
                }
                
            ]
        },
        
        {
            func: FixedDecimalDemo.toString,
            // For avoiding webpacking minifying issues:
            funcName: "FixedDecimal.toString",
            parameters: [
                
                {
                    name: "ICU4XFixedDecimal Value",
                    type: "number"
                }
                
            ]
        }
        ],
};