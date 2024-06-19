import * as ICU4XFixedDecimalFormatterDemo from "./ICU4XFixedDecimalFormatter.mjs";
export * as ICU4XFixedDecimalFormatterDemo from "./ICU4XFixedDecimalFormatter.mjs";
import * as ICU4XFixedDecimalDemo from "./ICU4XFixedDecimal.mjs";
export * as ICU4XFixedDecimalDemo from "./ICU4XFixedDecimal.mjs";


export const RenderInfo = {
	termini: [
		{
			func: ICU4XFixedDecimalFormatterDemo.formatWrite,
			parameters: [
				
				{
					name: "name",
					type: "string"
				},
				
				{
					name: "grouping_strategy",
					type: "ICU4XFixedDecimalGroupingStrategy"
				},
				
				{
					name: "some_other_config",
					type: "boolean"
				},
				
				{
					name: "v",
					type: "number"
				}
				
			]
		},
		
		{
			func: ICU4XFixedDecimalDemo.toString,
			parameters: [
				
				{
					name: "v",
					type: "number"
				}
				
			]
		}
		],
};