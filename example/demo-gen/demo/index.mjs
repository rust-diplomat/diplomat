import * as ICU4XFixedDecimalFormatterDemo from "./ICU4XFixedDecimalFormatter.mjs";
export * as ICU4XFixedDecimalFormatterDemo from "./ICU4XFixedDecimalFormatter.mjs";
import * as ICU4XFixedDecimalDemo from "./ICU4XFixedDecimal.mjs";
export * as ICU4XFixedDecimalDemo from "./ICU4XFixedDecimal.mjs";


export const RenderInfo = {
	termini: [
		{
			func: ICU4XFixedDecimalFormatterDemo.formatWrite,
			// For avoiding webpacking minifying issues:
			funcName: "ICU4XFixedDecimalFormatter.formatWrite",
			parameters: [
				
				{
					name: "Locale Name",
					type: "string"
				},
				
				{
					name: "ICU4X Fixed Decimal Grouping Strategy",
					type: "ICU4XFixedDecimalGroupingStrategy"
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
			func: ICU4XFixedDecimalDemo.toString,
			// For avoiding webpacking minifying issues:
			funcName: "ICU4XFixedDecimal.toString",
			parameters: [
				
				{
					name: "ICU4XFixedDecimal Value",
					type: "number"
				}
				
			]
		}
		],
};